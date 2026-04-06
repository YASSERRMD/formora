using Formora.Native;
using System.Runtime.InteropServices;
using System.Text.Json;

namespace Formora;

/// <summary>A JSON-encoded validation rule. Build with the static factory methods.</summary>
public readonly struct Rule
{
    internal string Json { get; }
    private Rule(string json) => Json = json;

    private static Rule FromPtr(IntPtr ptr) => new(NativeMethods.ConsumeString(ptr));

    /// <summary>Field must have a value.</summary>
    public static Rule Required(string? message = null) => FromPtr(NativeMethods.RuleRequired(message));

    /// <summary>String must be at least <paramref name="n"/> characters.</summary>
    public static Rule MinLength(uint n, string? message = null) => FromPtr(NativeMethods.RuleMinLength(n, message));

    /// <summary>String must be at most <paramref name="n"/> characters.</summary>
    public static Rule MaxLength(uint n, string? message = null) => FromPtr(NativeMethods.RuleMaxLength(n, message));

    /// <summary>Numeric value must be ≥ <paramref name="n"/>.</summary>
    public static Rule Min(double n, string? message = null) => FromPtr(NativeMethods.RuleMin(n, message));

    /// <summary>Numeric value must be ≤ <paramref name="n"/>.</summary>
    public static Rule Max(double n, string? message = null) => FromPtr(NativeMethods.RuleMax(n, message));

    /// <summary>Value must match the given regex pattern.</summary>
    public static Rule Regex(string pattern, string? message = null) => FromPtr(NativeMethods.RuleRegex(pattern, message));

    /// <summary>Value must be a valid email address.</summary>
    public static Rule Email(string? message = null) => FromPtr(NativeMethods.RuleEmail(message));

    internal static string? ToJsonArray(IEnumerable<Rule>? rules)
    {
        if (rules == null) return null;
        var list = rules.Select(r => JsonDocument.Parse(r.Json).RootElement).ToList();
        return list.Count == 0 ? null : JsonSerializer.Serialize(list);
    }
}

/// <summary>Defines a conditional visibility rule for a field.</summary>
public readonly struct Condition
{
    internal string Json { get; }

    /// <summary>
    /// Create a condition: show the field when <paramref name="fieldId"/> <paramref name="op"/> <paramref name="value"/>.
    /// <para>Operators: "eq" | "neq" | "contains" | "gt" | "lt" | "in_list"</para>
    /// </summary>
    public Condition(string fieldId, string op, object value)
    {
        var valueJson = JsonSerializer.Serialize(value);
        Json = NativeMethods.ConsumeString(NativeMethods.Condition(fieldId, op, valueJson));
    }
}
