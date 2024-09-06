use std::path::PathBuf;
use crate::parsing::ast::astree::ASTreeNode;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::{Groups, LocalEnv, TypeEnv};
use crate::r#static::type_check::statement::xs_tc_stmt;
use crate::r#static::xs_error::XSError;

pub fn xs_tc(
    path: &PathBuf,
    stmts: &Vec<Spanned<ASTreeNode>>,
    local_env: &mut Option<TypeEnv>,
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut Groups,
    errs: &mut Vec<XSError>,
) {
    for stmt in stmts {
        xs_tc_stmt(
            path, stmt, local_env, type_env, local_envs, groups, errs,
            true, false, false
        );
    }
}
