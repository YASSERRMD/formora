// Simple contact form example.
// Go equivalent of examples/simple_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/barq-chat-form/go/barq"
)

func main() {
	form := barq.NewForm("contact-form")
	defer form.Free()

	form.
		Title("Contact Us").
		Description("Send us a message and we'll get back to you.").
		CSSFramework(barq.Bootstrap()).
		Text(barq.TextField{
			ID: "name", Label: "Full Name",
			Placeholder: barq.Ptr("Your full name"), Required: true,
			Rules: []barq.Rule{
				barq.RuleRequired(nil),
				barq.RuleMinLength(2, barq.Ptr("Name must be at least 2 characters")),
			},
		}).
		Email(barq.EmailField{
			ID: "email", Label: "Email Address", Required: true,
			Rules: []barq.Rule{barq.RuleRequired(nil), barq.RuleEmail(nil)},
		}).
		Select(barq.SelectField{
			ID: "subject", Label: "Subject", Required: true,
			Options: []barq.Option{
				{Label: "General Inquiry", Value: "general"},
				{Label: "Support", Value: "support"},
				{Label: "Billing", Value: "billing"},
				{Label: "Other", Value: "other"},
			},
		}).
		Textarea(barq.TextareaField{
			ID: "message", Label: "Message", Rows: 5,
			Placeholder: barq.Ptr("Type your message here..."), Required: true,
			Rules: []barq.Rule{
				barq.RuleRequired(nil),
				barq.RuleMinLength(10, barq.Ptr("Message must be at least 10 characters")),
			},
		}).
		SubmitLabel("Send Message").
		SuccessMessage("Thank you! We'll be in touch soon.")

	fmt.Println(form.Build())
}
