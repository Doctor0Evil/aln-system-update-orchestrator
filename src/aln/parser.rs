use crate::aln::ast::*;
use crate::aln::lexer::{lex, LexError, Token, TokenKind};
use crate::aln::model::{
    AlnComponentConfig,
    AlnInteropConfig,
    AlnRenderConfig,
    AlnRegoExecConfig,
    AlnUpdatePlan,
};
use serde_json::json;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadAlnError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Lex error: {0}")]
    Lex(#[from] LexError),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub fn parse_file(path: &str) -> Result<AlnFile, LoadAlnError> {
    let src = fs::read_to_string(path)?;
    let tokens = lex(&src)?;
    parse_tokens(&tokens).map_err(LoadAlnError::Parse)
}

fn parse_tokens(tokens: &[Token]) -> Result<AlnFile, String> {
    let mut p = Parser { tokens, pos: 0 };
    p.parse_file()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse_file(&mut self) -> Result<AlnFile, String> {
        let mut items = Vec::new();
        while !self.eof() {
            items.push(AlnItem::Block(self.parse_block()?));
        }
        Ok(AlnFile { items })
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(TokenKind::At, "@")?;
        let name = match self.next() {
            Some(Token { kind: TokenKind::Identifier(id), .. }) => id.clone(),
            other => {
                return Err(format!("Expected identifier after '@', got {:?}", other));
            }
        };

        let mut args = None;
        if self.peek_is(TokenKind::LBrace) {
            // no args, block starts
        } else if let Some(Token { kind: TokenKind::Identifier(raw), .. }) = self.peek() {
            // lightweight arg payload until '{'
            let mut s = raw.clone();
            self.next();
            while let Some(t) = self.peek() {
                if matches!(t.kind, TokenKind::LBrace) {
                    break;
                }
                match &t.kind {
                    TokenKind::Identifier(id) => {
                        s.push(' ');
                        s.push_str(id);
                    }
                    _ => {}
                }
                self.next();
            }
            args = Some(BlockArgs { raw: s });
        }

        self.expect(TokenKind::LBrace, "{")?;
        let mut body = Vec::new();

        while !self.peek_is(TokenKind::RBrace) && !self.eof() {
            if self.peek_is(TokenKind::At) {
                let nested = self.parse_block()?;
                body.push(BlockEntry::NestedBlock(nested));
            } else {
                body.push(self.parse_entry()?);
            }
        }

        self.expect(TokenKind::RBrace, "}")?;

        Ok(Block { name, args, body })
    }

    fn parse_entry(&mut self) -> Result<BlockEntry, String> {
        // key: value
        let key = match self.next() {
            Some(Token { kind: TokenKind::Identifier(id), .. }) => id.clone(),
            other => return Err(format!("Expected identifier key, got {:?}", other)),
        };

        self.expect(TokenKind::Colon, ":")?;

        if self.peek_is(TokenKind::LBracket) {
            let list = self.parse_list()?;
            Ok(BlockEntry::KeyValue { key, value: Value::Array(list) })
        } else {
            let value = self.parse_value()?;
            Ok(BlockEntry::KeyValue { key, value })
        }
    }

    fn parse_list(&mut self) -> Result<Vec<Value>, String> {
        self.expect(TokenKind::LBracket, "[")?;
        let mut values = Vec::new();
        while !self.peek_is(TokenKind::RBracket) && !self.eof() {
            let v = self.parse_value()?;
            values.push(v);
            if self.peek_is(TokenKind::Comma) {
                self.next();
            }
        }
        self.expect(TokenKind::RBracket, "]")?;
        Ok(values)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        match self.next() {
            Some(Token { kind: TokenKind::StringLiteral(s), .. }) => Ok(Value::Str(s)),
            Some(Token { kind: TokenKind::Identifier(id), .. }) => Ok(Value::Str(id)),
            Some(Token { kind: TokenKind::BoolLiteral(b), .. }) => Ok(Value::Bool(b)),
            Some(Token { kind: TokenKind::NumberLiteral(n), .. }) => {
                let num: f64 = n.parse().map_err(|e| e.to_string())?;
                Ok(Value::Number(num))
            }
            Some(Token { kind: TokenKind::LBracket, .. }) => {
                self.pos -= 1;
                let list = self.parse_list()?;
                Ok(Value::Array(list))
            }
            other => Err(format!("Unexpected token in value: {:?}", other)),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_is(&self, kind: TokenKind) -> bool {
        matches!(self.peek().map(|t| &t.kind), Some(k) if std::mem::discriminant(k) == std::mem::discriminant(&kind))
    }

    fn next(&mut self) -> Option<&Token> {
        let res = self.tokens.get(self.pos);
        if res.is_some() {
            self.pos += 1;
        }
        res
    }

    fn expect(&mut self, kind: TokenKind, what: &str) -> Result<(), String> {
        match self.next() {
            Some(Token { kind: k, .. }) if std::mem::discriminant(k) == std::mem::discriminant(&kind) => Ok(()),
            other => Err(format!("Expected {}, got {:?}", what, other)),
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

/// High-level helper: load an ALN update plan from file and map to a model
impl AlnUpdatePlan {
    pub fn from_file(path: &str) -> Result<Self, LoadAlnError> {
        let file = parse_file(path)?;
        let mut components = None;
        let mut interop = None;
        let mut render = None;
        let mut rego_exec = None;

        for item in file.items {
            if let AlnItem::Block(b) = item {
                if b.name == "ALN_UPDATE_SYSTEM" {
                    for entry in b.body {
                        match entry {
                            BlockEntry::KeyValue { key, value } if key == "version" => {
                                // version is optional, may be missing
                                let _ = value;
                            }
                            BlockEntry::NestedBlock(nb) if nb.name == "SEPARATE" => {
                                components = Some(map_components(nb)?);
                            }
                            BlockEntry::NestedBlock(nb) if nb.name == "INTEROP" => {
                                interop = Some(map_interop(nb)?);
                            }
                            BlockEntry::NestedBlock(nb) if nb.name == "RENDER_IN_FRAME" => {
                                render = Some(map_render(nb)?);
                            }
                            BlockEntry::NestedBlock(nb) if nb.name == "EXEC_REGO_POLICY" => {
                                rego_exec = Some(map_rego_exec(nb)?);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let components = components.ok_or_else(|| LoadAlnError::Parse("Missing @SEPARATE components".into()))?;
        let interop = interop.ok_or_else(|| LoadAlnError::Parse("Missing @INTEROP block".into()))?;
        let render = render.ok_or_else(|| LoadAlnError::Parse("Missing @RENDER_IN_FRAME block".into()))?;
        let rego_exec = rego_exec.ok_or_else(|| LoadAlnError::Parse("Missing @EXEC_REGO_POLICY block".into()))?;

        Ok(Self {
            version: "1.0.1.7".into(),
            components,
            interop,
            render,
            rego_exec,
        })
    }
}

fn map_components(block: Block) -> Result<AlnComponentConfig, LoadAlnError> {
    let mut game_engine = None;
    let mut ai_chat_ui = None;
    let mut renderers = Vec::new();

    for entry in block.body {
        if let BlockEntry::KeyValue { key, value } = entry {
            match (key.as_str(), value) {
                ("game_engine", Value::Str(s)) => game_engine = Some(s),
                ("ai_chat_ui", Value::Str(s)) => ai_chat_ui = Some(s),
                ("renderers", Value::Array(arr)) => {
                    for v in arr {
                        if let Value::Str(s) = v {
                            renderers.push(s);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(AlnComponentConfig {
        game_engine: game_engine.unwrap_or_default(),
        ai_chat_ui: ai_chat_ui.unwrap_or_default(),
        renderers,
    })
}

fn map_interop(block: Block) -> Result<AlnInteropConfig, LoadAlnError> {
    let mut cross_link = None;
    let mut maintain_func = None;
    let mut enable_lan = None;

    for entry in block.body {
        if let BlockEntry::KeyValue { key, value } = entry {
            match (key.as_str(), value) {
                ("cross_link", Value::Str(s)) => cross_link = Some(s),
                ("maintain_func", Value::Bool(b)) => maintain_func = Some(b),
                ("enable_lan", Value::Str(s)) => enable_lan = Some(s),
                _ => {}
            }
        }
    }

    Ok(AlnInteropConfig {
        cross_link: cross_link.unwrap_or_default(),
        maintain_func: maintain_func.unwrap_or(true),
        enable_lan: enable_lan.unwrap_or_default(),
    })
}

fn map_render(block: Block) -> Result<AlnRenderConfig, LoadAlnError> {
    let mut mode = None;
    let mut merge_sources = None;
    let mut playable_platforms = Vec::new();

    for entry in block.body {
        if let BlockEntry::KeyValue { key, value } = entry {
            match (key.as_str(), value) {
                ("mode", Value::Str(s)) => mode = Some(s),
                ("merge_sources", Value::Bool(b)) => merge_sources = Some(b),
                ("playable_platforms", Value::Array(arr)) => {
                    for v in arr {
                        if let Value::Str(s) = v {
                            playable_platforms.push(s);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(AlnRenderConfig {
        mode: mode.unwrap_or_default(),
        merge_sources: merge_sources.unwrap_or(true),
        playable_platforms,
    })
}

fn map_rego_exec(block: Block) -> Result<AlnRegoExecConfig, LoadAlnError> {
    let mut always_active = None;
    let mut policy = None;
    let mut features = Vec::new();

    for entry in block.body {
        if let BlockEntry::KeyValue { key, value } = entry {
            match (key.as_str(), value) {
                ("always_active", Value::Bool(b)) => always_active = Some(b),
                ("policy", Value::Str(s)) => policy = Some(s),
                ("features", Value::Array(arr)) => {
                    for v in arr {
                        if let Value::Str(s) = v {
                            features.push(s);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(AlnRegoExecConfig {
        always_active: always_active.unwrap_or(true),
        policy: policy.unwrap_or_default(),
        features,
    })
}
