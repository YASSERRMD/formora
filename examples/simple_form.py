"""
Simple Formora Example

This example demonstrates basic form creation with Formora.
"""

from formora import Form, CssFramework, Rule

# Create a simple contact form
html = (
    Form("contact")
    .title("Contact Us")
    .description("We'd love to hear from you!")
    .css(CssFramework.BOOTSTRAP)
    .text("name", "Your Name", required=True, rules=[Rule.required(), Rule.min_length(2)])
    .email("email", "Email Address", required=True, rules=[Rule.required(), Rule.email()])
    .select("topic", "Topic", options=[
        ("General Inquiry", "general"),
        ("Support", "support"),
        ("Sales", "sales")
    ])
    .textarea("message", "Message", rows=5, required=True)
    .submit_label("Send Message")
    .success_message("Thanks for reaching out! We'll get back to you soon.")
    .build()
)

print(html)
