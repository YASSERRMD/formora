package barq

// Ptr returns a pointer to a string literal.
// Convenience helper for optional string parameters.
func Ptr(s string) *string { return &s }

// F64 returns a pointer to a float64 literal.
// Convenience helper for optional numeric parameters.
func F64(f float64) *float64 { return &f }
