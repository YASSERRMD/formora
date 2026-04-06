/**
 * formora — TypeScript form rendering engine for interactive chat widgets
 *
 * Re-exports all public API from the WASM bindings, mirroring Python's formora/__init__.py
 */

export {
  Form,
  Rule,
  JsCondition as Condition,
  CssFramework,
  JsCssProfile as CssProfile,
  FormResult,
  parseMessage as parse,
  isFormora as isFormoraMessage,
} from "../wasm/formora_js";

export type {
  InitInput,
  InitOutput,
} from "../wasm/formora_js";

// Re-export default init so bundlers can initialise the WASM module
export { default as init } from "../wasm/formora_js";
