use crate::{
    ecmascript::{
        abstract_operations::{operations_on_objects::set, type_conversion::to_object},
        execution::{
            agent::{self, ExceptionType},
            get_global_object, EnvironmentIndex,
        },
        types::{InternalMethods, Object, PropertyKey, String, Symbol, Value},
    },
    heap::indexes::StringIndex,
};
use agent::{Agent, JsResult};
use small_string::SmallString;

/// ### [6.2.5 The Reference Record Specification Type](https://tc39.es/ecma262/#sec-reference-record-specification-type)
///
/// The Reference Record type is used to explain the behaviour of such
/// operators as delete, typeof, the assignment operators, the super keyword
/// and other language features. For example, the left-hand operand of an
/// assignment is expected to produce a Reference Record.
#[derive(Debug)]
pub struct Reference {
    /// ### \[\[Base]]
    ///
    /// The value or Environment Record which holds the binding. A \[\[Base]]
    /// of UNRESOLVABLE indicates that the binding could not be resolved.
    pub(crate) base: Base,

    /// ### \[\[ReferencedName]]
    ///
    /// The name of the binding. Always a String if \[\[Base]] value is an
    /// Environment Record.
    pub(crate) referenced_name: ReferencedName,

    /// ### \[\[Strict]]
    ///
    /// true if the Reference Record originated in strict mode code, false
    /// otherwise.
    pub(crate) strict: bool,

    /// ### \[\[ThisValue]]
    ///
    /// If not EMPTY, the Reference Record represents a property binding that
    /// was expressed using the super keyword; it is called a Super Reference
    /// Record and its \[\[Base]] value will never be an Environment Record. In
    /// that case, the \[\[ThisValue]] field holds the this value at the time
    /// the Reference Record was created.
    pub(crate) this_value: Option<Value>,
}

/// ### [6.2.5.1 IsPropertyReference ( V )](https://tc39.es/ecma262/#sec-ispropertyreference)
///
/// The abstract operation IsPropertyReference takes argument V (a Reference
/// Record) and returns a Boolean.
pub(crate) fn is_property_reference(reference: &Reference) -> bool {
    match reference.base {
        // 1. if V.[[Base]] is unresolvable, return false.
        Base::Unresolvable => false,

        // 2. If V.[[Base]] is an Environment Record, return false; otherwise return true.
        Base::Environment(_) => false,
        _ => true,
    }
}

/// ### [6.2.5.2 IsUnresolvableReference ( V )](https://tc39.es/ecma262/#sec-isunresolvablereference)
///
/// The abstract operation IsUnresolvableReference takes argument V (a
/// Reference Record) and returns a Boolean.
pub(crate) fn is_unresolvable_reference(reference: &Reference) -> bool {
    // 1. If V.[[Base]] is unresolvable, return true; otherwise return false.
    matches!(reference.base, Base::Unresolvable)
}

/// ### [6.2.5.3 IsSuperReference ( V )](https://tc39.es/ecma262/#sec-issuperreference)
///
/// The abstract operation IsSuperReference takes argument V (a Reference
/// Record) and returns a Boolean.
pub(crate) fn is_super_reference(reference: &Reference) -> bool {
    // 1. If V.[[ThisValue]] is not empty, return true; otherwise return false.
    reference.this_value.is_some()
}

/// ### [6.2.5.4 IsPrivateReference ( V )](https://tc39.es/ecma262/#sec-isprivatereference)
///
/// The abstract operation IsPrivateReference takes argument V (a Reference
/// Record) and returns a Boolean.
pub(crate) fn is_private_reference(reference: &Reference) -> bool {
    // 1. If V.[[ReferencedName]] is a Private Name, return true; otherwise return false.
    matches!(reference.referenced_name, ReferencedName::PrivateName)
}

