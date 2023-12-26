//! ## [8.2 Scope Analysis](https://tc39.es/ecma262/#sec-syntax-directed-operations-scope-analysis)

use oxc_ast::ast::{BindingIdentifier, Expression, Statement, BlockStatement};

pub(crate) fn bound_names(names: &mut Vec<BindingIdentifier>, statement: &Statement) {
    match statement {
        Statement::ExpressionStatement(expression) => {
            match expression {
                
            };
        },
        Statement::ForInStatement(_) => todo!(),
        Statement::ForOfStatement(_) => todo!(),
        Statement::ForStatement(_) => todo!(),
        Statement::IfStatement(_) => todo!(),
        Statement::LabeledStatement(_) => todo!(),
        Statement::ReturnStatement(_) => todo!(),
        Statement::SwitchStatement(_) => todo!(),
        Statement::ThrowStatement(_) => todo!(),
        Statement::TryStatement(_) => todo!(),
        Statement::WhileStatement(_) => todo!(),
        Statement::WithStatement(_) => todo!(),
        Statement::ModuleDeclaration(_) => todo!(),
        Statement::Declaration(_) => todo!(),
        _ => {},
    }
}