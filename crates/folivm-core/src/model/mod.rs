use serde::{Deserialize, Serialize};
use crate::text_engine::buffer::RunBuffer;

pub type BlockId = uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentModel {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub blocks: Vec<Block>,
    pub fvm_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub title: String,
    pub author: Option<String>,
    pub date: Option<String>,
    pub lang: Option<String>,
    pub version: Option<String>,
    pub tags: Vec<String>,
    pub page_size: Option<String>,
    pub page_width: Option<f64>,
    pub page_height: Option<f64>,
    pub orientation: Option<String>,
    pub margins: Option<Margins>,
    pub theme: Option<String>,
    pub header: Option<HeaderFooter>,
    pub footer: Option<HeaderFooter>,
    pub extra: std::collections::BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
    pub right: Option<f64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HeaderFooter {
    pub left: Option<String>,
    pub center: Option<String>,
    pub right: Option<String>,
}

// NOTE: Block and Inline carry NO font, size, colour, or weight fields.
// All presentation is expressed through named styles from the active theme.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Block {
    Paragraph {
        id: BlockId,
        style: String,
        content: RunBuffer,
    },
    Heading {
        id: BlockId,
        level: u8,
        style: String,
        content: RunBuffer,
    },
    List {
        id: BlockId,
        style: String,
        items: Vec<ListItem>,
        ordered: bool,
    },
    Blockquote {
        id: BlockId,
        style: String,
        blocks: Vec<Block>,
    },
    Table {
        id: BlockId,
        style: String,
        rows: Vec<TableRow>,
    },
    Section {
        id: BlockId,
        attrs: SectionAttrs,
        blocks: Vec<Block>,
    },
    Callout {
        id: BlockId,
        style: String,
        blocks: Vec<Block>,
    },
    Cell(CellBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Inline {
    Text(String),
    Styled {
        style: String,
        inlines: Vec<Inline>,
    },
    /// Token substitution, e.g. `{title}`, `{page}`.
    Token {
        path: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListItem {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableCell {
    pub colspan: u32,
    pub rowspan: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionAttrs {
    pub id: Option<String>,
    pub title: Option<String>,
    pub numbered: bool,
    pub page_break_before: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellBlock {
    pub id: BlockId,
    /// "cell:image" | "cell:math" | "cell:include" | "cell:data" | "cell:ai" |
    /// "cell:diagram" | "cell:citation" | "cell:signature"
    pub cell_type: String,
    pub attrs: serde_json::Value,
}

impl Block {
    /// Get the block's ID if it has one.
    pub fn id(&self) -> Option<BlockId> {
        match self {
            Block::Paragraph { id, .. }
            | Block::Heading { id, .. }
            | Block::List { id, .. }
            | Block::Blockquote { id, .. }
            | Block::Table { id, .. }
            | Block::Section { id, .. }
            | Block::Callout { id, .. } => Some(*id),
            Block::Cell(cell) => Some(cell.id),
        }
    }
}

/// An atomic edit to the document. All measurements are in points (pt); never px.
///
/// Invariant: `parse(serialise(model)) == model` must hold after every applied operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EditOperation {
    Insert {
        block_id: BlockId,
        offset: usize,
        text: String,
    },
    Delete {
        block_id: BlockId,
        start: usize,
        end: usize,
    },
    Split {
        block_id: BlockId,
        offset: usize,
    },
    Merge {
        block_a: BlockId,
        block_b: BlockId,
    },
    SetBlock {
        block_id: BlockId,
        style: String,
    },
    SetInline {
        block_id: BlockId,
        start: usize,
        end: usize,
        style: Option<String>,
    },
    InsertBlock {
        after: Option<BlockId>,
        block: Block,
    },
    RemoveBlock {
        block_id: BlockId,
    },
}
