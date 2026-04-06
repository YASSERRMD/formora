using System.Text.Json;
using System.Text.Json.Serialization;

namespace Formora;

// ── JSON Schema types ──────────────────────────────────────────────────────────

public record JsonSchemaProperty(
    [property: JsonPropertyName("type")]        string Type,
    [property: JsonPropertyName("description")] string? Description = null,
    [property: JsonPropertyName("minimum")]     double? Minimum = null,
    [property: JsonPropertyName("maximum")]     double? Maximum = null,
    [property: JsonPropertyName("enum")]        string[]? Enum = null,
    [property: JsonPropertyName("items")]       JsonSchemaProperty? Items = null
);

public record JsonSchema(
    [property: JsonPropertyName("type")]       string Type,
    [property: JsonPropertyName("properties")] Dictionary<string, JsonSchemaProperty> Properties,
    [property: JsonPropertyName("required")]   string[]? Required = null
);

/// <summary>Thrown when a form result value does not match the expected field type.</summary>
public class FormoraTypeError(string message) : Exception(message);

// ── LLM utilities ──────────────────────────────────────────────────────────────

public static class Llm
{
    private record SchemaField(
        [property: JsonPropertyName("id")]         string Id,
        [property: JsonPropertyName("field_type")] string FieldType,
        [property: JsonPropertyName("required")]   bool Required,
        [property: JsonPropertyName("placeholder")]string? Placeholder,
        [property: JsonPropertyName("min")]        double? Min,
        [property: JsonPropertyName("max")]        double? Max,
        [property: JsonPropertyName("options")]    OptionEntry[]? Options
    );

    private record OptionEntry(
        [property: JsonPropertyName("label")] string Label,
        [property: JsonPropertyName("value")] string Value
    );

    private record RawSchema([property: JsonPropertyName("fields")] SchemaField[] Fields);

    private static SchemaField[] ParseFields(Form form)
    {
        var raw = JsonSerializer.Deserialize<RawSchema>(form.SchemaJson());
        return raw?.Fields ?? [];
    }

    /// <summary>
    /// Convert a Form's schema to a JSON Schema object for LLM tool/function calling.
    /// Hidden fields are excluded.
    /// </summary>
    public static JsonSchema FormToJsonSchema(Form form)
    {
        var fields = ParseFields(form);
        var props = new Dictionary<string, JsonSchemaProperty>();
        var required = new List<string>();

        foreach (var f in fields)
        {
            if (f.FieldType == "Hidden") continue;
            props[f.Id] = FieldToJsonSchemaProp(f);
            if (f.Required) required.Add(f.Id);
        }

        return new JsonSchema("object", props, required.Count > 0 ? [.. required] : null);
    }

    private static JsonSchemaProperty FieldToJsonSchemaProp(SchemaField f) => f.FieldType switch
    {
        "Text" or "Email" or "Textarea" or "Date" or "File"
            => new JsonSchemaProperty("string", f.Placeholder),

        "Number" or "Range"
            => new JsonSchemaProperty("number", Minimum: f.Min, Maximum: f.Max),

        "Checkbox"
            => new JsonSchemaProperty("boolean"),

        "Select" or "Radio"
            => new JsonSchemaProperty("string", Enum: f.Options?.Select(o => o.Value).ToArray()),

        "MultiSelect"
            => new JsonSchemaProperty("array",
                Items: new JsonSchemaProperty("string",
                    Enum: f.Options?.Select(o => o.Value).ToArray())),

        _ => new JsonSchemaProperty("string"),
    };

    /// <summary>
    /// Convert a FormResult into a dictionary suitable for tool invocation.
    /// Hidden fields are excluded. Throws <see cref="FormoraTypeError"/> on type mismatch.
    /// </summary>
    public static Dictionary<string, object?> FormResultToToolArgs(FormResult result, Form form)
    {
        var fields = ParseFields(form);
        var typed = result.TypedData;
        var args = new Dictionary<string, object?>();

        foreach (var f in fields)
        {
            if (f.FieldType == "Hidden") continue;

            if (!typed.TryGetValue(f.Id, out var val))
            {
                if (f.Required)
                    throw new FormoraTypeError($"Required field '{f.Id}' missing from form result");
                continue;
            }

            ValidateFieldType(f, val);
            args[f.Id] = val;
        }

        return args;
    }

    private static void ValidateFieldType(SchemaField f, object? val)
    {
        if (val is null)
        {
            if (f.Required) throw new FormoraTypeError($"Required field '{f.Id}' is null");
            return;
        }

        // Values arrive as JsonElement when deserialised from System.Text.Json
        if (val is not JsonElement je) return;

        switch (f.FieldType)
        {
            case "Text" or "Email" or "Textarea" or "Date" or "Select" or "Radio":
                if (je.ValueKind != JsonValueKind.String)
                    throw new FormoraTypeError($"Field '{f.Id}': expected string, got {je.ValueKind}");
                break;

            case "Number" or "Range":
                if (je.ValueKind != JsonValueKind.Number)
                    throw new FormoraTypeError($"Field '{f.Id}': expected number, got {je.ValueKind}");
                var n = je.GetDouble();
                if (f.Min.HasValue && n < f.Min.Value)
                    throw new FormoraTypeError($"Field '{f.Id}': {n} is below minimum {f.Min}");
                if (f.Max.HasValue && n > f.Max.Value)
                    throw new FormoraTypeError($"Field '{f.Id}': {n} is above maximum {f.Max}");
                break;

            case "Checkbox":
                if (je.ValueKind is not JsonValueKind.True and not JsonValueKind.False)
                    throw new FormoraTypeError($"Field '{f.Id}': expected boolean, got {je.ValueKind}");
                break;

            case "MultiSelect":
                if (je.ValueKind != JsonValueKind.Array)
                    throw new FormoraTypeError($"Field '{f.Id}': expected array, got {je.ValueKind}");
                break;
        }
    }

    /// <summary>
    /// Format a FormResult as a human-readable prompt string.
    /// Booleans → "Yes"/"No", arrays → joined with " and ".
    /// </summary>
    public static string FormResultToPrompt(FormResult result)
    {
        var typed = result.TypedData;
        if (typed.Count == 0) return "The user has provided no information.";

        var parts = typed.Select(kv =>
        {
            string formatted = kv.Value switch
            {
                JsonElement { ValueKind: JsonValueKind.True }  => "Yes",
                JsonElement { ValueKind: JsonValueKind.False } => "No",
                JsonElement je when je.ValueKind == JsonValueKind.Array
                    => string.Join(" and ", je.EnumerateArray().Select(e => e.ToString())),
                _ => kv.Value?.ToString() ?? "",
            };
            return $"{kv.Key}: {formatted}";
        });

        return "The user has provided the following information: " + string.Join(", ", parts);
    }
}
