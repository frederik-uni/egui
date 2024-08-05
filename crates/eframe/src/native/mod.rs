mod app_icon;
mod epi_integration;
mod event_loop_context;
pub mod run;

/// File storage which can be used by native backends.
#[cfg(feature = "persistence")]
pub mod file_storage;

pub(crate) mod winit_integration;

#[cfg(feature = "glow")]
mod glow_integration;

#[cfg(target_os = "ios")]
pub mod ios_safe_area;
#[cfg(feature = "wgpu")]
mod wgpu_integration;
