# barq-chat-form-go

Go bindings for **barq-chat-form** — a high-performance form rendering engine that generates rich, interactive HTML forms for chat and LLM applications.

Powered by a Rust core compiled to a native shared library (`barq-chat-form-c`) and wrapped via **cgo**.

---

## Requirements

| Tool | Version |
|------|---------|
| Go | 1.21+ |
| Rust | stable (for building `barq-chat-form-c`) |
| gcc / clang | any (cgo build dependency) |

---

## Installation

### 1 — Build the native library

```bash
cd barq-chat-form-go
./build.sh
```

This compiles `barq-chat-form-c` in release mode and places the shared library at
`../barq-chat-form-c/target/release/libbarq-chat-form_c.{so,dylib,dll}`.

### 2 — Set the library search path

**macOS**
```bash
export DYLD_LIBRARY_PATH="$PWD/../barq-chat-form-c/target/release:$DYLD_LIBRARY_PATH"
```

**Linux**
```bash
export LD_LIBRARY_PATH="$PWD/../barq-chat-form-c/target/release:$LD_LIBRARY_PATH"
```

### 3 — Use in your module

Add the module path to your `go.mod`, or use a local `replace` directive:

```go
module your-app

require github.com/YASSERRMD/barq-chat-form/go v0.1.0
```

---

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/YASSERRMD/barq-chat-form/go/barq-chat-form"
)

func main() {
    form := barq-chat-form.NewForm("")   // auto UUID
    defer form.Free()

    html := form.
        Title("Contact Us").
        CSSFramework(barq-chat-form.Bootstrap()).
        Text(barq-chat-form.TextField{
            ID: "name", Label: "Your Name",
            Required: true,
            Rules: []barq-chat-form.Rule{barq-chat-form.RuleRequired(nil)},
        }).
        Email(barq-chat-form.EmailField{
            ID: "email", Label: "Email Address",
            Required: true,
            Rules: []barq-chat-form.Rule{barq-chat-form.RuleRequired(nil), barq-chat-form.RuleEmail(nil)},
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
form := barq-chat-form.NewForm("my-form-id")  // or "" for auto UUID
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
barq-chat-form.RuleRequired(nil)
barq-chat-form.RuleMinLength(3, barq-chat-form.Ptr("Too short"))
barq-chat-form.RuleMaxLength(100, nil)
barq-chat-form.RuleMin(0, nil)
barq-chat-form.RuleMax(100, nil)
barq-chat-form.RuleRegex(`^\d{5}$`, barq-chat-form.Ptr("Must be a 5-digit zip code"))
barq-chat-form.RuleEmail(nil)
```

### Conditions (conditional visibility)

```go
cond := barq-chat-form.NewCondition("ticket_type", "eq", "vip")

form.Text(barq-chat-form.TextField{
    ID:     "company",
    Label:  "Company Name",
    ShowIf: &cond,
})
```

**Operators:** `"eq"` `"neq"` `"contains"` `"gt"` `"lt"` `"in_list"`

### CSS Frameworks

```go
form.CSSFramework(barq-chat-form.Bootstrap())
form.CSSFramework(barq-chat-form.Tailwind())
form.CSSFramework(barq-chat-form.Custom())

// Or a custom profile:
profile := barq-chat-form.CssProfileFromMap(map[string]string{
    "button_submit": "my-btn my-btn-primary",
    "form_wrapper":  "my-form-container",
})
defer profile.Free()
form.CSSProfile(profile)
```

### Parsing Submissions

Forms emit `__barq-chat-form__{...}` messages when submitted.

```go
if barq-chat-form.IsBarq Chat Form(message) {
    result := barq-chat-form.ParseMessage(message)
    defer result.Free()

    fmt.Println(result.FormID())
    fmt.Println(result.TypedData())   // map[string]any with coerced types
    fmt.Println(result.AsText())
}
```

### LLM Integration

```go
// Convert form to JSON Schema for tool calling
schema, _ := barq-chat-form.FormToJSONSchema(form)

// Extract typed args from result (validates types)
args, err := barq-chat-form.FormResultToToolArgs(result, form)

// Format result as a natural-language prompt
prompt := barq-chat-form.FormResultToPrompt(result)
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
    Text(barq-chat-form.TextField{ID: "name", Label: "Name", Required: true}).
    Email(barq-chat-form.EmailField{ID: "email", Label: "Email", Required: true}).
    Step("Preferences").
    Select(barq-chat-form.SelectField{ID: "plan", Label: "Plan", Options: []barq-chat-form.Option{...}})
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
