package formora

/*
#include "formora.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"unsafe"
)

// ── CssFramework ──────────────────────────────────────────────────────────────

// CssFramework selects a CSS preset for form styling.
type CssFramework struct {
	ptr *C.FormoraCssFramework
}

// Bootstrap returns a Bootstrap 5 CSS framework.
func Bootstrap() *CssFramework {
	return &CssFramework{ptr: C.formora_css_bootstrap()}
}

// Tailwind returns a Tailwind CSS v3 framework.
func Tailwind() *CssFramework {
	return &CssFramework{ptr: C.formora_css_tailwind()}
}

// Custom returns a minimal custom styles framework.
func Custom() *CssFramework {
	return &CssFramework{ptr: C.formora_css_custom()}
}

// Free releases the underlying Rust handle.
func (f *CssFramework) Free() {
	if f.ptr != nil {
		C.formora_css_framework_free(f.ptr)
		f.ptr = nil
	}
}

// ── CssProfile ───────────────────────────────────────────────────────────────

// CssProfile holds 43 customisable CSS class names.
type CssProfile struct {
	ptr *C.FormoraCssProfile
}

// NewCssProfile creates a profile from a CssFramework (nil → Bootstrap default).
func NewCssProfile(fw *CssFramework) *CssProfile {
	var fwPtr *C.FormoraCssFramework
	if fw != nil {
		fwPtr = fw.ptr
	}
	return &CssProfile{ptr: C.formora_css_profile_new(fwPtr)}
}

// CssProfileFromMap creates a profile from a map of CSS class keys to values.
func CssProfileFromMap(classes map[string]string) *CssProfile {
	data, _ := json.Marshal(classes)
	s := cStr(string(data))
	defer C.free(unsafe.Pointer(s))
	return &CssProfile{ptr: C.formora_css_profile_from_json(s)}
}

// Override returns a new profile with specific class names replaced.
func (p *CssProfile) Override(overrides map[string]string) *CssProfile {
	data, _ := json.Marshal(overrides)
	s := cStr(string(data))
	defer C.free(unsafe.Pointer(s))
	return &CssProfile{ptr: C.formora_css_profile_override(p.ptr, s)}
}

// Free releases the underlying Rust handle.
func (p *CssProfile) Free() {
	if p.ptr != nil {
		C.formora_css_profile_free(p.ptr)
		p.ptr = nil
	}
}
