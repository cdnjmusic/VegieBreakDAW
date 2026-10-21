//! Phase 6 — Performance, profiling, and packaging notes.
//! Clean-room original only; no FL Studio performance data copied.

/// Performance target reference (low-end PC: Ryzen 5 5500 / 16 GB).
pub const PERFORMANCE_TARGETS: &str = r#"
- Cold start < 2 s
- 100-track project at 64 samples < 60% CPU
- Block-based audio with zero callback allocations
- SIMD/denormal-safe DSP (future)
- Graph parallelism for independent buses (future)
"#;

/// A profiling hook for startup and project-load measurement.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProfilePoint {
    pub label: String,
    pub duration_ms: f64,
}
