// Custom CSS profile implementation
// See CssProfile::custom() in profile.rs for the actual implementation
// This module provides helper functions for custom CSS mode

use super::CssProfile;

/// Get the custom CSS profile (all fields empty)
pub fn profile() -> CssProfile {
    CssProfile::custom()
}

/// Get the minimal inline styles needed for a functional form with no design opinions
/// These are scoped to .formora-widget to avoid polluting the host page
pub fn minimal_inline_styles() -> String {
    r#"
<style>
.formora-widget {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: #212529;
  box-sizing: border-box;
}
.formora-widget *,
.formora-widget *::before,
.formora-widget *::after {
  box-sizing: inherit;
}
.formora-widget input[type="text"],
.formora-widget input[type="email"],
.formora-widget input[type="number"],
.formora-widget input[type="date"],
.formora-widget input[type="file"],
.formora-widget textarea,
.formora-widget select {
  display: block;
  width: 100%;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  color: #212529;
  background-color: #fff;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
  margin: 0;
}
.formora-widget input:focus,
.formora-widget textarea:focus,
.formora-widget select:focus {
  outline: 2px solid #0d6efd;
  outline-offset: 0;
}
.formora-widget label {
  display: inline-block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}
.formora-widget button {
  display: inline-block;
  font-weight: 400;
  line-height: 1.5;
  text-align: center;
  text-decoration: none;
  vertical-align: middle;
  cursor: pointer;
  user-select: none;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  border-radius: 0.25rem;
  border: 1px solid transparent;
  background-color: #0d6efd;
  color: #fff;
}
.formora-widget button:hover {
  background-color: #0b5ed7;
}
</style>
"#.to_string()
}
