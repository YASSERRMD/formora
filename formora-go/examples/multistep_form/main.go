// Multi-step onboarding form example.
// Go equivalent of examples/multistep_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/formora/go/formora"
)

func main() {
	form := formora.NewForm("onboarding")
	defer form.Free()

	form.
		Title("Welcome to Formora").
		Description("Complete your profile in 3 easy steps.").
		CSSFramework(formora.Tailwind()).
		// Step 1
		Step("Personal Information").
		Text(formora.TextField{
			ID: "full_name", Label: "Full Name", Required: true,
			Rules: []formora.Rule{formora.RuleRequired(nil), formora.RuleMinLength(2, nil)},
		}).
		Email(formora.EmailField{
			ID: "email", Label: "Work Email", Required: true,
			Rules: []formora.Rule{formora.RuleRequired(nil), formora.RuleEmail(nil)},
		}).
		Text(formora.TextField{ID: "company", Label: "Company", Required: true}).
		// Step 2
		Step("Role & Experience").
		Select(formora.SelectField{
			ID: "role", Label: "Your Role", Required: true,
			Options: []formora.Option{
				{Label: "Developer", Value: "developer"},
				{Label: "Designer", Value: "designer"},
				{Label: "Product Manager", Value: "pm"},
				{Label: "Data Scientist", Value: "data_scientist"},
				{Label: "Other", Value: "other"},
			},
		}).
		Range(formora.RangeField{
			ID: "years_exp", Label: "Years of Experience",
			Min: 0, Max: 30, Step: formora.F64(1), Default: formora.F64(0),
		}).
		MultiSelect(formora.MultiSelectField{
			ID: "technologies", Label: "Technologies You Use",
			Options: []formora.Option{
				{Label: "TypeScript", Value: "typescript"},
				{Label: "Rust", Value: "rust"},
				{Label: "Python", Value: "python"},
				{Label: "Go", Value: "go"},
				{Label: "C#", Value: "csharp"},
			},
		}).
		// Step 3
		Step("Preferences").
		Radio(formora.RadioField{
			ID: "frequency", Label: "How often do you ship?", Required: true,
			Options: []formora.Option{
				{Label: "Daily", Value: "daily"},
				{Label: "Weekly", Value: "weekly"},
				{Label: "Monthly", Value: "monthly"},
			},
		}).
		Checkbox(formora.CheckboxField{
			ID: "agree_terms", Label: "I agree to the Terms of Service",
			Rules: []formora.Rule{formora.RuleRequired(formora.Ptr("You must accept the terms"))},
		}).
		Checkbox(formora.CheckboxField{
			ID: "agree_privacy", Label: "I agree to the Privacy Policy",
			Rules: []formora.Rule{formora.RuleRequired(formora.Ptr("You must accept the privacy policy"))},
		}).
		SubmitLabel("Complete Setup").
		SuccessMessage("You're all set! Welcome aboard.")

	fmt.Println(form.Build())
}
