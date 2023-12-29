//! ## [8.4 Function Name Inference](https://tc39.es/ecma262/#sec-syntax-directed-operations-function-name-inference)

/// ## [8.4.1 Static Semantics: HasName](https://tc39.es/ecma262/#sec-static-semantics-hasname)
///
/// The syntax-directed operation HasName takes no arguments and returns a
/// Boolean.
pub(crate) fn has_name() -> bool {
    todo!()
}

/// ## [8.4.2 Static Semantics: IsFunctionDefinition](https://tc39.es/ecma262/#sec-static-semantics-isfunctiondefinition)
///
/// The syntax-directed operation IsFunctionDefinition takes no arguments and
/// returns a Boolean.
pub(crate) fn is_function_definition() -> bool {
    todo!()
}

/// ## [8.4.3 Static Semantics: IsAnonymousFunctionDefinition ( expr )](https://tc39.es/ecma262/#sec-isanonymousfunctiondefinition)
///
/// The abstract operation IsAnonymousFunctionDefinition takes argument expr
/// (an AssignmentExpression Parse Node, an Initializer Parse Node, or an
/// Expression Parse Node) and returns a Boolean. It determines if its argument
/// is a function definition that does not bind a name.
pub(crate) fn is_anonymous_function_definition() -> bool {
    todo!()
}

/// ## [8.4.4 Static Semantics: IsIdentifierRef](https://tc39.es/ecma262/#sec-static-semantics-isidentifierref)
///
/// The syntax-directed operation IsIdentifierRef takes no arguments and
/// returns a Boolean.
pub(crate) fn is_identifier_ref() -> bool {
    todo!()
}

/// ## [8.4.5 Runtime Semantics: NamedEvaluation](https://tc39.es/ecma262/#sec-runtime-semantics-namedevaluation)
///
/// The syntax-directed operation NamedEvaluation takes argument name (a
/// property key or a Private Name) and returns either a normal completion
/// containing a function object or an abrupt completion.
pub(crate) fn named_evaluation() {
    todo!()
}
