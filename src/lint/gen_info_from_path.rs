use std::collections::{HashSet};
use std::fs;
use std::path::PathBuf;
use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};
use crate::lint::gen_info_from_src::gen_info_from_src;
use crate::r#static::type_check::{LocalEnv, TypeEnv};
use crate::r#static::xs_error::XSError;

pub fn gen_info_from_path(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: PathBuf
) {
    let src = fs::read_to_string(&path).expect("Failed to read file");
    let filename = path.to_str().unwrap();
    
    let errs = gen_info_from_src(
        type_env, local_envs, groups,
        &path, &src
    );

    let mut colors = ColorGenerator::new();

    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

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
            .print((filename, Source::from(src.as_str())))
            .unwrap();
    }
}
