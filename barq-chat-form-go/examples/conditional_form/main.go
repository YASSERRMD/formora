// Conditional field visibility example.
// Go equivalent of examples/conditional_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/barq-chat-form/go/barq"
)

func main() {
	isVIP := barq.NewCondition("ticket_type", "eq", "vip")
	isStudent := barq.NewCondition("ticket_type", "eq", "student")

	form := barq.NewForm("event-registration")
	defer form.Free()

	form.
		Title("Event Registration").
		Description("Register for the annual developer conference.").
		CSSFramework(barq.Bootstrap()).
		Text(barq.TextField{
			ID: "full_name", Label: "Full Name", Required: true,
			Rules: []barq.Rule{barq.RuleRequired(nil)},
		}).
		Email(barq.EmailField{
			ID: "email", Label: "Email Address", Required: true,
			Rules: []barq.Rule{barq.RuleRequired(nil), barq.RuleEmail(nil)},
		}).
		Select(barq.SelectField{
			ID: "ticket_type", Label: "Ticket Type", Required: true,
			Options: []barq.Option{
				{Label: "General Admission", Value: "general"},
				{Label: "VIP", Value: "vip"},
				{Label: "Student", Value: "student"},
			},
		}).
		// VIP only
		Select(barq.SelectField{
			ID: "dietary", Label: "Dietary Restrictions",
			HelpText: barq.Ptr("Required for VIP dinner seating"),
			Options: []barq.Option{
				{Label: "None", Value: "none"},
				{Label: "Vegetarian", Value: "vegetarian"},
				{Label: "Vegan", Value: "vegan"},
				{Label: "Gluten-Free", Value: "gluten_free"},
			},
			ShowIf: &isVIP,
		}).
		// Student only
		Text(barq.TextField{
			ID: "student_id", Label: "Student ID",
			Placeholder: barq.Ptr("Your university student ID"), Required: true,
			HelpText: barq.Ptr("Required for student discount verification"),
			Rules:    []barq.Rule{barq.RuleRequired(barq.Ptr("Student ID is required for student tickets"))},
			ShowIf:   &isStudent,
		}).
		// VIP only
		Text(barq.TextField{
			ID: "company", Label: "Company Name",
			Placeholder: barq.Ptr("Your company or organisation"),
			HelpText:    barq.Ptr("For badge and networking"),
			ShowIf:      &isVIP,
		}).
		SubmitLabel("Register Now").
		SuccessMessage("Registration complete! Check your email for confirmation.")

	fmt.Println(form.Build())
}
