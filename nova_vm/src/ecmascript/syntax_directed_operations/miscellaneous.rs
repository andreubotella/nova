//! ## [8.6 Miscellaneous](https://tc39.es/ecma262/#sec-syntax-directed-operations-miscellaneous)
//! 
//! These operations are used in multiple places throughout the specification.

/// ## [8.6.1 Runtime Semantics: InstantiateFunctionObject](https://tc39.es/ecma262/#sec-runtime-semantics-instantiatefunctionobject)
/// 
/// The syntax-directed operation InstantiateFunctionObject takes arguments env
/// (an Environment Record) and privateEnv (a PrivateEnvironment Record or
/// null) and returns an ECMAScript function object.
pub(crate) fn instantiate_function_object() {
    todo!()
}

/// ## [8.6.2 Runtime Semantics: BindingInitialization](https://tc39.es/ecma262/#sec-runtime-semantics-bindinginitialization)
/// 
/// The syntax-directed operation BindingInitialization takes arguments value
/// (an ECMAScript language value) and environment (an Environment Record or
/// undefined) and returns either a normal completion containing UNUSED or an
/// abrupt completion.
/// 
/// *NOTE:* undefined is passed for environment to indicate that a PutValue
/// operation should be used to assign the initialization value. This is the
/// case for var statements and formal parameter lists of some non-strict
/// functions (See [10.2.11]()). In those cases a lexical binding is hoisted
/// and preinitialized prior to evaluation of its initializer.
pub(crate) fn binding_initialization() {
    todo!()
}

/// ## [8.6.2.1 InitializeBoundName ( name, value, environment )](https://tc39.es/ecma262/#sec-initializeboundname)
/// 
/// The abstract operation InitializeBoundName takes arguments name (a String),
/// value (an ECMAScript language value), and environment (an Environment
/// Record or undefined) and returns either a normal completion containing
/// UNUSED or an abrupt completion.
pub(crate) fn initialize_bound_name() {
    todo!()
}

/// ## [8.6.3 Runtime Semantics: IteratorBindingInitialization](https://tc39.es/ecma262/#sec-runtime-semantics-iteratorbindinginitialization)
/// 
/// The syntax-directed operation IteratorBindingInitialization takes arguments
/// iteratorRecord (an Iterator Record) and environment (an Environment Record
/// or undefined) and returns either a normal completion containing UNUSED or
/// an abrupt completion.
/// 
/// *NOTE:* When undefined is passed for environment it indicates that a
/// PutValue operation should be used to assign the initialization value. This
/// is the case for formal parameter lists of non-strict functions. In that
/// case the formal parameter bindings are preinitialized in order to deal with
/// the possibility of multiple parameters with the same name.
pub(crate) fn iterator_binding_initialization() {
    todo!()
}


/// ## [8.6.4 Static Semantics: AssignmentTargetType](https://tc39.es/ecma262/#sec-static-semantics-assignmenttargettype)
/// 
/// The syntax-directed operation AssignmentTargetType takes no arguments and
/// returns SIMPLE or INVALID.
pub(crate) fn assignment_target_type() {
    todo!()
}

/// ## [8.6.5 Static Semantics: PropName](https://tc39.es/ecma262/#sec-static-semantics-propname)
/// 
/// The syntax-directed operation PropName takes no arguments and returns a
/// String or EMPTY.
pub(crate) fn prop_name() {
    todo!()
}