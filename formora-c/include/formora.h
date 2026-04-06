#ifndef FORMORA_H
#define FORMORA_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Opaque types ─────────────────────────────────────────────────────────── */
typedef struct FormoraCssFramework FormoraCssFramework;
typedef struct FormoraCssProfile   FormoraCssProfile;
typedef struct FormoraForm         FormoraForm;
typedef struct FormoraFormResult   FormoraFormResult;

/* ── Memory ───────────────────────────────────────────────────────────────── */
void formora_free_string(char *s);

/* ── CssFramework ─────────────────────────────────────────────────────────── */
FormoraCssFramework *formora_css_bootstrap(void);
FormoraCssFramework *formora_css_tailwind(void);
FormoraCssFramework *formora_css_custom(void);
void                 formora_css_framework_free(FormoraCssFramework *fw);

/* ── CssProfile ───────────────────────────────────────────────────────────── */
FormoraCssProfile *formora_css_profile_new(const FormoraCssFramework *fw);
FormoraCssProfile *formora_css_profile_from_json(const char *json);
FormoraCssProfile *formora_css_profile_override(const FormoraCssProfile *profile, const char *overrides_json);
void               formora_css_profile_free(FormoraCssProfile *profile);

/* ── Form lifecycle ───────────────────────────────────────────────────────── */
FormoraForm *formora_form_new(const char *id);   /* id may be NULL → auto UUID */
void         formora_form_free(FormoraForm *form);

/* ── Form builder ─────────────────────────────────────────────────────────── */
FormoraForm *formora_form_title(FormoraForm *form, const char *text);
FormoraForm *formora_form_description(FormoraForm *form, const char *text);
FormoraForm *formora_form_css_framework(FormoraForm *form, const FormoraCssFramework *fw);
FormoraForm *formora_form_css_profile(FormoraForm *form, const FormoraCssProfile *profile);
FormoraForm *formora_form_step(FormoraForm *form, const char *title);  /* title may be NULL */
FormoraForm *formora_form_submit_label(FormoraForm *form, const char *text);
FormoraForm *formora_form_success_message(FormoraForm *form, const char *text);
char        *formora_form_build(const FormoraForm *form);       /* caller frees */
char        *formora_form_schema_json(const FormoraForm *form); /* caller frees */

/* ── Field methods ────────────────────────────────────────────────────────── */
/*  rules_json   : JSON array of rule objects, or NULL
    show_if_json : JSON condition object, or NULL
    required     : 1 = required, 0 = optional                                */

FormoraForm *formora_form_text(FormoraForm *form, const char *id, const char *label,
    const char *placeholder, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_email(FormoraForm *form, const char *id, const char *label,
    int required, const char *help_text, const char *default_val,
    const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_number(FormoraForm *form, const char *id, const char *label,
    double min, double max, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_textarea(FormoraForm *form, const char *id, const char *label,
    uint32_t rows, const char *placeholder, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_select(FormoraForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_multi_select(FormoraForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val_json, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_checkbox(FormoraForm *form, const char *id, const char *label,
    int default_val, const char *help_text,
    const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_radio(FormoraForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_date(FormoraForm *form, const char *id, const char *label,
    int required, const char *help_text, const char *default_val,
    const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_range(FormoraForm *form, const char *id, const char *label,
    double min, double max, double step, double default_val,
    const char *help_text, const char *show_if_json);

FormoraForm *formora_form_file(FormoraForm *form, const char *id, const char *label,
    const char *accept_json, int required, const char *help_text,
    const char *rules_json, const char *show_if_json);

FormoraForm *formora_form_hidden(FormoraForm *form, const char *id, const char *value);

/* ── Rule helpers (return JSON strings, caller frees) ─────────────────────── */
char *formora_rule_required(const char *message);
char *formora_rule_min_length(uint32_t n, const char *message);
char *formora_rule_max_length(uint32_t n, const char *message);
char *formora_rule_min(double n, const char *message);
char *formora_rule_max(double n, const char *message);
char *formora_rule_regex(const char *pattern, const char *message);
char *formora_rule_email(const char *message);

/* ── Condition helper (returns JSON string, caller frees) ─────────────────── */
char *formora_condition(const char *field_id, const char *operator_, const char *value_json);

/* ── Parser ───────────────────────────────────────────────────────────────── */
FormoraFormResult *formora_parse(const char *message);    /* returns NULL if not formora */
int                formora_is_formora(const char *message);
void               formora_result_free(FormoraFormResult *result);
char              *formora_result_form_id(const FormoraFormResult *result);     /* caller frees */
char              *formora_result_data_json(const FormoraFormResult *result);   /* caller frees */
char              *formora_result_typed_data_json(const FormoraFormResult *result); /* caller frees */
char              *formora_result_as_text(const FormoraFormResult *result);     /* caller frees */

#ifdef __cplusplus
}
#endif

#endif /* FORMORA_H */
