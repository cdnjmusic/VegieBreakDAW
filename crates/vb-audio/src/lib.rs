#![forbid(unsafe_code)]
#![doc = "Audio device abstraction, transport, and graph scaffolding."]

pub mod backend;
pub mod engine;
pub mod transport;

pub use backend::{AudioBackend, AudioConfig, AudioDeviceInfo, NullBackend};
pub use engine::{AudioEngine, AudioEngineStats};
pub use transport::Transport;
