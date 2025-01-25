use crate::parsing::ast::astree::ASTreeNode;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::statement::xs_tc_stmt;
use std::path::PathBuf;
use crate::r#static::info::type_env::TypeEnv;

pub fn xs_tc(
    path: &PathBuf,
    stmts: &Vec<Spanned<ASTreeNode>>,
    type_env: &mut TypeEnv,
) {
    for stmt in stmts {
        xs_tc_stmt(
            path, stmt, type_env,
            true, false, false,
        );
    }
}
