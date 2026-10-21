#![forbid(unsafe_code)]
#![doc = "Plugin discovery, loading, state, and process-isolation scaffolding."]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A plugin ABI supported by the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginFormat {
    /// VST3, the current Steinberg standard. Requires the VST3 SDK.
    Vst3,
    /// CLAP, a permissive open standard. Preferred first-class target.
    Clap,
    /// Legacy VST2. Optional; the SDK/ABI is not redistributed.
    Vst2,
    /// Avid AAX. Optional; requires the proprietary AAX SDK.
    Aax,
}

impl PluginFormat {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Vst3 => "VST3",
            Self::Clap => "CLAP",
            Self::Vst2 => "VST2 (legacy)",
            Self::Aax => "AAX (SDK required)",
        }
    }

    /// Whether the host can distribute support for this format without an
    /// external, user-supplied SDK.
    #[must_use]
    pub fn is_distributable(self) -> bool {
        matches!(self, Self::Vst3 | Self::Clap)
    }
}

/// A plugin found by the scanner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginDescriptor {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub format: PluginFormat,
    pub path: PathBuf,
    pub is_instrument: bool,
    /// Latency reported by the plugin, in samples at the current rate.
    pub latency_samples: u32,
}

/// What the current host build supports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostCapabilities {
    pub formats: Vec<PluginFormat>,
    pub out_of_process: bool,
    pub sandboxed_scan: bool,
}

impl Default for HostCapabilities {
    fn default() -> Self {
        Self {
            formats: vec![PluginFormat::Vst3, PluginFormat::Clap],
            out_of_process: false,
            sandboxed_scan: false,
        }
    }
}

/// Result of a scan. An empty report is valid: it means no plugins were found
/// in the configured paths, not that scanning failed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanReport {
    pub plugins: Vec<PluginDescriptor>,
    pub scanned_paths: Vec<PathBuf>,
    pub errors: Vec<ScanError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanError {
    pub path: PathBuf,
    pub message: String,
}

/// Host-side plugin operations.
///
/// The production implementation will use an out-of-process worker. This trait
/// keeps the engine independent of the ABI and of the isolation strategy.
pub trait PluginHost {
    type Error;

    fn capabilities(&self) -> HostCapabilities;
    fn scan(&mut self, paths: &[PathBuf]) -> Result<ScanReport, Self::Error>;
    fn load(&mut self, descriptor: &PluginDescriptor) -> Result<(), Self::Error>;
    fn unload(&mut self, instance_id: &str) -> Result<(), Self::Error>;
}

/// A host that reports capabilities but does not load plugins yet.
#[derive(Debug, Default)]
pub struct StubHost {
    capabilities: HostCapabilities,
}

impl StubHost {
    #[must_use]
    pub fn new(capabilities: HostCapabilities) -> Self {
        Self { capabilities }
    }
}

impl PluginHost for StubHost {
    type Error = HostError;

    fn capabilities(&self) -> HostCapabilities {
        self.capabilities.clone()
    }

    fn scan(&mut self, paths: &[PathBuf]) -> Result<ScanReport, Self::Error> {
        Ok(ScanReport {
            plugins: Vec::new(),
            scanned_paths: paths.to_vec(),
            errors: Vec::new(),
        })
    }

    fn load(&mut self, _descriptor: &PluginDescriptor) -> Result<(), Self::Error> {
        Err(HostError::NotImplemented)
    }

    fn unload(&mut self, _instance_id: &str) -> Result<(), Self::Error> {
        Err(HostError::NotImplemented)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HostError {
    #[error("plugin hosting is not implemented in this build")]
    NotImplemented,
    #[error("plugin format {0:?} is unavailable in this build")]
    FormatUnavailable(PluginFormat),
    #[error("plugin process failed: {0}")]
    Process(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_labels_are_stable() {
        assert_eq!(PluginFormat::Vst3.label(), "VST3");
        assert_eq!(PluginFormat::Clap.label(), "CLAP");
        assert!(PluginFormat::Clap.is_distributable());
        assert!(!PluginFormat::Aax.is_distributable());
    }

    #[test]
    fn stub_scan_is_empty_but_valid() {
        let mut host = StubHost::default();
        let report = host
            .scan(&[PathBuf::from("C:\\Program Files\\Common Files\\VST3")])
            .unwrap();
        assert!(report.plugins.is_empty());
        assert_eq!(report.scanned_paths.len(), 1);
    }
}
