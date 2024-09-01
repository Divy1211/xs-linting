use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::literal::Literal;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::{env_get, TypeEnv};
use crate::r#static::type_check::util::{arith_op, chk_int_lit, chk_num_lit, logical_op, reln_op, type_cmp};
use crate::r#static::xs_error::{name_err, syntax_err, type_err, XSError};

pub fn xs_tc_expr<'src>(
    (expr, span): &'src Spanned<Expr>,
    local_env: &'src Option<TypeEnv>,
    type_env: &'src TypeEnv,
    errs: &mut Vec<XSError>
) -> Option<&'src Type> { match expr {
    Expr::Literal(lit) => match lit {
        Literal::Int(val) => {
            errs.extend(chk_int_lit(&val, &span));
            Some(&Type::Int)
        }
        Literal::Float(_) => { Some(&Type::Float) }
        Literal::Bool(_) => { Some(&Type::Bool) }
        Literal::Str(_) => { Some(&Type::Str) }
    }
    Expr::Identifier(id) => {
        let Some((type_, _span)) = env_get(local_env.as_ref(), type_env, id) else {
            errs.push(name_err(&format!("Undefined name `{:}`", id.0), span));
            return None;
        };
        Some(type_)
    }
    Expr::Paren(expr) => { xs_tc_expr(expr, local_env, type_env, errs) }
    Expr::Vec { x, y, z } => {
        errs.extend(chk_num_lit(x, false));
        errs.extend(chk_num_lit(y, false));
        errs.extend(chk_num_lit(z, false));
        Some(&Type::Vec)
    }
    Expr::FnCall { name: (name, name_span), args } => {
        let Some((type_, _span)) = env_get(local_env.as_ref(), type_env, name) else {
            errs.push(name_err(&format!("Undefined name `{:}`", name.0), name_span));
            return None;
        };
        let Type::Func { type_sign, .. } = type_ else {
            errs.push(type_err(
                &format!(
                    "Variable '{:}' is of type `{:}` and is not callable", name.0, type_
                ), name_span
            ));
            return None;
        };
        for (param_type, arg_expr) in type_sign[..type_sign.len()-1].iter().zip(args) {
            let Some(arg_type) = xs_tc_expr(arg_expr, local_env, type_env, errs) else {
                // expr will generate its own error if the type cannot be inferred
                continue;
            };
            type_cmp(param_type, arg_type, &arg_expr.1, errs, true, false);
        }
        
        for (_expr, span) in args[type_sign.len()-1..].iter() {
            errs.push(syntax_err(
                &format!(
                    "Function '{:}' takes {:} arguments, but {:} were given",
                    name.0,
                    type_sign.len()-1,
                    args.len(),
                ), span
            ));
        }

        type_sign.last()
    }

    Expr::Neg(expr) => {
        errs.extend(chk_num_lit(expr, true));
        xs_tc_expr(expr, local_env, type_env, errs)
    }
    Expr::Not(_) => {
        errs.push(type_err("Unary not is not allowed in XS", span));
        Some(&Type::Bool)
    }
    
    Expr::Star(expr1, expr2) => {
        arith_op(span, expr1, expr2, local_env, type_env, errs, "multiply")
    }
    Expr::FSlash(expr1, expr2) => {
        arith_op(span, expr1, expr2, local_env, type_env, errs, "divide")
    }
    Expr::PCent(expr1, expr2) => {
        arith_op(span, expr1, expr2, local_env, type_env, errs, "reduce modulo")
    }
    
    Expr::Minus(expr1, expr2) => {
        arith_op(span, expr1, expr2, local_env, type_env, errs, "subtract")
    }
    Expr::Plus(expr1, expr2) => {
        arith_op(span, expr1, expr2, local_env, type_env, errs, "add")
    }
    
    Expr::Lt(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "lt")
    }
    Expr::Gt(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "gt")
    }
    Expr::Le(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "le")
    }
    Expr::Ge(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "ge")
    }
    
    Expr::Eq(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "eq")
    }
    Expr::Ne(expr1, expr2) => {
        reln_op(span, expr1, expr2, local_env, type_env, errs, "ne")
    }
    
    Expr::And(expr1, expr2) => {
        logical_op(span, expr1, expr2, local_env, type_env, errs, "and")
    }
    Expr::Or(expr1, expr2) => {
        logical_op(span, expr1, expr2, local_env, type_env, errs, "or")
    }
}}