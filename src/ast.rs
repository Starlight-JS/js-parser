pub struct Expression {
    pub pos: i32,
    pub is_paranthesized: bool,
    pub kind: ExpressionKind
}
pub struct Statement {
    pub pos: i32,
    pub kind: StatementKind
}
impl Expression {
    pub fn is_paranthesized(&self) -> bool {
        self.is_paranthesized
    }

    pub fn mark_paranthesized(&mut self) {
        self.is_paranthesized = true;
    }

    pub fn clear_paranthesized(&mut self) {
        self.is_paranthesized = false;
    }
}

pub enum ExpressionKind {

}

pub enum StatementKind {
    Breakable,
    Block()
}

pub enum BreakableStatement {
    Block(BlockStatement),
    Iteration()
}

pub struct BlockStatement {
    pub is_breakable: bool,
    pub ignore_completion_field: bool,
    pub statements: Vec<Box<Statement>>,
}

pub struct IterationStatement {
    pub body: Box<Statement>
}

pub enum IterationKind {
    While(WhileStatement),
    DoWhile(WhileStatement)
}

pub struct WhileStatement {
    pub cond: Box<Expression>,
}

pub struct ForStatement {
    pub init: Box<Statement>,
    pub cond: Box<Expression>,
    pub next: Box<Statement>,
}
pub enum ForEachVisitMode {
    /// for in 
    Enumerate,
    /// for of
    Iterate,
}
pub struct ForEachStatement {
    pub each: Box<Expression>,
    pub subject: Box<Expression>,
}

pub struct ExpressionStatement {
    pub expression: Box<Expression>,
}