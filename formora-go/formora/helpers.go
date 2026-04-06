package formora

/*
#include "formora.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

// cStr converts a Go string to a C string. The caller must free it with C.free.
func cStr(s string) *C.char { return C.CString(s) }

// cStrOpt converts a *string to a C string (nil → NULL). The caller must free.
func cStrOpt(s *string) *C.char {
	if s == nil {
		return nil
	}
	return C.CString(*s)
}

// goStr converts a C string returned by formora into a Go string and frees it.
func goStr(s *C.char) string {
	if s == nil {
		return ""
	}
	v := C.GoString(s)
	C.formora_free_string(s)
	return v
}

// strPtr is a helper that returns a pointer to a string literal.
func strPtr(s string) *string { return &s }
