//! Main entry point for Cosmin

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use cosmin::application::APP;

/// Boot Cosmin
fn main() {
    abscissa_core::boot(&APP);
}
