/**
 * Simple contact form example.
 * TypeScript equivalent of examples/simple_form.py
 */

import init, { Form, Rule, CssFramework } from "../wasm/formora_js";

async function main() {
  await init();

  const html = new Form("contact-form")
    .title("Contact Us")
    .description("Send us a message and we'll get back to you.")
    .css(CssFramework.bootstrap())
    .text("name", "Full Name", "Your full name", true, undefined, undefined, [
      Rule.required("Name is required"),
      Rule.minLength(2, "Name must be at least 2 characters"),
    ])
    .email("email", "Email Address", true, undefined, undefined, [
      Rule.required("Email is required"),
      Rule.email("Please enter a valid email"),
    ])
    .select(
      "subject",
      "Subject",
      [
        ["General Inquiry", "general"],
        ["Support", "support"],
        ["Billing", "billing"],
        ["Other", "other"],
      ],
      true
    )
    .textarea(
      "message",
      "Message",
      5,
      "Type your message here...",
      true,
      undefined,
      undefined,
      [
        Rule.required("Message is required"),
        Rule.minLength(10, "Message must be at least 10 characters"),
      ]
    )
    .submitLabel("Send Message")
    .successMessage("Thank you! We'll be in touch soon.")
    .build();

  console.log(html);
}

main().catch(console.error);
