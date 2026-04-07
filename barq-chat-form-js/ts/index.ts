/**
 * barq-chat-form — TypeScript chat form rendering engine for interactive chat widgets
 *
 * Re-exports all public API from the WASM bindings, mirroring Python's barq_chat_form/__init__.py
 */

export {
  Form,
  Rule,
  JsCondition as Condition,
  CssFramework,
  JsCssProfile as CssProfile,
  FormResult,
  parseMessage as parse,
  isBarq as isBarqMessage,
} from "../wasm/barq_chat_form_js";

export type {
  InitInput,
  InitOutput,
} from "../wasm/barq_chat_form_js";

// Re-export default init so bundlers can initialise the WASM module
export { default as init } from "../wasm/barq_chat_form_js";
