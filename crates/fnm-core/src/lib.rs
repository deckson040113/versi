mod client;
mod detection;
mod error;
mod progress;
mod schedule;
mod version;

pub mod commands;

pub use client::{Environment, FnmClient};
pub use detection::{detect_fnm, install_fnm, FnmDetection};
pub use error::FnmError;
pub use progress::{InstallPhase, InstallProgress};
pub use schedule::{fetch_release_schedule, ReleaseSchedule};
pub use version::{InstalledVersion, NodeVersion, RemoteVersion, VersionGroup};
