# formora-c

C-compatible FFI layer for **formora** — exposes the Rust core as a native shared library (`.so` / `.dylib` / `.dll`) for consumption by any language that supports C interop.

This crate is the shared foundation for:

- **[formora-go](../formora-go)** — Go bindings (cgo)
- **[formora-cs](../formora-cs)** — C# bindings (P/Invoke)

---

## Building

```bash
# Shared library (recommended)
cargo build --release

# Both shared and static
cargo build --release
```

Output: `target/release/libformora_c.{so,dylib,dll}`

---

## API Overview

The complete public API is declared in [`include/formora.h`](include/formora.h).

### Memory contract

- All strings returned by `formora_*` functions are **Rust-allocated** and must be freed with `formora_free_string()`.
- All opaque handles (`FormoraForm *`, `FormoraFormResult *`, etc.) must be freed with their respective `_free` functions.
- String parameters passed **in** are borrowed — the caller retains ownership.

### Opaque types

| Type | Description |
|------|-------------|
| `FormoraCssFramework *` | CSS framework selector handle |
| `FormoraCssProfile *` | CSS class profile handle |
| `FormoraForm *` | Form builder handle |
| `FormoraFormResult *` | Parsed submission result handle |

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
char *rule_json = formora_rule_required(NULL);
// use rule_json...
formora_free_string(rule_json);

char *cond_json = formora_condition("role", "eq", "\"admin\"");
// use cond_json...
formora_free_string(cond_json);
```

### Example (C)

```c
#include "formora.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    FormoraCssFramework *fw   = formora_css_bootstrap();
    FormoraForm         *form = formora_form_new(NULL);  /* auto UUID */

    formora_form_title(form, "Contact Us");
    formora_form_css_framework(form, fw);

    char *rule_req   = formora_rule_required(NULL);
    char *rule_email = formora_rule_email(NULL);
    char *rules_arr  = /* build JSON array from rule_req + rule_email */;

    formora_form_email(form, "email", "Email Address", 1, NULL, NULL, rules_arr, NULL);
    formora_free_string(rule_req);
    formora_free_string(rule_email);
    free(rules_arr);

    char *html = formora_form_build(form);
    puts(html);
    formora_free_string(html);

    formora_form_free(form);
    formora_css_framework_free(fw);
    return 0;
}
```

---

## Rule helpers

| Function | Description |
|----------|-------------|
| `formora_rule_required(msg)` | Field must have a value |
| `formora_rule_min_length(n, msg)` | String min length |
| `formora_rule_max_length(n, msg)` | String max length |
| `formora_rule_min(n, msg)` | Numeric minimum |
| `formora_rule_max(n, msg)` | Numeric maximum |
| `formora_rule_regex(pattern, msg)` | Regex pattern |
| `formora_rule_email(msg)` | Email format |

All return a heap-allocated JSON string. Free with `formora_free_string()`.

## Condition helper

```c
// value_json is any valid JSON (string, number, bool, array)
char *cond = formora_condition("field_id", "eq", "\"vip\"");
formora_free_string(cond);
```

**Operators:** `"eq"` `"neq"` `"contains"` `"gt"` `"lt"` `"in_list"`
