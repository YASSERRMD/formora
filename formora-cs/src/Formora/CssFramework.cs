using Formora.Native;
using System.Text.Json;

namespace Formora;

/// <summary>Selects a CSS preset for form styling.</summary>
public sealed class CssFramework : IDisposable
{
    internal IntPtr Handle { get; private set; }

    private CssFramework(IntPtr handle) => Handle = handle;

    /// <summary>Bootstrap 5 preset.</summary>
    public static CssFramework Bootstrap() => new(NativeMethods.CssBootstrap());

    /// <summary>Tailwind CSS v3 preset.</summary>
    public static CssFramework Tailwind() => new(NativeMethods.CssTailwind());

    /// <summary>Minimal custom styles preset.</summary>
    public static CssFramework Custom() => new(NativeMethods.CssCustom());

    public void Dispose()
    {
        if (Handle != IntPtr.Zero) { NativeMethods.CssFrameworkFree(Handle); Handle = IntPtr.Zero; }
    }
}

/// <summary>Holds 43 customisable CSS class names for form styling.</summary>
public sealed class CssProfile : IDisposable
{
    internal IntPtr Handle { get; private set; }

    private CssProfile(IntPtr handle) => Handle = handle;

    /// <summary>Create a profile from a CssFramework (null → Bootstrap).</summary>
    public static CssProfile FromFramework(CssFramework? fw = null)
        => new(NativeMethods.CssProfileNew(fw?.Handle ?? IntPtr.Zero));

    /// <summary>Create a profile from a dictionary of CSS class name overrides.</summary>
    public static CssProfile FromDictionary(Dictionary<string, string> classes)
        => new(NativeMethods.CssProfileFromJson(JsonSerializer.Serialize(classes)));

    /// <summary>Return a new profile with specific keys overridden.</summary>
    public CssProfile Override(Dictionary<string, string> overrides)
        => new(NativeMethods.CssProfileOverride(Handle, JsonSerializer.Serialize(overrides)));

    public void Dispose()
    {
        if (Handle != IntPtr.Zero) { NativeMethods.CssProfileFree(Handle); Handle = IntPtr.Zero; }
    }
}
