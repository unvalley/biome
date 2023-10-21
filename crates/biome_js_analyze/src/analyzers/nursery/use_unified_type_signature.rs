use crate::semantic_services::{Semantic, SemanticServices};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::JsIdentifierBinding;

use ::serde::{Deserialize, Serialize};
use biome_rowan::{declare_node_union, TextRange};
use bpaf::Bpaf;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

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

#[derive(Debug)]
pub(crate) struct LinterTarget {
    name: String,
    declaration: TextRange,
    redeclaration: TextRange,
}

// similar to no_redeclare
impl Rule for UseUnifiedTypeSignature {
    type Query = SemanticServices;
    type State = LinterTarget;
    type Signals = Vec<Self::State>;
    type Options = RuleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut lint_targets = Vec::default();
        for scope in ctx.query().scopes() {
            checker(&scope, &mut lint_targets);
        }
        lint_targets
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
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

/// Returns
fn checker(scope: &Scope, lint_targets: &mut Vec<LinterTarget>) {}
