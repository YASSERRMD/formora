package barq

/*
#include "barq_chat_form.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"unsafe"
)

// FormResult holds a parsed form submission.
type FormResult struct {
	ptr *C.BarqFormResult
}

// ParseMessage parses a __barq__{...} message.
// Returns nil if the message is not a barq submission.
func ParseMessage(message string) *FormResult {
	s := cStr(message)
	defer C.free(unsafe.Pointer(s))
	ptr := C.barq_parse(s)
	if ptr == nil {
		return nil
	}
	return &FormResult{ptr: ptr}
}

// IsFormora returns true if message is a barq submission message.
func IsBarq(message string) bool {
	s := cStr(message)
	defer C.free(unsafe.Pointer(s))
	return C.barq_is_barq(s) == 1
}

// Free releases the underlying Rust handle.
func (r *FormResult) Free() {
	if r.ptr != nil {
		C.barq_result_free(r.ptr)
		r.ptr = nil
	}
}

// FormID returns the form ID this result belongs to.
func (r *FormResult) FormID() string {
	return goStr(C.barq_result_form_id(r.ptr))
}

// Data returns the raw form data as a map[string]any.
func (r *FormResult) Data() map[string]any {
	jsonStr := goStr(C.barq_result_data_json(r.ptr))
	var m map[string]any
	json.Unmarshal([]byte(jsonStr), &m) //nolint:errcheck
	return m
}

// TypedData returns the type-coerced form data as a map[string]any.
func (r *FormResult) TypedData() map[string]any {
	jsonStr := goStr(C.barq_result_typed_data_json(r.ptr))
	var m map[string]any
	json.Unmarshal([]byte(jsonStr), &m) //nolint:errcheck
	return m
}

// AsText returns a human-readable summary of the result.
func (r *FormResult) AsText() string {
	return goStr(C.barq_result_as_text(r.ptr))
}
