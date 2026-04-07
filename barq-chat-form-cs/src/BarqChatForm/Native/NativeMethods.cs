using System.Runtime.InteropServices;

namespace BarqChatForm.Native;

/// <summary>
/// P/Invoke declarations for the barq_c native library.
/// All string parameters are UTF-8. Strings returned from Rust must be freed
/// with BarqFreeString(). Opaque handles must be freed with their _Free counterparts.
/// </summary>
internal static partial class NativeMethods
{
    private const string Lib = "barq_chat_form_c";

    // ── Memory ────────────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_free_string")]
    internal static partial void BarqFreeString(IntPtr s);

    // ── CssFramework ──────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_css_bootstrap")]
    internal static partial IntPtr CssBootstrap();

    [LibraryImport(Lib, EntryPoint = "barq_css_tailwind")]
    internal static partial IntPtr CssTailwind();

    [LibraryImport(Lib, EntryPoint = "barq_css_custom")]
    internal static partial IntPtr CssCustom();

    [LibraryImport(Lib, EntryPoint = "barq_css_framework_free")]
    internal static partial void CssFrameworkFree(IntPtr fw);

    // ── CssProfile ────────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_css_profile_new")]
    internal static partial IntPtr CssProfileNew(IntPtr fw);

    [LibraryImport(Lib, EntryPoint = "barq_css_profile_from_json",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr CssProfileFromJson(string json);

    [LibraryImport(Lib, EntryPoint = "barq_css_profile_override",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr CssProfileOverride(IntPtr profile, string overridesJson);

    [LibraryImport(Lib, EntryPoint = "barq_css_profile_free")]
    internal static partial void CssProfileFree(IntPtr profile);

    // ── Form lifecycle ────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_form_new",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormNew(string? id);

    [LibraryImport(Lib, EntryPoint = "barq_form_free")]
    internal static partial void FormFree(IntPtr form);

    // ── Form builder ──────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_form_title",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormTitle(IntPtr form, string text);

    [LibraryImport(Lib, EntryPoint = "barq_form_description",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormDescription(IntPtr form, string text);

    [LibraryImport(Lib, EntryPoint = "barq_form_css_framework")]
    internal static partial IntPtr FormCssFramework(IntPtr form, IntPtr fw);

    [LibraryImport(Lib, EntryPoint = "barq_form_css_profile")]
    internal static partial IntPtr FormCssProfile(IntPtr form, IntPtr profile);

    [LibraryImport(Lib, EntryPoint = "barq_form_step",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormStep(IntPtr form, string? title);

    [LibraryImport(Lib, EntryPoint = "barq_form_submit_label",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormSubmitLabel(IntPtr form, string text);

    [LibraryImport(Lib, EntryPoint = "barq_form_success_message",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormSuccessMessage(IntPtr form, string text);

    [LibraryImport(Lib, EntryPoint = "barq_form_build")]
    internal static partial IntPtr FormBuild(IntPtr form);

    [LibraryImport(Lib, EntryPoint = "barq_form_schema_json")]
    internal static partial IntPtr FormSchemaJson(IntPtr form);

    // ── Field methods ─────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_form_text",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormText(IntPtr form, string id, string label,
        string? placeholder, int required, string? helpText,
        string? defaultVal, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_email",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormEmail(IntPtr form, string id, string label,
        int required, string? helpText, string? defaultVal,
        string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_number",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormNumber(IntPtr form, string id, string label,
        double min, double max, int required, string? helpText,
        string? defaultVal, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_textarea",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormTextarea(IntPtr form, string id, string label,
        uint rows, string? placeholder, int required, string? helpText,
        string? defaultVal, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_select",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormSelect(IntPtr form, string id, string label,
        string? optionsJson, int required, string? helpText,
        string? defaultVal, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_multi_select",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormMultiSelect(IntPtr form, string id, string label,
        string? optionsJson, int required, string? helpText,
        string? defaultValJson, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_checkbox",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormCheckbox(IntPtr form, string id, string label,
        int defaultVal, string? helpText, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_radio",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormRadio(IntPtr form, string id, string label,
        string? optionsJson, int required, string? helpText,
        string? defaultVal, string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_date",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormDate(IntPtr form, string id, string label,
        int required, string? helpText, string? defaultVal,
        string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_range",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormRange(IntPtr form, string id, string label,
        double min, double max, double step, double defaultVal,
        string? helpText, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_file",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormFile(IntPtr form, string id, string label,
        string? acceptJson, int required, string? helpText,
        string? rulesJson, string? showIfJson);

    [LibraryImport(Lib, EntryPoint = "barq_form_hidden",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr FormHidden(IntPtr form, string id, string value);

    // ── Rule helpers ──────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_rule_required",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleRequired(string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_min_length",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleMinLength(uint n, string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_max_length",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleMaxLength(uint n, string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_min",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleMin(double n, string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_max",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleMax(double n, string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_regex",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleRegex(string pattern, string? message);

    [LibraryImport(Lib, EntryPoint = "barq_rule_email",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr RuleEmail(string? message);

    // ── Condition helper ──────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_condition",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr Condition(string fieldId, string @operator, string valueJson);

    // ── Parser ────────────────────────────────────────────────────────────────
    [LibraryImport(Lib, EntryPoint = "barq_parse",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial IntPtr Parse(string message);

    [LibraryImport(Lib, EntryPoint = "barq_is_barq",
        StringMarshalling = StringMarshalling.Utf8)]
    internal static partial int IsBarq(string message);

    [LibraryImport(Lib, EntryPoint = "barq_result_free")]
    internal static partial void ResultFree(IntPtr result);

    [LibraryImport(Lib, EntryPoint = "barq_result_form_id")]
    internal static partial IntPtr ResultFormId(IntPtr result);

    [LibraryImport(Lib, EntryPoint = "barq_result_data_json")]
    internal static partial IntPtr ResultDataJson(IntPtr result);

    [LibraryImport(Lib, EntryPoint = "barq_result_typed_data_json")]
    internal static partial IntPtr ResultTypedDataJson(IntPtr result);

    [LibraryImport(Lib, EntryPoint = "barq_result_as_text")]
    internal static partial IntPtr ResultAsText(IntPtr result);

    // ── Utility ───────────────────────────────────────────────────────────────
    /// <summary>
    /// Convert a Rust-allocated C string pointer to a managed string and free it.
    /// </summary>
    internal static string ConsumeString(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero) return string.Empty;
        var s = Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
        BarqFreeString(ptr);
        return s;
    }
}
