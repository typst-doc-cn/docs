//! Markdown to Typst conversion.

use pulldown_cmark::{self as md, LinkType};
use typst::diag::StrResult;

use std::io;

use md::escape::StrWrite;
use md::{CodeBlockKind, Event, Tag};

/// Converts a Markdown document to Typst.
pub fn push_typst<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = Event<'a>>,
{
    TypstWriter::new(iter, s).run().unwrap();
}

/// Converts a Markdown document to Typst.
pub fn md_to_typst(text: &str) -> io::Result<String> {
    let options = md::Options::ENABLE_TABLES
        | md::Options::ENABLE_FOOTNOTES
        | md::Options::ENABLE_STRIKETHROUGH
        | md::Options::ENABLE_HEADING_ATTRIBUTES;

    // Convert `[foo]` to `[foo]($foo)`.
    let mut link = |broken: md::BrokenLink| {
        assert_eq!(
            broken.link_type,
            md::LinkType::Shortcut,
            "unsupported link type: {:?}",
            broken.link_type,
        );

        Some((
            format!("${}", broken.reference.trim_matches('`')).into(),
            broken.reference.into_string().into(),
        ))
    };

    let mut events =
        md::Parser::new_with_broken_link_callback(text, options, Some(&mut link)).peekable();
    let mut handler = Handler::new();

    let iter = std::iter::from_fn(|| loop {
        let mut event = events.next()?;
        if handler.handle(&mut event) {
            return Some(event);
        }
    });

    let mut result = String::new();
    let writer = TypstWriter::new(iter, &mut result);
    writer.run()?;

    Ok(result)
}

/// A writer that converts Markdown to Typst.
struct TypstWriter<I, W> {
    /// Iterator supplying events.
    iter: I,
    /// Writer to write to.
    writer: W,
    /// Whether or not the last write wrote a newline.
    end_newline: bool,
    /// Whether or not we are in an enum.
    in_enum: Vec<Option<u64>>,
    /// Whether or not we are in a raw block.
    in_raw: bool,
}