/// ### [6.2.5.5 GetValue ( V )](https://tc39.es/ecma262/#sec-getvalue)
/// The abstract operation GetValue takes argument V (a Reference Record or an
/// ECMAScript language value) and returns either a normal completion
/// containing an ECMAScript language value or an abrupt completion.
pub(crate) fn get_value(agent: &mut Agent, reference: &Reference) -> JsResult<Value> {
    match reference.base {
        Base::Value(value) => {
            // 3. If IsPropertyReference(V) is true, then
            // a. Let baseObj be ? ToObject(V.[[Base]]).

            // NOTE
            // The object that may be created in step 3.a is not
            // accessible outside of the above abstract operation
            // and the ordinary object [[Get]] internal method. An
            // implementation might choose to avoid the actual
            // creation of the object.
            let referenced_name = match &reference.referenced_name {
                ReferencedName::String(data) => PropertyKey::String(*data),
                ReferencedName::SmallString(data) => PropertyKey::SmallString(*data),
                ReferencedName::Symbol(symbol) => PropertyKey::from(*symbol),
                ReferencedName::PrivateName => {
                    // b. If IsPrivateReference(V) is true, then
                    // i. Return ? PrivateGet(baseObj, V.[[ReferencedName]]).
                    todo!()
                }
            };
            if let Ok(object) = Object::try_from(value) {
                // c. Return ? baseObj.[[Get]](V.[[ReferencedName]], GetThisValue(V)).
                Ok(object.internal_get(agent, referenced_name, get_this_value(reference))?)
            } else {
                // Primitive value. annoying stuff.
                match value {
                    Value::Undefined => Err(agent.throw_exception(
                        ExceptionType::TypeError,
                        "Cannot read properties of undefined",
                    )),
                    Value::Null => Err(agent.throw_exception(
                        ExceptionType::TypeError,
                        "Cannot read properties of null",
                    )),
                    Value::Boolean(_) => agent
                        .current_realm()
                        .intrinsics()
                        .boolean_prototype()
                        .internal_get(agent, referenced_name, value),
                    Value::String(_) | Value::SmallString(_) => agent
                        .current_realm()
                        .intrinsics()
                        .string_prototype()
                        .internal_get(agent, referenced_name, value),
                    Value::Symbol(_) => agent
                        .current_realm()
                        .intrinsics()
                        .symbol_prototype()
                        .internal_get(agent, referenced_name, value),
                    Value::Number(_) | Value::Integer(_) | Value::Float(_) => agent
                        .current_realm()
                        .intrinsics()
                        .number_prototype()
                        .internal_get(agent, referenced_name, value),
                    Value::BigInt(_) | Value::SmallBigInt(_) => agent
                        .current_realm()
                        .intrinsics()
                        .big_int_prototype()
                        .internal_get(agent, referenced_name, value),
                    _ => unreachable!(),
                }
            }
        }
        Base::Environment(env) => {
            // 4. Else,
            // a. Let base be V.[[Base]].
            // b. Assert: base is an Environment Record.
            // c. Return ? base.GetBindingValue(V.[[ReferencedName]], V.[[Strict]]) (see 9.1).
            let referenced_name = match &reference.referenced_name {
                ReferencedName::String(data) => String::String(*data),
                ReferencedName::SmallString(data) => String::SmallString(*data),
                _ => unreachable!(),
            };
            Ok(env.get_binding_value(agent, referenced_name, reference.strict)?)
        }
        Base::Unresolvable => {
            // 2. If IsUnresolvableReference(V) is true, throw a ReferenceError exception.
            Err(agent.throw_exception(
                ExceptionType::ReferenceError,
                "Unable to resolve identifier.",
            ))
        }
    }
}

