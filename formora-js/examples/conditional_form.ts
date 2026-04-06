/**
 * Conditional field visibility example.
 * TypeScript equivalent of examples/conditional_form.py
 */

import init, { Form, Rule, JsCondition as Condition, CssFramework } from "../wasm/formora_js";

async function main() {
  await init();

  const html = new Form("event-registration")
    .title("Event Registration")
    .description("Register for the annual developer conference.")
    .css(CssFramework.bootstrap())
    .text("full_name", "Full Name", undefined, true, undefined, undefined, [
      Rule.required(),
    ])
    .email("email", "Email Address", true, undefined, undefined, [
      Rule.required(),
      Rule.email(),
    ])
    .select(
      "ticket_type",
      "Ticket Type",
      [
        ["General Admission", "general"],
        ["VIP", "vip"],
        ["Student", "student"],
      ],
      true
    )
    // Only shown when ticket_type == "vip"
    .select(
      "dietary",
      "Dietary Restrictions",
      [
        ["None", "none"],
        ["Vegetarian", "vegetarian"],
        ["Vegan", "vegan"],
        ["Gluten-Free", "gluten_free"],
      ],
      false,
      "Required for VIP dinner seating",
      undefined,
      undefined,
      new Condition("ticket_type", "eq", "vip")
    )
    // Only shown when ticket_type == "student"
    .text(
      "student_id",
      "Student ID",
      "Your university student ID",
      true,
      "Required for student discount verification",
      undefined,
      [Rule.required("Student ID is required for student tickets")],
      new Condition("ticket_type", "eq", "student")
    )
    // Only shown when ticket_type == "vip"
    .text(
      "company",
      "Company Name",
      "Your company or organisation",
      false,
      "For badge and networking",
      undefined,
      undefined,
      new Condition("ticket_type", "eq", "vip")
    )
    .submitLabel("Register Now")
    .successMessage("Registration complete! Check your email for confirmation.")
    .build();

  console.log(html);
}

main().catch(console.error);
