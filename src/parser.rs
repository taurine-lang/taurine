use crate::lexer::{Token, TokenKind};
use crate::ast::{Expr, Stmt, Program};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(Program { statements })
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        if self.match_token(&TokenKind::Import) {
            return self.import_statement(line);
        }
        if self.match_token(&TokenKind::Loc) 
            || self.match_token(&TokenKind::Let)
            || self.match_token(&TokenKind::Const) {
            return self.variable_declaration(line);
        }
        if self.match_token(&TokenKind::Function) {
            return self.function_declaration(line);
        }
        if self.match_token(&TokenKind::Try) {
            return self.try_statement(line);
        }
        self.statement()
    }

    fn import_statement(&mut self, line: usize) -> Result<Stmt, String> {
        if !self.match_token(&TokenKind::String) {
            return Err("Expected string path after 'import'".to_string());
        }
        let path = self.previous().lexeme.clone();
        let path = path[1..path.len()-1].to_string();
        
        let alias = if self.match_token(&TokenKind::As) {
            if !self.match_token(&TokenKind::Identifier) {
                return Err("Expected identifier after 'as'".to_string());
            }
            Some(self.previous().lexeme.clone())
        } else {
            None
        };
        
        Ok(Stmt::Import { path, alias, line })
    }

    fn variable_declaration(&mut self, line: usize) -> Result<Stmt, String> {
        // Определяем тип декларации
        let is_const = self.previous().kind == TokenKind::Const;
        
        // Check for destructuring: loc/let/const {a, b} = expr
        if self.match_token(&TokenKind::LBrace) {
            let mut names = Vec::new();
            loop {
                names.push(self.consume_identifier()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.consume(&TokenKind::RBrace, "Expected '}' after destructuring")?;
            self.consume(&TokenKind::Equal, "Expected '=' after destructuring")?;
            let initializer = self.expression()?;
            return Ok(Stmt::Destructure { names, initializer, line });
        }
        
        let name = self.consume_identifier()?;
        let initializer = if self.match_token(&TokenKind::Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Stmt::Declaration { name, initializer, line, is_const })
    }

    fn function_declaration(&mut self, line: usize) -> Result<Stmt, String> {
        let name = self.consume_identifier()?;
        self.consume(&TokenKind::LParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.consume_identifier()?;
                let default = if self.match_token(&TokenKind::Equal) {
                    Some(self.expression()?)
                } else {
                    None
                };
                params.push((param_name, default));
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RParen, "Expected ')' after parameters")?;
        self.consume(&TokenKind::LBrace, "Expected '{' before function body")?;

        let body = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after function body")?;

        Ok(Stmt::Function { name, params, body, line })
    }

    fn try_statement(&mut self, line: usize) -> Result<Stmt, String> {
        self.consume(&TokenKind::LBrace, "Expected '{' after 'try'")?;
        let body = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after try block")?;
        
        let mut catch_var = None;
        let mut catch_body = Vec::new();
        
        if self.match_token(&TokenKind::Catch) {
            if self.match_token(&TokenKind::LParen) {
                if !self.match_token(&TokenKind::Identifier) {
                    return Err("Expected identifier in catch".to_string());
                }
                catch_var = Some(self.previous().lexeme.clone());
                self.consume(&TokenKind::RParen, "Expected ')' after catch variable")?;
            }
            self.consume(&TokenKind::LBrace, "Expected '{' after catch")?;
            catch_body = self.block()?;
            self.consume(&TokenKind::RBrace, "Expected '}' after catch block")?;
        }
        
        Ok(Stmt::Try { body, catch_var, catch_body, line })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        let line = self.peek().line;
        if self.match_token(&TokenKind::If) {
            return self.if_statement(line);
        }
        if self.match_token(&TokenKind::While) {
            return self.while_statement(line);
        }
        if self.match_token(&TokenKind::For) {
            return self.for_statement(line);
        }
        if self.match_token(&TokenKind::Return) {
            if self.check(&TokenKind::RBrace) || self.is_at_end() {
                return Ok(Stmt::Return(None));
            }
            // Check for multiple return values
            let first = self.expression()?;
            if self.match_token(&TokenKind::Comma) {
                let mut values = vec![first];
                loop {
                    values.push(self.expression()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                return Ok(Stmt::ReturnMulti(values));
            }
            return Ok(Stmt::Return(Some(first)));
        }
        if self.match_token(&TokenKind::Break) {
            return Ok(Stmt::Break);
        }
        if self.match_token(&TokenKind::Continue) {
            return Ok(Stmt::Continue);
        }
        if self.match_token(&TokenKind::Throw) {
            let expr = self.expression()?;
            return Ok(Stmt::Expression(Expr::Throw { expr: Box::new(expr), line }));
        }
        if self.check(&TokenKind::LBrace) {
            return self.block().map(Stmt::Block);
        }
        
        if self.check(&TokenKind::Identifier)
            && self.current + 1 < self.tokens.len()
                && self.tokens[self.current + 1].kind == TokenKind::Equal {
                    return self.assignment_statement(line);
                }
        
        self.expression_statement()
    }

    fn for_statement(&mut self, line: usize) -> Result<Stmt, String> {
        if self.check(&TokenKind::Identifier) {
            let var_name = self.peek().lexeme.clone();
            let saved_current = self.current;
            self.advance();
            if self.match_token(&TokenKind::In) {
                let iterable = self.expression()?;
                self.consume(&TokenKind::LBrace, "Expected '{' after for-in")?;
                let body = self.block()?;
                self.consume(&TokenKind::RBrace, "Expected '}' after for-in body")?;
                return Ok(Stmt::ForIn { variable: var_name, iterable, body, line });
            } else {
                self.current = saved_current;
            }
        }
        
        let has_parens = self.match_token(&TokenKind::LParen);
        
        let initializer = if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::LBrace) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        
        let condition;
        let increment;
        
        if has_parens {
            self.consume(&TokenKind::Semicolon, "Expected ';' after for initializer")?;
            
            condition = if !self.check(&TokenKind::Semicolon) {
                Box::new(self.expression()?)
            } else {
                Box::new(Expr::LiteralTrue)
            };
            self.consume(&TokenKind::Semicolon, "Expected ';' after for condition")?;
            
            increment = if !self.check(&TokenKind::RParen) {
                Some(Box::new(self.expression()?))
            } else {
                None
            };
            self.consume(&TokenKind::RParen, "Expected ')' after for increment")?;
        } else {
            condition = if initializer.is_some() {
                self.consume(&TokenKind::Comma, "Expected ',' after for initializer")?;
                Box::new(self.expression()?)
            } else {
                Box::new(Expr::LiteralTrue)
            };
            increment = None;
        }
        
        self.consume(&TokenKind::LBrace, "Expected '{' after for header")?;
        let body = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after for body")?;
        
        Ok(Stmt::For { initializer, condition, increment, body, line })
    }

    fn assignment_statement(&mut self, line: usize) -> Result<Stmt, String> {
        let name = self.consume_identifier()?;
        self.consume(&TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.expression()?;
        Ok(Stmt::Assignment { name, value, line, is_const_assign: false })
    }

    fn if_statement(&mut self, line: usize) -> Result<Stmt, String> {
        let condition = self.expression()?;
        self.consume(&TokenKind::LBrace, "Expected '{' after if condition")?;
        let then_branch = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after if branch")?;
        
        let else_branch = if self.match_token(&TokenKind::Else) {
            self.consume(&TokenKind::LBrace, "Expected '{' after else")?;
            let branch = self.block()?;
            self.consume(&TokenKind::RBrace, "Expected '}' after else branch")?;
            Some(branch)
        } else {
            None
        };
        
        Ok(Stmt::If { condition, then_branch, else_branch, line })
    }

    fn while_statement(&mut self, line: usize) -> Result<Stmt, String> {
        let condition = self.expression()?;
        self.consume(&TokenKind::LBrace, "Expected '{' after while condition")?;
        let body = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after while body")?;
        Ok(Stmt::While { condition, body, line })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.and_expr()?;
        while self.match_token(&TokenKind::Or) {
            let right = self.and_expr()?;
            left = Expr::Binary { left: Box::new(left), op: TokenKind::Or, right: Box::new(right), line };
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.equality()?;
        while self.match_token(&TokenKind::And) {
            let right = self.equality()?;
            left = Expr::Binary { left: Box::new(left), op: TokenKind::And, right: Box::new(right), line };
        }
        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.comparison()?;
        while self.match_token(&TokenKind::EqualEqual) || self.match_token(&TokenKind::NotEqual) {
            let op = self.previous().kind.clone();
            let right = self.comparison()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), line };
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.term()?;
        while self.match_token(&TokenKind::Less) || self.match_token(&TokenKind::Greater) 
            || self.match_token(&TokenKind::LessEqual) || self.match_token(&TokenKind::GreaterEqual) {
            let op = self.previous().kind.clone();
            let right = self.term()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), line };
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.factor()?;
        while self.match_token(&TokenKind::Plus) || self.match_token(&TokenKind::Minus) {
            let op = self.previous().kind.clone();
            let right = self.factor()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), line };
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        let mut left = self.unary()?;
        while self.match_token(&TokenKind::Star) || self.match_token(&TokenKind::Slash) {
            let op = self.previous().kind.clone();
            let right = self.unary()?;
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right), line };
        }
        
        // Range expression: start..end
        if self.match_token(&TokenKind::DotDot) {
            let right = self.unary()?;
            return Ok(Expr::Range { start: Box::new(left), end: Box::new(right), line });
        }
        
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;
        if self.match_token(&TokenKind::Minus) || self.match_token(&TokenKind::Not) {
            let op = self.previous().kind.clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { op, expr: Box::new(right), line });
        }
        if self.match_token(&TokenKind::Hash) {
            let _op = self.previous().kind.clone();
            let right = self.unary()?;
            return Ok(Expr::Length { expr: Box::new(right), line });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
    let line = self.peek().line;
    let mut expr = self.primary()?;
    loop {
        if self.match_token(&TokenKind::LParen) {
            let arguments = self.arguments()?;
            expr = Expr::Call { callee: Box::new(expr), arguments, line };
        } else if self.match_token(&TokenKind::Dot) {
            if !self.match_token(&TokenKind::Identifier) {
                return Err("Expected property name after '.'".to_string());
            }
            let name = self.previous().lexeme.clone();
            if self.match_token(&TokenKind::Equal) {
                let value = self.expression()?;
                expr = Expr::Set { object: Box::new(expr), name, value: Box::new(value), line };
            } else {
                expr = Expr::Get { object: Box::new(expr), name, line };
            }
        } else if self.match_token(&TokenKind::Colon) {
            if !self.match_token(&TokenKind::Identifier) {
                return Err("Expected method name after ':'".to_string());
            }
            let method = self.previous().lexeme.clone();
            if self.match_token(&TokenKind::LParen) {
                let mut arguments = self.arguments()?;
                arguments.insert(0, expr.clone());
                expr = Expr::Call {
                    callee: Box::new(Expr::Get { object: Box::new(expr.clone()), name: method, line }),
                    arguments,
                    line
                };
            } else {
                return Err("Expected '(' after method name".to_string());
            }
        } else if self.match_token(&TokenKind::QuestionDot) {
            if !self.match_token(&TokenKind::Identifier) {
                return Err("Expected property name after '?.'".to_string());
            }
            let name = self.previous().lexeme.clone();
            expr = Expr::SafeGet { object: Box::new(expr), name, line };
        } else if self.match_token(&TokenKind::LBracket) {
            let index = self.expression()?;
            self.consume(&TokenKind::RBracket, "Expected ']' after index")?;
            if self.match_token(&TokenKind::Equal) {
                let value = self.expression()?;
                expr = Expr::SetIndex {
                    object: Box::new(expr),
                    index: Box::new(index),
                    value: Box::new(value),
                    line
                };
            } else {
                expr = Expr::Index { object: Box::new(expr), index: Box::new(index), line };
            }
        } else {
            break;
        }
    }
    Ok(expr)
}

    fn arguments(&mut self) -> Result<Vec<Expr>, String> {
    let mut args = Vec::new();
    
    if self.check(&TokenKind::RParen) {
        self.advance();
        return Ok(args);
    }
    
    loop {
        args.push(self.expression()?);
        if !self.match_token(&TokenKind::Comma) {
            break;
        }
    }
    
    if !self.match_token(&TokenKind::RParen) {
        return Err("Expected ')' after arguments".to_string());
    }
    
    Ok(args)
}

    fn primary(&mut self) -> Result<Expr, String> {
        let line = self.peek().line;

        if self.match_token(&TokenKind::True) {
            return Ok(Expr::LiteralTrue);
        }
        if self.match_token(&TokenKind::False) {
            return Ok(Expr::LiteralFalse);
        }
        if self.match_token(&TokenKind::Nil) {
            return Ok(Expr::LiteralNil);
        }
        if self.match_token(&TokenKind::Number) {
            let num: f64 = self.previous().lexeme.parse().map_err(|_| "Invalid number")?;
            return Ok(Expr::Number(num));
        }
        if self.match_token(&TokenKind::String) {
            let s = self.previous().lexeme[1..self.previous().lexeme.len()-1].to_string();
            let s = s.replace("\\n", "\n")
                     .replace("\\t", "\t")
                     .replace("\\r", "\r")
                     .replace("\\\"", "\"")
                     .replace("\\'", "'")
                     .replace("\\\\", "\\");
            return Ok(Expr::String(s));
        }
        if self.match_token(&TokenKind::FString) {
            return self.fstring_literal(line);
        }
        if self.match_token(&TokenKind::Function) {
            return self.function_literal(line);
        }
        if self.match_token(&TokenKind::Identifier) {
            return Ok(Expr::Identifier(self.previous().lexeme.clone()));
        }
        if self.match_token(&TokenKind::LBrace) {
            return self.table_literal(line);
        }
        if self.match_token(&TokenKind::LBracket) {
            return self.array_literal(line);
        }
        if self.match_token(&TokenKind::LParen) {
            let expr = self.expression()?;
            self.consume(&TokenKind::RParen, "Expected ')' after expression")?;
            return Ok(expr);
        }
        Err(format!("Unexpected token: {:?} at line {}", self.peek().kind, self.peek().line))
    }

    fn function_literal(&mut self, line: usize) -> Result<Expr, String> {
        // Для function literal имя опционально (анонимные функции)
        if self.check(&TokenKind::Identifier) {
            self.advance(); // Пропускаем имя
        }
        self.consume(&TokenKind::LParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.consume_identifier()?;
                let default = if self.match_token(&TokenKind::Equal) {
                    Some(self.expression()?)
                } else {
                    None
                };
                params.push((param_name, default));
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RParen, "Expected ')' after parameters")?;
        self.consume(&TokenKind::LBrace, "Expected '{' before function body")?;

        let body = self.block()?;
        self.consume(&TokenKind::RBrace, "Expected '}' after function body")?;

        Ok(Expr::FunctionLiteral { params, body, line })
    }

    fn fstring_literal(&mut self, line: usize) -> Result<Expr, String> {
        let raw = self.previous().lexeme.clone();
        // Remove 'f"' prefix and '"' suffix
        let content = &raw[2..raw.len()-1];
        
        let mut parts = Vec::new();
        let mut current_str = String::new();
        let mut chars = content.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        'n' => current_str.push('\n'),
                        't' => current_str.push('\t'),
                        'r' => current_str.push('\r'),
                        '"' => current_str.push('"'),
                        '\\' => current_str.push('\\'),
                        _ => {
                            current_str.push('\\');
                            current_str.push(next);
                        }
                    }
                }
            } else if c == '{' {
                if !current_str.is_empty() {
                    parts.push((current_str, None));
                    current_str = String::new();
                }
                // Parse expression inside {}
                let mut expr_str = String::new();
                let mut depth = 1;
                while let Some(c) = chars.next() {
                    if c == '{' {
                        depth += 1;
                        expr_str.push(c);
                    } else if c == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        expr_str.push(c);
                    } else {
                        expr_str.push(c);
                    }
                }
                // Parse the expression
                let mut expr_parser = crate::parser::Parser::new(crate::lexer::tokenize(&expr_str));
                let expr = expr_parser.expression()?;
                parts.push((String::new(), Some(Box::new(expr))));
            } else {
                current_str.push(c);
            }
        }
        
        if !current_str.is_empty() {
            parts.push((current_str, None));
        }
        
        Ok(Expr::FString { parts, line })
    }

    fn table_literal(&mut self, line: usize) -> Result<Expr, String> {
        let mut entries = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            if !self.match_token(&TokenKind::Identifier) {
                return Err("Expected identifier in table".to_string());
            }
            let key = self.previous().lexeme.clone();
            
            // Поддерживаем оба стиля: {key = val} и {key: val}
            if self.match_token(&TokenKind::Equal) || self.match_token(&TokenKind::Colon) {
                let value = self.expression()?;
                entries.push((key, value));
            } else {
                return Err("Expected '=' or ':' after table key".to_string());
            }
            
            self.match_token(&TokenKind::Comma);
        }
        self.consume(&TokenKind::RBrace, "Expected '}' after table")?;
        Ok(Expr::Table { entries, line })
    }

    fn array_literal(&mut self, line: usize) -> Result<Expr, String> {
        let mut items = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            items.push(self.expression()?);
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.consume(&TokenKind::RBracket, "Expected ']' after array")?;
        Ok(Expr::Array { items, line })
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() { return false; }
        self.peek().kind == *kind
    }

    fn advance(&mut self) {
        if !self.is_at_end() { self.current += 1; }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<(), String> {
        if self.match_token(kind) { return Ok(()); }
        Err(format!("{} at line {}", message, self.peek().line))
    }

    fn consume_identifier(&mut self) -> Result<String, String> {
        if self.match_token(&TokenKind::Identifier) {
            Ok(self.previous().lexeme.clone())
        } else {
            Err(format!("Expected identifier at line {}", self.peek().line))
        }
    }
}