impl<'a, I, W> TypstWriter<I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    /// Creates a new `TypstWriter`.
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            in_enum: vec![],
            in_raw: false,
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => self.write("\n\n"),
            Tag::Heading(level, _id, classes) => {
                let _ = classes;

                self.writer
                    .write_fmt(format_args!("#heading(depth: {})[", level as i32))?;

                Ok(())
            }
            Tag::Table(alignments) => {
                let _ = alignments;

                Ok(())
            }
            Tag::TableHead => Ok(()),
            Tag::TableRow => Ok(()),
            Tag::TableCell => Ok(()),
            Tag::BlockQuote => self.write("#quote["),
            Tag::CodeBlock(info) => {
                if !self.end_newline {
                    self.write_newline()?;
                }

                self.write("``````")?;
                self.in_raw = true;
                match info {
                    CodeBlockKind::Fenced(info) => {
                        self.write(&info)?;
                    }
                    CodeBlockKind::Indented => {}
                }
                self.write("\n")
            }
            Tag::List(v) => {
                self.in_enum.push(v);
                Ok(())
            }
            Tag::Item => {
                let num = self.in_enum.last_mut();

                if let Some(Some(v)) = num {
                    self.writer.write_fmt(format_args!("#enum.item({v})["))?;
                    *v += 1;
                    Ok(())
                } else {
                    self.write("#list.item[")
                }
            }
            Tag::Emphasis => self.write("#emph["),
            Tag::Strong => self.write("#strong["),
            Tag::Strikethrough => self.write("#strike["),
            Tag::Link(LinkType::Inline, dest, _title) => {
                self.writer
                    .write_fmt(format_args!("#link({:?})[", dest.as_ref()))?;
                Ok(())
            }
            Tag::Link(
                LinkType::Reference
                | LinkType::ReferenceUnknown
                | LinkType::Collapsed
                | LinkType::CollapsedUnknown
                | LinkType::Shortcut
                | LinkType::ShortcutUnknown
                | LinkType::Autolink,
                dest,
                _title,
            ) => {
                self.writer
                    .write_fmt(format_args!("#link({:?})[", dest.as_ref()))?;
                Ok(())
            }
            Tag::Link(LinkType::Email, dest, _title) => {
                let mailto = format!("mailto:{}", dest.as_ref());
                self.writer.write_fmt(format_args!("#link({mailto:?})["))?;
                Ok(())
            }
            Tag::Image(_link_type, _dest, _title) => Ok(()),
            // Tag::Link(_link_type, dest, _title) => self
            //     .writer
            //     .write_fmt(format_args!("\n#link({:?})[", dest.as_ref())),
            // Tag::Image(_link_type, dest, title) => self.writer.write_fmt(format_args!(
            //     "\n#image({:?}, alt: {:?})",
            //     dest.as_ref(),
            //     title.as_ref()
            // )),
            Tag::FootnoteDefinition(name) => {
                let _ = name;
                Ok(())
            }
        }
    }

    /// Writes the end of an HTML tag.
    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => self.write("\n")?,
            Tag::Heading(_level, id, _classes) => {
                self.write("];")?;
                if let Some(id) = id {
                    self.writer.write_fmt(format_args!(" #label({id:?})"))?;
                }
            }
            Tag::Table(_) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::BlockQuote => self.write("];")?,
            Tag::CodeBlock(_) => {
                self.in_raw = false;
                self.write("``````\n")?;
            }
            Tag::List(_) => {
                self.in_enum.pop();
            }
            Tag::Item => self.write("];")?,
            Tag::Emphasis => self.write("];")?,
            Tag::Strong => self.write("];")?,
            Tag::Strikethrough => self.write("];")?,
            Tag::Link(_, _, _) => self.write("];")?,
            Tag::Image(_, _, _) => (), // shouldn't happen, handled in start
            Tag::FootnoteDefinition(_) => {}
        }
        Ok(())
    }

    /// Runs the writer, processing all events.
    fn run(mut self) -> io::Result<()> {
        use md::Event::*;
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    if self.in_raw {
                        self.write(&text)?;
                    } else {
                        escape_typst(&mut self.writer, &text)?;
                    }
                }

                // Inline raw. todo: special rules for typst docs.
                Code(text) => {
                    let mut chars = text.chars();
                    let (text, tag) = match (chars.next(), chars.next_back()) {
                        (Some('['), Some(']')) => (&text[1..text.len() - 1], Some("typ")),
                        (Some('{'), Some('}')) => (&text[1..text.len() - 1], Some("typc")),
                        _ => (text.as_ref(), None),
                    };

                    if let Some(tag) = tag {
                        self.writer.write_fmt(format_args!("```{tag} "))?;
                    } else {
                        self.write("``` ")?;
                    }

                    self.write(text)?;
                    self.write(" ```")?;
                }

                Html(html) => {
                    self.write("```raw-html ")?;
                    self.write(&html)?;
                    self.write("```")?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("\n\n")?;
                }
                // todo: what is this
                Rule => {

                    // if self.end_newline {
                    //     self.write("<hr />\n")?;
                    // } else {
                    //     self.write("\n<hr />\n")?;
                    // }
                }
                FootnoteReference(name) => {
                    let _ = name;
                    // let len = self.numbers.len() + 1;
                    // self.write("<sup class=\"footnote-reference\"><a
                    // href=\"#")?; escape_html(&mut
                    // self.writer, &name)?; self.write("\"
                    // >")?; let number =
                    // *self.numbers.entry(name).or_insert(len);
                    // write!(&mut self.writer, "{}", number)?;
                    // self.write("</a></sup>")?;
                }
                TaskListMarker(true) => {}
                TaskListMarker(false) => {}
            }
        }
        Ok(())
    }
}

/// A handler for Markdown events.
struct Handler {
    // outline: Vec<OutlineItem>,
}

impl Handler {
    /// Creates a new `Handler`.
    fn new() -> Self {
        Self {
            // outline: vec![],
        }
    }

    /// Handles a Markdown event.
    fn handle(&mut self, event: &mut md::Event) -> bool {
        match event {
            // Rewrite Markdown images.
            md::Event::Start(md::Tag::Image(_, path, _)) => {
                *path = self.handle_image(path).into();
            }

            // Rewrite HTML images.
            // md::Event::Html(html) if html.starts_with("<img") => {
            //     let range = html_attr_range(html, "src").unwrap();
            //     let path = &html[range.clone()];
            //     let mut buf = html.to_string();
            //     buf.replace_range(range, &self.handle_image(path));
            //     *html = buf.into();
            // }

            // Register HTML headings for the outline.
            // md::Event::Start(md::Tag::Heading(level, id, _)) => {
            //     self.handle_heading(id, level);
            // }

            // Also handle heading closings.
            // md::Event::End(md::Tag::Heading(level, _, _)) => {
            //     nest_heading(level, self.nesting());
            // }

            // Rewrite contributor sections.
            // md::Event::Html(html) if html.starts_with("<contributors") => {
            //     let from = html_attr(html, "from").unwrap();
            //     let to = html_attr(html, "to").unwrap();
            //     let Some(output) = contributors(self.resolver, from, to) else {
            //         return false;
            //     };
            //     *html = output.raw.into();
            // }

            // Rewrite links.
            md::Event::Start(md::Tag::Link(ty, dest, _)) => {
                assert!(
                    matches!(
                        ty,
                        md::LinkType::Inline
                            | md::LinkType::Reference
                            | md::LinkType::ShortcutUnknown
                            | md::LinkType::Autolink
                    ),
                    "unsupported link type: {ty:?}",
                );

                *dest = match self.handle_link(dest) {
                    Ok(link) => link.into(),
                    Err(err) => panic!("invalid link: {dest} ({err})"),
                };
            }

            _ => {}
        }

        true
    }

