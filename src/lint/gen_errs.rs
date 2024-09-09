use std::fmt::Display;
use std::path::PathBuf;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use crate::r#static::xs_error::XSError;

pub fn gen_xs_errs(errs: &Vec<XSError>, filename: &str, src: &str) {
    let mut colors = ColorGenerator::new();

    let a = colors.next();
    // let b = colors.next();
    // let out = Color::Fixed(81);

    for error in errs.iter() {
        let (msg, span) = match error {
            XSError::TypeError((msg, span)) => { (msg, span) }
            XSError::NameError((msg, span)) => { (msg, span) }
            XSError::SyntaxError((msg, span)) => { (msg, span) }
            XSError::Warning((msg, span)) => { (msg, span) }
        };
        Report::build(ReportKind::Error, filename, span.start)
            // .with_code(3)
            .with_message("XsError")
            .with_label(
                Label::new((filename, span.start..span.end))
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
            .print((filename, Source::from(src)))
            .unwrap();
    }
}

pub fn gen_errs<T: Display>(kind: &str, errs: &Vec<Rich<T>>, path: &PathBuf, src: &str) {
    let filename = path.to_str().unwrap();
    let mut colors = ColorGenerator::new();

    let a = colors.next();
    for err in errs {
        Report::build(ReportKind::Error, filename, err.span().start)
            .with_message(kind)
            .with_label(
                Label::new((filename, err.span().start..err.span().end))
                    .with_message(err.to_string())
                    .with_color(a)
            )
            .finish()
            .print((filename, Source::from(src)))
            .unwrap();
    }
}