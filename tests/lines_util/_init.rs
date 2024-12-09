use std::fs::File;
use std::path::{Path, PathBuf};
use floem::kurbo::Rect;
use floem::reactive::{RwSignal, Scope};
use floem::views::editor::EditorStyle;
use floem_editor_core::buffer::Buffer;
use floem_editor_core::buffer::rope_text::RopeText;
use itertools::Itertools;
use lapce_xi_rope::Interval;
use lapce_xi_rope::spans::{Spans, SpansBuilder};
use log::info;
use lsp_types::{InlayHint, Position};
use doc::config::EditorConfig;
use doc::{DiagnosticData, EditorViewKind};
use doc::language::LapceLanguage;
use doc::lines::fold::{FoldingDisplayItem, FoldingDisplayType, FoldingRange};
use doc::lines::{DocLines, RopeTextPosition};
use doc::syntax::{BracketParser, Syntax};

pub fn _init_lsp_folding_range() -> Vec<FoldingRange> {
    let folding_range = r#"[{"startLine":0,"startCharacter":10,"endLine":7,"endCharacter":1},{"startLine":1,"startCharacter":12,"endLine":3,"endCharacter":5},{"startLine":3,"startCharacter":11,"endLine":5,"endCharacter":5}]"#;
    let folding_range: Vec<lsp_types::FoldingRange> = serde_json::from_str(folding_range).unwrap();

    folding_range
        .into_iter()
        .map(FoldingRange::from_lsp)
        .sorted_by(|x, y| x.start.line.cmp(&y.start.line))
        .collect()
}

pub fn _init_inlay_hint(buffer: &Buffer) -> Spans<InlayHint> {
    let hints = r#"[{"position":{"line":6,"character":9},"label":[{"value":": "},{"value":"A","location":{"uri":"file:///d:/git/check/src/main.rs","range":{"start":{"line":8,"character":7},"end":{"line":8,"character":8}}}}],"kind":1,"textEdits":[{"range":{"start":{"line":6,"character":9},"end":{"line":6,"character":9}},"newText":": A"}],"paddingLeft":false,"paddingRight":false}]"#;
    let mut hints: Vec<InlayHint> = serde_json::from_str(hints).unwrap();
    let len = buffer.len();
    hints.sort_by(|left, right| left.position.cmp(&right.position));
    let mut hints_span = SpansBuilder::new(len);
    for hint in hints {
        let offset = buffer.offset_of_position(&hint.position).min(len);
        hints_span.add_span(
            Interval::new(offset, (offset + 1).min(len)),
            hint,
        );
    }
    hints_span.build()
}

pub fn _init_code(file: PathBuf) -> (String, Buffer) {
    // let code = "pub fn main() {\r\n    if true {\r\n        println!(\"startss\");\r\n    } else {\r\n        println!(\"startss\");\r\n    }\r\n    let a = A;\r\n}\r\nstruct A;\r\n";
    let code = load_code(&file);
    let buffer = Buffer::new(
        code.as_str()
    );
    info!("line_ending {:?} len={}", buffer.line_ending(), code.len());
    (code, buffer)
}

pub fn _init_origin_code((code, buffer): (String, Buffer)) -> (DocLines, RwSignal<EditorConfig>) {
    _init_lines(None, (code, buffer))
}

