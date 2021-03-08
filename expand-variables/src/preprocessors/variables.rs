use super::*;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::{collections::HashMap, ops::Index};

type Table = toml::value::Map<String, toml::Value>;

/// A variable substitution preprocessor.
pub struct Plugin;

impl Plugin {
    pub fn new() -> Self {
        Self
    }
}

fn handle_item(vars: &Table, item: &mut BookItem) {
    if let BookItem::Chapter(chapter) = item {
        chapter.content = substitute_variables(vars, &chapter.content);

        for sub_item in chapter.sub_items.iter_mut() {
            handle_item(vars, sub_item);
        }
    }
}

lazy_static::lazy_static! {
    static ref EXPANSION_REGEX: Regex = Regex::new("${(.*)}").unwrap();
}

fn substitute_variables(vars: &Table, content: &str) -> String {
    let mut new_content = String::new();
    for part in content.split("${{") {
        if let Some(end_index) = part.find("}}") {
            let (var, tail) = part.split_at(end_index);
            let sub = vars
                .get(&var.replace("}}", ""))
                .map(|val| val.as_str())
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("UNKNOWN VARIABLE '{}'", var));
            new_content.push_str(&sub);
            new_content.push_str(&tail.replace("}}", ""));
        } else {
            new_content.push_str(part);
        }
    }
    new_content
}

impl Preprocessor for Plugin {
    fn name(&self) -> &str {
        "variable substitution preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(cfg) = ctx.config.get("variables") {
            book.for_each_mut(|item| handle_item(&cfg.as_table().unwrap(), item));
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
