# formora-go

Go bindings for **formora** — a high-performance form rendering engine that generates rich, interactive HTML forms for chat and LLM applications.

Powered by a Rust core compiled to a native shared library (`formora-c`) and wrapped via **cgo**.

---

## Requirements

| Tool | Version |
|------|---------|
| Go | 1.21+ |
| Rust | stable (for building `formora-c`) |
| gcc / clang | any (cgo build dependency) |

---

## Installation

### 1 — Build the native library

```bash
cd formora-go
./build.sh
```

This compiles `formora-c` in release mode and places the shared library at
`../formora-c/target/release/libformora_c.{so,dylib,dll}`.

### 2 — Set the library search path

**macOS**
```bash
export DYLD_LIBRARY_PATH="$PWD/../formora-c/target/release:$DYLD_LIBRARY_PATH"
```

**Linux**
```bash
export LD_LIBRARY_PATH="$PWD/../formora-c/target/release:$LD_LIBRARY_PATH"
```

### 3 — Use in your module

Add the module path to your `go.mod`, or use a local `replace` directive:

```go
module your-app

require github.com/YASSERRMD/formora/go v0.1.0
```

---

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/YASSERRMD/formora/go/formora"
)

func main() {
    form := formora.NewForm("")   // auto UUID
    defer form.Free()

    html := form.
        Title("Contact Us").
        CSSFramework(formora.Bootstrap()).
        Text(formora.TextField{
            ID: "name", Label: "Your Name",
            Required: true,
            Rules: []formora.Rule{formora.RuleRequired(nil)},
        }).
        Email(formora.EmailField{
            ID: "email", Label: "Email Address",
            Required: true,
            Rules: []formora.Rule{formora.RuleRequired(nil), formora.RuleEmail(nil)},
        }).
        SubmitLabel("Send").
        Build()

    fmt.Println(html)
}
```

---

## API Reference

### Creating a Form

```go
form := formora.NewForm("my-form-id")  // or "" for auto UUID
defer form.Free()
```

### Builder Methods (all return `*Form` for chaining)

| Method | Description |
|--------|-------------|
| `Title(text)` | Set the form title |
| `Description(text)` | Set the form description |
| `CSSFramework(fw)` | Apply Bootstrap / Tailwind / Custom styles |
| `CSSProfile(p)` | Apply a fully custom CSS profile |
| `Step(title)` | Add a step (enables multi-step mode) |
| `SubmitLabel(text)` | Set the submit button label |
| `SuccessMessage(text)` | Set the post-submission message |
| `Build()` | Render and return the HTML string |

### Field Methods

Each method accepts a typed config struct:

| Method | Config Struct | Description |
|--------|--------------|-------------|
| `Text(cfg)` | `TextField` | Single-line text input |
| `Email(cfg)` | `EmailField` | Email input |
| `Number(cfg)` | `NumberField` | Numeric input with optional min/max |
| `Textarea(cfg)` | `TextareaField` | Multi-line text area |
| `Select(cfg)` | `SelectField` | Dropdown with options |
| `MultiSelect(cfg)` | `MultiSelectField` | Multi-select with options |
| `Checkbox(cfg)` | `CheckboxField` | Boolean toggle |
| `Radio(cfg)` | `RadioField` | Radio button group |
| `Date(cfg)` | `DateField` | Date picker |
| `Range(cfg)` | `RangeField` | Range slider with min/max |
| `File(cfg)` | `FileField` | File upload |
| `Hidden(id, value)` | — | Hidden context field |

### Rules

```go
formora.RuleRequired(nil)
formora.RuleMinLength(3, formora.Ptr("Too short"))
formora.RuleMaxLength(100, nil)
formora.RuleMin(0, nil)
formora.RuleMax(100, nil)
formora.RuleRegex(`^\d{5}$`, formora.Ptr("Must be a 5-digit zip code"))
formora.RuleEmail(nil)
```

### Conditions (conditional visibility)

```go
cond := formora.NewCondition("ticket_type", "eq", "vip")

form.Text(formora.TextField{
    ID:     "company",
    Label:  "Company Name",
    ShowIf: &cond,
})
```

**Operators:** `"eq"` `"neq"` `"contains"` `"gt"` `"lt"` `"in_list"`

### CSS Frameworks

```go
form.CSSFramework(formora.Bootstrap())
form.CSSFramework(formora.Tailwind())
form.CSSFramework(formora.Custom())

// Or a custom profile:
profile := formora.CssProfileFromMap(map[string]string{
    "button_submit": "my-btn my-btn-primary",
    "form_wrapper":  "my-form-container",
})
defer profile.Free()
form.CSSProfile(profile)
```

### Parsing Submissions

Forms emit `__formora__{...}` messages when submitted.

```go
if formora.IsFormora(message) {
    result := formora.ParseMessage(message)
    defer result.Free()

    fmt.Println(result.FormID())
    fmt.Println(result.TypedData())   // map[string]any with coerced types
    fmt.Println(result.AsText())
}
```

### LLM Integration

```go
// Convert form to JSON Schema for tool calling
schema, _ := formora.FormToJSONSchema(form)

// Extract typed args from result (validates types)
args, err := formora.FormResultToToolArgs(result, form)

// Format result as a natural-language prompt
prompt := formora.FormResultToPrompt(result)
```

---

## CSS Frameworks

| Framework | Description |
|-----------|-------------|
| `Bootstrap()` | Bootstrap 5 — classes like `form-control`, `btn btn-primary` |
| `Tailwind()` | Tailwind CSS v3 — utility-first classes |
| `Custom()` | Minimal semantic class names for your own stylesheet |

---

## Multi-Step Forms

```go
form.
    Step("Personal Details").
    Text(formora.TextField{ID: "name", Label: "Name", Required: true}).
    Email(formora.EmailField{ID: "email", Label: "Email", Required: true}).
    Step("Preferences").
    Select(formora.SelectField{ID: "plan", Label: "Plan", Options: []formora.Option{...}})
```

---

## Examples

| Example | File |
|---------|------|
| Contact form | `examples/simple_form/main.go` |
| 3-step onboarding | `examples/multistep_form/main.go` |
| Conditional fields | `examples/conditional_form/main.go` |

```bash
# Run an example (after build.sh)
cd examples/simple_form
go run main.go
```
