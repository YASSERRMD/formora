/**
 * Multi-step onboarding form example.
 * TypeScript equivalent of examples/multistep_form.py
 */

import init, { Form, Rule, CssFramework } from "../wasm/formora_js";

async function main() {
  await init();

  const html = new Form("onboarding")
    .title("Welcome to Formora")
    .description("Complete your profile in 3 easy steps.")
    .css(CssFramework.tailwind())
    // Step 1 — Personal Info
    .step("Personal Information")
    .text("full_name", "Full Name", "Your full name", true, undefined, undefined, [
      Rule.required(),
      Rule.minLength(2),
    ])
    .email("email", "Work Email", true, undefined, undefined, [
      Rule.required(),
      Rule.email(),
    ])
    .text("company", "Company", "Your company name", true)
    // Step 2 — Role
    .step("Role & Experience")
    .select(
      "role",
      "Your Role",
      [
        ["Developer", "developer"],
        ["Designer", "designer"],
        ["Product Manager", "pm"],
        ["Data Scientist", "data_scientist"],
        ["Other", "other"],
      ],
      true
    )
    .range("years_exp", "Years of Experience", 0, 30, 1, 0)
    .multiSelect(
      "technologies",
      "Technologies You Use",
      [
        ["TypeScript", "typescript"],
        ["Rust", "rust"],
        ["Python", "python"],
        ["React", "react"],
        ["Node.js", "nodejs"],
      ]
    )
    // Step 3 — Preferences
    .step("Preferences")
    .radio(
      "frequency",
      "How often do you ship?",
      [
        ["Daily", "daily"],
        ["Weekly", "weekly"],
        ["Monthly", "monthly"],
      ],
      true
    )
    .checkbox("agree_terms", "I agree to the Terms of Service", false, undefined, [
      Rule.required("You must accept the terms"),
    ])
    .checkbox("agree_privacy", "I agree to the Privacy Policy", false, undefined, [
      Rule.required("You must accept the privacy policy"),
    ])
    .submitLabel("Complete Setup")
    .successMessage("You're all set! Welcome aboard.")
    .build();

  console.log(html);
}

main().catch(console.error);
