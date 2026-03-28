"""
Conditional Form Example

This example demonstrates conditional field visibility based on other field values.
"""

from formora import Form, CssFramework, Condition

html = (
    Form("event_registration")
    .title("Event Registration")
    .css(CssFramework.TAILWIND)
    .text("name", "Full Name", required=True)
    .email("email", "Email", required=True)
    .select("ticket_type", "Ticket Type", options=[
        ("General Admission", "general"),
        ("VIP Pass", "vip"),
        ("Student", "student")
    ], required=True)
    # Show dietary preferences only for VIP
    .multi_select(
        "dietary",
        "Dietary Restrictions",
        options=[
            ("Vegetarian", "veg"),
            ("Vegan", "vegan"),
            ("Gluten-Free", "gf"),
            ("Nut-Free", "nf")
        ],
        show_if=Condition("ticket_type", "eq", "vip")
    )
    # Show student ID field only for student tickets
    .text(
        "student_id",
        "Student ID",
        placeholder="Enter your student ID",
        show_if=Condition("ticket_type", "eq", "student")
    )
    # Show company name only for VIP
    .text(
        "company",
        "Company Name",
        show_if=Condition("ticket_type", "eq", "vip")
    )
    .checkbox("newsletter", "Subscribe to newsletter", default=False)
    .submit_label("Register")
    .success_message("Registration confirmed! See you at the event.")
    .build()
)

print(html)
