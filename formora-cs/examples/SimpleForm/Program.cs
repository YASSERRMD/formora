// Simple contact form example.
// C# equivalent of examples/simple_form.py
using Formora;

using var form = new Form("contact-form");

var html = form
    .Title("Contact Us")
    .Description("Send us a message and we'll get back to you.")
    .Css(CssFramework.Bootstrap())
    .Text("name", "Full Name",
        placeholder: "Your full name", required: true,
        rules: [Rule.Required(), Rule.MinLength(2, "Name must be at least 2 characters")])
    .Email("email", "Email Address",
        required: true,
        rules: [Rule.Required(), Rule.Email()])
    .Select("subject", "Subject",
        options: [
            new("General Inquiry", "general"),
            new("Support", "support"),
            new("Billing", "billing"),
            new("Other", "other"),
        ],
        required: true)
    .Textarea("message", "Message",
        rows: 5, placeholder: "Type your message here...", required: true,
        rules: [Rule.Required(), Rule.MinLength(10, "Message must be at least 10 characters")])
    .SubmitLabel("Send Message")
    .SuccessMessage("Thank you! We'll be in touch soon.")
    .Build();

Console.WriteLine(html);
