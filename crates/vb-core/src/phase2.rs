//! Phase 2 — Sequencer primitives for VegieBreakDAW.
//! Keeps the UI read-only against the audio engine; mutations go through a
//! command queue. No FL Studio DSP, assets, or strings copied.

use std::collections::VecDeque;

/// A reversible edit command for undo/redo across arrangement operations.
#[derive(Debug, Clone, PartialEq)]
pub enum EditCommand {
    AddNote,
    MoveNote,
    ResizeNote,
    DeleteNote,
    MoveClip,
    ResizeClip,
    CreatePattern,
    DeletePattern,
}

/// Undo/redo stack with bounded memory for low-end performance.
#[derive(Debug, Default)]
pub struct UndoStack {
    undo: VecDeque<EditCommand>,
    redo: VecDeque<EditCommand>,
    max_depth: usize,
}

impl UndoStack {
    pub const DEFAULT_MAX_DEPTH: usize = 256;

    pub fn new() -> Self {
        Self {
            undo: VecDeque::with_capacity(Self::DEFAULT_MAX_DEPTH),
            redo: VecDeque::with_capacity(Self::DEFAULT_MAX_DEPTH),
            max_depth: Self::DEFAULT_MAX_DEPTH,
        }
    }

    pub fn push_undo(&mut self, cmd: EditCommand) {
        if self.undo.len() >= self.max_depth {
            self.undo.pop_front();
        }
        self.undo.push_back(cmd);
        self.redo.clear();
    }

    pub fn undo(&mut self) -> Option<EditCommand> {
        let cmd = self.undo.pop_back()?;
        self.redo.push_back(cmd.clone());
        Some(cmd)
    }

    pub fn redo(&mut self) -> Option<EditCommand> {
        let cmd = self.redo.pop_back()?;
        self.undo.push_back(cmd.clone());
        Some(cmd)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}

/// A 16-bar arrangement reference for Phase 2 acceptance.
/// The arrangement holds patterns, tracks, and automation lanes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArrangementSnapshot {
    pub bars: u16,
    pub patterns: u16,
    pub tracks: u16,
}
