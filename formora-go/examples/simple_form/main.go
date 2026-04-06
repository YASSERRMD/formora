// Simple contact form example.
// Go equivalent of examples/simple_form.py
package main

import (
	"fmt"
	"github.com/YASSERRMD/formora/go/formora"
)

func main() {
	form := formora.NewForm("contact-form")
	defer form.Free()

	form.
		Title("Contact Us").
		Description("Send us a message and we'll get back to you.").
		CSSFramework(formora.Bootstrap()).
		Text(formora.TextField{
			ID: "name", Label: "Full Name",
			Placeholder: formora.Ptr("Your full name"), Required: true,
			Rules: []formora.Rule{
				formora.RuleRequired(nil),
				formora.RuleMinLength(2, formora.Ptr("Name must be at least 2 characters")),
			},
		}).
		Email(formora.EmailField{
			ID: "email", Label: "Email Address", Required: true,
			Rules: []formora.Rule{formora.RuleRequired(nil), formora.RuleEmail(nil)},
		}).
		Select(formora.SelectField{
			ID: "subject", Label: "Subject", Required: true,
			Options: []formora.Option{
				{Label: "General Inquiry", Value: "general"},
				{Label: "Support", Value: "support"},
				{Label: "Billing", Value: "billing"},
				{Label: "Other", Value: "other"},
			},
		}).
		Textarea(formora.TextareaField{
			ID: "message", Label: "Message", Rows: 5,
			Placeholder: formora.Ptr("Type your message here..."), Required: true,
			Rules: []formora.Rule{
				formora.RuleRequired(nil),
				formora.RuleMinLength(10, formora.Ptr("Message must be at least 10 characters")),
			},
		}).
		SubmitLabel("Send Message").
		SuccessMessage("Thank you! We'll be in touch soon.")

	fmt.Println(form.Build())
}
