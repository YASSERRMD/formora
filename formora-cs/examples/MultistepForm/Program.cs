// Multi-step onboarding form example.
// C# equivalent of examples/multistep_form.py
using Formora;

using var form = new Form("onboarding");

var html = form
    .Title("Welcome to Formora")
    .Description("Complete your profile in 3 easy steps.")
    .Css(CssFramework.Tailwind())
    // Step 1
    .Step("Personal Information")
    .Text("full_name", "Full Name", required: true,
        rules: [Rule.Required(), Rule.MinLength(2)])
    .Email("email", "Work Email", required: true,
        rules: [Rule.Required(), Rule.Email()])
    .Text("company", "Company", required: true)
    // Step 2
    .Step("Role & Experience")
    .Select("role", "Your Role", required: true,
        options: [
            new("Developer", "developer"),
            new("Designer", "designer"),
            new("Product Manager", "pm"),
            new("Data Scientist", "data_scientist"),
            new("Other", "other"),
        ])
    .Range("years_exp", "Years of Experience", min: 0, max: 30, step: 1, defaultValue: 0)
    .MultiSelect("technologies", "Technologies You Use",
        options: [
            new("TypeScript", "typescript"),
            new("Rust", "rust"),
            new("Python", "python"),
            new("Go", "go"),
            new("C#", "csharp"),
        ])
    // Step 3
    .Step("Preferences")
    .Radio("frequency", "How often do you ship?", required: true,
        options: [
            new("Daily", "daily"),
            new("Weekly", "weekly"),
            new("Monthly", "monthly"),
        ])
    .Checkbox("agree_terms", "I agree to the Terms of Service",
        rules: [Rule.Required("You must accept the terms")])
    .Checkbox("agree_privacy", "I agree to the Privacy Policy",
        rules: [Rule.Required("You must accept the privacy policy")])
    .SubmitLabel("Complete Setup")
    .SuccessMessage("You're all set! Welcome aboard.")
    .Build();

Console.WriteLine(html);
