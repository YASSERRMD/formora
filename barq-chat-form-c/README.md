# barq-chat-form-c

C-compatible FFI layer for **barq-chat-form** — exposes the Rust core as a native shared library (`.so` / `.dylib` / `.dll`) for consumption by any language that supports C interop.

This crate is the shared foundation for:

- **[barq-chat-form-go](../barq-chat-form-go)** — Go bindings (cgo)
- **[barq-chat-form-cs](../barq-chat-form-cs)** — C# bindings (P/Invoke)

---

## Building

```bash
# Shared library (recommended)
cargo build --release

# Both shared and static
cargo build --release
```

Output: `target/release/libbarq_chat_form_c.{so,dylib,dll}`

---

## API Overview

The complete public API is declared in [`include/barq_chat_form.h`](include/barq_chat_form.h).

### Memory contract

- All strings returned by `barq-chat-form_*` functions are **Rust-allocated** and must be freed with `barq_free_string()`.
- All opaque handles (`BarqForm *`, `BarqFormResult *`, etc.) must be freed with their respective `_free` functions.
- String parameters passed **in** are borrowed — the caller retains ownership.

### Opaque types

| Type | Description |
|------|-------------|
| `BarqCssFramework *` | CSS framework selector handle |
| `Barq Chat FormCssProfile *` | CSS class profile handle |
| `BarqForm *` | Form builder handle |
| `BarqFormResult *` | Parsed submission result handle |

### Passing complex data

Rules, conditions, and option lists are passed as **JSON strings** to keep the C API simple:

```c
// Rules — JSON array of rule objects
const char *rules = "[{\"rule_type\":\"required\",\"value\":null,\"message\":null}]";

// Condition — JSON object
const char *show_if = "{\"field_id\":\"role\",\"operator\":\"eq\",\"value\":\"admin\"}";

// Options — JSON array of [label, value] pairs
const char *opts = "[[\"Admin\",\"admin\"],[\"User\",\"user\"]]";
```

Or use the built-in rule/condition helpers that return JSON strings:

```c
char *rule_json = barq_rule_required(NULL);
// use rule_json...
barq_free_string(rule_json);

char *cond_json = barq_chat_form_condition("role", "eq", "\"admin\"");
// use cond_json...
barq_free_string(cond_json);
```

### Example (C)

```c
#include "barq_chat_form.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    BarqCssFramework *fw   = barq_chat_form_css_bootstrap();
    BarqForm         *form = barq_form_new(NULL);  /* auto UUID */

    barq_form_title(form, "Contact Us");
    barq_form_css_framework(form, fw);

    char *rule_req   = barq_rule_required(NULL);
    char *rule_email = barq_rule_email(NULL);
    char *rules_arr  = /* build JSON array from rule_req + rule_email */;

    barq_form_email(form, "email", "Email Address", 1, NULL, NULL, rules_arr, NULL);
    barq_free_string(rule_req);
    barq_free_string(rule_email);
    free(rules_arr);

    char *html = barq_form_build(form);
    puts(html);
    barq_free_string(html);

    barq_form_free(form);
    barq_chat_form_css_framework_free(fw);
    return 0;
}
```

---

## Rule helpers

| Function | Description |
|----------|-------------|
| `barq_rule_required(msg)` | Field must have a value |
| `barq_rule_min_length(n, msg)` | String min length |
| `barq_rule_max_length(n, msg)` | String max length |
| `barq_rule_min(n, msg)` | Numeric minimum |
| `barq_rule_max(n, msg)` | Numeric maximum |
| `barq_rule_regex(pattern, msg)` | Regex pattern |
| `barq_rule_email(msg)` | Email format |

All return a heap-allocated JSON string. Free with `barq_free_string()`.

## Condition helper

```c
// value_json is any valid JSON (string, number, bool, array)
char *cond = barq_chat_form_condition("field_id", "eq", "\"vip\"");
barq_free_string(cond);
```

**Operators:** `"eq"` `"neq"` `"contains"` `"gt"` `"lt"` `"in_list"`
