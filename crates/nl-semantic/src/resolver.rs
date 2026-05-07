use nl_ast::*;
use std::collections::HashMap;

pub fn resolve(program: &Program) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for func in &program.functions {
        let mut symbols = HashMap::new();
        for (param, _) in &func.params {
            symbols.insert(param.clone(), Type::Unknown);
        }
        check_block(&func.body, &mut symbols, &mut errors);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_block(stmts: &[Statement], symbols: &mut HashMap<String, Type>, errors: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Statement::Let { name, init } => {
                let _ = check_expr(init, symbols, errors);
                symbols.insert(name.clone(), Type::Unknown);
            }
            Statement::Assign { name, value } => {
                if !symbols.contains_key(name) {
                    errors.push(format!("Pemboleh ubah '{}' belum diisytihar", name));
                }
                let _ = check_expr(value, symbols, errors);
            }
            Statement::Expression(expr) => {
                let _ = check_expr(expr, symbols, errors);
            }
            Statement::If { condition, then_branch, else_branch } => {
                let _ = check_expr(condition, symbols, errors);
                check_block(then_branch, symbols, errors);
                if let Some(else_block) = else_branch {
                    check_block(else_block, symbols, errors);
                }
            }
            Statement::For { var, start, end, body } => {
                let _ = check_expr(start, symbols, errors);
                let _ = check_expr(end, symbols, errors);
                let old = symbols.insert(var.clone(), Type::Int);
                check_block(body, symbols, errors);
                if let Some(old) = old {
                    symbols.insert(var.clone(), old);
                } else {
                    symbols.remove(var);
                }
            }
            Statement::While { condition, body } => {
                let _ = check_expr(condition, symbols, errors);
                check_block(body, symbols, errors);
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let _ = check_expr(e, symbols, errors);
                }
            }
        }
    }
}

fn check_expr(expr: &Expression, symbols: &HashMap<String, Type>, errors: &mut Vec<String>) -> Type {
    match expr {
        Expression::Ident(name) => {
            if symbols.contains_key(name) {
                symbols[name].clone()
            } else {
                errors.push(format!("Pengenal '{}' tidak dikenal", name));
                Type::Unknown
            }
        }
        Expression::IntLiteral(_) => Type::Int,
        Expression::StringLiteral(_) => Type::String,
        Expression::Binary { left, right, .. } => {
            let _ = check_expr(left, symbols, errors);
            let _ = check_expr(right, symbols, errors);
            Type::Unknown
        }
        Expression::Unary { expr: inner, .. } => {
            check_expr(inner, symbols, errors)
        }
        Expression::Call { func, args } => {
            if !symbols.contains_key(func) && func != "println" {
                errors.push(format!("Fungsi '{}' tidak dikenal", func));
            }
            for arg in args {
                let _ = check_expr(arg, symbols, errors);
            }
            Type::Void
        }
    }
}
