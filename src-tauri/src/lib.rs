//! Public crate entry for the desktop host.
//!
//! This surface re-exports the testable bootstrap entrypoints and shared host
//! state handle used by the binary entrypoint and host transport smoke tests.

/// Desktop host bootstrap assembly and startup entrypoints.
pub mod bootstrap;

/// Typed IPC command/query handlers and shared transport envelopes.
pub mod commands;

/// Shared host state wrappers around composition-root services.
pub mod state;

/// Re-export the testable desktop host bootstrap surface for callers and tests.
pub use bootstrap::{build_desktop_host_bootstrap, run_desktop_host, DesktopHostBootstrap};

/// Re-export the shared desktop host services handle attached to state.
pub use state::DesktopAppServicesHandle;