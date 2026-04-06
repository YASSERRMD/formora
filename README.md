# Formora

> Interactive form widgets for chat and LLM applications — powered by a high-performance Rust core with first-class bindings for **Python**, **TypeScript/WASM**, **Go**, and **C#**.

---

## What is Formora?

Formora generates rich, interactive HTML forms that integrate seamlessly into chat UIs and LLM pipelines. Write your form once, render it in any language, parse submissions back to strongly-typed data.

```
┌─────────────────────────────┐
│        formora-core         │  ← Rust engine (rendering, parsing, validation)
└───────────────┬─────────────┘
                │
    ┌───────────┼───────────────────┐
    │           │                   │
  PyO3       wasm-bindgen      C FFI (cdylib)
    │           │                   │
┌───┴───┐  ┌───┴───┐    ┌──────────┴──────────┐
│  py   │  │  js   │    │   formora-c          │
└───────┘  └───────┘    └──────┬───────────────┘
                               │
                     ┌─────────┴─────────┐
                     │                   │
                   cgo               P/Invoke
                     │                   │
                  ┌──┴──┐           ┌───┴───┐
                  │ go  │           │  cs   │
                  └─────┘           └───────┘
```

---

## Features

| Feature | Description |
|---------|-------------|
| **12 Field Types** | Text, Email, Number, Textarea, Select, Multi-Select, Checkbox, Radio, Date, Range, File, Hidden |
| **CSS Framework Agnostic** | Built-in Bootstrap 5, Tailwind CSS v3, and custom profiles |
| **Client-side Validation** | Required, min/max length, numeric bounds, regex, email format |
| **Conditional Logic** | Show/hide fields based on other field values (eq, neq, contains, gt, lt, in_list) |
| **Multi-step Forms** | Progress tracking and step-by-step navigation |
| **LLM Integration** | JSON Schema generation, type-validated args extraction, prompt formatting |
| **Framework Adapters** | Open WebUI, Chainlit, Gradio, generic chat UIs (Python) |

---

## Language Bindings

| Package | Language | Mechanism | Status |
|---------|----------|-----------|--------|
| [`formora-py`](formora-py) | Python 3.9+ | PyO3 native extension | ✅ |
| [`formora-js`](formora-js) | TypeScript / Node.js / Browser | wasm-bindgen + wasm-pack | ✅ |
| [`formora-go`](formora-go) | Go 1.21+ | cgo → formora-c | ✅ |
| [`formora-cs`](formora-cs) | C# / .NET 8+ | P/Invoke → formora-c | ✅ |
| [`formora-c`](formora-c) | C / any C-ABI language | cdylib + `formora.h` | ✅ |

---

## Quick Start by Language

### Python

```bash
pip install formora
```

```python
from formora import Form, Rule, CssFramework

html = (Form("contact")
    .title("Contact Us")
    .css(CssFramework.bootstrap())
    .text("name", "Full Name", required=True,
          rules=[Rule.required(), Rule.min_length(2)])
    .email("email", "Email Address", required=True,
           rules=[Rule.required(), Rule.email()])
    .submit_label("Send")
    .build())
```

### TypeScript / JavaScript

```bash
# Build the WASM package first
cd formora-js && wasm-pack build --target bundler --out-dir wasm --out-name formora_js
```

```typescript
import init, { Form, Rule, CssFramework } from "./wasm/formora_js";

await init();

const html = new Form("contact")
    .title("Contact Us")
    .css(CssFramework.bootstrap())
    .text("name", "Full Name", undefined, true, undefined, undefined,
          [Rule.required(), Rule.minLength(2)])
    .email("email", "Email Address", true, undefined, undefined,
           [Rule.required(), Rule.email()])
    .submitLabel("Send")
    .build();
```

### Go

```bash
cd formora-go && ./build.sh
export DYLD_LIBRARY_PATH="$PWD/../formora-c/target/release:$DYLD_LIBRARY_PATH"  # macOS
```

```go
import "github.com/YASSERRMD/formora/go/formora"

form := formora.NewForm("")
defer form.Free()

html := form.
    Title("Contact Us").
    CSSFramework(formora.Bootstrap()).
    Text(formora.TextField{ID: "name", Label: "Full Name", Required: true}).
    Email(formora.EmailField{ID: "email", Label: "Email", Required: true}).
    SubmitLabel("Send").
    Build()
```

