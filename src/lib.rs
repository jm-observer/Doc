use floem::reactive::RwSignal;
use lapce_xi_rope::spans::Spans;
use lsp_types::Diagnostic;
use serde::{Deserialize, Serialize};
use crate::lines::diff::DiffInfo;

pub mod lines;
pub mod language;
pub mod syntax;
pub mod lens;
// mod meta;
pub mod config;

#[derive(Clone, Debug)]
pub struct DiagnosticData {
    pub expanded: RwSignal<bool>,
    pub diagnostics: RwSignal<im::Vector<Diagnostic>>,
    pub diagnostics_span: RwSignal<Spans<Diagnostic>>,
}

#[derive(Clone)]
pub enum EditorViewKind {
    Normal,
    Diff(DiffInfo),
}

impl EditorViewKind {
    pub fn is_normal(&self) -> bool {
        matches!(self, EditorViewKind::Normal)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LineStyle {
    pub start: usize,
    pub end: usize,
    pub text: Option<String>,
    pub fg_color: Option<String>
}
