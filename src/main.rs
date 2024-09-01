pub mod parsing;
pub mod r#static;

use std::{env, fs};
use std::collections::{HashMap, HashSet};
use chumsky::prelude::*;
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use crate::parsing::lexer::lexer;
use crate::parsing::parser::parser;
use crate::r#static::type_check::statements::xs_tc;
use crate::r#static::xs_error::XSError;

fn main() {
    let filename = env::args().nth(1).expect("Filename not provided");
    let src = fs::read_to_string(filename.as_str()).expect("Failed to read file");
    
    let (tokens, errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();
    
    let Some(tokens) = tokens else {
        println!("TokenizationErrors: {:?}", errs);
        return;
    };

    let (ast, parse_errors) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        println!("ParsingErrors: {:?}", parse_errors);
        return;
    };

    let mut type_env = HashMap::new();
    let mut local_envs = HashMap::new();
    let mut groups = HashSet::new();
    let mut errs = vec![];

    xs_tc(&ast, &mut None, &mut type_env, &mut local_envs, &mut groups, &mut errs);
    println!("TypeEnv: {:?}", type_env);
    println!("Errors: {:?}", errs);

    let mut colors = ColorGenerator::new();

    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    for error in errs {
        let (msg, span) = match error {
            XSError::TypeError((msg, span)) => { (msg, span) }
            XSError::NameError((msg, span)) => { (msg, span) }
            XSError::SyntaxError((msg, span)) => { (msg, span) }
            XSError::Warning((msg, span)) => { (msg, span) }
        };
        Report::build(ReportKind::Error, filename.as_str(), span.start)
            .with_code(3)
            .with_message("XsError")
            .with_label(
                Label::new((filename.as_str(), span.start..span.end))
                    .with_message(msg)
                    .with_color(a),
            )
            // .with_label(
            //     Label::new((filename.as_str(), 42..45))
            //         .with_message(format!("This is of type {}", "Str".fg(b)))
            //         .with_color(b),
            // )
            // .with_label(
            //     Label::new((filename.as_str(), 11..48))
            //         .with_message(format!(
            //             "The values are outputs of this {} expression",
            //             "match".fg(out),
            //         ))
            //         .with_color(out),
            // )
            // .with_note(format!(
            //     "Outputs of {} expressions must coerce to the same type",
            //     "match".fg(out)
            // ))
            .finish()
            .print((filename.as_str(), Source::from(src.as_str())))
            .unwrap();
    }
}