///  2|   if true {...} else {\r\n
pub fn _init_folded_code_v1((code, buffer): (String, Buffer)) -> (DocLines, RwSignal<EditorConfig>) {
    _init_lines(Some(vec![FoldingDisplayItem {
        position: Position {
            line: 1,
            character: 12,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldStart,
    }]), (code, buffer))
}

///  2|   if true {...} else {...}\r\n
pub fn _init_folded_code_v2((code, buffer): (String, Buffer)) -> (DocLines, RwSignal<EditorConfig>) {
    _init_lines(Some(vec![FoldingDisplayItem {
        position: Position {
            line: 1,
            character: 12,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldStart,
    }, FoldingDisplayItem {
        position: Position {
            line: 5,
            character: 5,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldEnd,
    }]), (code, buffer))
}

fn _init_lines(folded: Option<Vec<FoldingDisplayItem>>, (code, buffer): (String, Buffer)) -> (DocLines, RwSignal<EditorConfig>) {
    let folding = _init_lsp_folding_range();
    let hints = _init_inlay_hint(&buffer);

    let config_str = r##"{"auto_closing_matching_pairs":true, "auto_surround":true,"font_family":"JetBrains Mono","font_size":13,"line_height":23,"enable_inlay_hints":true,"inlay_hint_font_size":0,"enable_error_lens":false,"error_lens_end_of_line":true,"error_lens_multiline":false,"error_lens_font_size":0,"enable_completion_lens":false,"enable_inline_completion":true,"completion_lens_font_size":0,"only_render_error_styling":false,"diagnostic_error":{"r":229,"g":20,"b":0,"a":255},"diagnostic_warn":{"r":233,"g":167,"b":0,"a":255},"inlay_hint_fg":{"r":108,"g":118,"b":128,"a":255},"inlay_hint_bg":{"r":245,"g":245,"b":245,"a":255},"error_lens_error_foreground":{"r":228,"g":86,"b":73,"a":255},"error_lens_warning_foreground":{"r":193,"g":132,"b":1,"a":255},"error_lens_other_foreground":{"r":160,"g":161,"b":167,"a":255},"completion_lens_foreground":{"r":160,"g":161,"b":167,"a":255},"editor_foreground":{"r":56,"g":58,"b":66,"a":255},"syntax":{"punctuation.delimiter":{"r":193,"g":132,"b":1,"a":255},"attribute":{"r":193,"g":132,"b":1,"a":255},"method":{"r":64,"g":120,"b":242,"a":255},"bracket.color.3":{"r":166,"g":38,"b":164,"a":255},"builtinType":{"r":18,"g":63,"b":184,"a":255},"enumMember":{"r":146,"g":17,"b":167,"a":255},"bracket.color.2":{"r":193,"g":132,"b":1,"a":255},"markup.heading":{"r":228,"g":86,"b":73,"a":255},"markup.link.url":{"r":64,"g":120,"b":242,"a":255},"string.escape":{"r":1,"g":132,"b":188,"a":255},"structure":{"r":193,"g":132,"b":1,"a":255},"text.reference":{"r":193,"g":132,"b":1,"a":255},"comment":{"r":160,"g":161,"b":167,"a":255},"markup.list":{"r":209,"g":154,"b":102,"a":255},"variable.other.member":{"r":228,"g":86,"b":73,"a":255},"type":{"r":56,"g":58,"b":66,"a":255},"keyword":{"r":7,"g":60,"b":183,"a":255},"text.uri":{"r":1,"g":132,"b":188,"a":255},"enum":{"r":56,"g":58,"b":66,"a":255},"constructor":{"r":193,"g":132,"b":1,"a":255},"interface":{"r":56,"g":58,"b":66,"a":255},"selfKeyword":{"r":166,"g":38,"b":164,"a":255},"type.builtin":{"r":1,"g":132,"b":188,"a":255},"escape":{"r":1,"g":132,"b":188,"a":255},"field":{"r":228,"g":86,"b":73,"a":255},"function.method":{"r":64,"g":120,"b":242,"a":255},"markup.link.text":{"r":166,"g":38,"b":164,"a":255},"property":{"r":136,"g":22,"b":150,"a":255},"struct":{"r":56,"g":58,"b":66,"a":255},"bracket.color.1":{"r":64,"g":120,"b":242,"a":255},"enum-member":{"r":228,"g":86,"b":73,"a":255},"string":{"r":80,"g":161,"b":79,"a":255},"text.title":{"r":209,"g":154,"b":102,"a":255},"bracket.unpaired":{"r":228,"g":86,"b":73,"a":255},"constant":{"r":193,"g":132,"b":1,"a":255},"typeAlias":{"r":56,"g":58,"b":66,"a":255},"function":{"r":61,"g":108,"b":126,"a":255},"markup.link.label":{"r":166,"g":38,"b":164,"a":255},"markup.bold":{"r":209,"g":154,"b":102,"a":255},"markup.italic":{"r":209,"g":154,"b":102,"a":255},"number":{"r":193,"g":132,"b":1,"a":255},"tag":{"r":64,"g":120,"b":242,"a":255},"variable":{"r":56,"g":58,"b":66,"a":255},"embedded":{"r":1,"g":132,"b":188,"a":255}}}"##;
    let config: EditorConfig = serde_json::from_str(config_str).unwrap();
    let cx = Scope::new();
    let config = cx.create_rw_signal(config);
    let diagnostics = DiagnosticData {
        expanded: cx.create_rw_signal(false),
        diagnostics: cx.create_rw_signal(im::Vector::new()),
        diagnostics_span: cx.create_rw_signal(Spans::default()),
    };
    // { x0: 0.0, y0: 0.0, x1: 591.1680297851563, y1: 538.1586303710938 }
    let view = Rect::new(0.0, 0.0, 591.0, 538.0);
    let editor_style = EditorStyle::default();
    let kind = cx.create_rw_signal(EditorViewKind::Normal);
    let language = LapceLanguage::Rust;
    let grammars_dir: PathBuf = "C:\\Users\\36225\\AppData\\Local\\lapce\\Lapce-Debug\\data\\grammars".into();


    let queries_directory: PathBuf = "C:\\Users\\36225\\AppData\\Roaming\\lapce\\Lapce-Debug\\config\\queries".into();

    let syntax = Syntax::from_language(language, &grammars_dir, &queries_directory);
    let parser = BracketParser::new(code.to_string(), true, 30000);
    let mut lines = DocLines::new(
        cx,
        diagnostics, syntax, parser,
        view,
        editor_style,
        config.read_only(),
        buffer,
        kind,
    );
    lines.update_folding_ranges(folding.into());
    lines.set_inlay_hints(hints);
    if let Some(folded) = folded {
        for folded in folded {
            lines.update_folding_ranges(folded.into());
        }
    }
    lines.log();
    (lines, config)
}

fn load_code(file: &Path) -> String {
    std::fs::read_to_string(file).unwrap()
}