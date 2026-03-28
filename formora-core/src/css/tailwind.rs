// Tailwind CSS v3 profile implementation
// See CssProfile::tailwind() in profile.rs for the actual implementation
// This module provides helper functions for Tailwind-specific functionality

use super::CssProfile;

/// Get the Tailwind CSS v3 profile
pub fn profile() -> CssProfile {
    CssProfile::tailwind()
}

/// Get the minimum inline styles needed for custom Tailwind theme
/// Not used in standard Tailwind mode - user is expected to have Tailwind configured
pub fn minimal_inline_styles() -> String {
    String::new()
}
