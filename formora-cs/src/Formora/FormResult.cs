using Formora.Native;
using System.Text.Json;

namespace Formora;

/// <summary>A parsed form submission result.</summary>
public sealed class FormResult : IDisposable
{
    private IntPtr _handle;

    internal FormResult(IntPtr handle) => _handle = handle;

    public void Dispose()
    {
        if (_handle != IntPtr.Zero) { NativeMethods.ResultFree(_handle); _handle = IntPtr.Zero; }
    }

    /// <summary>The form ID this result belongs to.</summary>
    public string FormId => NativeMethods.ConsumeString(NativeMethods.ResultFormId(_handle));

    /// <summary>Raw form data (all values as strings).</summary>
    public Dictionary<string, object?> Data
    {
        get
        {
            var json = NativeMethods.ConsumeString(NativeMethods.ResultDataJson(_handle));
            return JsonSerializer.Deserialize<Dictionary<string, object?>>(json) ?? [];
        }
    }

    /// <summary>Type-coerced form data (numbers, booleans, arrays inferred).</summary>
    public Dictionary<string, object?> TypedData
    {
        get
        {
            var json = NativeMethods.ConsumeString(NativeMethods.ResultTypedDataJson(_handle));
            return JsonSerializer.Deserialize<Dictionary<string, object?>>(json) ?? [];
        }
    }

    /// <summary>Human-readable summary of the result.</summary>
    public string AsText() => NativeMethods.ConsumeString(NativeMethods.ResultAsText(_handle));
}

/// <summary>Parse and detect formora messages.</summary>
public static class FormoraParser
{
    /// <summary>
    /// Parse a <c>__formora__{...}</c> message.
    /// Returns null if the message is not a formora submission.
    /// </summary>
    public static FormResult? ParseMessage(string message)
    {
        var ptr = NativeMethods.Parse(message);
        return ptr == IntPtr.Zero ? null : new FormResult(ptr);
    }

    /// <summary>Returns true if the message is a formora submission.</summary>
    public static bool IsFormora(string message) => NativeMethods.IsFormora(message) == 1;
}
