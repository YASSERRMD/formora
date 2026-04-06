// Conditional field visibility example.
// C# equivalent of examples/conditional_form.py
using Formora;

var isVip     = new Condition("ticket_type", "eq", "vip");
var isStudent = new Condition("ticket_type", "eq", "student");

using var form = new Form("event-registration");

var html = form
    .Title("Event Registration")
    .Description("Register for the annual developer conference.")
    .Css(CssFramework.Bootstrap())
    .Text("full_name", "Full Name", required: true, rules: [Rule.Required()])
    .Email("email", "Email Address", required: true,
        rules: [Rule.Required(), Rule.Email()])
    .Select("ticket_type", "Ticket Type", required: true,
        options: [
            new("General Admission", "general"),
            new("VIP", "vip"),
            new("Student", "student"),
        ])
    // VIP only
    .Select("dietary", "Dietary Restrictions",
        helpText: "Required for VIP dinner seating",
        options: [
            new("None", "none"),
            new("Vegetarian", "vegetarian"),
            new("Vegan", "vegan"),
            new("Gluten-Free", "gluten_free"),
        ],
        showIf: isVip)
    // Student only
    .Text("student_id", "Student ID",
        placeholder: "Your university student ID", required: true,
        helpText: "Required for student discount verification",
        rules: [Rule.Required("Student ID is required for student tickets")],
        showIf: isStudent)
    // VIP only
    .Text("company", "Company Name",
        placeholder: "Your company or organisation",
        helpText: "For badge and networking",
        showIf: isVip)
    .SubmitLabel("Register Now")
    .SuccessMessage("Registration complete! Check your email for confirmation.")
    .Build();

Console.WriteLine(html);
