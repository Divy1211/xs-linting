use std::collections::{HashMap};
use std::path::PathBuf;

use chumsky::container::Container;

use crate::lint::gen_info::gen_info_from_path;
use crate::parsing::ast::astree::{ASTreeNode, RuleOpt};
use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::literal::Literal;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::{Span, Spanned};
use crate::r#static::type_check::{env_get, env_set, Groups, LocalEnv, TypeEnv};
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::type_check::util::{chk_rule_opt, type_cmp};
use crate::r#static::xs_error::{XSError};

pub fn xs_tc_stmt(
    path: &PathBuf,
    (stmt, span): &Spanned<ASTreeNode>,
    local_env: &mut Option<TypeEnv>,
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut Groups,
    errs: &mut Vec<XSError>,
    is_top_level: bool,
    is_breakable: bool,
    is_continuable: bool,
) { match stmt {
    // an include statement is always parsed with a string literal
    ASTreeNode::Include((filename, _span)) => {
        if !is_top_level {
            errs.push(XSError::syntax(
                span,
                "An {0} statement is only allowed at the top level",
                vec!["include"],
            ));
            return;
        }
        let mut inc_path = path.clone();
        inc_path.pop();
        inc_path.push(&filename[1..(filename.len()-1)]);
        gen_info_from_path(
            type_env,
            local_envs,
            groups,
            inc_path,
        );
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
        match env_get(local_env, type_env, name) {
            Some((_, og_span)) => {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    None,
                ))
            }
            None => {
                env_set(local_env, type_env, name, (type_.clone(), name_span.clone()));
            }
        };

        if !is_top_level && *is_extern {
            errs.push(XSError::syntax(
                name_span,
                "Local variables cannot be declared as {0}",
                vec!["extern"],
            ));
        }
        let Some(spanned_expr) = value else {
            if *is_const {
                errs.push(XSError::syntax(
                    name_span,
                    "Variable declared as {0} must be initialised with a value",
                    vec!["const"],
                ));
            }

            return;
        };

        let (expr, expr_span) = spanned_expr;

        if is_top_level || *is_const {
            match expr {
                Expr::Literal(Literal::Str(_)) if is_top_level => {
                    errs.push(XSError::warning(
                        expr_span,
                        "Top level initialized {0} do not work correctly. yES",
                        vec!["string"],
                    ));
                }
                Expr::Literal(_) | Expr::Neg(_) | Expr::Vec { .. } => { }
                _ => {
                    errs.push(XSError::syntax(
                        expr_span,
                        "Top level or {0} variable initializers must be literals",
                        vec!["const"],
                    ));
                }
            }
        }

        let Some(init_type) = xs_tc_expr(spanned_expr, local_env, type_env, errs) else {
            return;
        };

        type_cmp(type_, init_type, expr_span, errs, false, false);

    }
    ASTreeNode::VarAssign {
        name: spanned_name,
        value: spanned_expr
    } => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "Assignments are only allowed in a local scope",
                vec![],
            ));
        }

        let (name, name_span) = spanned_name;

        let Some((type_, _span)) = env_get(local_env, type_env, name) else {
            errs.push(XSError::undefined_name(
                &name.0,
                name_span,
            ));
            return;
        };

        let Some(init_type) = xs_tc_expr(spanned_expr, local_env, type_env, errs) else {
            // An invalid expr will generate its own error
            return;
        };

        type_cmp(type_, init_type, &spanned_expr.1, errs, false, false);
    },
    ASTreeNode::RuleDef {
        name: (name, name_span),
        rule_opts, // todo check for dups, add grp names
        body: (body, body_span)
    } => {
        if !is_top_level {
            errs.push(XSError::syntax(
                name_span,
                "A rule definition is only allowed at the top level",
                vec![],
            ));
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
        
        match env_get(local_env, type_env, name) {
            Some((_, og_span)) => {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    None,
                ))
            }
            None => {
                env_set(local_env, type_env, name, (Type::Rule, name_span.clone()));
            }
        };

        let mut local_type_env = Some(HashMap::from([
            (Identifier::new("return"), (Type::Void, Span::new(0, 0)))
        ]));
        for spanned_stmt in body.0.iter() {
            xs_tc_stmt(
                path, spanned_stmt, &mut local_type_env, type_env, local_envs, groups, errs,
                false, is_breakable, is_continuable
            );
        }
        local_envs
            .entry(name.clone()).or_insert(vec![])
            .push((local_type_env.unwrap(), body_span.clone()))
    }
    ASTreeNode::FnDef {
        is_mutable,
        return_type,
        name: (name, name_span),
        params,
        body: (body, body_span)
    } => {
        if !is_top_level {
            errs.push(XSError::syntax(
                name_span,
                "A function definition is only allowed at the top level",
                vec![],
            ));
        }
        
        let mut local_type_env = Some(HashMap::with_capacity(params.len()));
        for param in params {
            let (param_name, param_name_span) = &param.name;
            if let Some((_, og_span)) = env_get(&local_type_env, type_env, param_name) {
                errs.push(XSError::redefined_name(
                    &name.0,
                    param_name_span,
                    og_span,
                    None,
                ))
            }
            env_set(&mut local_type_env, type_env, param_name, (param.type_.clone(), param_name_span.clone()));

            let (expr, expr_span) = &param.default;
            
            match expr {
                Expr::Literal(_)  | Expr::Neg(_) | Expr::Vec { .. } => { }
                _ => {
                    errs.push(XSError::syntax(
                        expr_span,
                        "Parameter defaults must be literals",
                        vec![],
                    ));
                }
            }
            
            // expr will generate its own error when it returns None
            let Some(param_default_value_type) = xs_tc_expr(&param.default, local_env, type_env, errs)
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
        
        // Nested fns are not allowed. If someone has accidentally defined a nested fn, pretend it
        // exists in the global space, an error was already issued for this above.
        match type_env.get(name) {
            Some((Type::Func {
                is_mutable: was_mutable,
                type_sign 
            }, og_span)) => if !was_mutable {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    Some("Only mutable functions may be overridden"),
                ))
            } else if new_type_sign != *type_sign {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    Some("Type signatures of mutable functions must be the same"),
                ))
            } else {
                type_env.push((
                    name.clone(),
                    (Type::Func { is_mutable: is_mutable.clone(), type_sign: new_type_sign }, name_span.clone())
                ));
            },
            Some((_, og_span)) => {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    None,
                ))
            },
            _ => {
                type_env.push((
                    name.clone(),
                    (Type::Func { is_mutable: is_mutable.clone(), type_sign: new_type_sign }, name_span.clone())
                ));
            }
        }

        local_type_env.as_mut().unwrap().push((
            Identifier::new("return"), (return_type.clone(), Span::new(0, 0))
        ));
        // todo: figure out how to check returns on all fn paths
        for spanned_stmt in body.0.iter() {
            xs_tc_stmt(
                path, spanned_stmt, &mut local_type_env, type_env, local_envs, groups, errs,
                false, is_breakable, is_continuable
            );
        }
        local_envs
            .entry(name.clone()).or_insert(vec![])
            .push((local_type_env.unwrap(), body_span.clone()))
    },
    ASTreeNode::Return(spanned_expr) => {
        let Some((return_type, _span)) = env_get(local_env, type_env, &Identifier::new("return")) else {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["return"],
            ));
            return;
        };

        let Some(spanned_expr) = spanned_expr else {
            if *return_type != Type::Void {
                errs.push(XSError::type_mismatch(
                    "void",
                    &return_type.to_string(),
                    span,
                    Some(&format!("This function's return type was declared as '{}'", return_type)),
                ));
            }
            return;
        };
        if *return_type == Type::Void {
            errs.push(XSError::syntax(
                span,
                "This function's return type was declared as {0}",
                vec!["void"]
            ));
            return;
        }

        let (expr, expr_span) = spanned_expr;
        if let Expr::Paren(_) = expr {} else {
            errs.push(XSError::syntax(
                expr_span,
                "A {0} statement's expression must be enclosed in parenthesis. yES",
                vec!["return"]
            ));
        };

        // if expr returns None, it'll generate its own error
        let Some(return_expr_type) = xs_tc_expr(spanned_expr, local_env, type_env, errs) else {
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
            errs.push(XSError::syntax(
                span,
                "An {0} statement is only allowed in a local scope",
                vec!["if"]
            ));
        }
        
        if let Some(type_) = xs_tc_expr(condition, local_env, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }

        for spanned_stmt in consequent.0.0.iter() {
            xs_tc_stmt(
                path, spanned_stmt, local_env, type_env, local_envs, groups, errs,
                false, is_breakable, is_continuable
            );
        }

        if let Some(alternate) = alternate {
            for spanned_stmt in alternate.0.0.iter() {
                xs_tc_stmt(
                    path, spanned_stmt, local_env, type_env, local_envs, groups, errs,
                    false, is_breakable, is_continuable
                );
            }
        }
    },
    ASTreeNode::While { condition, body } => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["while"]
            ));
        }
        
        if let Some(type_) = xs_tc_expr(condition, local_env, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }

        for spanned_stmt in body.0.0.iter() {
            xs_tc_stmt(
                path, spanned_stmt, local_env, type_env, local_envs, groups, errs,
                false, true, true
            );
        }
    },
    ASTreeNode::For { var, condition, body } => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["for"]
            ));
        }
        
        let (ASTreeNode::VarAssign { name: (name, name_span), value }, _span) = var.as_ref()
            else { return; }; // unreachable
        
        match env_get(local_env, type_env, name) {
            Some((_, og_span)) => {
                errs.push(XSError::redefined_name(
                    &name.0,
                    name_span,
                    og_span,
                    None,
                ));
                return;
            }
            _ => {}
        };
        
        if let Some(value_type) = xs_tc_expr(value, local_env, type_env, errs) {
            type_cmp(&Type::Int, value_type, &value.1, errs, false, false);
        }
        
        env_set(local_env, type_env, name, (Type::Int, name_span.clone()));
        if let Some(type_) = xs_tc_expr(condition, local_env, type_env, errs) {
            if *type_ != Type::Bool {
                errs.push(XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }

        for spanned_stmt in body.0.0.iter() {
            xs_tc_stmt(
                path, spanned_stmt, local_env, type_env, local_envs, groups, errs,
                false, true, true
            );
        }
    },
    ASTreeNode::Switch { clause, cases } => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["switch"]
            ));
        }
        
        // expression generates its own error for a None return
        if let Some(clause_type) = xs_tc_expr(clause, local_env, type_env, errs) {
            type_cmp(&Type::Int, clause_type, &clause.1, errs, false, false);
        }

        let mut default_span: Option<&Span> = None;
        let mut case_spans: HashMap<&Expr, &Span> = HashMap::with_capacity(cases.len());

        for (case_clause, (body, body_span)) in cases {
            // expression generates its own error for a None return
            for spanned_stmt in body.0.iter() {
                xs_tc_stmt(
                    path, spanned_stmt, local_env, type_env, local_envs, groups, errs,
                    false, true, is_continuable
                );
            }
            let Some(spanned_case_expr) = case_clause else {
                let Some(og_span) = default_span else {
                    default_span = Some(body_span);
                    continue;
                };
                errs.push(XSError::warning(
                    og_span,
                    "Only the first default block will run when case matching fails",
                    vec![]
                ));
                errs.push(XSError::warning(
                    body_span,
                    "Only the first default block will run when case matching fails",
                    vec![]
                ));
                continue;
            };
            let (case_expr, case_expr_span) = spanned_case_expr;
            if let Some(clause_type) = xs_tc_expr(spanned_case_expr, local_env, type_env, errs) {
                type_cmp(&Type::Int, clause_type, case_expr_span, errs, false, true);
            }
            if let Some(&og_span) = case_spans.get(case_expr) {
                errs.push(XSError::warning(
                    og_span,
                    "Only the first case will run on a match",
                    vec![]
                ));
                errs.push(XSError::warning(
                    &spanned_case_expr.1,
                    "Only the first case will run on a match",
                    vec![]
                ));
            } else {
                case_spans.push((case_expr, case_expr_span));
            }
        }
    },
    ASTreeNode::PostDPlus((id, id_span)) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A postfix increment ({0}) statement is only allowed in a local scope",
                vec!["++"]
            ));
        }
        
        let Some((id_type, _span)) = env_get(local_env, type_env, id) else {
            errs.push(XSError::undefined_name(
                &id.0,
                id_span,
            ));
            return;
        };

        if let Type::Int | Type::Float = id_type {
            return;
        }
        errs.push(XSError::syntax(
            span,
            "A postfix increment ({0}) statement is only allowed on {1} values",
            vec!["++", "int | float"]
        ));
    },
    ASTreeNode::PostDMinus((id, id_span)) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A postfix decrement ({0}) statement is only allowed in a local scope",
                vec!["--"]
            ));
        }
        
        let Some((id_type, _span)) = env_get(local_env, type_env, id) else {
            errs.push(XSError::undefined_name(
                &id.0,
                id_span,
            ));
            return;
        };

        if let Type::Int | Type::Float = id_type {
            return;
        }
        errs.push(XSError::syntax(
            span,
            "A postfix decrement ({0}) statement is only allowed on {1} values",
            vec!["--", "int | float"]
        ));
    },
    ASTreeNode::Break => {
        if !is_breakable {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside loops or switch cases",
                vec!["return"],
            ));
        }
    },
    ASTreeNode::Continue => {
        if !is_continuable {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside loops",
                vec!["continue"],
            ));
        }
    },
    ASTreeNode::LabelDef((id, id_span)) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} definition is only allowed inside a local scope",
                vec!["label"],
            ));
        }

        match env_get(local_env, type_env, id) {
            Some((_, og_span)) => {
                errs.push(XSError::redefined_name(
                    &id.0,
                    id_span,
                    og_span,
                    None,
                ));
                return;
            }
            _ => {}
        };
        env_set(local_env, type_env, id, (Type::Label, id_span.clone()));
    },
    ASTreeNode::Goto((id, id_span)) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["goto"],
            ));
        }
        let Some((id_type, _span)) = env_get(local_env, type_env, id) else {
            errs.push(XSError::undefined_name(
                &id.0,
                id_span,
            ));
            return;
        };

        type_cmp(&Type::Label, id_type, id_span, errs, false, false);
    },
    ASTreeNode::Discarded(spanned_expr) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A discarded expression is only allowed in a local scope",
                vec![]
            ));
        }
        
        let (expr, expr_span) = spanned_expr;
        let Expr::FnCall { .. } = expr else {
            errs.push(XSError::syntax(
                expr_span,
                "Only function calls can be discarded",
                vec![],
            ));
            return;
        };

        let Some(return_value_type) = xs_tc_expr(spanned_expr, local_env, type_env, errs)
            else { return; }; // unreachable

        if let Type::Void = return_value_type {
            return;
        }
        errs.push(XSError::warning(
            expr_span,
            "The return value of this function call is being ignored",
            vec![],
        ));
    },
    ASTreeNode::Debug((id, id_span)) => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["dbg"],
            ));
        }
        let Some((id_type, _span)) = env_get(local_env, type_env, id) else {
            errs.push(XSError::undefined_name(
                &id.0,
                id_span,
            ));
            return;
        };
        
        let (Type::Func { .. } | Type::Rule | Type::Class | Type::Label) = id_type else {
            return;
        };

        errs.push(XSError::syntax(
            id_span,
            "A {0} statement can only be given {1} values",
            vec!["dbg", "int | float | bool | string | vector"],
        ));
    },
    ASTreeNode::Breakpoint => {
        if is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} statement is only allowed inside a local scope",
                vec!["breakpoint"],
            ));
        }

        errs.push(XSError::warning(
            span,
            "Breakpoints cause XS execution to pause irrecoverably",
            vec![],
        ));
    },
    ASTreeNode::Class { name: (id, id_span), member_vars } => {
        if !is_top_level {
            errs.push(XSError::syntax(
                span,
                "A {0} definition is only allowed at the top level",
                vec!["class"],
            ));
        }
        if let Some((_, og_span)) = env_get(local_env, type_env, id) {
            errs.push(XSError::redefined_name(
                &id.0,
                id_span,
                og_span,
                None,
            ))
        } else {
            env_set(local_env, type_env, id, (Type::Class, id_span.clone()));
        }

        let mut mem_name: HashMap<&Identifier, &Span> = HashMap::with_capacity(member_vars.len());
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
                errs.push(XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["extern"],
                ));
            }
            if *is_const {
                errs.push(XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["const"],
                ));
            }
            if *is_static {
                errs.push(XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["static"],
                ));
            }
            
            if let Some(og_span) = mem_name.get(id) {
                errs.push(XSError::redefined_name(
                    &id.0,
                    id_span,
                    og_span,
                    None,
                ))
            } else {
                mem_name.push((id, id_span));
            }
            let Some(init_value) = value
                else { continue; }; // unreachable
            let (_init_value_expr, init_value_span) = init_value;
            
            let Some(init_value_type) = xs_tc_expr(init_value, local_env, type_env, errs) else { continue; };
            type_cmp(type_, init_value_type, init_value_span, errs, false, false);
        }

        errs.push(XSError::warning(
            span,
            "Classes are unusable in XS",
            vec![],
        ));
    },
}}
