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

// FormResult holds a parsed form submission.
type FormResult struct {
	ptr *C.FormoraFormResult
}

// ParseMessage parses a __formora__{...} message.
// Returns nil if the message is not a formora submission.
func ParseMessage(message string) *FormResult {
	s := cStr(message)
	defer C.free(unsafe.Pointer(s))
	ptr := C.formora_parse(s)
	if ptr == nil {
		return nil
	}
	return &FormResult{ptr: ptr}
}

// IsFormora returns true if message is a formora submission message.
func IsFormora(message string) bool {
	s := cStr(message)
	defer C.free(unsafe.Pointer(s))
	return C.formora_is_formora(s) == 1
}

// Free releases the underlying Rust handle.
func (r *FormResult) Free() {
	if r.ptr != nil {
		C.formora_result_free(r.ptr)
		r.ptr = nil
	}
}

// FormID returns the form ID this result belongs to.
func (r *FormResult) FormID() string {
	return goStr(C.formora_result_form_id(r.ptr))
}

// Data returns the raw form data as a map[string]any.
func (r *FormResult) Data() map[string]any {
	jsonStr := goStr(C.formora_result_data_json(r.ptr))
	var m map[string]any
	json.Unmarshal([]byte(jsonStr), &m) //nolint:errcheck
	return m
}

// TypedData returns the type-coerced form data as a map[string]any.
func (r *FormResult) TypedData() map[string]any {
	jsonStr := goStr(C.formora_result_typed_data_json(r.ptr))
	var m map[string]any
	json.Unmarshal([]byte(jsonStr), &m) //nolint:errcheck
	return m
}

// AsText returns a human-readable summary of the result.
func (r *FormResult) AsText() string {
	return goStr(C.formora_result_as_text(r.ptr))
}
