# Formora

Interactive form widgets for chat applications powered by Rust and Python.

## Features

- **CSS Framework Agnostic**: Built-in support for Bootstrap 5, Tailwind CSS v3, and custom CSS
- **Rich Field Types**: Text, Email, Number, Textarea, Select, Multi-Select, Checkbox, Radio, Date, Range, File, Hidden
- **Validation**: Client-side validation with custom rules and error messages
- **Conditional Logic**: Show/hide fields based on other field values
- **Multi-step Forms**: Progress tracking and step-by-step navigation
- **LLM Integration**: JSON Schema generation and form result formatting
- **Framework Adapters**: Ready-to-use JavaScript snippets for Open WebUI, Chainlit, Gradio, and generic chat UIs

## Installation

```bash
pip install formora
```

## Quick Start

```python
from formora import Form, CssFramework

# Create a simple form
html = Form("signup") \
    .title("Sign Up") \
    .css(CssFramework.BOOTSTRAP) \
    .text("name", "Full Name", required=True) \
    .email("email", "Email Address", required=True) \
    .submit_label("Create Account") \
    .build()
```

## CSS Frameworks

### Bootstrap (Default)

```python
from formora import Form, CssFramework

html = Form("contact").css(CssFramework.BOOTSTRAP).text("name", "Name").build()
```

### Tailwind

```python
from formora import Form, CssFramework

html = Form("contact").css(CssFramework.TAILWIND).text("name", "Name").build()
```

### Custom Profile

```python
from formora import Form, CssProfile

profile = CssProfile(CssFramework.CUSTOM).override_(
    form_wrapper="my-form",
    input_text="my-input",
    button_submit="my-btn"
)

html = Form("contact").css(profile).text("name", "Name").build()
```

## Field Types

```python
from formora import Form, Rule

form = Form("survey")

# Text field
form.text("name", "Full Name", required=True, rules=[Rule.min_length(2)])

# Email with validation
form.email("email", "Email", required=True, rules=[Rule.required(), Rule.email()])

# Number with range
form.number("age", "Age", min=0, max=120)

# Textarea
form.textarea("bio", "About You", rows=5)

# Select dropdown
form.select("country", "Country", options=[("United States", "us"), ("Canada", "ca")])

# Multi-select
form.multi_select("interests", "Interests", options=[("Tech", "tech"), ("Sports", "sports")])

# Checkbox
form.checkbox("agree", "I agree to the terms", default=True)

# Radio buttons
form.radio("plan", "Plan", options=[("Free", "free"), ("Pro", "pro")])

# Date picker
form.date("birthdate", "Birth Date")

# Range slider
form.range("satisfaction", "Satisfaction", min=1, max=10, step=1)

# File upload
form.file("resume", "Resume", accept=[".pdf", ".docx"])

# Hidden field
form.hidden("source", "newsletter")
```

## Validation

```python
from formora import Rule

# Required field
Rule.required(message="This field is required")

# String length
Rule.min_length(10, message="Too short")
Rule.max_length(100, message="Too long")

# Numeric range
Rule.min(18, message="Must be 18 or older")
Rule.max(100, message="Must be 100 or less")

# Regular expression
Rule.regex(r"^\d{3}-\d{3}-\d{4}$", message="Invalid phone format")

# Email validation
Rule.email(message="Please enter a valid email")
```

## Conditional Fields

```python
from formora import Form, Condition

form = Form("registration")

form.text("role", "Role", options=[("Developer", "dev"), ("Manager", "mgr")])

# Show this field only if role is "Manager"
form.textarea(
    "manager_note",
    "Manager Notes",
    show_if=Condition("role", "eq", "mgr")
)

# Show this field if role is NOT empty
form.number(
    "team_size",
    "Team Size",
    show_if=Condition("role", "neq", "")
)
```

## Multi-step Forms

```python
from formora import Form, CssFramework

form = Form("onboarding").css(CssFramework.TAILWIND)

# Step 1
form.step("Basic Info")
form.text("name", "Name", required=True)
form.email("email", "Email", required=True)

# Step 2
form.step("Your Role")
form.select("role", "Role", options=[("Dev", "dev"), ("Designer", "design")])
form.textarea("bio", "Bio")

# Step 3
form.step("Finish")
form.checkbox("newsletter", "Subscribe to newsletter")

html = form.build()
```

## Parsing Form Data

```python
from formora import parse, is_formora_message

# Check if incoming message is a formora message
if is_formora_message(incoming_message):
    result = parse(incoming_message)

    print(result.form_id)       # "onboarding"
    print(result.typed_data)     # {"name": "John", "email": "john@example.com", ...}
    print(result.as_text())      # "name: John, email: john@example.com, ..."
```

## LLM Integration

```python
from formora import Form
from formora.llm import form_to_json_schema, form_result_to_tool_args, form_result_to_prompt

form = Form("user_info").text("name", "Name").email("email", "Email")

# Generate JSON Schema for LLM tool calling
schema = form_to_json_schema(form)
# {
#     "type": "object",
#     "properties": {
#         "name": {"type": "string"},
#         "email": {"type": "string"}
#     },
#     "required": ["name", "email"]
# }

# Parse result and get validated tool args
result = parse(incoming_message)
tool_args = form_result_to_tool_args(result, form)

# Format as human-readable text
prompt = form_result_to_prompt(result)
# "The user has provided the following information: name: John, email: john@example.com"
```

## Chat Adapters

Formora includes JavaScript adapters for popular chat frameworks:

- **Generic**: Works with any chat UI
- **Open WebUI**: Integrates with Open WebUI
- **Chainlit**: Works with Chainlit apps
- **Gradio**: Integrates with Gradio chatbots

See `formora/adapters/` for implementation details.

## License

MIT License - see LICENSE file for details.
