use crate::parsing::ast::astree::ASTreeNode;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::{Groups, TypeEnv};
use crate::r#static::type_check::statement::xs_tc_stmt;
use crate::r#static::xs_error::XSError;

pub fn xs_tc<'src>(
    stmts: &'src Vec<Spanned<ASTreeNode>>,
    type_env: &'src mut TypeEnv,
    groups: &'src mut Groups,
    errs: &mut Vec<XSError>,
) {
    for stmt in stmts {
        xs_tc_stmt(stmt, type_env, groups, errs, true, false, false);
    }
}
