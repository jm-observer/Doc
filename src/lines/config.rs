use std::collections::HashMap;
use floem::peniko::Color;
use lsp_types::DiagnosticSeverity;
use serde::{Deserialize, Serialize};

pub const SCALE_OR_SIZE_LIMIT: f64 = 5.0;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub enum WrapStyle {
    /// No wrapping
    None,
    /// Wrap at the editor width
    #[default]
    EditorWidth,
    // /// Wrap at the wrap-column
    // WrapColumn,
    /// Wrap at a specific width
    WrapWidth,
}
impl WrapStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            WrapStyle::None => "none",
            WrapStyle::EditorWidth => "editor-width",
            // WrapStyle::WrapColumn => "wrap-column",
            WrapStyle::WrapWidth => "wrap-width",
        }
    }

    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(WrapStyle::None),
            "editor-width" => Some(WrapStyle::EditorWidth),
            // "wrap-column" => Some(WrapStyle::WrapColumn),
            "wrap-width" => Some(WrapStyle::WrapWidth),
            _ => None,
        }
    }
}

impl std::fmt::Display for WrapStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;

        Ok(())
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct EditorConfig {
    pub font_family: String,

    font_size: usize,

    pub code_glance_font_size: usize,
    line_height: f64,

    pub smart_tab: bool,

    pub tab_width: usize,

    pub show_tab: bool,

    pub show_bread_crumbs: bool,

    pub scroll_beyond_last_line: bool,
    pub cursor_surrounding_lines: usize,

    pub wrap_style: WrapStyle,

    // pub wrap_column: usize,

    pub wrap_width: usize,
    pub sticky_header: bool,

    pub completion_width: usize,
    pub completion_show_documentation: bool,

    pub completion_item_show_detail: bool,

    pub show_signature: bool,

    pub signature_label_code_block: bool,

    pub auto_closing_matching_pairs: bool,

    pub auto_surround: bool,

    pub hover_delay: u64,

    pub modal_mode_relative_line_numbers: bool,

    pub format_on_save: bool,

    pub normalize_line_endings: bool,

    pub highlight_matching_brackets: bool,

    pub highlight_scope_lines: bool,

    pub enable_inlay_hints: bool,

    pub inlay_hint_font_family: String,

    pub inlay_hint_font_size: usize,

    pub enable_error_lens: bool,

    pub only_render_error_styling: bool,

    pub error_lens_end_of_line: bool,

    pub error_lens_multiline: bool,
    // TODO: Error lens but put entirely on the next line
    // TODO: error lens with indentation matching.



    pub error_lens_font_family: String,

    pub error_lens_font_size: usize,

    pub enable_completion_lens: bool,

    pub enable_inline_completion: bool,

    pub completion_lens_font_family: String,

    pub completion_lens_font_size: usize,

    blink_interval: u64,

    pub multicursor_case_sensitive: bool,

    pub multicursor_whole_words: bool,

    pub show_indent_guide: bool,

    pub autosave_interval: u64,

    pub format_on_autosave: bool,

    pub atomic_soft_tabs: bool,


    pub move_focus_while_search: bool,

    pub diff_context_lines: i32,

    pub bracket_pair_colorization: bool,

    pub bracket_colorization_limit: u64,

    pub files_exclude: String,

    pub diagnostic_error: Color,
    pub diagnostic_warn: Color,
    /// foreground
    pub inlay_hint_fg: Color,
    /// background
    pub inlay_hint_bg: Color,

    pub error_lens_error_foreground: Color,
    pub error_lens_warning_foreground: Color,
    pub error_lens_other_foreground: Color,

    pub completion_lens_foreground: Color,

    pub editor_foreground: Color,

    syntax: HashMap<String, Color>,
}

impl EditorConfig {
    pub fn font_size(&self) -> usize {
        self.font_size.clamp(6, 32)
    }

    pub fn line_height(&self) -> usize {
        let line_height = if self.line_height < SCALE_OR_SIZE_LIMIT {
            self.line_height * self.font_size as f64
        } else {
            self.line_height
        };

        // Prevent overlapping lines
        (line_height.round() as usize).max(self.font_size)
    }

    pub fn inlay_hint_font_size(&self) -> usize {
        if self.inlay_hint_font_size < 5
            || self.inlay_hint_font_size > self.font_size
        {
            self.font_size()
        } else {
            self.inlay_hint_font_size
        }
    }

    pub fn error_lens_font_size(&self) -> usize {
        if self.error_lens_font_size == 0 {
            self.inlay_hint_font_size()
        } else {
            self.error_lens_font_size
        }
    }

    pub fn completion_lens_font_size(&self) -> usize {
        if self.completion_lens_font_size == 0 {
            self.inlay_hint_font_size()
        } else {
            self.completion_lens_font_size
        }
    }

    /// Returns the tab width if atomic soft tabs are enabled.
    pub fn atomic_soft_tab_width(&self) -> Option<usize> {
        if self.atomic_soft_tabs {
            Some(self.tab_width)
        } else {
            None
        }
    }

    pub fn blink_interval(&self) -> u64 {
        if self.blink_interval == 0 {
            return 0;
        }
        self.blink_interval.max(200)
    }

    pub fn color_of_diagnostic(&self, diagnostic_severity: DiagnosticSeverity) -> Option<Color> {
        use DiagnosticSeverity;
        match diagnostic_severity {
            DiagnosticSeverity::ERROR => Some(self.diagnostic_error),
            DiagnosticSeverity::WARNING => Some(self.diagnostic_warn),
            _=> None
        }
    }

    pub fn color_of_error_lens(&self, diagnostic_severity: DiagnosticSeverity) -> Color {
        match diagnostic_severity {
            DiagnosticSeverity::ERROR => self.error_lens_error_foreground,
            DiagnosticSeverity::WARNING => self.error_lens_warning_foreground,
            _=> self.error_lens_other_foreground,
        }
    }

    pub fn syntax_style_color(&self, name: &str) -> Option<Color> {
        self.syntax.get(name).copied()
    }
}


