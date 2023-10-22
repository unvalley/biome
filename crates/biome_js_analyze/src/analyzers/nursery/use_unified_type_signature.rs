use crate::semantic_services::SemanticServices;
use ::serde::{Deserialize, Serialize};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::Scope;
use biome_js_syntax::{
    binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding},
    AnyJsBindingPattern, AnyJsFormalParameter, AnyJsParameter, JsParameters, TsTypeAnnotation,
};
use biome_rowan::{AstNode, TextRange};
use bpaf::Bpaf;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use std::collections::HashMap;

declare_rule! {
    /// Disallow two overloads that could be unified into one with a union or an optional/rest parameter
    ///
    ///
    /// Source: https://typescript-eslint.io/rules/unified-signatures/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// function x(x: number): void;
    /// function x(x: string): void;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function y(): void;
    /// function y(...x: number[]): void;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// function x(x: number | string): void;
    /// ```
    ///
    /// ```ts
    /// function y(...x: number[]): void;
    /// ```
    ///
    /// ```ts
    /// This rule won't check overload signatures with different rest parameter types.
    /// See https://github.com/microsoft/TypeScript/issues/5077
    /// function f(...a: number[]): void;
    /// function f(...a: string[]): void;
    /// ```
    ///
    pub(crate) UseUnifiedTypeSignature {
        version: "next",
        name: "useUnifiedTypeSignature",
        recommended: false,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleOptions {
    /// Ignore differently named parameters
    #[bpaf(hide)]
    #[serde(
        default = "default_ignore_differently_named_parameters",
        skip_serializing_if = "is_default_ignore_differently_named_parameters"
    )]
    pub ignore_differently_named_parameters: bool,
}

const fn default_ignore_differently_named_parameters() -> bool {
    true
}

const fn is_default_ignore_differently_named_parameters(given_option: &bool) -> bool {
    *given_option == default_ignore_differently_named_parameters()
}

impl Default for RuleOptions {
    fn default() -> Self {
        Self {
            ignore_differently_named_parameters: default_ignore_differently_named_parameters(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FunctionOverload {
    func_name: String,
    type_signatures: Vec<(AnyJsBindingPattern, TsTypeAnnotation)>,
    /// The range of the function parameters that can be unified
    unifiable_params_range: TextRange,
}

impl Rule for UseUnifiedTypeSignature {
    type Query = SemanticServices;
    type State = FunctionOverload;
    type Signals = Vec<Self::State>;
    type Options = RuleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut unifiable_type_signatures = Vec::default();
        // collects unifiable type signatures from each scope
        for scope in ctx.query().scopes() {
            check_unifiable_type_signatures(&scope, &mut unifiable_type_signatures);
        }
        dbg!(&unifiable_type_signatures);
        unifiable_type_signatures
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let FunctionOverload {
            func_name,
            type_signatures,
            unifiable_params_range: unifiable_declaration_range,
        } = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                unifiable_declaration_range,
                markup! {
                    "These overloads can be combined into one signature."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

/// check for disallowing two overloads that could be unified into one with a union or an optional parameter
/// TODO: Need to handle a lot of type signature cases
///
/// ref: https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/src/rules/unified-signatures.ts
/// - typescript-eslint's behavior is not expected, because they doesn't report to implementation part of the overloads
fn check_unifiable_type_signatures(scope: &Scope, redeclarations: &mut Vec<FunctionOverload>) {
    // check only 1 previous function declaration

    let mut declarations: HashMap<String, FunctionOverload> = HashMap::new();

    for binding in scope.bindings() {
        let AnyJsIdentifierBinding::JsIdentifierBinding(id_binding) = binding.tree() else {
            continue;
        };
        let Some(binding_decl) = id_binding.declaration() else {
            continue;
        };

        // handle typescript function only
        let (func_name, overload) = match binding_decl {
            AnyJsBindingDeclaration::JsFunctionDeclaration(js_func_decl) => {
                let Some(parameters) = js_func_decl.parameters().ok() else {
                    continue;
                };
                let func_name = id_binding.text();
                let overload = FunctionOverload {
                    func_name: func_name.clone(),
                    unifiable_params_range: parameters.range(),
                    type_signatures: collect_type_signatures(parameters),
                };
                (func_name, overload)
            }
            AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(ts_func_decl) => {
                let Some(parameters) = ts_func_decl.parameters().ok() else {
                    continue;
                };
                let func_name = id_binding.text();
                let overload = FunctionOverload {
                    func_name: func_name.clone(),
                    unifiable_params_range: parameters.range(),
                    type_signatures: collect_type_signatures(parameters),
                };
                (func_name, overload)
            }
            _ => continue
            // AnyJsBindingDeclaration::JsVariableDeclarator(_) => todo!(),
            // AnyJsBindingDeclaration::JsFormalParameter(_) => todo!(),
            // AnyJsBindingDeclaration::JsRestParameter(_) => todo!(),
            // AnyJsBindingDeclaration::JsBogusParameter(_) => todo!(),
            // AnyJsBindingDeclaration::TsIndexSignatureParameter(_) => todo!(),
            // AnyJsBindingDeclaration::TsPropertyParameter(_) => todo!(),
            // AnyJsBindingDeclaration::TsInferType(_) => todo!(),
            // AnyJsBindingDeclaration::TsMappedType(_) => todo!(),
            // AnyJsBindingDeclaration::TsTypeParameter(_) => todo!(),
            // AnyJsBindingDeclaration::JsFunctionExpression(_) => todo!(),
            // AnyJsBindingDeclaration::JsClassDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::JsClassExpression(_) => todo!(),
            // AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::TsEnumDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::TsModuleDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::JsImportDefaultClause(_) => todo!(),
            // AnyJsBindingDeclaration::JsImportNamespaceClause(_) => todo!(),
            // AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => todo!(),
            // AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => todo!(),
            // AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_) => todo!(),
            // AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => todo!(),
            // AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => todo!(),
            // AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => todo!(),
            // AnyJsBindingDeclaration::JsCatchDeclaration(_) => todo!(),
        };
        if let Some(overload) = declarations.get(&func_name) {
            redeclarations.push(overload.clone())
        } else {
            declarations.insert(func_name.clone(), overload.clone());
        }
    }
}

fn collect_type_signatures(
    parameters: JsParameters,
) -> Vec<(AnyJsBindingPattern, TsTypeAnnotation)> {
    let type_signatures = parameters
        .items()
        .into_iter()
        .filter_map(|param| {
            let param = param.ok()?;
            match param {
                AnyJsParameter::AnyJsFormalParameter(param) => match param {
                    AnyJsFormalParameter::JsFormalParameter(param) => {
                        let param_binding = param.binding().ok()?;
                        let type_annotation = param.type_annotation()?;
                        Some((param_binding, type_annotation))
                    }
                    AnyJsFormalParameter::JsBogusParameter(_) => None,
                },
                AnyJsParameter::JsRestParameter(param) => {
                    let param_binding = param.binding().ok()?;
                    let type_annotation = param.type_annotation()?;
                    Some((param_binding, type_annotation))
                }
                AnyJsParameter::TsThisParameter(_) => None,
            }
        })
        .collect::<Vec<_>>();
    type_signatures
}
