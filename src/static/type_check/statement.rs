use std::collections::{HashMap, HashSet};
use chumsky::container::{Container};
use crate::parsing::ast::astree::{ASTreeNode, RuleOpt};
use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::literal::Literal;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::{Span, Spanned};
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::type_check::{Groups, TypeEnv};
use crate::r#static::type_check::util::{chk_rule_opt, type_cmp};
use crate::r#static::xs_error::{name_err, syntax_err, type_err, warn, XSError};

pub fn xs_tc_stmt<'src>(
    (stmt, span): &'src Spanned<ASTreeNode>,
    type_env: &'src mut TypeEnv,
    groups: &'src mut Groups,
    errs: &mut Vec<XSError>,
    is_top_level: bool,
    is_breakable: bool,
    is_continuable: bool,
) { match stmt {
    // an include statement is always parsed with a string literal
    ASTreeNode::Include(_) => {
        if !is_top_level {
            errs.push(syntax_err(
                "An include statement may only appear at the top of the module", span
            ))
        }
    }
    ASTreeNode::VarDef {
        is_extern,
        is_static: _is_static,
        is_const,
        type_,
        name: spanned_name,
        value
    } => {
        let (name, name_span) = spanned_name;
        match type_env.get(name) {
            Some(_) => {
                errs.push(name_err(
                    "Variable name is already in use", name_span
                ))
            }
            None => {
                type_env.push((name.clone(), type_.clone()));
            }
        };

        if !is_top_level && *is_extern {
            errs.push(syntax_err(
                "Local variables cannot be declared as `extern`", name_span
            ))
        }
        let Some(spanned_expr) = value else {
            if *is_const {
                errs.push(syntax_err(
                    "Variable declared as `const` must be initialised with a value", name_span
                ))
            }

            return;
        };

        let (expr, expr_span) = spanned_expr;

        if is_top_level || *is_const {
            match expr {
                Expr::Literal(Literal::Str(_)) if is_top_level => {
                    errs.push(warn(
                        "Top level string initializers do not work correctly. yES",
                        expr_span
                    ));
                }
                Expr::Literal(_) => {}
                _ => {
                    errs.push(syntax_err(
                        "Top level or `const` variable initializers must be literals",
                        expr_span
                    ));
                }
            }
        }

        let Some(init_type) = xs_tc_expr(spanned_expr, type_env, errs) else {
            return;
        };

        type_cmp(type_, init_type, expr_span, errs, false, false);

    }
    ASTreeNode::VarAssign {
        name: spanned_name,
        value: spanned_expr
    } => {
        if is_top_level {
            errs.push(syntax_err(
                "Assignments are not allowed at the top level", span
            ));
        }

        let (name, name_span) = spanned_name;

        let Some(type_) = type_env.get(name) else {
            errs.push(name_err(
                "Undefined variable", name_span
            ));
            return;
        };

        let Some(init_type) = xs_tc_expr(spanned_expr, type_env, errs) else {
            // An invalid expr will generate its own error
            return;
        };

        type_cmp(type_, init_type, &spanned_expr.1, errs, false, false);
    },
    ASTreeNode::RuleDef {
        name: (name, name_span),
        rule_opts, // todo check for dups, add grp names
        body: (body, _body_span)
    } => {
        if !is_top_level {
            errs.push(syntax_err(
                "Rule definitions are only allowed at the top level", name_span
            ))
        }

        let mut opt_spans: HashMap<&str, &Span> = HashMap::with_capacity(rule_opts.len());
        
        for (opt, opt_span) in rule_opts {
            match opt {
                RuleOpt::Active | RuleOpt::Inactive => {
                    chk_rule_opt("activity", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::RunImmediately => {
                    chk_rule_opt("run immediately", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::HighFrequency => {
                    chk_rule_opt("min interval", opt_span, &mut opt_spans, errs);
                    chk_rule_opt("max interval", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::MinInterval(_) => {
                    chk_rule_opt("min interval", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::MaxInterval(_) => {
                    chk_rule_opt("max interval", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::Priority(_) => {
                    chk_rule_opt("priority", opt_span, &mut opt_spans, errs);
                }
                RuleOpt::Group((grp, _grp_span)) => {
                    if chk_rule_opt("group", opt_span, &mut opt_spans, errs) {
                        groups.push(grp.clone());
                    }
                }
            }
        }
        
        match type_env.get(name) {
            Some(_) => {
                errs.push(name_err(
                    "Variable name is already in use", name_span
                ))
            }
            None => {
                type_env.push((name.clone(), Type::Rule));
            }
        };

        let mut local_type_env = type_env.clone();
        local_type_env.push((Identifier::new("return"), Type::Void));
        for spanned_stmt in body.0.iter() {
            xs_tc_stmt(spanned_stmt, &mut local_type_env, groups, errs, false, is_breakable, is_continuable);
        }
    }
    ASTreeNode::FnDef {
        is_mutable,
        return_type,
        name: (name, name_span),
        params,
        body: (body, _body_span)
    } => {
        if !is_top_level {
            errs.push(syntax_err(
                "Function definitions are only allowed at the top level", name_span
            ))
        }
        
        let mut local_type_env = HashMap::with_capacity(params.len());
        for param in params {
            let (param_name, param_name_span) = &param.name;
            if let (Some(_), _) | (_, Some(_)) = (
                type_env.get(param_name), local_type_env.get(param_name)
            ) {
                errs.push(name_err(
                    "Variable name is already in use", param_name_span
                ))
            }
            local_type_env.push((param_name.clone(), param.type_.clone()));

            let (expr, expr_span) = &param.default;
            if let Expr::Literal(_) = expr {} else {
                errs.push(syntax_err(
                    "Parameter defaults must be literals",
                    expr_span,
                ));
            };

            // expr will generate its own error when it returns None
            let Some(param_default_value_type) = xs_tc_expr(&param.default, type_env, errs)
                else { continue; };
            type_cmp(
                &param.type_,
                param_default_value_type,
                expr_span,
                errs,
                false,
                false
            );
        }

        let mut new_type_sign = params
            .iter()
            .map(|param| param.type_.clone())
            .collect::<Vec<Type>>();
        new_type_sign.push(return_type.clone());
        
        match type_env.get(name) {
            Some(Type::Func {
                is_mutable: was_mutable,
                type_sign 
            }) => if !was_mutable {
                errs.push(name_err(
                    "This function is not mutable and cannot be redefined", name_span,
                ))
            } else if new_type_sign != *type_sign {
                errs.push(syntax_err(
                    "Type signature of mutable functions must be consistent", name_span,
                ))
            } else {
                type_env.push((
                    name.clone(),
                    Type::Func { is_mutable: is_mutable.clone(), type_sign: new_type_sign }
                ));
            },
            Some(_) => errs.push(name_err(
                "Variable name is already in use", name_span
            )),
            _ => {
                type_env.push((
                    name.clone(),
                    Type::Func { is_mutable: is_mutable.clone(), type_sign: new_type_sign }
                ));
            }
        }

        local_type_env.extend(type_env.clone());
        local_type_env.push((Identifier::new("return"), return_type.clone()));

        // todo: figure out how to check returns on all fn paths
        for spanned_stmt in body.0.iter() {
            xs_tc_stmt(spanned_stmt, &mut local_type_env, groups, errs, false, is_breakable, is_continuable);
        }
    },
    ASTreeNode::Return(spanned_expr) => {
        let Some(return_type) = type_env.get(&Identifier::new("return")) else {
            errs.push(syntax_err(
                "`return` statement is not allowed here",
                span
            ));
            return;
        };

        let Some(spanned_expr) = spanned_expr else {
            if *return_type != Type::Void {
                errs.push(syntax_err(
                    &format!("This function must return a value of type `{:}`", return_type),
                    span,
                ));
            }
            return;
        };
        if *return_type == Type::Void {
            errs.push(syntax_err("This function cannot return a value", span));
            return;
        }

        let (expr, expr_span) = spanned_expr;
        if let Expr::Paren(_) = expr {} else {
            errs.push(syntax_err(
                "`return` statement expressions must be enclosed in parenthesis. yES",
                expr_span,
            ));
        };

        // if expr returns None, it'll generate its own error
        let Some(return_expr_type) = xs_tc_expr(spanned_expr, type_env, errs) else {
            return;
        };

        type_cmp(return_type, return_expr_type, expr_span, errs, false, false);
    },
    ASTreeNode::IfElse {
        condition,
        consequent,
        alternate
    } => {
        if is_top_level {
            errs.push(syntax_err(
                "`if` statements are only allowed inside a local scope", span
            ))
        }
        
        if let Some(type_) = xs_tc_expr(condition, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(type_err(
                    "`Conditional expression must be a boolean value",
                    &condition.1,
                ));
            }
        }

        for spanned_stmt in consequent.0.0.iter() {
            xs_tc_stmt(spanned_stmt, type_env, groups, errs, false, is_breakable, is_continuable);
        }

        if let Some(alternate) = alternate {
            for spanned_stmt in alternate.0.0.iter() {
                xs_tc_stmt(spanned_stmt, type_env, groups, errs, false, is_breakable, is_continuable);
            }
        }
    },
    ASTreeNode::While { condition, body } => {
        if is_top_level {
            errs.push(syntax_err(
                "`while` statements are only allowed inside a local scope", span
            ))
        }
        
        if let Some(type_) = xs_tc_expr(condition, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(type_err(
                    "Conditional expression must be a boolean value",
                    &condition.1,
                ));
            }
        }

        for spanned_stmt in body.0.0.iter() {
            xs_tc_stmt(spanned_stmt, type_env, groups, errs, false, true, true);
        }
    },
    ASTreeNode::For { var, condition, body } => {
        if is_top_level {
            errs.push(syntax_err(
                "`for` statements are only allowed inside a local scope", span
            ))
        }
        
        let (ASTreeNode::VarAssign { name: (name, name_span), value }, _span) = var.as_ref()
            else { return; }; // unreachable
        
        let None = type_env.get(name) else {
            errs.push(name_err(
                "Variable name already in use",
                name_span,
            ));
            return;
        };
        
        if let Some(value_type) = xs_tc_expr(value, type_env, errs) {
            type_cmp(&Type::Int, value_type, &value.1, errs, false, false);
        }
        
        type_env.push((name.clone(), Type::Int));
        if let Some(type_) = xs_tc_expr(condition, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(type_err(
                    "Conditional expression must be a boolean value",
                    &condition.1,
                ));
            }
        }

        for spanned_stmt in body.0.0.iter() {
            xs_tc_stmt(spanned_stmt, type_env, groups, errs, false, true, true);
        }
    },
    ASTreeNode::Switch { clause, cases } => {
        if is_top_level {
            errs.push(syntax_err(
                "`switch` statements are only allowed inside a local scope", span
            ))
        }
        
        // expression generates its own error for a None return
        if let Some(clause_type) = xs_tc_expr(clause, type_env, errs) {
            type_cmp(&Type::Int, clause_type, &clause.1, errs, false, false);
        }

        let mut default_span: Option<&Span> = None;
        let mut case_spans: HashMap<&Expr, &Span> = HashMap::with_capacity(cases.len());

        for (case_clause, (body, body_span)) in cases {
            // expression generates its own error for a None return
            for spanned_stmt in body.0.iter() {
                xs_tc_stmt(spanned_stmt, type_env, groups, errs, false, true, is_continuable);
            }
            let Some(spanned_case_expr) = case_clause else {
                let Some(og_span) = default_span else {
                    default_span = Some(body_span);
                    continue;
                };
                errs.push(warn(
                    "Only the first default block will run when case matching fails",
                    og_span,
                ));
                errs.push(warn(
                    "Only the first default block will run when case matching fails",
                    body_span,
                ));
                continue;
            };
            let (case_expr, case_expr_span) = spanned_case_expr;
            if let Some(clause_type) = xs_tc_expr(spanned_case_expr, type_env, errs) {
                type_cmp(&Type::Int, clause_type, case_expr_span, errs, false, true);
            }
            if let Some(&og_span) = case_spans.get(case_expr) {
                errs.push(warn(
                    "Only the first case will run on a match",
                    og_span,
                ));
                errs.push(warn(
                    "Only the first case will run on a match",
                    &spanned_case_expr.1,
                ));
            } else {
                case_spans.push((case_expr, case_expr_span));
            }
        }
    },
    ASTreeNode::PostDPlus((id, id_span)) => {
        if is_top_level {
            errs.push(syntax_err(
                "`postfix` statements are only allowed inside a local scope", span
            ))
        }
        
        let Some(id_type) = type_env.get(id) else {
            errs.push(name_err(&format!("Undefined name `{:}`", id.0), id_span));
            return;
        };

        if let Type::Int | Type::Float = id_type {
            return;
        }
        errs.push(syntax_err(
            &format!("Postfix increment is only allowed on `int | float` values {:}", id.0),
            span
        ));
    },
    ASTreeNode::PostDMinus((id, id_span)) => {
        if is_top_level {
            errs.push(syntax_err(
                "`postfix` statements are only allowed inside a local scope", span
            ))
        }
        
        let Some(id_type) = type_env.get(id) else {
            errs.push(name_err(&format!("Undefined name `{:}`", id.0), id_span));
            return;
        };

        if let Type::Int | Type::Float = id_type {
            return;
        }
        errs.push(syntax_err(
            &format!("Postfix decrement is only allowed on `int | float` values {:}", id.0),
            span
        ));
    },
    ASTreeNode::Break => {
        if !is_breakable {
            errs.push(syntax_err(
                "`break` statements are only allowed inside a loop or a case block", span
            ))
        }
    },
    ASTreeNode::Continue => {
        if !is_continuable {
            errs.push(syntax_err(
                "`continue` statements are only allowed inside a loop", span
            ))
        }
    },
    ASTreeNode::LabelDef((id, id_span)) => {
        if is_top_level {
            errs.push(syntax_err(
                "`label` definitions are only allowed inside a local scope", span
            ))
        }
        let None = type_env.get(id) else {
            errs.push(name_err("Variable name already in use", id_span));
            return;
        };
        type_env.push((id.clone(), Type::Label));
    },
    ASTreeNode::Goto((id, id_span)) => {
        if is_top_level {
            errs.push(syntax_err(
                "`goto` statements are only allowed inside a local scope", span
            ))
        }
        let Some(id_type) = type_env.get(id) else {
            errs.push(name_err(&format!("Undefined name `{:}`", id.0), id_span));
            return;
        };

        type_cmp(&Type::Label, id_type, id_span, errs, false, false);
    },
    ASTreeNode::Discarded(spanned_expr) => {
        if is_top_level {
            errs.push(syntax_err(
                "Discarded expressions are only allowed inside a local scope", span
            ))
        }
        
        let (expr, expr_span) = spanned_expr;
        let Expr::FnCall { .. } = expr else {
            errs.push(syntax_err("Only function calls may be discarded", expr_span));
            return;
        };

        let Some(return_value_type) = xs_tc_expr(spanned_expr, type_env, errs)
            else { return; }; // unreachable

        if let Type::Void = return_value_type {
            return;
        }

        errs.push(warn("The return value of this function call is being ignored", expr_span));
    },
    ASTreeNode::Debug((id, id_span)) => {
        if is_top_level {
            errs.push(syntax_err(
                "`dbg` statements are only allowed inside a local scope", span
            ))
        }
        let Some(id_type) = type_env.get(id) else {
            errs.push(name_err(&format!("Undefined name `{:}`", id.0), id_span));
            return;
        };
        
        let (Type::Func { .. } | Type::Rule | Type::Class | Type::Label) = id_type else {
            return;
        };

        errs.push(syntax_err(
            "Invalid variable name for `dbg`", id_span
        ))
    },
    ASTreeNode::Breakpoint => {
        if is_top_level {
            errs.push(syntax_err(
                "Breakpoints are only allowed inside a local scope", span
            ))
        }
        
        errs.push(warn(
            "Breakpoints cause XS execution to pause irrecoverably", span
        ));
    },
    ASTreeNode::Class { name: (id, id_span), member_vars } => {
        if !is_top_level {
            errs.push(syntax_err(
                "`class` definitions are only allowed at the top level", span
            ))
        }
        if let Some(_) = type_env.get(id) {
            errs.push(name_err("Variable name already in use", id_span));
        } else {
            type_env.push((id.clone(), Type::Class));
        }

        let mut mem_name: HashSet<&Identifier> = HashSet::with_capacity(member_vars.len());
        for (member_var, _var_span) in member_vars {
            let ASTreeNode::VarDef {
                type_,
                name: (id, id_span),
                value,
                is_extern,
                is_const,
                is_static
            } = member_var
                else { continue; }; // unreachable
            
            if *is_extern {
                errs.push(syntax_err("Member variables cannot be declared as `extern`", id_span));
            }
            if *is_const {
                errs.push(syntax_err("Member variables cannot be declared as `const`", id_span));
            }
            if *is_static {
                errs.push(syntax_err("Member variables cannot be declared as `static`", id_span));
            }
            
            if mem_name.contains(id) {
                errs.push(name_err("Variable name already in use", id_span));
            } else {
                mem_name.push(id);
            }
            let Some(init_value) = value
                else { continue; }; // unreachable
            let (_init_value_expr, init_value_span) = init_value;
            
            let Some(init_value_type) = xs_tc_expr(init_value, type_env, errs) else { continue; };
            type_cmp(type_, init_value_type, init_value_span, errs, false, false);
        }
        
        errs.push(warn(
            "Classes are unusable in XS", span
        ))
    },
}}
