/**
 * LLM integration utilities for barq-chat-form.
 * Mirrors Python's barq-chat-form/llm.py.
 */

import type { Form, FormResult } from "../wasm/barq_chat_form_js";

// ─── Types ────────────────────────────────────────────────────────────────────

export interface JsonSchemaProperty {
  type: string;
  description?: string;
  minimum?: number;
  maximum?: number;
  enum?: string[];
  items?: { type: string; enum?: string[] };
}

export interface JsonSchema {
  type: "object";
  properties: Record<string, JsonSchemaProperty>;
  required?: string[];
}

/** Thrown when a form result value does not match the expected field type. */
export class BarqTypeError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "BarqTypeError";
  }
}

// ─── formToJsonSchema ────────────────────────────────────────────────────────

/**
 * Convert a Form's schema to a JSON Schema object suitable for LLM tool/function calling.
 * Hidden fields are excluded — they carry context, not user input.
 */
export function formToJsonSchema(form: Form): JsonSchema {
  // Access the underlying schema via the WASM object's internal representation.
  // We serialize the rendered HTML to extract schema details — instead we use
  // the schema JSON that wasm-bindgen exposes via the form's internal state.
  //
  // Since wasm-bindgen does not expose FormSchema fields directly, we call
  // build() to get the HTML and derive the schema from the form object's
  // schemaJson() helper if available, else we use the typed approach below.
  const rawSchema = (form as unknown as { schemaJson(): string }).schemaJson?.();
  if (rawSchema) {
    return buildSchemaFromJson(JSON.parse(rawSchema));
  }

  // Fallback: return an open schema (fields not inspectable without schemaJson)
  return { type: "object", properties: {} };
}

function buildSchemaFromJson(schema: {
  fields: Array<{
    id: string;
    field_type: string;
    label: string;
    placeholder?: string;
    required: boolean;
    min?: number;
    max?: number;
    options?: Array<{ label: string; value: string }>;
  }>;
}): JsonSchema {
  const properties: Record<string, JsonSchemaProperty> = {};
  const required: string[] = [];

  for (const field of schema.fields) {
    if (field.field_type === "Hidden") continue;

    properties[field.id] = fieldTypeToJsonSchema(field);

    if (field.required) {
      required.push(field.id);
    }
  }

  const result: JsonSchema = { type: "object", properties };
  if (required.length > 0) result.required = required;
  return result;
}

function fieldTypeToJsonSchema(field: {
  field_type: string;
  placeholder?: string;
  min?: number;
  max?: number;
  options?: Array<{ label: string; value: string }>;
}): JsonSchemaProperty {
  const { field_type } = field;

  if (["Text", "Email", "Textarea", "Date", "File"].includes(field_type)) {
    const prop: JsonSchemaProperty = { type: "string" };
    if (field.placeholder) prop.description = field.placeholder;
    return prop;
  }

  if (["Number", "Range"].includes(field_type)) {
    const prop: JsonSchemaProperty = { type: "number" };
    if (field.min != null) prop.minimum = field.min;
    if (field.max != null) prop.maximum = field.max;
    return prop;
  }

  if (field_type === "Checkbox") {
    return { type: "boolean" };
  }

  if (["Select", "Radio"].includes(field_type)) {
    const enumValues = (field.options ?? []).map((o) => o.value);
    return { type: "string", enum: enumValues };
  }

  if (field_type === "MultiSelect") {
    const enumValues = (field.options ?? []).map((o) => o.value);
    return { type: "array", items: { type: "string", enum: enumValues } };
  }

  return { type: "string" };
}

// ─── formResultToToolArgs ─────────────────────────────────────────────────────

/**
 * Convert a FormResult to a plain object suitable for passing as tool arguments.
 * Validates types against the form schema and strips hidden fields.
 *
 * @throws {BarqTypeError} if a required field is missing or a type mismatch occurs
 */
export function formResultToToolArgs(
  result: FormResult,
  form: Form
): Record<string, unknown> {
  const typed = result.typedData as Record<string, unknown>;
  const rawSchema = (form as unknown as { schemaJson(): string }).schemaJson?.();
  if (!rawSchema) {
    // No schema introspection available — return typed data minus nulls
    return Object.fromEntries(
      Object.entries(typed).filter(([, v]) => v != null)
    );
  }

  const schema = JSON.parse(rawSchema) as {
    fields: Array<{
      id: string;
      field_type: string;
      required: boolean;
      min?: number;
      max?: number;
    }>;
  };

  const args: Record<string, unknown> = {};

  for (const field of schema.fields) {
    if (field.field_type === "Hidden") continue;

    if (!(field.id in typed)) {
      if (field.required) {
        throw new BarqTypeError(
          `Required field '${field.id}' missing from form result`
        );
      }
      continue;
    }

    const value = typed[field.id];
    validateFieldType(field, value, field.id);
    args[field.id] = value;
  }

  return args;
}

function validateFieldType(
  field: { field_type: string; required: boolean; min?: number; max?: number },
  value: unknown,
  fieldId: string
): void {
  if (value == null) {
    if (field.required) {
      throw new BarqTypeError(`Required field '${fieldId}' is null/undefined`);
    }
    return;
  }

  const { field_type } = field;

  if (["Text", "Email", "Textarea", "Date", "Select", "Radio"].includes(field_type)) {
    if (typeof value !== "string") {
      throw new BarqTypeError(
        `Field '${fieldId}' expected string, got ${typeof value}`
      );
    }
  } else if (["Number", "Range"].includes(field_type)) {
    if (typeof value !== "number") {
      throw new BarqTypeError(
        `Field '${fieldId}' expected number, got ${typeof value}`
      );
    }
    if (field.min != null && value < field.min) {
      throw new BarqTypeError(
        `Field '${fieldId}' value ${value} is below minimum ${field.min}`
      );
    }
    if (field.max != null && value > field.max) {
      throw new BarqTypeError(
        `Field '${fieldId}' value ${value} is above maximum ${field.max}`
      );
    }
  } else if (field_type === "Checkbox") {
    if (typeof value !== "boolean") {
      throw new BarqTypeError(
        `Field '${fieldId}' expected boolean, got ${typeof value}`
      );
    }
  } else if (field_type === "MultiSelect") {
    if (!Array.isArray(value)) {
      throw new BarqTypeError(
        `Field '${fieldId}' expected array, got ${typeof value}`
      );
    }
    if (!value.every((item) => typeof item === "string")) {
      throw new BarqTypeError(
        `Field '${fieldId}' array must contain only strings`
      );
    }
  }
}

// ─── formResultToPrompt ───────────────────────────────────────────────────────

/**
 * Format a FormResult as a human-readable string for inclusion in a prompt.
 * Booleans become "Yes"/"No", arrays are joined with " and ".
 */
export function formResultToPrompt(result: FormResult): string {
  const typed = result.typedData as Record<string, unknown>;

  if (!typed || Object.keys(typed).length === 0) {
    return "The user has provided no information.";
  }

  const parts = Object.entries(typed).map(([key, value]) => {
    let formatted: string;
    if (typeof value === "boolean") {
      formatted = value ? "Yes" : "No";
    } else if (Array.isArray(value)) {
      formatted = value.map(String).join(" and ");
    } else {
      formatted = String(value);
    }
    return `${key}: ${formatted}`;
  });

  return `The user has provided the following information: ${parts.join(", ")}`;
}
