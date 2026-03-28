"""
LLM utility functions for Formora.

This module provides helper functions for integrating Formora with LLM applications,
including JSON Schema generation and form result formatting.
"""

from typing import Any, Dict, List, Optional
from .formora import Form, FormResult


class FormoraTypeError(Exception):
    """Raised when form result type doesn't match expected schema."""
    pass


def form_to_json_schema(form: Form) -> Dict[str, Any]:
    """
    Convert a Form to JSON Schema format.

    This is useful for passing form structure to an LLM as a function/tool schema.
    Hidden fields are excluded from the schema as they are context, not user input.

    Args:
        form: A Form instance

    Returns:
        A dictionary representing the JSON Schema
    """
    properties = {}
    required_fields = []

    # Expose fields via the internal schema
    for field in form.schema.fields:
        field_id = field.id

        # Skip hidden fields - they're context, not parameters
        if field.field_type == "Hidden":
            continue

        # Map FieldType to JSON Schema types
        field_schema = _field_type_to_json_schema(field)
        properties[field_id] = field_schema

        # Track required fields
        if field.required:
            required_fields.append(field_id)

    schema = {
        "type": "object",
        "properties": properties,
    }

    if required_fields:
        schema["required"] = required_fields

    return schema


def _field_type_to_json_schema(field) -> Dict[str, Any]:
    """Map a Formora field type to its JSON Schema representation."""
    field_type = field.field_type

    if field_type in ("Text", "Email", "Textarea", "Date", "File"):
        schema = {"type": "string"}
        if field.placeholder:
            schema["description"] = field.placeholder
        return schema

    elif field_type in ("Number", "Range"):
        schema = {"type": "number"}
        if field.min is not None:
            schema["minimum"] = field.min
        if field.max is not None:
            schema["maximum"] = field.max
        return schema

    elif field_type == "Checkbox":
        return {"type": "boolean"}

    elif field_type in ("Select", "Radio"):
        options = field.options or []
        enum_values = [opt.value for opt in options]
        return {"type": "string", "enum": enum_values}

    elif field_type == "MultiSelect":
        options = field.options or []
        enum_values = [opt.value for opt in options]
        return {"type": "array", "items": {"type": "string", "enum": enum_values}}

    else:
        # Fallback for unknown types
        return {"type": "string"}


def form_result_to_tool_args(result: FormResult, form: Form) -> Dict[str, Any]:
    """
    Convert a FormResult to a dictionary suitable for passing as tool kwargs.

    Validates that types match the form schema and removes hidden field values.
    The returned dict can be unpacked as **kwargs into a tool function.

    Args:
        result: A FormResult instance from parse()
        form: The Form instance that generated this result

    Returns:
        A cleaned dictionary with validated types

    Raises:
        FormoraTypeError: If a value's type doesn't match the schema
    """
    args = {}

    for field in form.schema.fields:
        field_id = field.id

        # Skip hidden fields - they're context, not tool args
        if field.field_type == "Hidden":
            continue

        # Get the value from typed_data
        if field_id not in result.typed_data:
            if field.required:
                raise FormoraTypeError(f"Required field '{field_id}' missing from form result")
            continue

        value = result.typed_data[field_id]

        # Validate type matches schema
        _validate_field_type(field, value, field_id)

        args[field_id] = value

    return args


def _validate_field_type(field, value: Any, field_id: str) -> None:
    """Validate that a value matches the expected type for its field."""
    field_type = field.field_type

    if value is None:
        if field.required:
            raise FormoraTypeError(f"Required field '{field_id}' is None")
        return

    if field_type in ("Text", "Email", "Textarea", "Date", "Select", "Radio"):
        if not isinstance(value, str):
            raise FormoraTypeError(
                f"Field '{field_id}' expected string, got {type(value).__name__}"
            )

    elif field_type in ("Number", "Range"):
        if not isinstance(value, (int, float)):
            raise FormoraTypeError(
                f"Field '{field_id}' expected number, got {type(value).__name__}"
            )
        # Check min/max constraints
        if field.min is not None and value < field.min:
            raise FormoraTypeError(
                f"Field '{field_id}' value {value} is below minimum {field.min}"
            )
        if field.max is not None and value > field.max:
            raise FormoraTypeError(
                f"Field '{field_id}' value {value} is above maximum {field.max}"
            )

    elif field_type == "Checkbox":
        if not isinstance(value, bool):
            raise FormoraTypeError(
                f"Field '{field_id}' expected bool, got {type(value).__name__}"
            )

    elif field_type == "MultiSelect":
        if not isinstance(value, list):
            raise FormoraTypeError(
                f"Field '{field_id}' expected list, got {type(value).__name__}"
            )
        if not all(isinstance(item, str) for item in value):
            raise FormoraTypeError(
                f"Field '{field_id}' list must contain only strings"
            )


def form_result_to_prompt(result: FormResult) -> str:
    """
    Convert a FormResult to a human-readable prompt text.

    Formats the form data as "Label: Value, Label: Value, ..." for display
    to the user or inclusion in a prompt. Boolean values are converted to
    Yes/No, and list values are joined with " and ".

    Args:
        result: A FormResult instance from parse()

    Returns:
        A formatted string like "The user has provided: Name: John, Age: 25, ..."
    """
    if not result.typed_data:
        return "The user has provided no information."

    parts = []
    for field_id, value in result.typed_data.items():
        # Format value for human reading
        if isinstance(value, bool):
            formatted = "Yes" if value else "No"
        elif isinstance(value, list):
            formatted = " and ".join(str(item) for item in value)
        else:
            formatted = str(value)

        parts.append(f"{field_id}: {formatted}")

    joined = ", ".join(parts)
    return f"The user has provided the following information: {joined}"
