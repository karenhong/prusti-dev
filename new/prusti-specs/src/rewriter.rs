use crate::specifications::common::{ExpressionIdGenerator, SpecificationIdGenerator};
use crate::specifications::untyped::{self, EncodeTypeCheck};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;

pub(crate) struct AstRewriter {
    expr_id_generator: ExpressionIdGenerator,
    spec_id_generator: SpecificationIdGenerator,
}

impl AstRewriter {
    pub(crate) fn new() -> Self {
        Self {
            expr_id_generator: ExpressionIdGenerator::new(),
            spec_id_generator: SpecificationIdGenerator::new(),
        }
    }
    /// Parse an assertion.
    ///
    /// Note: If this method encounters an error, it simply logs the error and
    /// returns `true`.
    pub fn parse_assertion(&mut self, tokens: TokenStream) -> syn::Result<untyped::Assertion> {
        untyped::Assertion::parse(tokens, &mut self.expr_id_generator)
    }
    /// Check whether function `item` contains a parameter called `keyword`. If
    /// yes, return its span.
    fn check_contains_keyword_in_params(&self, item: &syn::ItemFn, keyword: &str) -> Option<Span> {
        for param in &item.sig.inputs {
            match param {
                syn::FnArg::Typed(syn::PatType {
                    pat: box syn::Pat::Ident(syn::PatIdent { ident, .. }),
                    ..
                }) => {
                    if ident == keyword {
                        return Some(param.span());
                    }
                }
                _ => {}
            }
        }
        None
    }
    /// Generate a dummy function for checking the given precondition.
    ///
    /// `spec_type` should be either `"pre"` or `"post"`.
    pub fn generate_spec_item_fn(
        &mut self,
        spec_type: &str,
        assertion: untyped::Assertion,
        item: &syn::ItemFn,
    ) -> syn::Result<syn::Item> {
        if let Some(span) = self.check_contains_keyword_in_params(item, "result") {
            return Err(syn::Error::new(
                span,
                "it is not allowed to use the keyword `result` as a function argument".to_string(),
            ));
        }
        let spec_id = self.spec_id_generator.generate();
        let item_name = syn::Ident::new(
            &format!("prusti_{}_item_{}_{}", spec_type, item.sig.ident, spec_id),
            item.span(),
        );
        let mut statements = TokenStream::new();
        assertion.encode_type_check(&mut statements);
        let mut spec_item: syn::ItemFn = syn::parse_quote! {
            fn #item_name() {
                #statements
            }
        };
        spec_item.sig.generics = item.sig.generics.clone();
        spec_item.sig.inputs = item.sig.inputs.clone();
        Ok(syn::Item::Fn(spec_item))
    }
    /// Generate statements for checking the given loop invariant.
    pub fn generate_spec_loop(&mut self, assertion: untyped::Assertion) -> TokenStream {
        let mut statements = TokenStream::new();
        assertion.encode_type_check(&mut statements);
        quote! {
            let _prusti_loop_invariant = {
                #statements
            };
        }
    }
}