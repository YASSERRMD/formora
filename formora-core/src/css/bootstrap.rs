// Bootstrap 5 CSS profile implementation
// See CssProfile::bootstrap() in profile.rs for the actual implementation
// This module provides helper functions for Bootstrap-specific functionality

use super::CssProfile;

/// Get the Bootstrap 5 CSS profile
pub fn profile() -> CssProfile {
    CssProfile::bootstrap()
}

/// Get the minimum inline styles needed for custom Bootstrap theme
/// Not used in standard Bootstrap mode - user is expected to have Bootstrap loaded
pub fn minimal_inline_styles() -> String {
    String::new()
}
