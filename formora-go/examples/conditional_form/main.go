// Conditional field visibility example.
// Go equivalent of examples/conditional_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/formora/go/formora"
)

func main() {
	isVIP := formora.NewCondition("ticket_type", "eq", "vip")
	isStudent := formora.NewCondition("ticket_type", "eq", "student")

	form := formora.NewForm("event-registration")
	defer form.Free()

	form.
		Title("Event Registration").
		Description("Register for the annual developer conference.").
		CSSFramework(formora.Bootstrap()).
		Text(formora.TextField{
			ID: "full_name", Label: "Full Name", Required: true,
			Rules: []formora.Rule{formora.RuleRequired(nil)},
		}).
		Email(formora.EmailField{
			ID: "email", Label: "Email Address", Required: true,
			Rules: []formora.Rule{formora.RuleRequired(nil), formora.RuleEmail(nil)},
		}).
		Select(formora.SelectField{
			ID: "ticket_type", Label: "Ticket Type", Required: true,
			Options: []formora.Option{
				{Label: "General Admission", Value: "general"},
				{Label: "VIP", Value: "vip"},
				{Label: "Student", Value: "student"},
			},
		}).
		// VIP only
		Select(formora.SelectField{
			ID: "dietary", Label: "Dietary Restrictions",
			HelpText: formora.Ptr("Required for VIP dinner seating"),
			Options: []formora.Option{
				{Label: "None", Value: "none"},
				{Label: "Vegetarian", Value: "vegetarian"},
				{Label: "Vegan", Value: "vegan"},
				{Label: "Gluten-Free", Value: "gluten_free"},
			},
			ShowIf: &isVIP,
		}).
		// Student only
		Text(formora.TextField{
			ID: "student_id", Label: "Student ID",
			Placeholder: formora.Ptr("Your university student ID"), Required: true,
			HelpText: formora.Ptr("Required for student discount verification"),
			Rules:    []formora.Rule{formora.RuleRequired(formora.Ptr("Student ID is required for student tickets"))},
			ShowIf:   &isStudent,
		}).
		// VIP only
		Text(formora.TextField{
			ID: "company", Label: "Company Name",
			Placeholder: formora.Ptr("Your company or organisation"),
			HelpText:    formora.Ptr("For badge and networking"),
			ShowIf:      &isVIP,
		}).
		SubmitLabel("Register Now").
		SuccessMessage("Registration complete! Check your email for confirmation.")

	fmt.Println(form.Build())
}
