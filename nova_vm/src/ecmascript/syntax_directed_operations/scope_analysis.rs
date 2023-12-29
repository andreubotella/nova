//! ## [8.2 Scope Analysis](https://tc39.es/ecma262/#sec-syntax-directed-operations-scope-analysis)
//!
//! ### [5.2.2 Syntax-Directed Operations](https://tc39.es/ecma262/#sec-algorithm-conventions-syntax-directed-operations)
//!
//! A syntax-directed operation is a named operation whose definition consists
//! of algorithms, each of which is associated with one or more productions
//! from one of the ECMAScript grammars. A production that has multiple
//! alternative definitions will typically have a distinct algorithm for each
//! alternative. When an algorithm is associated with a grammar production, it
//! may reference the terminal and nonterminal symbols of the production
//! alternative as if they were parameters of the algorithm. When used in this
//! manner, nonterminal symbols refer to the actual alternative definition that
//! is matched when parsing the source text. The source text matched by a
//! grammar production or Parse Node derived from it is the portion of the
//! source text that starts at the beginning of the first terminal that
//! participated in the match and ends at the end of the last terminal that
//! participated in the match.

use oxc_ast::ast::Declaration;

/// ## [8.2.1 Static Semantics: BoundsNames](https://tc39.es/ecma262/#sec-syntax-directed-operations-scope-analysis)
///
/// The syntax-directed operation BoundNames takes no arguments and returns a
/// List of Strings.
pub(crate) fn bound_names() {
    todo!()
}

/// ## [8.2.2 Static Semantics: DeclarationPart](https://tc39.es/ecma262/#sec-static-semantics-declarationpart)
///
/// The syntax-directed operation DeclarationPart takes no arguments and returns a Parse Node.
pub(crate) fn declaration_part(_declaration: Declaration) {
    todo!()
}

/// ## [8.2.3 Static Semantics: IsConstantDeclaration](https://tc39.es/ecma262/#sec-static-semantics-isconstantdeclaration)
///
/// The syntax-directed operation IsConstantDeclaration takes no arguments and returns a Boolean.
pub(crate) fn is_constant_declaration(_declaration: Declaration) -> bool {
    todo!()
}

/// ## [8.2.4 Static Semantics: LexicallyDeclaredNames](https://tc39.es/ecma262/#sec-static-semantics-lexicallydeclarednames)
///
/// The syntax-directed operation LexicallyDeclaredNames takes no arguments and returns a List of Strings.
pub(crate) fn lexically_declared_names() {
    todo!()
}

/// ## [8.2.5 Static Semantics: LexicallyScopedDeclarations](https://tc39.es/ecma262/#sec-static-semantics-lexicallyscopeddeclarations)
///
/// The syntax-directed operation LexicallyScopedDeclarations takes no arguments and returns a List of Parse Nodes.
pub(crate) fn lexically_scoped_declarations() {
    todo!()
}

/// ## [8.2.6 Static Semantics: VarDeclaredNames](https://tc39.es/ecma262/#sec-static-semantics-vardeclarednames)
///
/// The syntax-directed operation VarDeclaredNames takes no arguments and returns a List of Strings.
pub(crate) fn var_declared_names() {
    todo!()
}

/// ## [8.2.7 Static Semantics: VarScopedDeclarations](https://tc39.es/ecma262/#sec-static-semantics-varscopeddeclarations)
///
/// The syntax-directed operation VarScopedDeclarations takes no arguments and returns a List of Parse Nodes.
pub(crate) fn var_scoped_declarations() {
    todo!()
}

/// ## [8.2.8 Static Semantics: TopLevelLexicallyDeclaredNames](https://tc39.es/ecma262/#sec-static-semantics-toplevellexicallydeclarednames)
///
/// The syntax-directed operation TopLevelLexicallyDeclaredNames takes no
/// arguments and returns a List of Strings.
pub(crate) fn top_level_lexically_declared_names() {
    todo!()
}

/// ## [8.2.9 Static Semantics: TopLevelLexicallyScopedDeclarations](https://tc39.es/ecma262/#sec-static-semantics-toplevellexicallyscopeddeclarations)
///
/// The syntax-directed operation TopLevelLexicallyScopedDeclarations takes no
/// arguments and returns a List of Parse Nodes.
pub(crate) fn top_level_lexically_scoped_declarations() {
    todo!()
}

/// ## [8.2.10 Static Semantics: TopLevelVarDeclaredNames](https://tc39.es/ecma262/#sec-static-semantics-toplevelvardeclarednames)
///
/// The syntax-directed operation TopLevelVarDeclaredNames takes no arguments
/// and returns a List of Strings.
pub(crate) fn top_level_var_declared_names() {
    todo!()
}

/// ## [8.2.11 Static Semantics: TopLevelVarScopedDeclarations](https://tc39.es/ecma262/#sec-static-semantics-toplevelvarscopeddeclarations)
///
/// The syntax-directed operation TopLevelVarScopedDeclarations takes no
/// arguments and returns a List of Parse Nodes.
pub(crate) fn top_level_var_scoped_declarations() {
    todo!()
}
