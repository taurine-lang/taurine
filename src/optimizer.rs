use crate::ast::{Expr, Stmt, Program};
use crate::lexer::TokenKind;
use std::collections::HashMap;

pub struct Optimizer {
    constants: HashMap<usize, Expr>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            constants: HashMap::new(),
        }
    }

    pub fn optimize(&mut self, program: Program) -> Program {
        let mut optimized_stmts = Vec::new();
        for stmt in program.statements {
            if let Some(opt_stmt) = self.optimize_stmt(stmt) {
                optimized_stmts.push(opt_stmt);
            }
        }
        Program { statements: optimized_stmts }
    }

    fn optimize_stmt(&mut self, stmt: Stmt) -> Option<Stmt> {
        match stmt {
            Stmt::Declaration { name, initializer, line, is_const } => {
                // Constant folding
                if let Some(ref init) = initializer {
                    if self.is_constant_expr(init) {
                        self.constants.insert(name.id(), init.clone());
                    }
                }
                Some(Stmt::Declaration { name, initializer, line, is_const })
            }
            Stmt::Expression(expr) => {
                Some(Stmt::Expression(self.optimize_expr(expr)))
            }
            Stmt::If { condition, then_branch, else_branch, line } => {
                // Dead code elimination
                if let Expr::LiteralTrue = condition {
                   // Always true, remove else
                    Some(Stmt::Block(then_branch))
                } else if let Expr::LiteralFalse = condition {
                    // Always false, we leave only else
                    else_branch.map(Stmt::Block)
                } else {
                    Some(Stmt::If {
                        condition: self.optimize_expr(condition),
                        then_branch: then_branch.into_iter()
                            .filter_map(|s| self.optimize_stmt(s))
                            .collect(),
                        else_branch: else_branch.map(|b| b.into_iter()
                            .filter_map(|s| self.optimize_stmt(s))
                            .collect()),
                        line,
                    })
                }
            }
            Stmt::While { condition, body, line } => {
                if let Expr::LiteralFalse = condition {
                    None
                } else {
                    Some(Stmt::While {
                        condition: self.optimize_expr(condition),
                        body: body.into_iter()
                            .filter_map(|s| self.optimize_stmt(s))
                            .collect(),
                        line,
                    })
                }
            }
            Stmt::For { initializer, condition, increment, body, line } => {
                Some(Stmt::For {
                    initializer: initializer.map(|i| Box::new(self.optimize_expr(*i))),
                    condition: Box::new(self.optimize_expr(*condition)),
                    increment: increment.map(|i| Box::new(self.optimize_expr(*i))),
                    body: body.into_iter()
                        .filter_map(|s| self.optimize_stmt(s))
                        .collect(),
                    line,
                })
            }
            Stmt::ForIn { variable, iterable, body, line } => {
                Some(Stmt::ForIn {
                    variable,
                    iterable: self.optimize_expr(iterable),
                    body: body.into_iter()
                        .filter_map(|s| self.optimize_stmt(s))
                        .collect(),
                    line,
                })
            }
            Stmt::Function { name, params, body, line } => {
                Some(Stmt::Function {
                    name,
                    params,
                    body: body.into_iter()
                        .filter_map(|s| self.optimize_stmt(s))
                        .collect(),
                    line,
                })
            }
            Stmt::Return(expr) => {
                Some(Stmt::Return(expr.map(|e| self.optimize_expr(e))))
            }
            Stmt::ReturnMulti(values) => {
                Some(Stmt::ReturnMulti(values.into_iter().map(|e| self.optimize_expr(e)).collect()))
            }
            Stmt::Block(stmts) => {
                let optimized: Vec<Stmt> = stmts.into_iter()
                    .filter_map(|s| self.optimize_stmt(s))
                    .collect();
                if optimized.is_empty() {
                    None
                } else if optimized.len() == 1 {
                    Some(optimized.into_iter().next().unwrap())
                } else {
                    Some(Stmt::Block(optimized))
                }
            }
            Stmt::Destructure { names, initializer, line } => {
                Some(Stmt::Destructure {
                    names,
                    initializer: self.optimize_expr(initializer),
                    line,
                })
            }
            Stmt::Assignment { name, value, line, is_const_assign } => {
                // Updating the cache when assigning
                self.constants.remove(&name.id());
                Some(Stmt::Assignment {
                    name,
                    value: self.optimize_expr(value),
                    line,
                    is_const_assign,
                })
            }
            Stmt::Import { path, alias, line } => {
                Some(Stmt::Import { path, alias, line })
            }
            Stmt::Try { body, catch_var, catch_body, line } => {
                Some(Stmt::Try {
                    body: body.into_iter()
                        .filter_map(|s| self.optimize_stmt(s))
                        .collect(),
                    catch_var,
                    catch_body: catch_body.into_iter()
                        .filter_map(|s| self.optimize_stmt(s))
                        .collect(),
                    line,
                })
            }
            Stmt::Break => Some(Stmt::Break),
            Stmt::Continue => Some(Stmt::Continue),
            _ => Some(stmt),
        }
    }

    fn optimize_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { left, op, right, line } => {
                let left_opt = self.optimize_expr(*left);
                let right_opt = self.optimize_expr(*right);
                
                // Constant folding
                if let (Expr::Number(l), Expr::Number(r)) = (&left_opt, &right_opt) {
                    return match op {
                        TokenKind::Plus => Expr::Number(l + r),
                        TokenKind::Minus => Expr::Number(l - r),
                        TokenKind::Star => Expr::Number(l * r),
                        TokenKind::Slash => Expr::Number(l / r),
                        _ => Expr::Binary { left: Box::new(left_opt), op, right: Box::new(right_opt), line }
                    };
                }
                
                Expr::Binary { left: Box::new(left_opt), op, right: Box::new(right_opt), line }
            }
            Expr::Unary { op, expr, line } => {
                let expr_opt = self.optimize_expr(*expr);
                
                // Constant unar folding
                if let Expr::Number(n) = &expr_opt {
                    if let TokenKind::Minus = op {
                        return Expr::Number(-n);
                    }
                }
                
                Expr::Unary { op, expr: Box::new(expr_opt), line }
            }
            Expr::Identifier(name) => {
                // Подставляем константы
                if let Some(const_expr) = self.constants.get(&name.id()) {
                    return const_expr.clone();
                }
                Expr::Identifier(name)
            }
            Expr::Call { callee, arguments, line } => {
                Expr::Call {
                    callee,
                    arguments: arguments.into_iter()
                        .map(|a| self.optimize_expr(a))
                        .collect(),
                    line,
                }
            }
            Expr::Table { entries, line } => {
                Expr::Table {
                    entries: entries.into_iter()
                        .map(|(k, v)| (k, self.optimize_expr(v)))
                        .collect(),
                    line,
                }
            }
            Expr::Array { items, line } => {
                Expr::Array {
                    items: items.into_iter()
                        .map(|i| self.optimize_expr(i))
                        .collect(),
                    line,
                }
            }
            _ => expr,
        }
    }

    fn is_constant_expr(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::LiteralTrue | 
            Expr::LiteralFalse | Expr::LiteralNil => true,
            Expr::Unary { expr, .. } => self.is_constant_expr(expr),
            Expr::Binary { left, right, .. } => {
                self.is_constant_expr(left) && self.is_constant_expr(right)
            }
            _ => false,
        }
    }
}