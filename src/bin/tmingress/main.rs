//! Main entry point for CosmosTxEndpoint

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use tm_ingress::application::APP;

/// Boot CosmosTxEndpoint
fn main() {
    abscissa_core::boot(&APP);
}
