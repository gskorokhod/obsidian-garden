use pulldown_cmark::Event;
use serde::Serialize;
use std::path::Path;
use thiserror::Error;

use crate::{
    metadata::{parse_frontmatter, Metadata, MetadataError},
    wikilink::{Wikilink, WikilinkParser},
};

#[derive(Debug, PartialEq, Serialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<Wikilink>,
    pub metadata: Metadata,
}

impl Note {
    pub fn parse(title: &str, content: &str) -> Result<Note, NoteError> {
        let (metadata, content) = parse_frontmatter(content)?;

        let parser = pulldown_cmark::Parser::new(content);

        let mut links = Vec::new();
        let mut tags = metadata.tags();

        let mut wikilink_parser = WikilinkParser::new();

        for event in parser {
            if let Event::Text(text) = event {
                if let Some(link) = wikilink_parser.feed(&text) {
                    links.push(link);
                }

                let new_tags = collect_tags(&text);
                tags.extend(new_tags);
            }
        }

        Ok(Note {
            title: title.into(),
            content: content.into(),
            tags,
            links,
            metadata,
        })
    }

    pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<Note, NoteError> {
        let title = path
            .as_ref()
            .file_stem()
            .ok_or(NoteError::NoFileName)?
            .to_string_lossy();
        let content = std::fs::read_to_string(path)?;
        Note::parse(&title, &content)
    }

    pub fn render_html(&self) -> String {
        let mut html_buf = String::new();
        let parser = pulldown_cmark::Parser::new_ext(&self.content, pulldown_cmark::Options::all());
        pulldown_cmark::html::push_html(&mut html_buf, parser);
        html_buf
    }
}

fn collect_tags(text: &str) -> Vec<String> {
    const ALLOWED: &str = "_-/";

    let mut tags = Vec::new();
    let mut next: String = String::new();
    let mut in_tag = false;

    text.chars().for_each(|c| {
        print!("{}", c);

        if c == '#' {
            in_tag = true;
        } else if in_tag {
            if c.is_alphanumeric() || ALLOWED.contains(c) {
                next.push(c);
            } else {
                if !next.is_empty() {
                    tags.push(next.clone());
                    next.clear();
                }
                in_tag = false;
            }
        }
    });

    if !next.is_empty() {
        tags.push(next);
    }
    tags
}

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("io error")]
    IOError(#[from] std::io::Error),

    // #[error("invalid frontmatter type")]
    // FrontMatterInvalidType(),
    #[error("frontmatter value error")]
    MetadataValueError(#[from] MetadataError),

    #[error("no file name")]
    NoFileName,
}

#[cfg(test)]
mod tests {
    use crate::metadata::MetadataValue;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_note() {
        let content = include_str!("../notes/example.md");
        let note = Note::parse("Example", content).expect("note parse");

        let mut metadata = HashMap::new();
        metadata.insert("published".to_string(), MetadataValue::Boolean(true));
        metadata.insert(
            "category".to_string(),
            MetadataValue::String("Example".to_string()),
        );

        assert_eq!(
            note,
            Note {
                title: "Example".to_string(),
                content: r#"#example

Example content. With #test tag inside. Текст может содержать символы Юникода: #даже_в-тегах/hello42.

## Heading 2

[[Page Name|Link label]]

This is a [[WikiLink]]. And this is a [Markdown Link](https://example.com)

Inline `let a = 2 + 2;` example

#code/rust

```rust
fn main () {
    println!("ok");
}
```"#
                    .to_string(),
                tags: vec![
                    "example".to_string(),
                    "test".to_string(),
                    "даже_в-тегах/hello42".to_string(),
                    "code/rust".to_string(),
                ],
                links: vec![
                    Wikilink::new("Page Name", Some("Link label")),
                    Wikilink::new("WikiLink", None),
                ],
                metadata: Metadata::from(metadata),
            }
        );
    }
}
