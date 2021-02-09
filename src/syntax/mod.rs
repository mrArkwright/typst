//! Syntax types.

mod expr;
mod ident;
mod node;
mod span;
mod token;
pub mod visit;

pub use expr::*;
pub use ident::*;
pub use node::*;
pub use span::*;
pub use token::*;

use crate::pretty::{Pretty, Printer};

/// The abstract syntax tree.
pub type Tree = Vec<Node>;

impl Pretty for Tree {
    fn pretty(&self, p: &mut Printer) {
        for node in self {
            node.pretty(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;
    use crate::pretty::pretty;

    #[track_caller]
    fn test(src: &str, exp: &str) {
        let tree = parse(src).output;
        let found = pretty(&tree);
        if exp != found {
            println!("tree:     {:#?}", tree);
            println!("expected: {}", exp);
            println!("found:    {}", found);
            panic!("test failed");
        }
    }

    #[track_caller]
    fn roundtrip(src: &str) {
        test(src, src);
    }

    #[test]
    fn test_pretty_print_node() {
        // Basic text and markup.
        roundtrip("*");
        roundtrip("_");
        roundtrip(" ");
        roundtrip("\\ ");
        roundtrip("\n\n");
        roundtrip("hi");

        // Heading.
        roundtrip("= *Ok*");

        // Raw.
        roundtrip("``");
        roundtrip("`nolang 1`");
        roundtrip("```lang 1```");
        roundtrip("```lang 1 ```");
        roundtrip("```hi  line  ```");
        roundtrip("```py\ndef\n```");
        roundtrip("```\n line \n```");
        roundtrip("```\n`\n```");
        roundtrip("``` ` ```");
        roundtrip("````\n```\n```\n````");
        test("```lang```", "```lang ```");
        test("```1 ```", "``");
        test("``` 1```", "`1`");
        test("``` 1 ```", "`1 `");
        test("```` ` ````", "``` ` ```");
    }

    #[test]
    fn test_pretty_print_expr() {
        // Basic expressions.
        roundtrip("{none}");
        roundtrip("{hi}");
        roundtrip("{true}");
        roundtrip("{10}");
        roundtrip("{3.14}");
        roundtrip("{10.0pt}");
        roundtrip("{14.1deg}");
        roundtrip("{20.0%}");
        roundtrip("{#abcdef}");
        roundtrip(r#"{"hi"}"#);
        test(r#"{"let's \" go"}"#, r#"{"let's \" go"}"#);

        // Arrays.
        roundtrip("{()}");
        roundtrip("{(1)}");
        roundtrip("{(1, 2, 3)}");

        // Dictionaries.
        roundtrip("{(:)}");
        roundtrip("{(key: value)}");
        roundtrip("{(a: 1, b: 2)}");

        // Templates.
        roundtrip("[]");
        roundtrip("[*Ok*]");
        roundtrip("{[f]}");

        // Groups.
        roundtrip("{(1)}");

        // Blocks.
        roundtrip("{}");
        roundtrip("{1}");
        roundtrip("{ #let x = 1; x += 2; x + 1 }");
        roundtrip("[{}]");

        // Operators.
        roundtrip("{-x}");
        roundtrip("{not true}");
        roundtrip("{1 + 3}");

        // Parenthesized calls.
        roundtrip("{v()}");
        roundtrip("{v(1)}");
        roundtrip("{v(a: 1, b)}");

        // Function templates.
        roundtrip("#[v]");
        roundtrip("#[v 1]");
        roundtrip("#[v 1, 2][*Ok*]");
        roundtrip("#[v 1 | f 2]");
        test("{#[v]}", "{v()}");
        test("#[v 1, #[f 2]]", "#[v 1 | f 2]");

        // Keywords.
        roundtrip("#let x = 1 + 2");
        roundtrip("#if x [y] #else [z]");
        roundtrip("#for x #in y {z}");
        roundtrip("#for k, x #in y {z}");
    }
}
