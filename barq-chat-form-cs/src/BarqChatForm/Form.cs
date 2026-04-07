using BarqChatForm.Native;
using System.Text.Json;

namespace BarqChatForm;

/// <summary>A label/value pair for select, radio, and multi-select fields.</summary>
public record SelectOption(string Label, string Value);

/// <summary>
/// Fluent form builder. Mirrors Python's Form / Go's Form.
/// Dispose when done to release the native handle.
/// </summary>
public sealed class Form : IDisposable
{
    private IntPtr _handle;

    /// <param name="id">Form ID. Null or empty → auto-generated UUID.</param>
    public Form(string? id = null)
        => _handle = NativeMethods.FormNew(string.IsNullOrEmpty(id) ? null : id);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero) { NativeMethods.FormFree(_handle); _handle = IntPtr.Zero; }
    }

    // ── Builder ───────────────────────────────────────────────────────────────

    public Form Title(string text)           { NativeMethods.FormTitle(_handle, text); return this; }
    public Form Description(string text)     { NativeMethods.FormDescription(_handle, text); return this; }
    public Form SubmitLabel(string text)     { NativeMethods.FormSubmitLabel(_handle, text); return this; }
    public Form SuccessMessage(string text)  { NativeMethods.FormSuccessMessage(_handle, text); return this; }

    public Form Css(CssFramework fw)  { NativeMethods.FormCssFramework(_handle, fw.Handle); return this; }
    public Form Css(CssProfile p)     { NativeMethods.FormCssProfile(_handle, p.Handle); return this; }

    /// <summary>Add a step (enables multi-step mode).</summary>
    public Form Step(string? title = null) { NativeMethods.FormStep(_handle, title); return this; }

    /// <summary>Render the form to an HTML string.</summary>
    public string Build() => NativeMethods.ConsumeString(NativeMethods.FormBuild(_handle));

    /// <summary>Return the form schema as a JSON string (used by LLM utilities).</summary>
    public string SchemaJson() => NativeMethods.ConsumeString(NativeMethods.FormSchemaJson(_handle));

    // ── Fields ────────────────────────────────────────────────────────────────

    public Form Text(string id, string label,
        string? placeholder = null, bool required = false,
        string? helpText = null, string? defaultValue = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormText(_handle, id, label, placeholder, B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Email(string id, string label,
        bool required = false, string? helpText = null,
        string? defaultValue = null, IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormEmail(_handle, id, label, B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Number(string id, string label,
        double min = double.NaN, double max = double.NaN,
        bool required = false, string? helpText = null,
        double? defaultValue = null, IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormNumber(_handle, id, label, min, max, B(required),
            helpText, defaultValue?.ToString(), Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Textarea(string id, string label,
        uint rows = 0, string? placeholder = null, bool required = false,
        string? helpText = null, string? defaultValue = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormTextarea(_handle, id, label, rows, placeholder, B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Select(string id, string label,
        IEnumerable<SelectOption> options, bool required = false,
        string? helpText = null, string? defaultValue = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormSelect(_handle, id, label, OptionsJson(options), B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form MultiSelect(string id, string label,
        IEnumerable<SelectOption> options, bool required = false,
        string? helpText = null, IEnumerable<string>? defaultValues = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        string? dvJson = defaultValues != null ? JsonSerializer.Serialize(defaultValues) : null;
        NativeMethods.FormMultiSelect(_handle, id, label, OptionsJson(options), B(required),
            helpText, dvJson, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Checkbox(string id, string label,
        bool defaultValue = false, string? helpText = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormCheckbox(_handle, id, label, B(defaultValue),
            helpText, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Radio(string id, string label,
        IEnumerable<SelectOption> options, bool required = false,
        string? helpText = null, string? defaultValue = null,
        IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormRadio(_handle, id, label, OptionsJson(options), B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Date(string id, string label,
        bool required = false, string? helpText = null,
        string? defaultValue = null, IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        NativeMethods.FormDate(_handle, id, label, B(required),
            helpText, defaultValue, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Range(string id, string label,
        double min, double max, double step = double.NaN,
        double defaultValue = double.NaN, string? helpText = null, Condition? showIf = null)
    {
        NativeMethods.FormRange(_handle, id, label, min, max, step, defaultValue,
            helpText, showIf?.Json);
        return this;
    }

    public Form File(string id, string label,
        IEnumerable<string>? accept = null, bool required = false,
        string? helpText = null, IEnumerable<Rule>? rules = null, Condition? showIf = null)
    {
        string? acceptJson = accept != null ? JsonSerializer.Serialize(accept) : null;
        NativeMethods.FormFile(_handle, id, label, acceptJson, B(required),
            helpText, Rule.ToJsonArray(rules), showIf?.Json);
        return this;
    }

    public Form Hidden(string id, string value)
    {
        NativeMethods.FormHidden(_handle, id, value);
        return this;
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    private static int B(bool v) => v ? 1 : 0;

    private static string OptionsJson(IEnumerable<SelectOption> opts)
        => JsonSerializer.Serialize(opts.Select(o => new[] { o.Label, o.Value }));
}