/// ### [6.2.5.6 PutValue ( V, W )](https://tc39.es/ecma262/#sec-putvalue)
///
/// The abstract operation PutValue takes arguments V (a Reference Record or an
/// ECMAScript language value) and W (an ECMAScript language value) and returns
/// either a normal completion containing UNUSED or an abrupt completion.
pub(crate) fn put_value(agent: &mut Agent, v: &Reference, w: Value) -> JsResult<()> {
    // 1. If V is not a Reference Record, throw a ReferenceError exception.
    // 2. If IsUnresolvableReference(V) is true, then
    if is_unresolvable_reference(v) {
        if v.strict {
            // a. If V.[[Strict]] is true, throw a ReferenceError exception.
            return Err(
                agent.throw_exception(ExceptionType::ReferenceError, "Could not resolve reference")
            );
        }
        // b. Let globalObj be GetGlobalObject().
        let global_obj = get_global_object(agent);
        // c. Perform ? Set(globalObj, V.[[ReferencedName]], W, false).
        let referenced_name = match &v.referenced_name {
            ReferencedName::String(data) => PropertyKey::String(*data),
            ReferencedName::SmallString(data) => PropertyKey::SmallString(*data),
            ReferencedName::Symbol(_) => todo!(),
            ReferencedName::PrivateName => todo!(),
        };
        set(agent, global_obj, referenced_name, w, false)?;
        // d. Return UNUSED.
        Ok(())
    } else if is_property_reference(v) {
        // 3. If IsPropertyReference(V) is true, then
        // a. Let baseObj be ? ToObject(V.[[Base]]).
        let base = match v.base {
            Base::Value(value) => value,
            Base::Environment(_) | Base::Unresolvable => unreachable!(),
        };
        let base_obj = to_object(agent, base)?;
        // b. If IsPrivateReference(V) is true, then
        if is_private_reference(v) {
            // i. Return ? PrivateSet(baseObj, V.[[ReferencedName]], W).
            todo!();
        }
        // c. Let succeeded be ? baseObj.[[Set]](V.[[ReferencedName]], W, GetThisValue(V)).
        let this_value = get_this_value(v);
        let referenced_name = match &v.referenced_name {
            ReferencedName::String(data) => PropertyKey::String(*data),
            ReferencedName::SmallString(data) => PropertyKey::SmallString(*data),
            ReferencedName::Symbol(_) => todo!(),
            ReferencedName::PrivateName => todo!(),
        };
        let succeeded = base_obj.internal_set(agent, referenced_name, w, this_value)?;
        if !succeeded && v.strict {
            // d. If succeeded is false and V.[[Strict]] is true, throw a TypeError exception.
            return Err(
                agent.throw_exception(ExceptionType::ReferenceError, "Could not set property")
            );
        }
        // e. Return UNUSED.
        Ok(())
    } else {
        // 4. Else,
        // a. Let base be V.[[Base]].
        let base = &v.base;
        // b. Assert: base is an Environment Record.
        let Base::Environment(base) = base else {
            unreachable!()
        };
        // c. Return ? base.SetMutableBinding(V.[[ReferencedName]], W, V.[[Strict]]) (see 9.1).
        let referenced_name = match &v.referenced_name {
            ReferencedName::String(data) => String::String(*data),
            ReferencedName::SmallString(data) => String::SmallString(*data),
            ReferencedName::Symbol(_) => todo!(),
            ReferencedName::PrivateName => todo!(),
        };
        base.set_mutable_binding(agent, referenced_name, w, v.strict)
    }
    // NOTE
    // The object that may be created in step 3.a is not accessible outside of the above abstract operation and the ordinary object [[Set]] internal method. An implementation might choose to avoid the actual creation of that object.
}

/// ### {6.2.5.7 GetThisValue ( V )}(https://tc39.es/ecma262/#sec-getthisvalue)
/// The abstract operation GetThisValue takes argument V (a Reference Record)
/// and returns an ECMAScript language value.
pub(crate) fn get_this_value(reference: &Reference) -> Value {
    // 1. Assert: IsPropertyReference(V) is true.
    debug_assert!(is_property_reference(reference));
    // 2. If IsSuperReference(V) is true, return V.[[ThisValue]]; otherwise return V.[[Base]].
    reference
        .this_value
        .unwrap_or_else(|| match reference.base {
            Base::Value(value) => value,
            Base::Environment(_) | Base::Unresolvable => unreachable!(),
        })
}

#[derive(Debug)]
pub(crate) enum Base {
    Value(Value),
    Environment(EnvironmentIndex),
    Unresolvable,
}

#[derive(Debug)]
pub enum ReferencedName {
    String(StringIndex),
    SmallString(SmallString),
    Symbol(Symbol),
    // TODO: implement private names
    PrivateName,
}

impl From<String> for ReferencedName {
    fn from(value: String) -> Self {
        match value {
            String::String(data) => Self::String(data),
            String::SmallString(data) => Self::SmallString(data),
        }
    }
}
