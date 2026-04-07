#ifndef BARQ_CHAT_FORM_H
#define BARQ_CHAT_FORM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Opaque types ─────────────────────────────────────────────────────────── */
typedef struct BarqCssFramework BarqCssFramework;
typedef struct BarqCssProfile   BarqCssProfile;
typedef struct BarqForm         BarqForm;
typedef struct BarqFormResult   BarqFormResult;

/* ── Memory ───────────────────────────────────────────────────────────────── */
void barq_free_string(char *s);

/* ── CssFramework ─────────────────────────────────────────────────────────── */
BarqCssFramework *barq_css_bootstrap(void);
BarqCssFramework *barq_css_tailwind(void);
BarqCssFramework *barq_css_custom(void);
void              barq_css_framework_free(BarqCssFramework *fw);

/* ── CssProfile ───────────────────────────────────────────────────────────── */
BarqCssProfile *barq_css_profile_new(const BarqCssFramework *fw);
BarqCssProfile *barq_css_profile_from_json(const char *json);
BarqCssProfile *barq_css_profile_override(const BarqCssProfile *profile, const char *overrides_json);
void            barq_css_profile_free(BarqCssProfile *profile);

/* ── Form lifecycle ───────────────────────────────────────────────────────── */
BarqForm *barq_form_new(const char *id);   /* id may be NULL → auto UUID */
void      barq_form_free(BarqForm *form);

/* ── Form builder ─────────────────────────────────────────────────────────── */
BarqForm *barq_form_title(BarqForm *form, const char *text);
BarqForm *barq_form_description(BarqForm *form, const char *text);
BarqForm *barq_form_css_framework(BarqForm *form, const BarqCssFramework *fw);
BarqForm *barq_form_css_profile(BarqForm *form, const BarqCssProfile *profile);
BarqForm *barq_form_step(BarqForm *form, const char *title);  /* title may be NULL */
BarqForm *barq_form_submit_label(BarqForm *form, const char *text);
BarqForm *barq_form_success_message(BarqForm *form, const char *text);
char     *barq_form_build(const BarqForm *form);       /* caller frees */
char     *barq_form_schema_json(const BarqForm *form); /* caller frees */

/* ── Field methods ────────────────────────────────────────────────────────── */
/*  rules_json   : JSON array of rule objects, or NULL
    show_if_json : JSON condition object, or NULL
    required     : 1 = required, 0 = optional                                */

BarqForm *barq_form_text(BarqForm *form, const char *id, const char *label,
    const char *placeholder, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_email(BarqForm *form, const char *id, const char *label,
    int required, const char *help_text, const char *default_val,
    const char *rules_json, const char *show_if_json);

BarqForm *barq_form_number(BarqForm *form, const char *id, const char *label,
    double min, double max, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_textarea(BarqForm *form, const char *id, const char *label,
    uint32_t rows, const char *placeholder, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_select(BarqForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_multi_select(BarqForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val_json, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_checkbox(BarqForm *form, const char *id, const char *label,
    int default_val, const char *help_text,
    const char *rules_json, const char *show_if_json);

BarqForm *barq_form_radio(BarqForm *form, const char *id, const char *label,
    const char *options_json, int required, const char *help_text,
    const char *default_val, const char *rules_json, const char *show_if_json);

BarqForm *barq_form_date(BarqForm *form, const char *id, const char *label,
    int required, const char *help_text, const char *default_val,
    const char *rules_json, const char *show_if_json);

BarqForm *barq_form_range(BarqForm *form, const char *id, const char *label,
    double min, double max, double step, double default_val,
    const char *help_text, const char *show_if_json);

BarqForm *barq_form_file(BarqForm *form, const char *id, const char *label,
    const char *accept_json, int required, const char *help_text,
    const char *rules_json, const char *show_if_json);

BarqForm *barq_form_hidden(BarqForm *form, const char *id, const char *value);

/* ── Rule helpers (return JSON strings, caller frees) ─────────────────────── */
char *barq_rule_required(const char *message);
char *barq_rule_min_length(uint32_t n, const char *message);
char *barq_rule_max_length(uint32_t n, const char *message);
char *barq_rule_min(double n, const char *message);
char *barq_rule_max(double n, const char *message);
char *barq_rule_regex(const char *pattern, const char *message);
char *barq_rule_email(const char *message);

/* ── Condition helper (returns JSON string, caller frees) ─────────────────── */
char *barq_condition(const char *field_id, const char *operator_, const char *value_json);

/* ── Parser ───────────────────────────────────────────────────────────────── */
BarqFormResult *barq_parse(const char *message);    /* returns NULL if not barq message */
int             barq_is_barq(const char *message);
void            barq_result_free(BarqFormResult *result);
char           *barq_result_form_id(const BarqFormResult *result);         /* caller frees */
char           *barq_result_data_json(const BarqFormResult *result);       /* caller frees */
char           *barq_result_typed_data_json(const BarqFormResult *result); /* caller frees */
char           *barq_result_as_text(const BarqFormResult *result);         /* caller frees */

#ifdef __cplusplus
}
#endif

#endif /* BARQ_CHAT_FORM_H */
