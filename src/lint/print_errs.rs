use std::collections::HashSet;
use std::path::PathBuf;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

use crate::lint::fmt::msg_fmt;
use crate::r#static::info::{ParseError, XSError};

pub fn print_xs_errs(errs: &Vec<XSError>, filename: &str, src: &str, ignores: &HashSet<u32>) {
    let kwds = Color::Fixed(5);
    let highlight = Color::Fixed(12);
    let names = Color::Fixed(13);
    let types = Color::Fixed(14);

    for error in errs.iter() {
        if ignores.contains(&error.code()) {
            continue;
        }
        let report = Report::build(error.report_kind(), filename, error.span().start)
            .with_code(error.code())
            .with_message(error.kind());
        let report = match error {
            XSError::ExtraArg { fn_name, span } => {
                report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("Extra argument provided to function {}", fn_name.fg(names)))
                        .with_color(highlight)
                )
            }
            XSError::TypeMismatch { actual, expected, span, note } => {
                let report = report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("Expected type {} but found {}", expected.fg(types), actual.fg(types)))
                        .with_color(highlight)
                );
                match note {
                    None => { report }
                    Some(note) => {
                        report.with_help(note)
                    }
                }
            }
            XSError::NotCallable { name, actual, span } => {
                report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("The variable {} is of type {} and not a function", name.fg(names), actual.fg(types)))
                        .with_color(highlight)
                )
            }
            XSError::OpMismatch { op, type1, type2, span, note } => {
                let report = report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("Cannot {} types {} and {}", op, type1.fg(types), type2.fg(types)))
                        .with_color(highlight)
                );
                match note {
                    None => { report }
                    Some(note) => {
                        report.with_help(note)
                    }
                }
            }
            XSError::UndefinedName { name, span } => {
                report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("Name {} is not defined", name.fg(names)))
                        .with_color(highlight)
                )
            }
            XSError::RedefinedName { name, span, note, .. } => {
                let report = report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(format!("Name {} is already defined", name.fg(names)))
                        .with_color(highlight)
                );
                match note {
                    None => { report }
                    Some(note) => {
                        report.with_help(note)
                    }
                }
            }
            XSError::Syntax { span, msg, keywords } => {
                report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(msg_fmt(msg, keywords, &kwds))
                        .with_color(highlight)
                )
            }

            XSError::Warning { span, msg, keywords, .. } => {
                report.with_label(
                    Label::new((filename, span.start..span.end))
                        .with_message(msg_fmt(msg, keywords, &types))
                        .with_color(highlight)
                )
            }
        };
        report
            .finish()
            .print((filename, Source::from(src)))
            .unwrap();
    }
}

pub fn print_parse_errs(errs: &Vec<ParseError>, path: &PathBuf, src: &str) {
    let filename = path.to_str().unwrap();
    let highlight = Color::Fixed(12);
    
    for err in errs {
        let kind = err.kind();
        let (span, msg) = (err.span(), err.msg());
        
        Report::build(ReportKind::Error, filename, span.start)
            .with_message(kind)
            .with_label(
                Label::new((filename, span.start..span.end))
                    .with_message(msg)
                    .with_color(highlight)
            )
            .finish()
            .print((filename, Source::from(src)))
            .unwrap();
    }
}