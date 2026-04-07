package barq

/*
#include "barq_chat_form.h"
#include <stdlib.h>
#include <math.h>
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"math"
	"unsafe"
)

// Form is a fluent form builder backed by a barq-chat-form-c handle.
type Form struct {
	ptr *C.BarqForm
}

// NewForm creates a new form. Pass an empty string for id to auto-generate a UUID.
func NewForm(id string) *Form {
	var cidPtr *C.char
	if id != "" {
		cidPtr = cStr(id)
		defer C.free(unsafe.Pointer(cidPtr))
	}
	return &Form{ptr: C.barq_form_new(cidPtr)}
}

// Free releases the underlying Rust handle.
func (f *Form) Free() {
	if f.ptr != nil {
		C.barq_form_free(f.ptr)
		f.ptr = nil
	}
}

// Title sets the form title.
func (f *Form) Title(text string) *Form {
	s := cStr(text)
	defer C.free(unsafe.Pointer(s))
	C.barq_form_title(f.ptr, s)
	return f
}

// Description sets the form description.
func (f *Form) Description(text string) *Form {
	s := cStr(text)
	defer C.free(unsafe.Pointer(s))
	C.barq_form_description(f.ptr, s)
	return f
}

// CSSFramework sets the CSS framework (Bootstrap, Tailwind, or Custom).
func (f *Form) CSSFramework(fw *CssFramework) *Form {
	C.barq_form_css_framework(f.ptr, fw.ptr)
	return f
}

// CSSProfile sets a custom CSS profile.
func (f *Form) CSSProfile(p *CssProfile) *Form {
	C.barq_form_css_profile(f.ptr, p.ptr)
	return f
}

// Step adds a step (enables multi-step mode). Pass empty string for no title.
func (f *Form) Step(title string) *Form {
	var s *C.char
	if title != "" {
		s = cStr(title)
		defer C.free(unsafe.Pointer(s))
	}
	C.barq_form_step(f.ptr, s)
	return f
}

// SubmitLabel sets the submit button label.
func (f *Form) SubmitLabel(text string) *Form {
	s := cStr(text)
	defer C.free(unsafe.Pointer(s))
	C.barq_form_submit_label(f.ptr, s)
	return f
}

// SuccessMessage sets the message shown after submission.
func (f *Form) SuccessMessage(text string) *Form {
	s := cStr(text)
	defer C.free(unsafe.Pointer(s))
	C.barq_form_success_message(f.ptr, s)
	return f
}

// Build renders the form to an HTML string.
func (f *Form) Build() string {
	return goStr(C.barq_form_build(f.ptr))
}

// SchemaJSON returns the form schema as a JSON string.
func (f *Form) SchemaJSON() string {
	return goStr(C.barq_form_schema_json(f.ptr))
}

// ── Field option types ─────────────────────────────────────────────────────────

// Option represents a [label, value] pair for select/radio/multi-select fields.
type Option struct {
	Label string
	Value string
}

func optionsJSON(opts []Option) *C.char {
	if len(opts) == 0 {
		return nil
	}
	pairs := make([][2]string, len(opts))
	for i, o := range opts {
		pairs[i] = [2]string{o.Label, o.Value}
	}
	data, _ := json.Marshal(pairs)
	return cStr(string(data))
}

// ── Field methods ──────────────────────────────────────────────────────────────

// TextField configures a single-line text input.
type TextField struct {
	ID          string
	Label       string
	Placeholder *string
	Required    bool
	HelpText    *string
	Default     *string
	Rules       []Rule
	ShowIf      *Condition
}

// Text adds a text input field.
func (f *Form) Text(cfg TextField) *Form {
	ph := cStrOpt(cfg.Placeholder)
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ph); freeIfNotNil(ht); freeIfNotNil(dv)
		freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_text(f.ptr, id, lb, ph, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// EmailField configures an email input.
type EmailField struct {
	ID       string
	Label    string
	Required bool
	HelpText *string
	Default  *string
	Rules    []Rule
	ShowIf   *Condition
}

// Email adds an email input field.
func (f *Form) Email(cfg EmailField) *Form {
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_email(f.ptr, id, lb, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// NumberField configures a number input.
type NumberField struct {
	ID       string
	Label    string
	Min      *float64
	Max      *float64
	Required bool
	HelpText *string
	Default  *float64
	Rules    []Rule
	ShowIf   *Condition
}

// Number adds a number input field.
func (f *Form) Number(cfg NumberField) *Form {
	ht := cStrOpt(cfg.HelpText)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	var dv *C.char
	if cfg.Default != nil {
		s := cStr(fmt.Sprintf("%g", *cfg.Default))
		dv = s
	}
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ht); freeIfNotNil(ru); freeIfNotNil(si); freeIfNotNil(dv)
	}()
	minVal := math.NaN()
	maxVal := math.NaN()
	if cfg.Min != nil { minVal = *cfg.Min }
	if cfg.Max != nil { maxVal = *cfg.Max }
	C.barq_form_number(f.ptr, id, lb, C.double(minVal), C.double(maxVal),
		boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// TextareaField configures a multi-line textarea.
type TextareaField struct {
	ID          string
	Label       string
	Rows        uint32
	Placeholder *string
	Required    bool
	HelpText    *string
	Default     *string
	Rules       []Rule
	ShowIf      *Condition
}

// Textarea adds a textarea field.
func (f *Form) Textarea(cfg TextareaField) *Form {
	ph := cStrOpt(cfg.Placeholder)
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ph); freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_textarea(f.ptr, id, lb, C.uint(cfg.Rows), ph, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// SelectField configures a dropdown.
type SelectField struct {
	ID       string
	Label    string
	Options  []Option
	Required bool
	HelpText *string
	Default  *string
	Rules    []Rule
	ShowIf   *Condition
}

// Select adds a dropdown select field.
func (f *Form) Select(cfg SelectField) *Form {
	opts := optionsJSON(cfg.Options)
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(opts); freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_select(f.ptr, id, lb, opts, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// MultiSelectField configures a multi-select.
type MultiSelectField struct {
	ID       string
	Label    string
	Options  []Option
	Required bool
	HelpText *string
	Default  []string
	Rules    []Rule
	ShowIf   *Condition
}

// MultiSelect adds a multi-select field.
func (f *Form) MultiSelect(cfg MultiSelectField) *Form {
	opts := optionsJSON(cfg.Options)
	ht := cStrOpt(cfg.HelpText)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	var dv *C.char
	if len(cfg.Default) > 0 {
		data, _ := json.Marshal(cfg.Default)
		dv = cStr(string(data))
	}
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(opts); freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_multi_select(f.ptr, id, lb, opts, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// CheckboxField configures a boolean checkbox.
type CheckboxField struct {
	ID       string
	Label    string
	Default  bool
	HelpText *string
	Rules    []Rule
	ShowIf   *Condition
}

// Checkbox adds a checkbox field.
func (f *Form) Checkbox(cfg CheckboxField) *Form {
	ht := cStrOpt(cfg.HelpText)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ht); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_checkbox(f.ptr, id, lb, boolToInt(cfg.Default), ht, ru, si)
	return f
}

// RadioField configures a radio group.
type RadioField struct {
	ID       string
	Label    string
	Options  []Option
	Required bool
	HelpText *string
	Default  *string
	Rules    []Rule
	ShowIf   *Condition
}

// Radio adds a radio button group.
func (f *Form) Radio(cfg RadioField) *Form {
	opts := optionsJSON(cfg.Options)
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(opts); freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_radio(f.ptr, id, lb, opts, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// DateField configures a date picker.
type DateField struct {
	ID       string
	Label    string
	Required bool
	HelpText *string
	Default  *string
	Rules    []Rule
	ShowIf   *Condition
}

// Date adds a date picker field.
func (f *Form) Date(cfg DateField) *Form {
	ht := cStrOpt(cfg.HelpText)
	dv := cStrOpt(cfg.Default)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ht); freeIfNotNil(dv); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_date(f.ptr, id, lb, boolToInt(cfg.Required), ht, dv, ru, si)
	return f
}

// RangeField configures a range slider.
type RangeField struct {
	ID       string
	Label    string
	Min      float64
	Max      float64
	Step     *float64
	Default  *float64
	HelpText *string
	ShowIf   *Condition
}

// Range adds a range slider field.
func (f *Form) Range(cfg RangeField) *Form {
	ht := cStrOpt(cfg.HelpText)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(ht); freeIfNotNil(si)
	}()
	step := math.NaN()
	if cfg.Step != nil { step = *cfg.Step }
	def := math.NaN()
	if cfg.Default != nil { def = *cfg.Default }
	C.barq_form_range(f.ptr, id, lb, C.double(cfg.Min), C.double(cfg.Max),
		C.double(step), C.double(def), ht, si)
	return f
}

// FileField configures a file upload.
type FileField struct {
	ID       string
	Label    string
	Accept   []string
	Required bool
	HelpText *string
	Rules    []Rule
	ShowIf   *Condition
}

// File adds a file upload field.
func (f *Form) File(cfg FileField) *Form {
	ht := cStrOpt(cfg.HelpText)
	ru := rulesJSON(cfg.Rules)
	si := conditionC(cfg.ShowIf)
	id := cStr(cfg.ID)
	lb := cStr(cfg.Label)
	var acc *C.char
	if len(cfg.Accept) > 0 {
		data, _ := json.Marshal(cfg.Accept)
		acc = cStr(string(data))
	}
	defer func() {
		C.free(unsafe.Pointer(id)); C.free(unsafe.Pointer(lb))
		freeIfNotNil(acc); freeIfNotNil(ht); freeIfNotNil(ru); freeIfNotNil(si)
	}()
	C.barq_form_file(f.ptr, id, lb, acc, boolToInt(cfg.Required), ht, ru, si)
	return f
}

// Hidden adds a hidden field carrying context data.
func (f *Form) Hidden(id, value string) *Form {
	cid := cStr(id)
	cv := cStr(value)
	defer func() { C.free(unsafe.Pointer(cid)); C.free(unsafe.Pointer(cv)) }()
	C.barq_form_hidden(f.ptr, cid, cv)
	return f
}

// ── helpers ───────────────────────────────────────────────────────────────────

func boolToInt(b bool) C.int {
	if b { return 1 }
	return 0
}

func freeIfNotNil(s *C.char) {
	if s != nil { C.free(unsafe.Pointer(s)) }
}
