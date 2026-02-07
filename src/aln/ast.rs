use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnFile {
    pub items: Vec<AlnItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlnItem {
    Block(Block),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub args: Option<BlockArgs>,
    pub body: Vec<BlockEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockArgs {
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockEntry {
    KeyValue { key: String, value: Value },
    NestedBlock(Block),
    List(Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Str(String),
    Bool(bool),
    Number(f64),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}
