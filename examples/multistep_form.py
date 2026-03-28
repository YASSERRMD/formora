"""
Multi-step Form Example

This example demonstrates a multi-step form with progress tracking.
"""

from formora import Form, CssFramework, Rule

html = (
    Form("onboarding")
    .title("Welcome! Let's get you set up.")
    .description("This will take about 2 minutes.")
    .css(CssFramework.TAILWIND)
    # Step 1: Basic Info
    .step("Basic Information")
    .text("full_name", "Full Name", required=True, rules=[Rule.required()])
    .email("email", "Email Address", required=True, rules=[Rule.required(), Rule.email()])
    .text("company", "Company Name")
    # Step 2: Role Details
    .step("Your Role")
    .select("role", "What's your role?", options=[
        ("Developer", "developer"),
        ("Designer", "designer"),
        ("Manager", "manager"),
        ("Other", "other")
    ], required=True)
    .number("years_experience", "Years of Experience", min=0, max=50)
    .multi_select("technologies", "Technologies (select all that apply)", options=[
        ("Python", "python"),
        ("JavaScript", "javascript"),
        ("Rust", "rust"),
        ("Go", "go"),
        ("TypeScript", "typescript")
    ])
    # Step 3: Preferences
    .step("Preferences")
    .select("frequency", "How often do you want to hear from us?", options=[
        ("Daily", "daily"),
        ("Weekly", "weekly"),
        ("Monthly", "monthly")
    ])
    .checkbox("terms", "I agree to the terms and conditions", default=True)
    .checkbox("privacy", "I agree to the privacy policy", default=True)
    .submit_label("Complete Setup")
    .success_message("Setup complete! Welcome aboard.")
    .build()
)

print(html)
