use std::fmt::{Display, Debug};

#[derive(Debug, PartialEq)]
pub enum TextTree {
    Concat(Vec<TextTree>),
    Lines(Vec<TextTree>),
    List {
        bullet: fn(usize) -> String,
        heading: String,
        children: Vec<TextTree>,
    },
    Nested(Box<TextTree>, Box<TextTree>),
    Text(String),
}

impl TextTree {
    pub fn concat(parts: Vec<TextTree>) -> TextTree {
        TextTree::Concat(parts)
    }

    pub fn debug<T>(value: T) -> TextTree where T: Debug {
        TextTree::Text(format!("{value:?}"))
    }

    pub fn lines(lines: Vec<TextTree>) -> TextTree {
        TextTree::Lines(lines)
    }

    pub fn nested(outer: TextTree, inner: TextTree) -> TextTree {
        TextTree::Nested(Box::new(outer), Box::new(inner))
    }

    pub fn ordered_list(heading: &str, children: Vec<TextTree>) -> TextTree {
        TextTree::List {
            bullet: |index| format!("{index}:"),
            heading: heading.to_string(),
            children,
        }
    }

    pub fn text(text: &str) -> TextTree {
        TextTree::Text(text.to_string())
    }

    pub fn unordered_list(heading: &str, children: Vec<TextTree>) -> TextTree {
        TextTree::List {
            bullet: |_| "*".to_string(),
            heading: heading.to_string(),
            children,
        }
    }

    fn write(&self, writer: &mut TextTreeWriter) {
        match self {
            TextTree::Concat(parts) => {
                for part in parts {
                    part.write(writer);
                }
            },
            TextTree::Lines(lines) => {
                let mut is_first = true;

                for line in lines {
                    if is_first {
                        is_first = false;
                    } else {
                        writer.new_line();
                    }
                    line.write(writer);
                }
            },
            TextTree::List { bullet, heading, children } => {
                writer.write_str(heading);
                writer.write_str(":");
                for (child_index, child) in children.iter().enumerate() {
                    writer.new_line();
                    let prefix = format!(" {} ", bullet(child_index));
                    writer.write_str(&prefix);
                    writer.indented(prefix.chars().count(), |writer| {
                        child.write(writer);
                    })
                }
            },
            TextTree::Nested(outer, inner) => {
                outer.write(writer);
                writer.write_str(":");
                writer.indented(2, |writer| {
                    writer.new_line();
                    inner.write(writer);
                })
            },
            TextTree::Text(text) => {
                writer.write_str(text);
            },
        }
    }
}

impl Display for TextTree {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut writer = TextTreeWriter::new();
        self.write(&mut writer);
        formatter.write_str(&writer.string)
    }
}

struct TextTreeWriter {
    indentation: usize,
    string: String,
}

impl TextTreeWriter {
    fn new() -> Self {
        TextTreeWriter { indentation: 0, string: String::new() }
    }

    fn indented<F>(&mut self, indentation: usize, f: F) where F: Fn(&mut Self) {
        self.indentation += indentation;
        f(self);
        self.indentation -= indentation;
    }

    fn new_line(&mut self) {
        self.string.push_str("\n");
        for _ in 0..self.indentation {
            self.string.push_str(" ");
        }
    }

    fn write_str(&mut self, text: &str) {
        let mut is_first = true;
        for line in text.split("\n") {
            if is_first {
                is_first = false;
            } else {
                self.new_line();
            }
            self.string.push_str(line);
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{TextTree, TextTreeWriter};

    #[test]
    fn concat_combines_children() {
        let tree = TextTree::concat(vec![
            TextTree::text("hello "),
            TextTree::text("world"),
        ]);

        let result = tree.to_string();

        assert_eq!(result, "hello world");
    }

    #[test]
    fn debug_uses_debug_representation_of_value() {
        let tree = TextTree::debug(42);

        let result = tree.to_string();

        assert_eq!(result, "42");
    }

    #[test]
    fn lines_are_separated_by_new_line() {
        let tree = TextTree::lines(vec![
            TextTree::text("hello"),
            TextTree::text("world"),
        ]);

        let result = tree.to_string();

        assert_eq!(result, "hello\nworld");
    }

    #[test]
    fn nested_indents_second_line_from_first_line() {
        let tree = TextTree::nested(
            TextTree::text("hello"),
            TextTree::text("world"),
        );

        let result = tree.to_string();

        assert_eq!(result, "hello:\n  world");
    }

    #[test]
    fn ordered_list_creates_numbered_list_with_indentation() {
        let tree = TextTree::ordered_list("fruit", vec![
            TextTree::text("apples\n1"),
            TextTree::text("bananas\n2"),
        ]);

        let result = tree.to_string();

        assert_eq!(result, indoc! {"
            fruit:
             0: apples
                1
             1: bananas
                2"});
    }

    #[test]
    fn text_without_new_lines_is_written_verbatim() {
        let tree = TextTree::text("apples");

        let result = tree.to_string();

        assert_eq!(result, "apples");
    }

    #[test]
    fn unordered_list_creates_bulleted_list_with_indentation() {
        let tree = TextTree::unordered_list("fruit", vec![
            TextTree::text("apples\n1"),
            TextTree::text("bananas\n2"),
        ]);

        let result = tree.to_string();

        assert_eq!(result, indoc! {"
            fruit:
             * apples
               1
             * bananas
               2"});
    }

    #[test]
    fn written_strings_are_indented_according_to_current_indent_level() {
        let mut writer = TextTreeWriter::new();
        writer.indented(3, |writer| {
            writer.write_str("hello\nworld");
        });

        let result = writer.string;

        assert_eq!(result, "hello\n   world");
    }
}
