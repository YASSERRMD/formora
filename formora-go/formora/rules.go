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

// Rule is a JSON-encoded validation rule string understood by formora-c.
type Rule string

func ruleFromC(s *C.char) Rule { return Rule(goStr(s)) }

// RuleRequired returns a "field must have a value" rule.
func RuleRequired(message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_required(msg))
}

// RuleMinLength returns a minimum-string-length rule.
func RuleMinLength(n uint32, message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_min_length(C.uint(n), msg))
}

// RuleMaxLength returns a maximum-string-length rule.
func RuleMaxLength(n uint32, message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_max_length(C.uint(n), msg))
}

// RuleMin returns a minimum-numeric-value rule.
func RuleMin(n float64, message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_min(C.double(n), msg))
}

// RuleMax returns a maximum-numeric-value rule.
func RuleMax(n float64, message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_max(C.double(n), msg))
}

// RuleRegex returns a regex-pattern rule.
func RuleRegex(pattern string, message *string) Rule {
	p := cStr(pattern)
	defer C.free(unsafe.Pointer(p))
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_regex(p, msg))
}

// RuleEmail returns an email-format rule.
func RuleEmail(message *string) Rule {
	msg := cStrOpt(message)
	defer func() {
		if msg != nil {
			C.free(unsafe.Pointer(msg))
		}
	}()
	return ruleFromC(C.formora_rule_email(msg))
}

// rulesJSON serialises a slice of Rule values to a JSON array string for the C API.
func rulesJSON(rules []Rule) *C.char {
	if len(rules) == 0 {
		return nil
	}
	raw := make([]json.RawMessage, len(rules))
	for i, r := range rules {
		raw[i] = json.RawMessage(r)
	}
	data, _ := json.Marshal(raw)
	return cStr(string(data))
}

// ── Condition ─────────────────────────────────────────────────────────────────

// Condition is a JSON-encoded visibility condition string.
type Condition string

// NewCondition creates a condition: show field when fieldID operator value.
// Operators: "eq" | "neq" | "contains" | "gt" | "lt" | "in_list"
// value must be a JSON-serialisable Go value (string, number, bool, []string).
func NewCondition(fieldID, operator string, value any) Condition {
	valJSON, _ := json.Marshal(value)
	fid := cStr(fieldID)
	op := cStr(operator)
	val := cStr(string(valJSON))
	defer func() {
		C.free(unsafe.Pointer(fid))
		C.free(unsafe.Pointer(op))
		C.free(unsafe.Pointer(val))
	}()
	return Condition(goStr(C.formora_condition(fid, op, val)))
}

// conditionC returns a C string for the condition, or nil if zero value.
func conditionC(c *Condition) *C.char {
	if c == nil || *c == "" {
		return nil
	}
	return cStr(string(*c))
}
