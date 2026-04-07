// Multi-step onboarding form example.
// Go equivalent of examples/multistep_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/barq-chat-form/go/barq"
)

func main() {
	form := barq.NewForm("onboarding")
	defer form.Free()

	form.
		Title("Welcome to Barq Chat Form").
		Description("Complete your profile in 3 easy steps.").
		CSSFramework(barq.Tailwind()).
		// Step 1
		Step("Personal Information").
		Text(barq.TextField{
			ID: "full_name", Label: "Full Name", Required: true,
			Rules: []barq.Rule{barq.RuleRequired(nil), barq.RuleMinLength(2, nil)},
		}).
		Email(barq.EmailField{
			ID: "email", Label: "Work Email", Required: true,
			Rules: []barq.Rule{barq.RuleRequired(nil), barq.RuleEmail(nil)},
		}).
		Text(barq.TextField{ID: "company", Label: "Company", Required: true}).
		// Step 2
		Step("Role & Experience").
		Select(barq.SelectField{
			ID: "role", Label: "Your Role", Required: true,
			Options: []barq.Option{
				{Label: "Developer", Value: "developer"},
				{Label: "Designer", Value: "designer"},
				{Label: "Product Manager", Value: "pm"},
				{Label: "Data Scientist", Value: "data_scientist"},
				{Label: "Other", Value: "other"},
			},
		}).
		Range(barq.RangeField{
			ID: "years_exp", Label: "Years of Experience",
			Min: 0, Max: 30, Step: barq.F64(1), Default: barq.F64(0),
		}).
		MultiSelect(barq.MultiSelectField{
			ID: "technologies", Label: "Technologies You Use",
			Options: []barq.Option{
				{Label: "TypeScript", Value: "typescript"},
				{Label: "Rust", Value: "rust"},
				{Label: "Python", Value: "python"},
				{Label: "Go", Value: "go"},
				{Label: "C#", Value: "csharp"},
			},
		}).
		// Step 3
		Step("Preferences").
		Radio(barq.RadioField{
			ID: "frequency", Label: "How often do you ship?", Required: true,
			Options: []barq.Option{
				{Label: "Daily", Value: "daily"},
				{Label: "Weekly", Value: "weekly"},
				{Label: "Monthly", Value: "monthly"},
			},
		}).
		Checkbox(barq.CheckboxField{
			ID: "agree_terms", Label: "I agree to the Terms of Service",
			Rules: []barq.Rule{barq.RuleRequired(barq.Ptr("You must accept the terms"))},
		}).
		Checkbox(barq.CheckboxField{
			ID: "agree_privacy", Label: "I agree to the Privacy Policy",
			Rules: []barq.Rule{barq.RuleRequired(barq.Ptr("You must accept the privacy policy"))},
		}).
		SubmitLabel("Complete Setup").
		SuccessMessage("You're all set! Welcome aboard.")

	fmt.Println(form.Build())
}
