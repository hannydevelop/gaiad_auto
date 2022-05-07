//! Main entry point for GaiadAuto

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use gaiad_auto::application::APP;

/// Boot GaiadAuto
fn main() {
    abscissa_core::boot(&APP);
}
