//! Splits a markdown string into paragraphs, taking care of code blocks.

use super::MARKDOWN_PAR_SEP;

/// Splits a markdown string into paragraphs.
pub fn split_markdown(markdown: &str) -> Vec<&str> {
    // parse raw code

    let mut ranges = vec![];

    let mut start = None;
    for (idx, ch) in markdown.char_indices() {
        if ch == '`' {
            if let Some((s, t)) = start {
                if idx < s + t {
                    // inside code block
                    continue;
                }
            }

            let mut ticks = 1;
            while idx + ticks < markdown.len() && markdown[idx + ticks..].starts_with('`') {
                ticks += 1;
            }
            if ticks >= 3 {
                if start.is_some_and(|(_, prev_ticks)| prev_ticks == ticks) {
                    // end of code block
                    ranges.push((start.unwrap().0, idx + ticks));
                    start = None;
                } else {
                    // start of code block
                    start = Some((idx, ticks));
                }
            }
        }
    }

    if ranges.is_empty() {
        return markdown
            .split(MARKDOWN_PAR_SEP)
            .filter(|s| !s.is_empty())
            .collect();
    }

    let mut result = vec![];

    ranges.reverse();
    let mut last_range_s = ranges.pop();
    let mut last_match = 0;
    'matching: for (s, _) in markdown.match_indices(MARKDOWN_PAR_SEP) {
        let Some(last_range) = last_range_s.as_mut() else {
            break;
        };

        if s >= last_range.0 && s < last_range.1 {
            // inside code block
            continue;
        }
        while s >= last_range.1 {
            push_result(&mut result, &markdown[last_match..last_range.0]);
            push_result(&mut result, &markdown[last_range.0..last_range.1]);
            last_match = last_range.1;

            let Some(range) = ranges.pop() else {
                last_range_s = None;
                break 'matching;
            };
            *last_range = range;
        }
        if s < last_match || (s >= last_range.0 && s < last_range.1) {
            // inside code block
            continue;
        }
        if last_match != s {
            // not inside code block
            push_result(&mut result, &markdown[last_match..s]);
        }
        last_match = s + MARKDOWN_PAR_SEP.len();
    }

    if let Some(last_range) = last_range_s.as_mut() {
        while markdown.len() >= last_range.1 {
            push_result(&mut result, &markdown[last_match..last_range.0]);
            push_result(&mut result, &markdown[last_range.0..last_range.1]);
            last_match = last_range.1;

            let Some(range) = ranges.pop() else {
                break;
            };

            *last_range = range;
        }
    }

    if last_match < markdown.len() {
        for res in markdown[last_match..]
            .split(MARKDOWN_PAR_SEP)
            .filter(|s| !s.is_empty())
        {
            push_result(&mut result, res);
        }
    }

    result
}

/// Push a non-empty text to the result vector.
fn push_result<'a>(result: &mut Vec<&'a str>, text: &'a str) {
    let text = text.trim();
    if !text.is_empty() {
        result.push(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_markdown() {
        fn test(markdown: String) -> String {
            format!("{:?}", split_markdown(&markdown))
        }

        macro_rules! do_test {
            ($markdown:expr) => {
                test(format!($markdown))
            };
        }

        let x = "This is a test.";
        let y = "This is another test.";
        let r = "````rust\nlet x = 1;\n````\n";
        let r2 = "````rust\nlet x\n\n = 1;\n````\n";
        let sep = "\n\n";
        insta::assert_snapshot!(do_test!("{x}{sep}{y}"), @r###"["This is a test.", "This is another test."]"###);
        insta::assert_snapshot!(do_test!("{x}{sep}{sep}{y}"), @r#"["This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{y}{sep}"), @r#"["This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{sep}{x}{sep}{y}"), @r#"["This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{sep}{x}{sep}{y}{sep}"), @r#"["This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{r}{x}{sep}{y}"), @r#"["````rust\nlet x = 1;\n````", "This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{r}{sep}{y}"), @r#"["This is a test.", "````rust\nlet x = 1;\n````", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{r}{y}"), @r#"["This is a test.", "````rust\nlet x = 1;\n````", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{y}{r}"), @r#"["This is a test.", "This is another test.", "````rust\nlet x = 1;\n````"]"#);
        insta::assert_snapshot!(do_test!("{r2}{x}{sep}{y}"), @r#"["````rust\nlet x\n\n = 1;\n````", "This is a test.", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{r2}{sep}{y}"), @r#"["This is a test.", "````rust\nlet x\n\n = 1;\n````", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{r2}{y}"), @r#"["This is a test.", "````rust\nlet x\n\n = 1;\n````", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{y}{r2}"), @r#"["This is a test.", "This is another test.", "````rust\nlet x\n\n = 1;\n````"]"#);
        insta::assert_snapshot!(do_test!("{r}{x}{sep}{y}{r2}"), @r#"["````rust\nlet x = 1;\n````", "This is a test.", "This is another test.", "````rust\nlet x\n\n = 1;\n````"]"#);
        insta::assert_snapshot!(do_test!("{x}{r}{sep}{y}{r2}"), @r#"["This is a test.", "````rust\nlet x = 1;\n````", "This is another test.", "````rust\nlet x\n\n = 1;\n````"]"#);
        insta::assert_snapshot!(do_test!("{x}{r}{r2}{sep}{y}"), @r#"["This is a test.", "````rust\nlet x = 1;\n````", "````rust\nlet x\n\n = 1;\n````", "This is another test."]"#);
        insta::assert_snapshot!(do_test!("{x}{sep}{y}{r}{r2}"), @r#"["This is a test.", "This is another test.", "````rust\nlet x = 1;\n````", "````rust\nlet x\n\n = 1;\n````"]"#);
    }
}
