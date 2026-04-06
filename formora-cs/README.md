# formora-cs

C# (.NET 8) bindings for **formora** — a high-performance form rendering engine that generates rich, interactive HTML forms for chat and LLM applications.

Powered by a Rust core compiled to a native shared library (`formora-c`) and wrapped via **P/Invoke** (`LibraryImport`).

---

## Requirements

| Tool | Version |
|------|---------|
| .NET | 8.0+ |
| Rust | stable (for building `formora-c`) |

---

## Installation

### 1 — Build the native library

```bash
cd formora-cs
./build.sh
```

This compiles `formora-c` in release mode. The native library is placed at
`../formora-c/target/release/libformora_c.{so,dylib,dll}`.

### 2 — Set the library search path

**Linux**
```bash
export LD_LIBRARY_PATH="$PWD/../formora-c/target/release:$LD_LIBRARY_PATH"
```

**macOS**
```bash
export DYLD_LIBRARY_PATH="$PWD/../formora-c/target/release:$DYLD_LIBRARY_PATH"
```

**Windows** — copy `formora_c.dll` to your output directory or add it to `PATH`.

### 3 — Reference the project

```xml
<ItemGroup>
  <ProjectReference Include="path/to/formora-cs/src/Formora/Formora.csproj" />
</ItemGroup>
```

---

## Quick Start

```csharp
using Formora;

using var form = new Form("contact-form");

var html = form
    .Title("Contact Us")
    .Css(CssFramework.Bootstrap())
    .Text("name", "Your Name",
        required: true, rules: [Rule.Required()])
    .Email("email", "Email Address",
        required: true, rules: [Rule.Required(), Rule.Email()])
    .SubmitLabel("Send")
    .Build();

Console.WriteLine(html);
```

---

## API Reference

### Creating a Form

```csharp
using var form = new Form();          // auto UUID
using var form = new Form("my-id");   // explicit ID
```

`Form` implements `IDisposable` — always wrap in `using` or call `Dispose()`.

### Builder Methods (all return `Form` for chaining)

| Method | Description |
|--------|-------------|
| `Title(text)` | Set the form title |
| `Description(text)` | Set the form description |
| `Css(CssFramework)` | Apply Bootstrap / Tailwind / Custom styles |
| `Css(CssProfile)` | Apply a fully custom CSS profile |
| `Step(title?)` | Add a step (enables multi-step mode) |
| `SubmitLabel(text)` | Set the submit button label |
| `SuccessMessage(text)` | Set the post-submission message |
| `Build()` | Render and return the HTML string |

### Field Methods

| Method | Description |
|--------|-------------|
| `Text(id, label, ...)` | Single-line text input |
| `Email(id, label, ...)` | Email input |
| `Number(id, label, ...)` | Numeric input with optional `min`/`max` |
| `Textarea(id, label, ...)` | Multi-line text area |
| `Select(id, label, options, ...)` | Dropdown |
| `MultiSelect(id, label, options, ...)` | Multi-select |
| `Checkbox(id, label, ...)` | Boolean toggle |
| `Radio(id, label, options, ...)` | Radio button group |
| `Date(id, label, ...)` | Date picker |
| `Range(id, label, min, max, ...)` | Range slider |
| `File(id, label, ...)` | File upload |
| `Hidden(id, value)` | Hidden context field |

All optional parameters use C# named arguments for clarity.

### Rules

```csharp
Rule.Required()
Rule.Required("This field is required")
Rule.MinLength(3)
Rule.MinLength(3, "Too short")
Rule.MaxLength(100)
Rule.Min(0)
Rule.Max(100)
Rule.Regex(@"^\d{5}$", "Must be a 5-digit zip code")
Rule.Email()
Rule.Email("Enter a valid email address")
```

Pass rules as collection expressions:
```csharp
rules: [Rule.Required(), Rule.MinLength(2)]
```

### Conditions (conditional visibility)

```csharp
var isVip = new Condition("ticket_type", "eq", "vip");

form.Text("company", "Company Name", showIf: isVip);
```

**Operators:** `"eq"` `"neq"` `"contains"` `"gt"` `"lt"` `"in_list"`

### CSS Frameworks

```csharp
form.Css(CssFramework.Bootstrap());
form.Css(CssFramework.Tailwind());
form.Css(CssFramework.Custom());

// Custom profile:
using var profile = CssProfile.FromDictionary(new() {
    ["button_submit"] = "my-btn my-btn-primary",
    ["form_wrapper"]  = "my-form-container",
});
form.Css(profile);
```

### Parsing Submissions

```csharp
if (FormoraParser.IsFormora(message))
{
    using var result = FormoraParser.ParseMessage(message)!;
    Console.WriteLine(result.FormId);
    Console.WriteLine(result.AsText());

    foreach (var (key, value) in result.TypedData)
        Console.WriteLine($"  {key} = {value}");
}
```

### LLM Integration

```csharp
// JSON Schema for tool/function calling
var schema = Llm.FormToJsonSchema(form);

// Validated args dictionary
var args = Llm.FormResultToToolArgs(result, form);   // throws FormoraTypeError on mismatch

// Human-readable prompt string
var prompt = Llm.FormResultToPrompt(result);
```

---

## CSS Frameworks

| Framework | Description |
|-----------|-------------|
| `Bootstrap()` | Bootstrap 5 — `form-control`, `btn btn-primary`, etc. |
| `Tailwind()` | Tailwind CSS v3 — utility-first classes |
| `Custom()` | Minimal semantic class names for your own stylesheet |

---

## Multi-Step Forms

```csharp
form
    .Step("Personal Details")
    .Text("name", "Name", required: true)
    .Email("email", "Email", required: true)
    .Step("Preferences")
    .Select("plan", "Plan", options: [...]);
```

---

## Examples

| Example | Path |
|---------|------|
| Contact form | `examples/SimpleForm/Program.cs` |
| 3-step onboarding | `examples/MultistepForm/Program.cs` |
| Conditional fields | `examples/ConditionalForm/Program.cs` |

```bash
# Build and run an example (after build.sh)
dotnet run --project examples/SimpleForm/SimpleForm.csproj
```
