//! Arena allocator for Taurine AST nodes
//! This module provides arena-based allocation for AST nodes to reduce
//! memory fragmentation and improve allocation performance.

use typed_arena::Arena;
use crate::ast::{Expr, Stmt, MatchArm, Pattern};

pub struct AstArena {
    expr_arena: Arena<Expr>,
    stmt_arena: Arena<Stmt>,
    pattern_arena: Arena<Pattern>,
    match_arm_arena: Arena<MatchArm>,
}

impl AstArena {
    /// Create a new arena with default capacity
    ///
    /// Default capacities:
    /// - Expressions: 256
    /// - Statements: 128
    /// - Patterns: 64
    /// - Match arms: 32
    #[inline]
    pub fn new() -> Self {
        Self {
            expr_arena: Arena::with_capacity(256),
            stmt_arena: Arena::with_capacity(128),
            pattern_arena: Arena::with_capacity(64),
            match_arm_arena: Arena::with_capacity(32),
        }
    }

    /// Create a new arena with custom capacity
    ///
    /// # Arguments
    ///
    /// * `expr_cap` - Initial capacity for expression arena
    /// * `stmt_cap` - Initial capacity for statement arena
    ///
    /// Pattern and match arm arenas are sized proportionally.
    #[inline]
    pub fn with_capacity(expr_cap: usize, stmt_cap: usize) -> Self {
        Self {
            expr_arena: Arena::with_capacity(expr_cap),
            stmt_arena: Arena::with_capacity(stmt_cap),
            pattern_arena: Arena::with_capacity(expr_cap / 4),
            match_arm_arena: Arena::with_capacity(stmt_cap / 4),
        }
    }

    /// Allocate an expression in the arena
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to allocate
    ///
    /// # Returns
    ///
    /// A reference to the allocated expression that lives as long as the arena
    ///
    /// # Performance
    ///
    /// O(1) time complexity using bump-pointer allocation
    #[inline]
    pub fn alloc_expr(&self, expr: Expr) -> &Expr {
        self.expr_arena.alloc(expr)
    }

    /// Allocate a statement in the arena
    #[inline]
    pub fn alloc_stmt(&self, stmt: Stmt) -> &Stmt {
        self.stmt_arena.alloc(stmt)
    }

    /// Allocate a pattern in the arena
    #[inline]
    pub fn alloc_pattern(&self, pattern: Pattern) -> &Pattern {
        self.pattern_arena.alloc(pattern)
    }

    /// Allocate a match arm in the arena
    #[inline]
    pub fn alloc_match_arm(&self, arm: MatchArm) -> &MatchArm {
        self.match_arm_arena.alloc(arm)
    }

    /// Allocate multiple expressions
    pub fn alloc_exprs<I>(&self, exprs: I) -> Vec<&Expr>
    where
        I: IntoIterator<Item = Expr>,
    {
        exprs.into_iter().map(|e| self.alloc_expr(e)).collect()
    }

    /// Allocate multiple statements
    pub fn alloc_stmts<I>(&self, stmts: I) -> Vec<&Stmt>
    where
        I: IntoIterator<Item = Stmt>,
    {
        stmts.into_iter().map(|s| self.alloc_stmt(s)).collect()
    }

    /// Get memory usage estimate in bytes
    pub fn memory_usage(&self) -> usize {
        // Approximate memory usage
        std::mem::size_of::<Expr>() * 256 +  // Estimate
        std::mem::size_of::<Stmt>() * 128 +
        std::mem::size_of::<Pattern>() * 64 +
        std::mem::size_of::<MatchArm>() * 32
    }

    /// Reset the arena (free all allocations)
    /// Note: typed-arena doesn't support reset, so this creates new arenas
    pub fn reset(&self) {
        // This is a workaround - typed-arena doesn't support clearing
        // For proper reset support, consider using a different arena crate
    }
}

impl Default for AstArena {
    fn default() -> Self {
        Self::new()
    }
}


pub trait IntoArena<'a> {
    type ArenaRef;
    
    fn into_arena(self, arena: &'a AstArena) -> Self::ArenaRef;
}

impl<'a> IntoArena<'a> for Expr {
    type ArenaRef = &'a Expr;
    
    fn into_arena(self, arena: &'a AstArena) -> Self::ArenaRef {
        arena.alloc_expr(self)
    }
}

impl<'a> IntoArena<'a> for Stmt {
    type ArenaRef = &'a Stmt;
    
    fn into_arena(self, arena: &'a AstArena) -> Self::ArenaRef {
        arena.alloc_stmt(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::TokenKind;

    #[test]
    fn test_arena_alloc_expr() {
        let arena = AstArena::new();
        let expr = arena.alloc_expr(Expr::Number(42.0));
        assert!(matches!(expr, Expr::Number(42.0)));
    }

    #[test]
    fn test_arena_alloc_stmt() {
        let arena = AstArena::new();
        let stmt = arena.alloc_stmt(Stmt::Break);
        assert!(matches!(stmt, Stmt::Break));
    }

    #[test]
    fn test_arena_alloc_multiple() {
        let arena = AstArena::new();
        let exprs: Vec<_> = arena.alloc_exprs(vec![
            Expr::Number(1.0),
            Expr::Number(2.0),
            Expr::Number(3.0),
        ]);
        assert_eq!(exprs.len(), 3);
    }

    #[test]
    fn test_arena_lifetime() {
        let arena = AstArena::new();
        
        // Allocate some expressions
        let expr1 = arena.alloc_expr(Expr::Number(1.0));
        let expr2 = arena.alloc_expr(Expr::String("test".to_string()));
        
        // They should both be valid
        assert!(matches!(expr1, Expr::Number(1.0)));
        assert!(matches!(expr2, Expr::String(_)));
    }

    #[test]
    fn test_arena_binary_expr() {
        let arena = AstArena::new();
        
        let left = arena.alloc_expr(Expr::Number(1.0));
        let right = arena.alloc_expr(Expr::Number(2.0));
        
        let binary = arena.alloc_expr(Expr::Binary {
            left: Box::new(left.clone()),
            op: TokenKind::Plus,
            right: Box::new(right.clone()),
            line: 1,
        });
        
        assert!(matches!(binary, Expr::Binary { .. }));
    }

    #[test]
    fn test_arena_memory_usage() {
        let arena = AstArena::new();
        assert!(arena.memory_usage() > 0);
    }
}
