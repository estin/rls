use analysis::{raw, Span};
use serde_json;

use actions::{Position, Provider};
use vfs::Change;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub pos: Position,
    pub span: Span,
}

#[derive(Debug, Serialize)]
pub enum Output {
    Ok(Position, Provider),
    Err,
}

#[derive(Debug, Serialize)]
pub enum FmtOutput {
    Change(String),
    Err,
}

#[derive(Debug, Deserialize)]
pub struct ChangeInput {
    pub project_path: String,
    pub changes: Vec<Change>,
}

#[derive(Debug, Deserialize)]
pub struct SaveInput {
    pub project_path: String,
    pub saved_file: String,
}

macro_rules! from_bytes {
    ($input: ty) => {
        impl $input {
            pub fn from_bytes(input: &[u8]) -> Result<$input, serde_json::Error> {
                let s = String::from_utf8(input.to_vec()).unwrap();
                // FIXME: this is gross. There should be a better way to unescape
                let s = unsafe {
                    s.slice_unchecked(1, s.len()-1)
                };
                let s = s.replace("\\\"", "\"");
                //println!("decoding: '{}'", s);
                serde_json::from_str(&s)
            }
        }
    }
}

from_bytes!(Input);
from_bytes!(ChangeInput);
from_bytes!(SaveInput);

pub fn parse_string(input: &[u8]) -> Result<String, serde_json::Error> {
    let s = String::from_utf8(input.to_vec()).unwrap();
    let s = s.replace("\\\"", "\"");
    //println!("decoding: '{}'", s);
    serde_json::from_str(&s)
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum VscodeKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null
}

impl From<raw::DefKind> for VscodeKind {
    fn from(k: raw::DefKind) -> VscodeKind {
        match k {
            raw::DefKind::Enum => VscodeKind::Enum,
            raw::DefKind::Tuple => VscodeKind::Array,
            raw::DefKind::Struct => VscodeKind::Class,
            raw::DefKind::Trait => VscodeKind::Interface,
            raw::DefKind::Function => VscodeKind::Function,
            raw::DefKind::Method => VscodeKind::Function,
            raw::DefKind::Macro => VscodeKind::Function,
            raw::DefKind::Mod => VscodeKind::Module,
            raw::DefKind::Type => VscodeKind::Interface,
            raw::DefKind::Local => VscodeKind::Variable,
            raw::DefKind::Static => VscodeKind::Variable,
            raw::DefKind::Const => VscodeKind::Variable,
            raw::DefKind::Field => VscodeKind::Variable,
            raw::DefKind::Import => VscodeKind::Module,
        }
    }
}

pub fn adjust_vscode_span_for_rls(mut source: Span) -> Span {
    source.line_start += 1;
    source.line_end += 1;
    source.column_start += 1;
    source.column_end += 1;
    source
}

pub fn adjust_vscode_pos_for_racer(mut source: Position) -> Position {
    source.line += 1;
    source
}

pub fn adjust_span_for_vscode(mut source: Span) -> Span {
    source.line_start -= 1;
    source.line_end -= 1;
    source.column_start -= 1;
    source.column_end -= 1;
    source
}

pub fn adjust_rls_pos_for_vscode(mut source: Position) -> Position {
    // FIXME Change RLS-analysis to use 0-indexed columns.
    if source.col > 0 {
        source.col -= 1;
    }
    if source.line > 0 {
        source.line -= 1;
    }
    source
}

pub fn adjust_racer_pos_for_vscode(mut source: Position) -> Position {
    if source.line > 0 {
        source.line -= 1;
    }
    source
}