### C# / .NET

```bash
cd formora-cs && ./build.sh
export LD_LIBRARY_PATH="$PWD/../formora-c/target/release:$LD_LIBRARY_PATH"  # Linux
```

```csharp
using Formora;

using var form = new Form("contact");
var html = form
    .Title("Contact Us")
    .Css(CssFramework.Bootstrap())
    .Text("name", "Full Name", required: true, rules: [Rule.Required()])
    .Email("email", "Email Address", required: true, rules: [Rule.Required(), Rule.Email()])
    .SubmitLabel("Send")
    .Build();
```

---

## Field Types

All bindings support the same 12 field types:

| Field | Description |
|-------|-------------|
| `text` | Single-line text input |
| `email` | Email address input |
| `number` | Numeric input with optional min/max |
| `textarea` | Multi-line text area |
| `select` | Single-value dropdown |
| `multi_select` | Multi-value selector with tags |
| `checkbox` | Boolean toggle |
| `radio` | Radio button group |
| `date` | Date picker |
| `range` | Slider with min/max/step |
| `file` | File upload with MIME type filter |
| `hidden` | Context field (not shown to user) |

---

## Validation Rules

| Rule | Description |
|------|-------------|
| `required` | Field must have a value |
| `min_length(n)` | String must be at least n characters |
| `max_length(n)` | String must be at most n characters |
| `min(n)` | Numeric value ≥ n |
| `max(n)` | Numeric value ≤ n |
| `regex(pattern)` | Value must match pattern |
| `email` | Must be a valid email format |

---

## Parsing Submissions

Forms emit `__formora__{...}` messages when submitted. Parse them in any language:

**Python**
```python
from formora import parse, is_formora_message

if is_formora_message(msg):
    result = parse(msg)
    print(result.typed_data)
```

**TypeScript**
```typescript
import { parseMessage, isFormora } from "./wasm/formora_js";

if (isFormora(msg)) {
    const result = parseMessage(msg)!;
    console.log(result.typedData);
}
```

**Go**
```go
if formora.IsFormora(msg) {
    result := formora.ParseMessage(msg)
    defer result.Free()
    fmt.Println(result.TypedData())
}
```

**C#**
```csharp
if (FormoraParser.IsFormora(msg)) {
    using var result = FormoraParser.ParseMessage(msg)!;
    Console.WriteLine(result.AsText());
}
```

---

## LLM Integration

All bindings expose three LLM utilities:

| Function | Description |
|----------|-------------|
| `form_to_json_schema(form)` | Convert form to JSON Schema for tool calling |
| `form_result_to_tool_args(result, form)` | Extract typed, validated args from a submission |
| `form_result_to_prompt(result)` | Format result as a human-readable prompt string |

---

## Repository Structure

```
formora/
├── formora-core/       # Rust library — rendering, parsing, validation engine
├── formora-py/         # Python bindings (PyO3 + Maturin)
├── formora-js/         # TypeScript/WASM bindings (wasm-bindgen + wasm-pack)
├── formora-c/          # C-compatible shared library (cdylib) + formora.h header
├── formora-go/         # Go bindings (cgo)
├── formora-cs/         # C# / .NET bindings (P/Invoke)
└── examples/           # Python examples
```

---

## Building from Source

### Core (required for all bindings)

```bash
cargo build --release -p formora-core
```

### Python
```bash
cd formora-py && maturin develop
```

### TypeScript / WASM
```bash
cd formora-js && wasm-pack build --target bundler --out-dir wasm --out-name formora_js
```

### C shared library (required for Go and C#)
```bash
cd formora-c && cargo build --release
```

### Go
```bash
cd formora-go && ./build.sh && go vet ./formora/...
```

### C#
```bash
cd formora-cs && ./build.sh && dotnet build src/Formora/Formora.csproj
```

---

## CI / GitHub Actions

The `build.yml` workflow runs on every push and pull request:

| Job | What it builds |
|-----|----------------|
| `test-rust` | formora-core unit tests |
| `build-python` | Python wheel via maturin |
| `build-go` | formora-c + Go vet |
| `build-csharp` | formora-c + dotnet build |
| `build-wasm` | wasm-pack + TypeScript type-check |

---

## License

MIT — see [LICENSE](LICENSE).