    /// Handles an image link.
    fn handle_image(&self, link: &str) -> String {
        // if let Some(data) = typst_dev_assets::get_by_name(link) {
        //     self.resolver.image(link, data)
        // } else if let Some(url) = self.resolver.link(link) {
        //     url
        // } else {
        //     panic!("missing image: {link}")
        // }
        link.to_string()
    }

    // fn handle_heading(&mut self, id_slot: &mut Option<&'a str>, level: &mut
    // md::HeadingLevel) {     nest_heading(level, self.nesting());
    //     if *level == md::HeadingLevel::H1 {
    //         return;
    //     }

    //     let body = self.peeked.as_ref();
    //     let default = body.map(|text| text.to_kebab_case());
    //     let has_id = id_slot.is_some();

    //     let id: &'a str = match (&id_slot, default) {
    //         (Some(id), default) => {
    //             if Some(*id) == default.as_deref() {
    //                 eprintln!("heading id #{id} was specified unnecessarily");
    //             }
    //             id
    //         }
    //         (None, Some(default)) => self.ids.alloc(default).as_str(),
    //         (None, None) => panic!("missing heading id {}", self.text),
    //     };

    //     *id_slot = (!id.is_empty()).then_some(id);

    //     // Special case for things like "v0.3.0".
    //     let name = match &body {
    //         _ if id.starts_with('v') && id.contains('.') => id.into(),
    //         Some(body) if !has_id => body.as_ref().into(),
    //         _ => id.to_title_case().into(),
    //     };

    //     let mut children = &mut self.outline;
    //     let mut depth = *level as usize;
    //     while depth > 2 {
    //         if !children.is_empty() {
    //             children = &mut children.last_mut().unwrap().children;
    //         }
    //         depth -= 1;
    //     }

    //     children.push(OutlineItem {
    //         id: id.into(),
    //         name,
    //         children: vec![],
    //     });
    // }

    // todo: handle link
    /// Handles a link.
    fn handle_link(&self, link: &str) -> StrResult<String> {
        // if let Some(link) = self.resolver.link(link) {
        //     return Ok(link);
        // }

        // typst_docs::link::resolve(link, self.resolver.base())
        Ok(link.to_string())
    }
}

/// Creates a table for escaping HTML characters.
const fn create_typst_escape_table() -> [u8; 256] {
    let mut table = [0; 256];
    table[b'"' as usize] = 1;
    table[b'[' as usize] = 2;
    table[b']' as usize] = 3;
    table
}

/// Escapes HTML characters in a string.
static HTML_ESCAPE_TABLE: [u8; 256] = create_typst_escape_table();
/// Escapes HTML characters in a string.
static HTML_ESCAPES: [&str; 4] = ["", "\\\"", "\\[", "\\]"];

/// Writes the given string to the Write sink, replacing special HTML bytes
/// (<, >, &, ") by escape sequences.
pub fn escape_typst<W: StrWrite>(w: W, s: &str) -> io::Result<()> {
    escape_typst_scalar(w, s)
}

/// Escapes HTML characters in a string.
fn escape_typst_scalar<W: StrWrite>(mut w: W, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    let mut mark = 0;
    let mut i = 0;
    while i < s.len() {
        match bytes[i..]
            .iter()
            .position(|&c| HTML_ESCAPE_TABLE[c as usize] != 0)
        {
            Some(pos) => {
                i += pos;
            }
            None => break,
        }
        let c = bytes[i];
        let escape = HTML_ESCAPE_TABLE[c as usize];
        let escape_seq = HTML_ESCAPES[escape as usize];
        w.write_str(&s[mark..i])?;
        w.write_str(escape_seq)?;
        i += 1;
        mark = i; // all escaped characters are ASCII
    }
    w.write_str(&s[mark..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md_to_typst() {
        let input = r#"
# Hello World
This is a test.
```rust
fn main() {
    println!("Hello, world!");
}
```
## Another Heading
"#;

        let expected = r##"#heading(depth: 1)[Hello World];

This is a test.
``````rust
fn main() {
    println!("Hello, world!");
}
``````
#heading(depth: 2)[Another Heading];"##;

        let result = md_to_typst(input).unwrap();
        assert_eq!(result, expected);

        let input = r"「内联盒子」（[box](#x-term-box)）";
        let expected = r##"

「内联盒子」（#link("#x-term-box")[box];）
"##;
        let result = md_to_typst(input).unwrap();
        assert_eq!(result, expected);
    }
}
