use std::collections::HashSet;
use std::fmt::Display;
use std::path::PathBuf;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::error::Rich;

use crate::static_old::xs_error::XSError;

fn msg_fmt(mut msg: &str, keywords: &[String], color: &Color) -> String {
    let mut result = String::new();

    for (idx, kw) in keywords.iter().enumerate() {
        let idx = format!("{{{}}}", idx);
        if let Some(pos) = msg.find(&idx) {
            result.push_str(&msg[..pos]);
            result.push_str(&kw.fg(*color).to_string());
            msg = &msg[pos + idx.len()..];
        }
    }
    result.push_str(&msg);

    result
}


pub fn gen_xs_errs(errs: &Vec<XSError>, filename: &str, src: &str, ignores: &HashSet<u32>) {
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

pub fn gen_errs<T: Display>(kind: &str, errs: &Vec<Rich<T>>, path: &PathBuf, src: &str) {
    let filename = path.to_str().unwrap();
    let highlight = Color::Fixed(12);

    for err in errs {
        Report::build(ReportKind::Error, filename, err.span().start)
            .with_message(kind)
            .with_label(
                Label::new((filename, err.span().start..err.span().end))
                    .with_message(err.to_string())
                    .with_color(highlight)
            )
            .finish()
            .print((filename, Source::from(src)))
            .unwrap();
    }
}