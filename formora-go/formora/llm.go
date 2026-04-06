package formora

import (
	"encoding/json"
	"fmt"
	"strings"
)

// ── JSON Schema types ─────────────────────────────────────────────────────────

// JSONSchemaProperty represents a JSON Schema property definition.
type JSONSchemaProperty struct {
	Type        string               `json:"type"`
	Description string               `json:"description,omitempty"`
	Minimum     *float64             `json:"minimum,omitempty"`
	Maximum     *float64             `json:"maximum,omitempty"`
	Enum        []string             `json:"enum,omitempty"`
	Items       *JSONSchemaProperty  `json:"items,omitempty"`
}

// JSONSchema is a JSON Schema object definition.
type JSONSchema struct {
	Type       string                        `json:"type"`
	Properties map[string]JSONSchemaProperty `json:"properties"`
	Required   []string                      `json:"required,omitempty"`
}

// FormoraTypeError is returned when a form result value has the wrong type.
type FormoraTypeError struct{ msg string }

func (e *FormoraTypeError) Error() string { return e.msg }

// ── formSchemaFields ──────────────────────────────────────────────────────────

type schemaField struct {
	ID        string          `json:"id"`
	FieldType string          `json:"field_type"`
	Required  bool            `json:"required"`
	Placeholder *string       `json:"placeholder"`
	Min       *float64        `json:"min"`
	Max       *float64        `json:"max"`
	Options   []struct {
		Label string `json:"label"`
		Value string `json:"value"`
	} `json:"options"`
}

type rawSchema struct {
	Fields []schemaField `json:"fields"`
}

func parseFormSchema(f *Form) ([]schemaField, error) {
	var rs rawSchema
	if err := json.Unmarshal([]byte(f.SchemaJSON()), &rs); err != nil {
		return nil, err
	}
	return rs.Fields, nil
}

// ── FormToJSONSchema ──────────────────────────────────────────────────────────

// FormToJSONSchema converts a Form to a JSON Schema suitable for LLM tool calling.
// Hidden fields are excluded.
func FormToJSONSchema(f *Form) (*JSONSchema, error) {
	fields, err := parseFormSchema(f)
	if err != nil {
		return nil, err
	}

	props := make(map[string]JSONSchemaProperty)
	var required []string

	for _, field := range fields {
		if field.FieldType == "Hidden" {
			continue
		}
		props[field.ID] = fieldToJSONSchemaProp(field)
		if field.Required {
			required = append(required, field.ID)
		}
	}

	return &JSONSchema{Type: "object", Properties: props, Required: required}, nil
}

func fieldToJSONSchemaProp(f schemaField) JSONSchemaProperty {
	switch f.FieldType {
	case "Text", "Email", "Textarea", "Date", "File":
		prop := JSONSchemaProperty{Type: "string"}
		if f.Placeholder != nil {
			prop.Description = *f.Placeholder
		}
		return prop

	case "Number", "Range":
		prop := JSONSchemaProperty{Type: "number"}
		prop.Minimum = f.Min
		prop.Maximum = f.Max
		return prop

	case "Checkbox":
		return JSONSchemaProperty{Type: "boolean"}

	case "Select", "Radio":
		enums := make([]string, len(f.Options))
		for i, o := range f.Options { enums[i] = o.Value }
		return JSONSchemaProperty{Type: "string", Enum: enums}

	case "MultiSelect":
		enums := make([]string, len(f.Options))
		for i, o := range f.Options { enums[i] = o.Value }
		return JSONSchemaProperty{
			Type:  "array",
			Items: &JSONSchemaProperty{Type: "string", Enum: enums},
		}

	default:
		return JSONSchemaProperty{Type: "string"}
	}
}

// ── FormResultToToolArgs ──────────────────────────────────────────────────────

// FormResultToToolArgs converts a FormResult into a map suitable for tool invocation.
// Hidden fields are excluded. Returns FormoraTypeError on type mismatch.
func FormResultToToolArgs(r *FormResult, f *Form) (map[string]any, error) {
	fields, err := parseFormSchema(f)
	if err != nil {
		return nil, err
	}
	typed := r.TypedData()
	args := make(map[string]any)

	for _, field := range fields {
		if field.FieldType == "Hidden" {
			continue
		}
		val, ok := typed[field.ID]
		if !ok {
			if field.Required {
				return nil, &FormoraTypeError{fmt.Sprintf("required field %q missing", field.ID)}
			}
			continue
		}
		if err := validateType(field, val); err != nil {
			return nil, err
		}
		args[field.ID] = val
	}
	return args, nil
}

func validateType(f schemaField, val any) error {
	if val == nil {
		if f.Required {
			return &FormoraTypeError{fmt.Sprintf("required field %q is nil", f.ID)}
		}
		return nil
	}
	switch f.FieldType {
	case "Text", "Email", "Textarea", "Date", "Select", "Radio":
		if _, ok := val.(string); !ok {
			return &FormoraTypeError{fmt.Sprintf("field %q: expected string, got %T", f.ID, val)}
		}
	case "Number", "Range":
		n, ok := val.(float64)
		if !ok {
			return &FormoraTypeError{fmt.Sprintf("field %q: expected number, got %T", f.ID, val)}
		}
		if f.Min != nil && n < *f.Min {
			return &FormoraTypeError{fmt.Sprintf("field %q: %v below minimum %v", f.ID, n, *f.Min)}
		}
		if f.Max != nil && n > *f.Max {
			return &FormoraTypeError{fmt.Sprintf("field %q: %v above maximum %v", f.ID, n, *f.Max)}
		}
	case "Checkbox":
		if _, ok := val.(bool); !ok {
			return &FormoraTypeError{fmt.Sprintf("field %q: expected bool, got %T", f.ID, val)}
		}
	case "MultiSelect":
		arr, ok := val.([]any)
		if !ok {
			return &FormoraTypeError{fmt.Sprintf("field %q: expected array, got %T", f.ID, val)}
		}
		for _, item := range arr {
			if _, ok := item.(string); !ok {
				return &FormoraTypeError{fmt.Sprintf("field %q: array items must be strings", f.ID)}
			}
		}
	}
	return nil
}

// ── FormResultToPrompt ────────────────────────────────────────────────────────

// FormResultToPrompt formats a FormResult as a human-readable prompt string.
func FormResultToPrompt(r *FormResult) string {
	typed := r.TypedData()
	if len(typed) == 0 {
		return "The user has provided no information."
	}
	parts := make([]string, 0, len(typed))
	for k, v := range typed {
		var formatted string
		switch val := v.(type) {
		case bool:
			if val { formatted = "Yes" } else { formatted = "No" }
		case []any:
			strs := make([]string, len(val))
			for i, item := range val { strs[i] = fmt.Sprintf("%v", item) }
			formatted = strings.Join(strs, " and ")
		default:
			formatted = fmt.Sprintf("%v", v)
		}
		parts = append(parts, fmt.Sprintf("%s: %s", k, formatted))
	}
	return "The user has provided the following information: " + strings.Join(parts, ", ")
}
