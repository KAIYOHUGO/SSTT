// Codegen file with version `0.1.0`
// !!!Don't modify this file manually!!!
// --------------------



pub use tmp::*;
#[allow(unused_parens, unused_variables, unreachable_patterns)]
mod tmp {
    /// `fn try_take_T(&mut self, T) -> Result<(), Self::Error>`
    ///
    /// It can take syntax tree and report error
    pub trait TryTake {
        type Error;
        fn try_take_abi(&mut self, t: syn::Abi) -> Result<(), Self::Error> {
            try_take_abi(self, t)
        }
        fn try_take_angle_bracketed_generic_arguments(
            &mut self,
            t: syn::AngleBracketedGenericArguments,
        ) -> Result<(), Self::Error> {
            try_take_angle_bracketed_generic_arguments(self, t)
        }
        fn try_take_arm(&mut self, t: syn::Arm) -> Result<(), Self::Error> {
            try_take_arm(self, t)
        }
        fn try_take_assoc_const(
            &mut self,
            t: syn::AssocConst,
        ) -> Result<(), Self::Error> {
            try_take_assoc_const(self, t)
        }
        fn try_take_assoc_type(&mut self, t: syn::AssocType) -> Result<(), Self::Error> {
            try_take_assoc_type(self, t)
        }
        fn try_take_attr_style(&mut self, t: syn::AttrStyle) -> Result<(), Self::Error> {
            try_take_attr_style(self, t)
        }
        fn try_take_attr_style_variant_inner(
            &mut self,
            t: (syn::token::Not),
        ) -> Result<(), Self::Error> {
            try_take_attr_style_variant_inner(self, t)
        }
        fn try_take_attribute(&mut self, t: syn::Attribute) -> Result<(), Self::Error> {
            try_take_attribute(self, t)
        }
        fn try_take_bare_fn_arg(
            &mut self,
            t: syn::BareFnArg,
        ) -> Result<(), Self::Error> {
            try_take_bare_fn_arg(self, t)
        }
        fn try_take_bare_variadic(
            &mut self,
            t: syn::BareVariadic,
        ) -> Result<(), Self::Error> {
            try_take_bare_variadic(self, t)
        }
        fn try_take_bin_op(&mut self, t: syn::BinOp) -> Result<(), Self::Error> {
            try_take_bin_op(self, t)
        }
        fn try_take_bin_op_variant_add(
            &mut self,
            t: (syn::token::Plus),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_add(self, t)
        }
        fn try_take_bin_op_variant_sub(
            &mut self,
            t: (syn::token::Minus),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_sub(self, t)
        }
        fn try_take_bin_op_variant_mul(
            &mut self,
            t: (syn::token::Star),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_mul(self, t)
        }
        fn try_take_bin_op_variant_div(
            &mut self,
            t: (syn::token::Slash),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_div(self, t)
        }
        fn try_take_bin_op_variant_rem(
            &mut self,
            t: (syn::token::Percent),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_rem(self, t)
        }
        fn try_take_bin_op_variant_and(
            &mut self,
            t: (syn::token::AndAnd),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_and(self, t)
        }
        fn try_take_bin_op_variant_or(
            &mut self,
            t: (syn::token::OrOr),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_or(self, t)
        }
        fn try_take_bin_op_variant_bit_xor(
            &mut self,
            t: (syn::token::Caret),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_xor(self, t)
        }
        fn try_take_bin_op_variant_bit_and(
            &mut self,
            t: (syn::token::And),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_and(self, t)
        }
        fn try_take_bin_op_variant_bit_or(
            &mut self,
            t: (syn::token::Or),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_or(self, t)
        }
        fn try_take_bin_op_variant_shl(
            &mut self,
            t: (syn::token::Shl),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_shl(self, t)
        }
        fn try_take_bin_op_variant_shr(
            &mut self,
            t: (syn::token::Shr),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_shr(self, t)
        }
        fn try_take_bin_op_variant_eq(
            &mut self,
            t: (syn::token::EqEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_eq(self, t)
        }
        fn try_take_bin_op_variant_lt(
            &mut self,
            t: (syn::token::Lt),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_lt(self, t)
        }
        fn try_take_bin_op_variant_le(
            &mut self,
            t: (syn::token::Le),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_le(self, t)
        }
        fn try_take_bin_op_variant_ne(
            &mut self,
            t: (syn::token::Ne),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_ne(self, t)
        }
        fn try_take_bin_op_variant_ge(
            &mut self,
            t: (syn::token::Ge),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_ge(self, t)
        }
        fn try_take_bin_op_variant_gt(
            &mut self,
            t: (syn::token::Gt),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_gt(self, t)
        }
        fn try_take_bin_op_variant_add_assign(
            &mut self,
            t: (syn::token::PlusEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_add_assign(self, t)
        }
        fn try_take_bin_op_variant_sub_assign(
            &mut self,
            t: (syn::token::MinusEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_sub_assign(self, t)
        }
        fn try_take_bin_op_variant_mul_assign(
            &mut self,
            t: (syn::token::StarEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_mul_assign(self, t)
        }
        fn try_take_bin_op_variant_div_assign(
            &mut self,
            t: (syn::token::SlashEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_div_assign(self, t)
        }
        fn try_take_bin_op_variant_rem_assign(
            &mut self,
            t: (syn::token::PercentEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_rem_assign(self, t)
        }
        fn try_take_bin_op_variant_bit_xor_assign(
            &mut self,
            t: (syn::token::CaretEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_xor_assign(self, t)
        }
        fn try_take_bin_op_variant_bit_and_assign(
            &mut self,
            t: (syn::token::AndEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_and_assign(self, t)
        }
        fn try_take_bin_op_variant_bit_or_assign(
            &mut self,
            t: (syn::token::OrEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_bit_or_assign(self, t)
        }
        fn try_take_bin_op_variant_shl_assign(
            &mut self,
            t: (syn::token::ShlEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_shl_assign(self, t)
        }
        fn try_take_bin_op_variant_shr_assign(
            &mut self,
            t: (syn::token::ShrEq),
        ) -> Result<(), Self::Error> {
            try_take_bin_op_variant_shr_assign(self, t)
        }
        fn try_take_block(&mut self, t: syn::Block) -> Result<(), Self::Error> {
            try_take_block(self, t)
        }
        fn try_take_bound_lifetimes(
            &mut self,
            t: syn::BoundLifetimes,
        ) -> Result<(), Self::Error> {
            try_take_bound_lifetimes(self, t)
        }
        fn try_take_const_param(
            &mut self,
            t: syn::ConstParam,
        ) -> Result<(), Self::Error> {
            try_take_const_param(self, t)
        }
        fn try_take_constraint(
            &mut self,
            t: syn::Constraint,
        ) -> Result<(), Self::Error> {
            try_take_constraint(self, t)
        }
        fn try_take_data(&mut self, t: syn::Data) -> Result<(), Self::Error> {
            try_take_data(self, t)
        }
        fn try_take_data_variant_struct(
            &mut self,
            t: (syn::DataStruct),
        ) -> Result<(), Self::Error> {
            try_take_data_variant_struct(self, t)
        }
        fn try_take_data_variant_enum(
            &mut self,
            t: (syn::DataEnum),
        ) -> Result<(), Self::Error> {
            try_take_data_variant_enum(self, t)
        }
        fn try_take_data_variant_union(
            &mut self,
            t: (syn::DataUnion),
        ) -> Result<(), Self::Error> {
            try_take_data_variant_union(self, t)
        }
        fn try_take_data_enum(&mut self, t: syn::DataEnum) -> Result<(), Self::Error> {
            try_take_data_enum(self, t)
        }
        fn try_take_data_struct(
            &mut self,
            t: syn::DataStruct,
        ) -> Result<(), Self::Error> {
            try_take_data_struct(self, t)
        }
        fn try_take_data_union(&mut self, t: syn::DataUnion) -> Result<(), Self::Error> {
            try_take_data_union(self, t)
        }
        fn try_take_derive_input(
            &mut self,
            t: syn::DeriveInput,
        ) -> Result<(), Self::Error> {
            try_take_derive_input(self, t)
        }
        fn try_take_expr(&mut self, t: syn::Expr) -> Result<(), Self::Error> {
            try_take_expr(self, t)
        }
        fn try_take_expr_variant_array(
            &mut self,
            t: (syn::ExprArray),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_array(self, t)
        }
        fn try_take_expr_variant_assign(
            &mut self,
            t: (syn::ExprAssign),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_assign(self, t)
        }
        fn try_take_expr_variant_async(
            &mut self,
            t: (syn::ExprAsync),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_async(self, t)
        }
        fn try_take_expr_variant_await(
            &mut self,
            t: (syn::ExprAwait),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_await(self, t)
        }
        fn try_take_expr_variant_binary(
            &mut self,
            t: (syn::ExprBinary),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_binary(self, t)
        }
        fn try_take_expr_variant_block(
            &mut self,
            t: (syn::ExprBlock),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_block(self, t)
        }
        fn try_take_expr_variant_break(
            &mut self,
            t: (syn::ExprBreak),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_break(self, t)
        }
        fn try_take_expr_variant_call(
            &mut self,
            t: (syn::ExprCall),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_call(self, t)
        }
        fn try_take_expr_variant_cast(
            &mut self,
            t: (syn::ExprCast),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_cast(self, t)
        }
        fn try_take_expr_variant_closure(
            &mut self,
            t: (syn::ExprClosure),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_closure(self, t)
        }
        fn try_take_expr_variant_const(
            &mut self,
            t: (syn::ExprConst),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_const(self, t)
        }
        fn try_take_expr_variant_continue(
            &mut self,
            t: (syn::ExprContinue),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_continue(self, t)
        }
        fn try_take_expr_variant_field(
            &mut self,
            t: (syn::ExprField),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_field(self, t)
        }
        fn try_take_expr_variant_for_loop(
            &mut self,
            t: (syn::ExprForLoop),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_for_loop(self, t)
        }
        fn try_take_expr_variant_group(
            &mut self,
            t: (syn::ExprGroup),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_group(self, t)
        }
        fn try_take_expr_variant_if(
            &mut self,
            t: (syn::ExprIf),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_if(self, t)
        }
        fn try_take_expr_variant_index(
            &mut self,
            t: (syn::ExprIndex),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_index(self, t)
        }
        fn try_take_expr_variant_infer(
            &mut self,
            t: (syn::ExprInfer),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_infer(self, t)
        }
        fn try_take_expr_variant_let(
            &mut self,
            t: (syn::ExprLet),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_let(self, t)
        }
        fn try_take_expr_variant_lit(
            &mut self,
            t: (syn::ExprLit),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_lit(self, t)
        }
        fn try_take_expr_variant_loop(
            &mut self,
            t: (syn::ExprLoop),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_loop(self, t)
        }
        fn try_take_expr_variant_macro(
            &mut self,
            t: (syn::ExprMacro),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_macro(self, t)
        }
        fn try_take_expr_variant_match(
            &mut self,
            t: (syn::ExprMatch),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_match(self, t)
        }
        fn try_take_expr_variant_method_call(
            &mut self,
            t: (syn::ExprMethodCall),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_method_call(self, t)
        }
        fn try_take_expr_variant_paren(
            &mut self,
            t: (syn::ExprParen),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_paren(self, t)
        }
        fn try_take_expr_variant_path(
            &mut self,
            t: (syn::ExprPath),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_path(self, t)
        }
        fn try_take_expr_variant_range(
            &mut self,
            t: (syn::ExprRange),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_range(self, t)
        }
        fn try_take_expr_variant_reference(
            &mut self,
            t: (syn::ExprReference),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_reference(self, t)
        }
        fn try_take_expr_variant_repeat(
            &mut self,
            t: (syn::ExprRepeat),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_repeat(self, t)
        }
        fn try_take_expr_variant_return(
            &mut self,
            t: (syn::ExprReturn),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_return(self, t)
        }
        fn try_take_expr_variant_struct(
            &mut self,
            t: (syn::ExprStruct),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_struct(self, t)
        }
        fn try_take_expr_variant_try(
            &mut self,
            t: (syn::ExprTry),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_try(self, t)
        }
        fn try_take_expr_variant_try_block(
            &mut self,
            t: (syn::ExprTryBlock),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_try_block(self, t)
        }
        fn try_take_expr_variant_tuple(
            &mut self,
            t: (syn::ExprTuple),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_tuple(self, t)
        }
        fn try_take_expr_variant_unary(
            &mut self,
            t: (syn::ExprUnary),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_unary(self, t)
        }
        fn try_take_expr_variant_unsafe(
            &mut self,
            t: (syn::ExprUnsafe),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_unsafe(self, t)
        }
        fn try_take_expr_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_verbatim(self, t)
        }
        fn try_take_expr_variant_while(
            &mut self,
            t: (syn::ExprWhile),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_while(self, t)
        }
        fn try_take_expr_variant_yield(
            &mut self,
            t: (syn::ExprYield),
        ) -> Result<(), Self::Error> {
            try_take_expr_variant_yield(self, t)
        }
        fn try_take_expr_array(&mut self, t: syn::ExprArray) -> Result<(), Self::Error> {
            try_take_expr_array(self, t)
        }
        fn try_take_expr_assign(
            &mut self,
            t: syn::ExprAssign,
        ) -> Result<(), Self::Error> {
            try_take_expr_assign(self, t)
        }
        fn try_take_expr_async(&mut self, t: syn::ExprAsync) -> Result<(), Self::Error> {
            try_take_expr_async(self, t)
        }
        fn try_take_expr_await(&mut self, t: syn::ExprAwait) -> Result<(), Self::Error> {
            try_take_expr_await(self, t)
        }
        fn try_take_expr_binary(
            &mut self,
            t: syn::ExprBinary,
        ) -> Result<(), Self::Error> {
            try_take_expr_binary(self, t)
        }
        fn try_take_expr_block(&mut self, t: syn::ExprBlock) -> Result<(), Self::Error> {
            try_take_expr_block(self, t)
        }
        fn try_take_expr_break(&mut self, t: syn::ExprBreak) -> Result<(), Self::Error> {
            try_take_expr_break(self, t)
        }
        fn try_take_expr_call(&mut self, t: syn::ExprCall) -> Result<(), Self::Error> {
            try_take_expr_call(self, t)
        }
        fn try_take_expr_cast(&mut self, t: syn::ExprCast) -> Result<(), Self::Error> {
            try_take_expr_cast(self, t)
        }
        fn try_take_expr_closure(
            &mut self,
            t: syn::ExprClosure,
        ) -> Result<(), Self::Error> {
            try_take_expr_closure(self, t)
        }
        fn try_take_expr_const(&mut self, t: syn::ExprConst) -> Result<(), Self::Error> {
            try_take_expr_const(self, t)
        }
        fn try_take_expr_continue(
            &mut self,
            t: syn::ExprContinue,
        ) -> Result<(), Self::Error> {
            try_take_expr_continue(self, t)
        }
        fn try_take_expr_field(&mut self, t: syn::ExprField) -> Result<(), Self::Error> {
            try_take_expr_field(self, t)
        }
        fn try_take_expr_for_loop(
            &mut self,
            t: syn::ExprForLoop,
        ) -> Result<(), Self::Error> {
            try_take_expr_for_loop(self, t)
        }
        fn try_take_expr_group(&mut self, t: syn::ExprGroup) -> Result<(), Self::Error> {
            try_take_expr_group(self, t)
        }
        fn try_take_expr_if(&mut self, t: syn::ExprIf) -> Result<(), Self::Error> {
            try_take_expr_if(self, t)
        }
        fn try_take_expr_index(&mut self, t: syn::ExprIndex) -> Result<(), Self::Error> {
            try_take_expr_index(self, t)
        }
        fn try_take_expr_infer(&mut self, t: syn::ExprInfer) -> Result<(), Self::Error> {
            try_take_expr_infer(self, t)
        }
        fn try_take_expr_let(&mut self, t: syn::ExprLet) -> Result<(), Self::Error> {
            try_take_expr_let(self, t)
        }
        fn try_take_expr_lit(&mut self, t: syn::ExprLit) -> Result<(), Self::Error> {
            try_take_expr_lit(self, t)
        }
        fn try_take_expr_loop(&mut self, t: syn::ExprLoop) -> Result<(), Self::Error> {
            try_take_expr_loop(self, t)
        }
        fn try_take_expr_macro(&mut self, t: syn::ExprMacro) -> Result<(), Self::Error> {
            try_take_expr_macro(self, t)
        }
        fn try_take_expr_match(&mut self, t: syn::ExprMatch) -> Result<(), Self::Error> {
            try_take_expr_match(self, t)
        }
        fn try_take_expr_method_call(
            &mut self,
            t: syn::ExprMethodCall,
        ) -> Result<(), Self::Error> {
            try_take_expr_method_call(self, t)
        }
        fn try_take_expr_paren(&mut self, t: syn::ExprParen) -> Result<(), Self::Error> {
            try_take_expr_paren(self, t)
        }
        fn try_take_expr_path(&mut self, t: syn::ExprPath) -> Result<(), Self::Error> {
            try_take_expr_path(self, t)
        }
        fn try_take_expr_range(&mut self, t: syn::ExprRange) -> Result<(), Self::Error> {
            try_take_expr_range(self, t)
        }
        fn try_take_expr_reference(
            &mut self,
            t: syn::ExprReference,
        ) -> Result<(), Self::Error> {
            try_take_expr_reference(self, t)
        }
        fn try_take_expr_repeat(
            &mut self,
            t: syn::ExprRepeat,
        ) -> Result<(), Self::Error> {
            try_take_expr_repeat(self, t)
        }
        fn try_take_expr_return(
            &mut self,
            t: syn::ExprReturn,
        ) -> Result<(), Self::Error> {
            try_take_expr_return(self, t)
        }
        fn try_take_expr_struct(
            &mut self,
            t: syn::ExprStruct,
        ) -> Result<(), Self::Error> {
            try_take_expr_struct(self, t)
        }
        fn try_take_expr_try(&mut self, t: syn::ExprTry) -> Result<(), Self::Error> {
            try_take_expr_try(self, t)
        }
        fn try_take_expr_try_block(
            &mut self,
            t: syn::ExprTryBlock,
        ) -> Result<(), Self::Error> {
            try_take_expr_try_block(self, t)
        }
        fn try_take_expr_tuple(&mut self, t: syn::ExprTuple) -> Result<(), Self::Error> {
            try_take_expr_tuple(self, t)
        }
        fn try_take_expr_unary(&mut self, t: syn::ExprUnary) -> Result<(), Self::Error> {
            try_take_expr_unary(self, t)
        }
        fn try_take_expr_unsafe(
            &mut self,
            t: syn::ExprUnsafe,
        ) -> Result<(), Self::Error> {
            try_take_expr_unsafe(self, t)
        }
        fn try_take_expr_while(&mut self, t: syn::ExprWhile) -> Result<(), Self::Error> {
            try_take_expr_while(self, t)
        }
        fn try_take_expr_yield(&mut self, t: syn::ExprYield) -> Result<(), Self::Error> {
            try_take_expr_yield(self, t)
        }
        fn try_take_field(&mut self, t: syn::Field) -> Result<(), Self::Error> {
            try_take_field(self, t)
        }
        fn try_take_field_mutability(
            &mut self,
            t: syn::FieldMutability,
        ) -> Result<(), Self::Error> {
            try_take_field_mutability(self, t)
        }
        fn try_take_field_pat(&mut self, t: syn::FieldPat) -> Result<(), Self::Error> {
            try_take_field_pat(self, t)
        }
        fn try_take_field_value(
            &mut self,
            t: syn::FieldValue,
        ) -> Result<(), Self::Error> {
            try_take_field_value(self, t)
        }
        fn try_take_fields(&mut self, t: syn::Fields) -> Result<(), Self::Error> {
            try_take_fields(self, t)
        }
        fn try_take_fields_variant_named(
            &mut self,
            t: (syn::FieldsNamed),
        ) -> Result<(), Self::Error> {
            try_take_fields_variant_named(self, t)
        }
        fn try_take_fields_variant_unnamed(
            &mut self,
            t: (syn::FieldsUnnamed),
        ) -> Result<(), Self::Error> {
            try_take_fields_variant_unnamed(self, t)
        }
        fn try_take_fields_named(
            &mut self,
            t: syn::FieldsNamed,
        ) -> Result<(), Self::Error> {
            try_take_fields_named(self, t)
        }
        fn try_take_fields_unnamed(
            &mut self,
            t: syn::FieldsUnnamed,
        ) -> Result<(), Self::Error> {
            try_take_fields_unnamed(self, t)
        }
        fn try_take_file(&mut self, t: syn::File) -> Result<(), Self::Error> {
            try_take_file(self, t)
        }
        fn try_take_fn_arg(&mut self, t: syn::FnArg) -> Result<(), Self::Error> {
            try_take_fn_arg(self, t)
        }
        fn try_take_fn_arg_variant_receiver(
            &mut self,
            t: (syn::Receiver),
        ) -> Result<(), Self::Error> {
            try_take_fn_arg_variant_receiver(self, t)
        }
        fn try_take_fn_arg_variant_typed(
            &mut self,
            t: (syn::PatType),
        ) -> Result<(), Self::Error> {
            try_take_fn_arg_variant_typed(self, t)
        }
        fn try_take_foreign_item(
            &mut self,
            t: syn::ForeignItem,
        ) -> Result<(), Self::Error> {
            try_take_foreign_item(self, t)
        }
        fn try_take_foreign_item_variant_fn(
            &mut self,
            t: (syn::ForeignItemFn),
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_variant_fn(self, t)
        }
        fn try_take_foreign_item_variant_static(
            &mut self,
            t: (syn::ForeignItemStatic),
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_variant_static(self, t)
        }
        fn try_take_foreign_item_variant_type(
            &mut self,
            t: (syn::ForeignItemType),
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_variant_type(self, t)
        }
        fn try_take_foreign_item_variant_macro(
            &mut self,
            t: (syn::ForeignItemMacro),
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_variant_macro(self, t)
        }
        fn try_take_foreign_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_variant_verbatim(self, t)
        }
        fn try_take_foreign_item_fn(
            &mut self,
            t: syn::ForeignItemFn,
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_fn(self, t)
        }
        fn try_take_foreign_item_macro(
            &mut self,
            t: syn::ForeignItemMacro,
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_macro(self, t)
        }
        fn try_take_foreign_item_static(
            &mut self,
            t: syn::ForeignItemStatic,
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_static(self, t)
        }
        fn try_take_foreign_item_type(
            &mut self,
            t: syn::ForeignItemType,
        ) -> Result<(), Self::Error> {
            try_take_foreign_item_type(self, t)
        }
        fn try_take_generic_argument(
            &mut self,
            t: syn::GenericArgument,
        ) -> Result<(), Self::Error> {
            try_take_generic_argument(self, t)
        }
        fn try_take_generic_argument_variant_lifetime(
            &mut self,
            t: (syn::Lifetime),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_lifetime(self, t)
        }
        fn try_take_generic_argument_variant_type(
            &mut self,
            t: (syn::Type),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_type(self, t)
        }
        fn try_take_generic_argument_variant_const(
            &mut self,
            t: (syn::Expr),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_const(self, t)
        }
        fn try_take_generic_argument_variant_assoc_type(
            &mut self,
            t: (syn::AssocType),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_assoc_type(self, t)
        }
        fn try_take_generic_argument_variant_assoc_const(
            &mut self,
            t: (syn::AssocConst),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_assoc_const(self, t)
        }
        fn try_take_generic_argument_variant_constraint(
            &mut self,
            t: (syn::Constraint),
        ) -> Result<(), Self::Error> {
            try_take_generic_argument_variant_constraint(self, t)
        }
        fn try_take_generic_param(
            &mut self,
            t: syn::GenericParam,
        ) -> Result<(), Self::Error> {
            try_take_generic_param(self, t)
        }
        fn try_take_generic_param_variant_lifetime(
            &mut self,
            t: (syn::LifetimeParam),
        ) -> Result<(), Self::Error> {
            try_take_generic_param_variant_lifetime(self, t)
        }
        fn try_take_generic_param_variant_type(
            &mut self,
            t: (syn::TypeParam),
        ) -> Result<(), Self::Error> {
            try_take_generic_param_variant_type(self, t)
        }
        fn try_take_generic_param_variant_const(
            &mut self,
            t: (syn::ConstParam),
        ) -> Result<(), Self::Error> {
            try_take_generic_param_variant_const(self, t)
        }
        fn try_take_generics(&mut self, t: syn::Generics) -> Result<(), Self::Error> {
            try_take_generics(self, t)
        }
        fn try_take_impl_item(&mut self, t: syn::ImplItem) -> Result<(), Self::Error> {
            try_take_impl_item(self, t)
        }
        fn try_take_impl_item_variant_const(
            &mut self,
            t: (syn::ImplItemConst),
        ) -> Result<(), Self::Error> {
            try_take_impl_item_variant_const(self, t)
        }
        fn try_take_impl_item_variant_fn(
            &mut self,
            t: (syn::ImplItemFn),
        ) -> Result<(), Self::Error> {
            try_take_impl_item_variant_fn(self, t)
        }
        fn try_take_impl_item_variant_type(
            &mut self,
            t: (syn::ImplItemType),
        ) -> Result<(), Self::Error> {
            try_take_impl_item_variant_type(self, t)
        }
        fn try_take_impl_item_variant_macro(
            &mut self,
            t: (syn::ImplItemMacro),
        ) -> Result<(), Self::Error> {
            try_take_impl_item_variant_macro(self, t)
        }
        fn try_take_impl_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_impl_item_variant_verbatim(self, t)
        }
        fn try_take_impl_item_const(
            &mut self,
            t: syn::ImplItemConst,
        ) -> Result<(), Self::Error> {
            try_take_impl_item_const(self, t)
        }
        fn try_take_impl_item_fn(
            &mut self,
            t: syn::ImplItemFn,
        ) -> Result<(), Self::Error> {
            try_take_impl_item_fn(self, t)
        }
        fn try_take_impl_item_macro(
            &mut self,
            t: syn::ImplItemMacro,
        ) -> Result<(), Self::Error> {
            try_take_impl_item_macro(self, t)
        }
        fn try_take_impl_item_type(
            &mut self,
            t: syn::ImplItemType,
        ) -> Result<(), Self::Error> {
            try_take_impl_item_type(self, t)
        }
        fn try_take_impl_restriction(
            &mut self,
            t: syn::ImplRestriction,
        ) -> Result<(), Self::Error> {
            try_take_impl_restriction(self, t)
        }
        fn try_take_index(&mut self, t: syn::Index) -> Result<(), Self::Error> {
            try_take_index(self, t)
        }
        fn try_take_item(&mut self, t: syn::Item) -> Result<(), Self::Error> {
            try_take_item(self, t)
        }
        fn try_take_item_variant_const(
            &mut self,
            t: (syn::ItemConst),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_const(self, t)
        }
        fn try_take_item_variant_enum(
            &mut self,
            t: (syn::ItemEnum),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_enum(self, t)
        }
        fn try_take_item_variant_extern_crate(
            &mut self,
            t: (syn::ItemExternCrate),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_extern_crate(self, t)
        }
        fn try_take_item_variant_fn(
            &mut self,
            t: (syn::ItemFn),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_fn(self, t)
        }
        fn try_take_item_variant_foreign_mod(
            &mut self,
            t: (syn::ItemForeignMod),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_foreign_mod(self, t)
        }
        fn try_take_item_variant_impl(
            &mut self,
            t: (syn::ItemImpl),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_impl(self, t)
        }
        fn try_take_item_variant_macro(
            &mut self,
            t: (syn::ItemMacro),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_macro(self, t)
        }
        fn try_take_item_variant_mod(
            &mut self,
            t: (syn::ItemMod),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_mod(self, t)
        }
        fn try_take_item_variant_static(
            &mut self,
            t: (syn::ItemStatic),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_static(self, t)
        }
        fn try_take_item_variant_struct(
            &mut self,
            t: (syn::ItemStruct),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_struct(self, t)
        }
        fn try_take_item_variant_trait(
            &mut self,
            t: (syn::ItemTrait),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_trait(self, t)
        }
        fn try_take_item_variant_trait_alias(
            &mut self,
            t: (syn::ItemTraitAlias),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_trait_alias(self, t)
        }
        fn try_take_item_variant_type(
            &mut self,
            t: (syn::ItemType),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_type(self, t)
        }
        fn try_take_item_variant_union(
            &mut self,
            t: (syn::ItemUnion),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_union(self, t)
        }
        fn try_take_item_variant_use(
            &mut self,
            t: (syn::ItemUse),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_use(self, t)
        }
        fn try_take_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_item_variant_verbatim(self, t)
        }
        fn try_take_item_const(&mut self, t: syn::ItemConst) -> Result<(), Self::Error> {
            try_take_item_const(self, t)
        }
        fn try_take_item_enum(&mut self, t: syn::ItemEnum) -> Result<(), Self::Error> {
            try_take_item_enum(self, t)
        }
        fn try_take_item_extern_crate(
            &mut self,
            t: syn::ItemExternCrate,
        ) -> Result<(), Self::Error> {
            try_take_item_extern_crate(self, t)
        }
        fn try_take_item_fn(&mut self, t: syn::ItemFn) -> Result<(), Self::Error> {
            try_take_item_fn(self, t)
        }
        fn try_take_item_foreign_mod(
            &mut self,
            t: syn::ItemForeignMod,
        ) -> Result<(), Self::Error> {
            try_take_item_foreign_mod(self, t)
        }
        fn try_take_item_impl(&mut self, t: syn::ItemImpl) -> Result<(), Self::Error> {
            try_take_item_impl(self, t)
        }
        fn try_take_item_macro(&mut self, t: syn::ItemMacro) -> Result<(), Self::Error> {
            try_take_item_macro(self, t)
        }
        fn try_take_item_mod(&mut self, t: syn::ItemMod) -> Result<(), Self::Error> {
            try_take_item_mod(self, t)
        }
        fn try_take_item_static(
            &mut self,
            t: syn::ItemStatic,
        ) -> Result<(), Self::Error> {
            try_take_item_static(self, t)
        }
        fn try_take_item_struct(
            &mut self,
            t: syn::ItemStruct,
        ) -> Result<(), Self::Error> {
            try_take_item_struct(self, t)
        }
        fn try_take_item_trait(&mut self, t: syn::ItemTrait) -> Result<(), Self::Error> {
            try_take_item_trait(self, t)
        }
        fn try_take_item_trait_alias(
            &mut self,
            t: syn::ItemTraitAlias,
        ) -> Result<(), Self::Error> {
            try_take_item_trait_alias(self, t)
        }
        fn try_take_item_type(&mut self, t: syn::ItemType) -> Result<(), Self::Error> {
            try_take_item_type(self, t)
        }
        fn try_take_item_union(&mut self, t: syn::ItemUnion) -> Result<(), Self::Error> {
            try_take_item_union(self, t)
        }
        fn try_take_item_use(&mut self, t: syn::ItemUse) -> Result<(), Self::Error> {
            try_take_item_use(self, t)
        }
        fn try_take_label(&mut self, t: syn::Label) -> Result<(), Self::Error> {
            try_take_label(self, t)
        }
        fn try_take_lifetime(&mut self, t: syn::Lifetime) -> Result<(), Self::Error> {
            try_take_lifetime(self, t)
        }
        fn try_take_lifetime_param(
            &mut self,
            t: syn::LifetimeParam,
        ) -> Result<(), Self::Error> {
            try_take_lifetime_param(self, t)
        }
        fn try_take_lit(&mut self, t: syn::Lit) -> Result<(), Self::Error> {
            try_take_lit(self, t)
        }
        fn try_take_lit_variant_str(
            &mut self,
            t: (syn::LitStr),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_str(self, t)
        }
        fn try_take_lit_variant_byte_str(
            &mut self,
            t: (syn::LitByteStr),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_byte_str(self, t)
        }
        fn try_take_lit_variant_byte(
            &mut self,
            t: (syn::LitByte),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_byte(self, t)
        }
        fn try_take_lit_variant_char(
            &mut self,
            t: (syn::LitChar),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_char(self, t)
        }
        fn try_take_lit_variant_int(
            &mut self,
            t: (syn::LitInt),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_int(self, t)
        }
        fn try_take_lit_variant_float(
            &mut self,
            t: (syn::LitFloat),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_float(self, t)
        }
        fn try_take_lit_variant_bool(
            &mut self,
            t: (syn::LitBool),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_bool(self, t)
        }
        fn try_take_lit_variant_verbatim(
            &mut self,
            t: (proc_macro2::Literal),
        ) -> Result<(), Self::Error> {
            try_take_lit_variant_verbatim(self, t)
        }
        fn try_take_lit_bool(&mut self, t: syn::LitBool) -> Result<(), Self::Error> {
            try_take_lit_bool(self, t)
        }
        fn try_take_lit_byte(&mut self, t: syn::LitByte) -> Result<(), Self::Error> {
            try_take_lit_byte(self, t)
        }
        fn try_take_lit_byte_str(
            &mut self,
            t: syn::LitByteStr,
        ) -> Result<(), Self::Error> {
            try_take_lit_byte_str(self, t)
        }
        fn try_take_lit_char(&mut self, t: syn::LitChar) -> Result<(), Self::Error> {
            try_take_lit_char(self, t)
        }
        fn try_take_lit_float(&mut self, t: syn::LitFloat) -> Result<(), Self::Error> {
            try_take_lit_float(self, t)
        }
        fn try_take_lit_int(&mut self, t: syn::LitInt) -> Result<(), Self::Error> {
            try_take_lit_int(self, t)
        }
        fn try_take_lit_str(&mut self, t: syn::LitStr) -> Result<(), Self::Error> {
            try_take_lit_str(self, t)
        }
        fn try_take_local(&mut self, t: syn::Local) -> Result<(), Self::Error> {
            try_take_local(self, t)
        }
        fn try_take_local_init(&mut self, t: syn::LocalInit) -> Result<(), Self::Error> {
            try_take_local_init(self, t)
        }
        fn try_take_macro(&mut self, t: syn::Macro) -> Result<(), Self::Error> {
            try_take_macro(self, t)
        }
        fn try_take_macro_delimiter(
            &mut self,
            t: syn::MacroDelimiter,
        ) -> Result<(), Self::Error> {
            try_take_macro_delimiter(self, t)
        }
        fn try_take_macro_delimiter_variant_paren(
            &mut self,
            t: (syn::token::Paren),
        ) -> Result<(), Self::Error> {
            try_take_macro_delimiter_variant_paren(self, t)
        }
        fn try_take_macro_delimiter_variant_brace(
            &mut self,
            t: (syn::token::Brace),
        ) -> Result<(), Self::Error> {
            try_take_macro_delimiter_variant_brace(self, t)
        }
        fn try_take_macro_delimiter_variant_bracket(
            &mut self,
            t: (syn::token::Bracket),
        ) -> Result<(), Self::Error> {
            try_take_macro_delimiter_variant_bracket(self, t)
        }
        fn try_take_member(&mut self, t: syn::Member) -> Result<(), Self::Error> {
            try_take_member(self, t)
        }
        fn try_take_member_variant_named(
            &mut self,
            t: (proc_macro2::Ident),
        ) -> Result<(), Self::Error> {
            try_take_member_variant_named(self, t)
        }
        fn try_take_member_variant_unnamed(
            &mut self,
            t: (syn::Index),
        ) -> Result<(), Self::Error> {
            try_take_member_variant_unnamed(self, t)
        }
        fn try_take_meta(&mut self, t: syn::Meta) -> Result<(), Self::Error> {
            try_take_meta(self, t)
        }
        fn try_take_meta_variant_path(
            &mut self,
            t: (syn::Path),
        ) -> Result<(), Self::Error> {
            try_take_meta_variant_path(self, t)
        }
        fn try_take_meta_variant_list(
            &mut self,
            t: (syn::MetaList),
        ) -> Result<(), Self::Error> {
            try_take_meta_variant_list(self, t)
        }
        fn try_take_meta_variant_name_value(
            &mut self,
            t: (syn::MetaNameValue),
        ) -> Result<(), Self::Error> {
            try_take_meta_variant_name_value(self, t)
        }
        fn try_take_meta_list(&mut self, t: syn::MetaList) -> Result<(), Self::Error> {
            try_take_meta_list(self, t)
        }
        fn try_take_meta_name_value(
            &mut self,
            t: syn::MetaNameValue,
        ) -> Result<(), Self::Error> {
            try_take_meta_name_value(self, t)
        }
        fn try_take_parenthesized_generic_arguments(
            &mut self,
            t: syn::ParenthesizedGenericArguments,
        ) -> Result<(), Self::Error> {
            try_take_parenthesized_generic_arguments(self, t)
        }
        fn try_take_pat(&mut self, t: syn::Pat) -> Result<(), Self::Error> {
            try_take_pat(self, t)
        }
        fn try_take_pat_variant_const(
            &mut self,
            t: (syn::ExprConst),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_const(self, t)
        }
        fn try_take_pat_variant_ident(
            &mut self,
            t: (syn::PatIdent),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_ident(self, t)
        }
        fn try_take_pat_variant_lit(
            &mut self,
            t: (syn::ExprLit),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_lit(self, t)
        }
        fn try_take_pat_variant_macro(
            &mut self,
            t: (syn::ExprMacro),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_macro(self, t)
        }
        fn try_take_pat_variant_or(
            &mut self,
            t: (syn::PatOr),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_or(self, t)
        }
        fn try_take_pat_variant_paren(
            &mut self,
            t: (syn::PatParen),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_paren(self, t)
        }
        fn try_take_pat_variant_path(
            &mut self,
            t: (syn::ExprPath),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_path(self, t)
        }
        fn try_take_pat_variant_range(
            &mut self,
            t: (syn::ExprRange),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_range(self, t)
        }
        fn try_take_pat_variant_reference(
            &mut self,
            t: (syn::PatReference),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_reference(self, t)
        }
        fn try_take_pat_variant_rest(
            &mut self,
            t: (syn::PatRest),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_rest(self, t)
        }
        fn try_take_pat_variant_slice(
            &mut self,
            t: (syn::PatSlice),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_slice(self, t)
        }
        fn try_take_pat_variant_struct(
            &mut self,
            t: (syn::PatStruct),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_struct(self, t)
        }
        fn try_take_pat_variant_tuple(
            &mut self,
            t: (syn::PatTuple),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_tuple(self, t)
        }
        fn try_take_pat_variant_tuple_struct(
            &mut self,
            t: (syn::PatTupleStruct),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_tuple_struct(self, t)
        }
        fn try_take_pat_variant_type(
            &mut self,
            t: (syn::PatType),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_type(self, t)
        }
        fn try_take_pat_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_verbatim(self, t)
        }
        fn try_take_pat_variant_wild(
            &mut self,
            t: (syn::PatWild),
        ) -> Result<(), Self::Error> {
            try_take_pat_variant_wild(self, t)
        }
        fn try_take_pat_ident(&mut self, t: syn::PatIdent) -> Result<(), Self::Error> {
            try_take_pat_ident(self, t)
        }
        fn try_take_pat_or(&mut self, t: syn::PatOr) -> Result<(), Self::Error> {
            try_take_pat_or(self, t)
        }
        fn try_take_pat_paren(&mut self, t: syn::PatParen) -> Result<(), Self::Error> {
            try_take_pat_paren(self, t)
        }
        fn try_take_pat_reference(
            &mut self,
            t: syn::PatReference,
        ) -> Result<(), Self::Error> {
            try_take_pat_reference(self, t)
        }
        fn try_take_pat_rest(&mut self, t: syn::PatRest) -> Result<(), Self::Error> {
            try_take_pat_rest(self, t)
        }
        fn try_take_pat_slice(&mut self, t: syn::PatSlice) -> Result<(), Self::Error> {
            try_take_pat_slice(self, t)
        }
        fn try_take_pat_struct(&mut self, t: syn::PatStruct) -> Result<(), Self::Error> {
            try_take_pat_struct(self, t)
        }
        fn try_take_pat_tuple(&mut self, t: syn::PatTuple) -> Result<(), Self::Error> {
            try_take_pat_tuple(self, t)
        }
        fn try_take_pat_tuple_struct(
            &mut self,
            t: syn::PatTupleStruct,
        ) -> Result<(), Self::Error> {
            try_take_pat_tuple_struct(self, t)
        }
        fn try_take_pat_type(&mut self, t: syn::PatType) -> Result<(), Self::Error> {
            try_take_pat_type(self, t)
        }
        fn try_take_pat_wild(&mut self, t: syn::PatWild) -> Result<(), Self::Error> {
            try_take_pat_wild(self, t)
        }
        fn try_take_path(&mut self, t: syn::Path) -> Result<(), Self::Error> {
            try_take_path(self, t)
        }
        fn try_take_path_arguments(
            &mut self,
            t: syn::PathArguments,
        ) -> Result<(), Self::Error> {
            try_take_path_arguments(self, t)
        }
        fn try_take_path_arguments_variant_angle_bracketed(
            &mut self,
            t: (syn::AngleBracketedGenericArguments),
        ) -> Result<(), Self::Error> {
            try_take_path_arguments_variant_angle_bracketed(self, t)
        }
        fn try_take_path_arguments_variant_parenthesized(
            &mut self,
            t: (syn::ParenthesizedGenericArguments),
        ) -> Result<(), Self::Error> {
            try_take_path_arguments_variant_parenthesized(self, t)
        }
        fn try_take_path_segment(
            &mut self,
            t: syn::PathSegment,
        ) -> Result<(), Self::Error> {
            try_take_path_segment(self, t)
        }
        fn try_take_predicate_lifetime(
            &mut self,
            t: syn::PredicateLifetime,
        ) -> Result<(), Self::Error> {
            try_take_predicate_lifetime(self, t)
        }
        fn try_take_predicate_type(
            &mut self,
            t: syn::PredicateType,
        ) -> Result<(), Self::Error> {
            try_take_predicate_type(self, t)
        }
        fn try_take_q_self(&mut self, t: syn::QSelf) -> Result<(), Self::Error> {
            try_take_q_self(self, t)
        }
        fn try_take_range_limits(
            &mut self,
            t: syn::RangeLimits,
        ) -> Result<(), Self::Error> {
            try_take_range_limits(self, t)
        }
        fn try_take_range_limits_variant_half_open(
            &mut self,
            t: (syn::token::DotDot),
        ) -> Result<(), Self::Error> {
            try_take_range_limits_variant_half_open(self, t)
        }
        fn try_take_range_limits_variant_closed(
            &mut self,
            t: (syn::token::DotDotEq),
        ) -> Result<(), Self::Error> {
            try_take_range_limits_variant_closed(self, t)
        }
        fn try_take_receiver(&mut self, t: syn::Receiver) -> Result<(), Self::Error> {
            try_take_receiver(self, t)
        }
        fn try_take_return_type(
            &mut self,
            t: syn::ReturnType,
        ) -> Result<(), Self::Error> {
            try_take_return_type(self, t)
        }
        fn try_take_return_type_variant_type(
            &mut self,
            t: (syn::token::RArrow, Box<syn::Type>),
        ) -> Result<(), Self::Error> {
            try_take_return_type_variant_type(self, t)
        }
        fn try_take_signature(&mut self, t: syn::Signature) -> Result<(), Self::Error> {
            try_take_signature(self, t)
        }
        fn try_take_static_mutability(
            &mut self,
            t: syn::StaticMutability,
        ) -> Result<(), Self::Error> {
            try_take_static_mutability(self, t)
        }
        fn try_take_static_mutability_variant_mut(
            &mut self,
            t: (syn::token::Mut),
        ) -> Result<(), Self::Error> {
            try_take_static_mutability_variant_mut(self, t)
        }
        fn try_take_stmt(&mut self, t: syn::Stmt) -> Result<(), Self::Error> {
            try_take_stmt(self, t)
        }
        fn try_take_stmt_variant_local(
            &mut self,
            t: (syn::Local),
        ) -> Result<(), Self::Error> {
            try_take_stmt_variant_local(self, t)
        }
        fn try_take_stmt_variant_item(
            &mut self,
            t: (syn::Item),
        ) -> Result<(), Self::Error> {
            try_take_stmt_variant_item(self, t)
        }
        fn try_take_stmt_variant_expr(
            &mut self,
            t: (syn::Expr, Option<syn::token::Semi>),
        ) -> Result<(), Self::Error> {
            try_take_stmt_variant_expr(self, t)
        }
        fn try_take_stmt_variant_macro(
            &mut self,
            t: (syn::StmtMacro),
        ) -> Result<(), Self::Error> {
            try_take_stmt_variant_macro(self, t)
        }
        fn try_take_stmt_macro(&mut self, t: syn::StmtMacro) -> Result<(), Self::Error> {
            try_take_stmt_macro(self, t)
        }
        fn try_take_trait_bound(
            &mut self,
            t: syn::TraitBound,
        ) -> Result<(), Self::Error> {
            try_take_trait_bound(self, t)
        }
        fn try_take_trait_bound_modifier(
            &mut self,
            t: syn::TraitBoundModifier,
        ) -> Result<(), Self::Error> {
            try_take_trait_bound_modifier(self, t)
        }
        fn try_take_trait_bound_modifier_variant_maybe(
            &mut self,
            t: (syn::token::Question),
        ) -> Result<(), Self::Error> {
            try_take_trait_bound_modifier_variant_maybe(self, t)
        }
        fn try_take_trait_item(&mut self, t: syn::TraitItem) -> Result<(), Self::Error> {
            try_take_trait_item(self, t)
        }
        fn try_take_trait_item_variant_const(
            &mut self,
            t: (syn::TraitItemConst),
        ) -> Result<(), Self::Error> {
            try_take_trait_item_variant_const(self, t)
        }
        fn try_take_trait_item_variant_fn(
            &mut self,
            t: (syn::TraitItemFn),
        ) -> Result<(), Self::Error> {
            try_take_trait_item_variant_fn(self, t)
        }
        fn try_take_trait_item_variant_type(
            &mut self,
            t: (syn::TraitItemType),
        ) -> Result<(), Self::Error> {
            try_take_trait_item_variant_type(self, t)
        }
        fn try_take_trait_item_variant_macro(
            &mut self,
            t: (syn::TraitItemMacro),
        ) -> Result<(), Self::Error> {
            try_take_trait_item_variant_macro(self, t)
        }
        fn try_take_trait_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_trait_item_variant_verbatim(self, t)
        }
        fn try_take_trait_item_const(
            &mut self,
            t: syn::TraitItemConst,
        ) -> Result<(), Self::Error> {
            try_take_trait_item_const(self, t)
        }
        fn try_take_trait_item_fn(
            &mut self,
            t: syn::TraitItemFn,
        ) -> Result<(), Self::Error> {
            try_take_trait_item_fn(self, t)
        }
        fn try_take_trait_item_macro(
            &mut self,
            t: syn::TraitItemMacro,
        ) -> Result<(), Self::Error> {
            try_take_trait_item_macro(self, t)
        }
        fn try_take_trait_item_type(
            &mut self,
            t: syn::TraitItemType,
        ) -> Result<(), Self::Error> {
            try_take_trait_item_type(self, t)
        }
        fn try_take_type(&mut self, t: syn::Type) -> Result<(), Self::Error> {
            try_take_type(self, t)
        }
        fn try_take_type_variant_array(
            &mut self,
            t: (syn::TypeArray),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_array(self, t)
        }
        fn try_take_type_variant_bare_fn(
            &mut self,
            t: (syn::TypeBareFn),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_bare_fn(self, t)
        }
        fn try_take_type_variant_group(
            &mut self,
            t: (syn::TypeGroup),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_group(self, t)
        }
        fn try_take_type_variant_impl_trait(
            &mut self,
            t: (syn::TypeImplTrait),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_impl_trait(self, t)
        }
        fn try_take_type_variant_infer(
            &mut self,
            t: (syn::TypeInfer),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_infer(self, t)
        }
        fn try_take_type_variant_macro(
            &mut self,
            t: (syn::TypeMacro),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_macro(self, t)
        }
        fn try_take_type_variant_never(
            &mut self,
            t: (syn::TypeNever),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_never(self, t)
        }
        fn try_take_type_variant_paren(
            &mut self,
            t: (syn::TypeParen),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_paren(self, t)
        }
        fn try_take_type_variant_path(
            &mut self,
            t: (syn::TypePath),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_path(self, t)
        }
        fn try_take_type_variant_ptr(
            &mut self,
            t: (syn::TypePtr),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_ptr(self, t)
        }
        fn try_take_type_variant_reference(
            &mut self,
            t: (syn::TypeReference),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_reference(self, t)
        }
        fn try_take_type_variant_slice(
            &mut self,
            t: (syn::TypeSlice),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_slice(self, t)
        }
        fn try_take_type_variant_trait_object(
            &mut self,
            t: (syn::TypeTraitObject),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_trait_object(self, t)
        }
        fn try_take_type_variant_tuple(
            &mut self,
            t: (syn::TypeTuple),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_tuple(self, t)
        }
        fn try_take_type_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_type_variant_verbatim(self, t)
        }
        fn try_take_type_array(&mut self, t: syn::TypeArray) -> Result<(), Self::Error> {
            try_take_type_array(self, t)
        }
        fn try_take_type_bare_fn(
            &mut self,
            t: syn::TypeBareFn,
        ) -> Result<(), Self::Error> {
            try_take_type_bare_fn(self, t)
        }
        fn try_take_type_group(&mut self, t: syn::TypeGroup) -> Result<(), Self::Error> {
            try_take_type_group(self, t)
        }
        fn try_take_type_impl_trait(
            &mut self,
            t: syn::TypeImplTrait,
        ) -> Result<(), Self::Error> {
            try_take_type_impl_trait(self, t)
        }
        fn try_take_type_infer(&mut self, t: syn::TypeInfer) -> Result<(), Self::Error> {
            try_take_type_infer(self, t)
        }
        fn try_take_type_macro(&mut self, t: syn::TypeMacro) -> Result<(), Self::Error> {
            try_take_type_macro(self, t)
        }
        fn try_take_type_never(&mut self, t: syn::TypeNever) -> Result<(), Self::Error> {
            try_take_type_never(self, t)
        }
        fn try_take_type_param(&mut self, t: syn::TypeParam) -> Result<(), Self::Error> {
            try_take_type_param(self, t)
        }
        fn try_take_type_param_bound(
            &mut self,
            t: syn::TypeParamBound,
        ) -> Result<(), Self::Error> {
            try_take_type_param_bound(self, t)
        }
        fn try_take_type_param_bound_variant_trait(
            &mut self,
            t: (syn::TraitBound),
        ) -> Result<(), Self::Error> {
            try_take_type_param_bound_variant_trait(self, t)
        }
        fn try_take_type_param_bound_variant_lifetime(
            &mut self,
            t: (syn::Lifetime),
        ) -> Result<(), Self::Error> {
            try_take_type_param_bound_variant_lifetime(self, t)
        }
        fn try_take_type_param_bound_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<(), Self::Error> {
            try_take_type_param_bound_variant_verbatim(self, t)
        }
        fn try_take_type_paren(&mut self, t: syn::TypeParen) -> Result<(), Self::Error> {
            try_take_type_paren(self, t)
        }
        fn try_take_type_path(&mut self, t: syn::TypePath) -> Result<(), Self::Error> {
            try_take_type_path(self, t)
        }
        fn try_take_type_ptr(&mut self, t: syn::TypePtr) -> Result<(), Self::Error> {
            try_take_type_ptr(self, t)
        }
        fn try_take_type_reference(
            &mut self,
            t: syn::TypeReference,
        ) -> Result<(), Self::Error> {
            try_take_type_reference(self, t)
        }
        fn try_take_type_slice(&mut self, t: syn::TypeSlice) -> Result<(), Self::Error> {
            try_take_type_slice(self, t)
        }
        fn try_take_type_trait_object(
            &mut self,
            t: syn::TypeTraitObject,
        ) -> Result<(), Self::Error> {
            try_take_type_trait_object(self, t)
        }
        fn try_take_type_tuple(&mut self, t: syn::TypeTuple) -> Result<(), Self::Error> {
            try_take_type_tuple(self, t)
        }
        fn try_take_un_op(&mut self, t: syn::UnOp) -> Result<(), Self::Error> {
            try_take_un_op(self, t)
        }
        fn try_take_un_op_variant_deref(
            &mut self,
            t: (syn::token::Star),
        ) -> Result<(), Self::Error> {
            try_take_un_op_variant_deref(self, t)
        }
        fn try_take_un_op_variant_not(
            &mut self,
            t: (syn::token::Not),
        ) -> Result<(), Self::Error> {
            try_take_un_op_variant_not(self, t)
        }
        fn try_take_un_op_variant_neg(
            &mut self,
            t: (syn::token::Minus),
        ) -> Result<(), Self::Error> {
            try_take_un_op_variant_neg(self, t)
        }
        fn try_take_use_glob(&mut self, t: syn::UseGlob) -> Result<(), Self::Error> {
            try_take_use_glob(self, t)
        }
        fn try_take_use_group(&mut self, t: syn::UseGroup) -> Result<(), Self::Error> {
            try_take_use_group(self, t)
        }
        fn try_take_use_name(&mut self, t: syn::UseName) -> Result<(), Self::Error> {
            try_take_use_name(self, t)
        }
        fn try_take_use_path(&mut self, t: syn::UsePath) -> Result<(), Self::Error> {
            try_take_use_path(self, t)
        }
        fn try_take_use_rename(&mut self, t: syn::UseRename) -> Result<(), Self::Error> {
            try_take_use_rename(self, t)
        }
        fn try_take_use_tree(&mut self, t: syn::UseTree) -> Result<(), Self::Error> {
            try_take_use_tree(self, t)
        }
        fn try_take_use_tree_variant_path(
            &mut self,
            t: (syn::UsePath),
        ) -> Result<(), Self::Error> {
            try_take_use_tree_variant_path(self, t)
        }
        fn try_take_use_tree_variant_name(
            &mut self,
            t: (syn::UseName),
        ) -> Result<(), Self::Error> {
            try_take_use_tree_variant_name(self, t)
        }
        fn try_take_use_tree_variant_rename(
            &mut self,
            t: (syn::UseRename),
        ) -> Result<(), Self::Error> {
            try_take_use_tree_variant_rename(self, t)
        }
        fn try_take_use_tree_variant_glob(
            &mut self,
            t: (syn::UseGlob),
        ) -> Result<(), Self::Error> {
            try_take_use_tree_variant_glob(self, t)
        }
        fn try_take_use_tree_variant_group(
            &mut self,
            t: (syn::UseGroup),
        ) -> Result<(), Self::Error> {
            try_take_use_tree_variant_group(self, t)
        }
        fn try_take_variadic(&mut self, t: syn::Variadic) -> Result<(), Self::Error> {
            try_take_variadic(self, t)
        }
        fn try_take_variant(&mut self, t: syn::Variant) -> Result<(), Self::Error> {
            try_take_variant(self, t)
        }
        fn try_take_vis_restricted(
            &mut self,
            t: syn::VisRestricted,
        ) -> Result<(), Self::Error> {
            try_take_vis_restricted(self, t)
        }
        fn try_take_visibility(
            &mut self,
            t: syn::Visibility,
        ) -> Result<(), Self::Error> {
            try_take_visibility(self, t)
        }
        fn try_take_visibility_variant_public(
            &mut self,
            t: (syn::token::Pub),
        ) -> Result<(), Self::Error> {
            try_take_visibility_variant_public(self, t)
        }
        fn try_take_visibility_variant_restricted(
            &mut self,
            t: (syn::VisRestricted),
        ) -> Result<(), Self::Error> {
            try_take_visibility_variant_restricted(self, t)
        }
        fn try_take_where_clause(
            &mut self,
            t: syn::WhereClause,
        ) -> Result<(), Self::Error> {
            try_take_where_clause(self, t)
        }
        fn try_take_where_predicate(
            &mut self,
            t: syn::WherePredicate,
        ) -> Result<(), Self::Error> {
            try_take_where_predicate(self, t)
        }
        fn try_take_where_predicate_variant_lifetime(
            &mut self,
            t: (syn::PredicateLifetime),
        ) -> Result<(), Self::Error> {
            try_take_where_predicate_variant_lifetime(self, t)
        }
        fn try_take_where_predicate_variant_type(
            &mut self,
            t: (syn::PredicateType),
        ) -> Result<(), Self::Error> {
            try_take_where_predicate_variant_type(self, t)
        }
    }
    pub fn try_take_abi<F>(f: &mut F, t: syn::Abi) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.name {
            Some(o) => {
                f.try_take_lit_str(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_angle_bracketed_generic_arguments<F>(
        f: &mut F,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.colon2_token {
            Some(o) => {}
            None => {}
        }
        for v in t.args {
            f.try_take_generic_argument(v)?;
        }
        Ok(())
    }
    pub fn try_take_arm<F>(f: &mut F, t: syn::Arm) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_pat(t.pat)?;
        match t.guard {
            Some(o) => {
                f.try_take_expr(*o.1)?;
            }
            None => {}
        }
        f.try_take_expr(*t.body)?;
        match t.comma {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_assoc_const<F>(
        f: &mut F,
        t: syn::AssocConst,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.generics {
            Some(o) => {
                f.try_take_angle_bracketed_generic_arguments(o)?;
            }
            None => {}
        }
        f.try_take_expr(t.value)?;
        Ok(())
    }
    pub fn try_take_assoc_type<F>(
        f: &mut F,
        t: syn::AssocType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.generics {
            Some(o) => {
                f.try_take_angle_bracketed_generic_arguments(o)?;
            }
            None => {}
        }
        f.try_take_type(t.ty)?;
        Ok(())
    }
    pub fn try_take_attr_style<F>(
        f: &mut F,
        t: syn::AttrStyle,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::AttrStyle::Inner(tmp0) => f.try_take_attr_style_variant_inner((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_attr_style_variant_inner<F>(
        f: &mut F,
        t: (syn::token::Not),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_attribute<F>(
        f: &mut F,
        t: syn::Attribute,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_attr_style(t.style)?;
        f.try_take_meta(t.meta)?;
        Ok(())
    }
    pub fn try_take_bare_fn_arg<F>(
        f: &mut F,
        t: syn::BareFnArg,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.name {
            Some(o) => {}
            None => {}
        }
        f.try_take_type(t.ty)?;
        Ok(())
    }
    pub fn try_take_bare_variadic<F>(
        f: &mut F,
        t: syn::BareVariadic,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.name {
            Some(o) => {}
            None => {}
        }
        match t.comma {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_bin_op<F>(
        f: &mut F,
        t: syn::BinOp,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::BinOp::Add(tmp0) => f.try_take_bin_op_variant_add((tmp0))?,
            syn::BinOp::Sub(tmp0) => f.try_take_bin_op_variant_sub((tmp0))?,
            syn::BinOp::Mul(tmp0) => f.try_take_bin_op_variant_mul((tmp0))?,
            syn::BinOp::Div(tmp0) => f.try_take_bin_op_variant_div((tmp0))?,
            syn::BinOp::Rem(tmp0) => f.try_take_bin_op_variant_rem((tmp0))?,
            syn::BinOp::And(tmp0) => f.try_take_bin_op_variant_and((tmp0))?,
            syn::BinOp::Or(tmp0) => f.try_take_bin_op_variant_or((tmp0))?,
            syn::BinOp::BitXor(tmp0) => f.try_take_bin_op_variant_bit_xor((tmp0))?,
            syn::BinOp::BitAnd(tmp0) => f.try_take_bin_op_variant_bit_and((tmp0))?,
            syn::BinOp::BitOr(tmp0) => f.try_take_bin_op_variant_bit_or((tmp0))?,
            syn::BinOp::Shl(tmp0) => f.try_take_bin_op_variant_shl((tmp0))?,
            syn::BinOp::Shr(tmp0) => f.try_take_bin_op_variant_shr((tmp0))?,
            syn::BinOp::Eq(tmp0) => f.try_take_bin_op_variant_eq((tmp0))?,
            syn::BinOp::Lt(tmp0) => f.try_take_bin_op_variant_lt((tmp0))?,
            syn::BinOp::Le(tmp0) => f.try_take_bin_op_variant_le((tmp0))?,
            syn::BinOp::Ne(tmp0) => f.try_take_bin_op_variant_ne((tmp0))?,
            syn::BinOp::Ge(tmp0) => f.try_take_bin_op_variant_ge((tmp0))?,
            syn::BinOp::Gt(tmp0) => f.try_take_bin_op_variant_gt((tmp0))?,
            syn::BinOp::AddAssign(tmp0) => f.try_take_bin_op_variant_add_assign((tmp0))?,
            syn::BinOp::SubAssign(tmp0) => f.try_take_bin_op_variant_sub_assign((tmp0))?,
            syn::BinOp::MulAssign(tmp0) => f.try_take_bin_op_variant_mul_assign((tmp0))?,
            syn::BinOp::DivAssign(tmp0) => f.try_take_bin_op_variant_div_assign((tmp0))?,
            syn::BinOp::RemAssign(tmp0) => f.try_take_bin_op_variant_rem_assign((tmp0))?,
            syn::BinOp::BitXorAssign(tmp0) => {
                f.try_take_bin_op_variant_bit_xor_assign((tmp0))?
            }
            syn::BinOp::BitAndAssign(tmp0) => {
                f.try_take_bin_op_variant_bit_and_assign((tmp0))?
            }
            syn::BinOp::BitOrAssign(tmp0) => {
                f.try_take_bin_op_variant_bit_or_assign((tmp0))?
            }
            syn::BinOp::ShlAssign(tmp0) => f.try_take_bin_op_variant_shl_assign((tmp0))?,
            syn::BinOp::ShrAssign(tmp0) => f.try_take_bin_op_variant_shr_assign((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_bin_op_variant_add<F>(
        f: &mut F,
        t: (syn::token::Plus),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_sub<F>(
        f: &mut F,
        t: (syn::token::Minus),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_mul<F>(
        f: &mut F,
        t: (syn::token::Star),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_div<F>(
        f: &mut F,
        t: (syn::token::Slash),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_rem<F>(
        f: &mut F,
        t: (syn::token::Percent),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_and<F>(
        f: &mut F,
        t: (syn::token::AndAnd),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_or<F>(
        f: &mut F,
        t: (syn::token::OrOr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_xor<F>(
        f: &mut F,
        t: (syn::token::Caret),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_and<F>(
        f: &mut F,
        t: (syn::token::And),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_or<F>(
        f: &mut F,
        t: (syn::token::Or),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_shl<F>(
        f: &mut F,
        t: (syn::token::Shl),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_shr<F>(
        f: &mut F,
        t: (syn::token::Shr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_eq<F>(
        f: &mut F,
        t: (syn::token::EqEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_lt<F>(
        f: &mut F,
        t: (syn::token::Lt),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_le<F>(
        f: &mut F,
        t: (syn::token::Le),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_ne<F>(
        f: &mut F,
        t: (syn::token::Ne),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_ge<F>(
        f: &mut F,
        t: (syn::token::Ge),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_gt<F>(
        f: &mut F,
        t: (syn::token::Gt),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_add_assign<F>(
        f: &mut F,
        t: (syn::token::PlusEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_sub_assign<F>(
        f: &mut F,
        t: (syn::token::MinusEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_mul_assign<F>(
        f: &mut F,
        t: (syn::token::StarEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_div_assign<F>(
        f: &mut F,
        t: (syn::token::SlashEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_rem_assign<F>(
        f: &mut F,
        t: (syn::token::PercentEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_xor_assign<F>(
        f: &mut F,
        t: (syn::token::CaretEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_and_assign<F>(
        f: &mut F,
        t: (syn::token::AndEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_bit_or_assign<F>(
        f: &mut F,
        t: (syn::token::OrEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_shl_assign<F>(
        f: &mut F,
        t: (syn::token::ShlEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_bin_op_variant_shr_assign<F>(
        f: &mut F,
        t: (syn::token::ShrEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_block<F>(
        f: &mut F,
        t: syn::Block,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.stmts {
            f.try_take_stmt(v)?;
        }
        Ok(())
    }
    pub fn try_take_bound_lifetimes<F>(
        f: &mut F,
        t: syn::BoundLifetimes,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.lifetimes {
            f.try_take_generic_param(v)?;
        }
        Ok(())
    }
    pub fn try_take_const_param<F>(
        f: &mut F,
        t: syn::ConstParam,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_type(t.ty)?;
        match t.eq_token {
            Some(o) => {}
            None => {}
        }
        match t.default {
            Some(o) => {
                f.try_take_expr(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_constraint<F>(
        f: &mut F,
        t: syn::Constraint,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.generics {
            Some(o) => {
                f.try_take_angle_bracketed_generic_arguments(o)?;
            }
            None => {}
        }
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        Ok(())
    }
    pub fn try_take_data<F>(f: &mut F, t: syn::Data) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Data::Struct(tmp0) => f.try_take_data_variant_struct((tmp0))?,
            syn::Data::Enum(tmp0) => f.try_take_data_variant_enum((tmp0))?,
            syn::Data::Union(tmp0) => f.try_take_data_variant_union((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_data_variant_struct<F>(
        f: &mut F,
        t: (syn::DataStruct),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_data_struct(t)?;
        Ok(())
    }
    pub fn try_take_data_variant_enum<F>(
        f: &mut F,
        t: (syn::DataEnum),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_data_enum(t)?;
        Ok(())
    }
    pub fn try_take_data_variant_union<F>(
        f: &mut F,
        t: (syn::DataUnion),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_data_union(t)?;
        Ok(())
    }
    pub fn try_take_data_enum<F>(
        f: &mut F,
        t: syn::DataEnum,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.variants {
            f.try_take_variant(v)?;
        }
        Ok(())
    }
    pub fn try_take_data_struct<F>(
        f: &mut F,
        t: syn::DataStruct,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_fields(t.fields)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_data_union<F>(
        f: &mut F,
        t: syn::DataUnion,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_fields_named(t.fields)?;
        Ok(())
    }
    pub fn try_take_derive_input<F>(
        f: &mut F,
        t: syn::DeriveInput,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        f.try_take_data(t.data)?;
        Ok(())
    }
    pub fn try_take_expr<F>(f: &mut F, t: syn::Expr) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Expr::Array(tmp0) => f.try_take_expr_variant_array((tmp0))?,
            syn::Expr::Assign(tmp0) => f.try_take_expr_variant_assign((tmp0))?,
            syn::Expr::Async(tmp0) => f.try_take_expr_variant_async((tmp0))?,
            syn::Expr::Await(tmp0) => f.try_take_expr_variant_await((tmp0))?,
            syn::Expr::Binary(tmp0) => f.try_take_expr_variant_binary((tmp0))?,
            syn::Expr::Block(tmp0) => f.try_take_expr_variant_block((tmp0))?,
            syn::Expr::Break(tmp0) => f.try_take_expr_variant_break((tmp0))?,
            syn::Expr::Call(tmp0) => f.try_take_expr_variant_call((tmp0))?,
            syn::Expr::Cast(tmp0) => f.try_take_expr_variant_cast((tmp0))?,
            syn::Expr::Closure(tmp0) => f.try_take_expr_variant_closure((tmp0))?,
            syn::Expr::Const(tmp0) => f.try_take_expr_variant_const((tmp0))?,
            syn::Expr::Continue(tmp0) => f.try_take_expr_variant_continue((tmp0))?,
            syn::Expr::Field(tmp0) => f.try_take_expr_variant_field((tmp0))?,
            syn::Expr::ForLoop(tmp0) => f.try_take_expr_variant_for_loop((tmp0))?,
            syn::Expr::Group(tmp0) => f.try_take_expr_variant_group((tmp0))?,
            syn::Expr::If(tmp0) => f.try_take_expr_variant_if((tmp0))?,
            syn::Expr::Index(tmp0) => f.try_take_expr_variant_index((tmp0))?,
            syn::Expr::Infer(tmp0) => f.try_take_expr_variant_infer((tmp0))?,
            syn::Expr::Let(tmp0) => f.try_take_expr_variant_let((tmp0))?,
            syn::Expr::Lit(tmp0) => f.try_take_expr_variant_lit((tmp0))?,
            syn::Expr::Loop(tmp0) => f.try_take_expr_variant_loop((tmp0))?,
            syn::Expr::Macro(tmp0) => f.try_take_expr_variant_macro((tmp0))?,
            syn::Expr::Match(tmp0) => f.try_take_expr_variant_match((tmp0))?,
            syn::Expr::MethodCall(tmp0) => f.try_take_expr_variant_method_call((tmp0))?,
            syn::Expr::Paren(tmp0) => f.try_take_expr_variant_paren((tmp0))?,
            syn::Expr::Path(tmp0) => f.try_take_expr_variant_path((tmp0))?,
            syn::Expr::Range(tmp0) => f.try_take_expr_variant_range((tmp0))?,
            syn::Expr::Reference(tmp0) => f.try_take_expr_variant_reference((tmp0))?,
            syn::Expr::Repeat(tmp0) => f.try_take_expr_variant_repeat((tmp0))?,
            syn::Expr::Return(tmp0) => f.try_take_expr_variant_return((tmp0))?,
            syn::Expr::Struct(tmp0) => f.try_take_expr_variant_struct((tmp0))?,
            syn::Expr::Try(tmp0) => f.try_take_expr_variant_try((tmp0))?,
            syn::Expr::TryBlock(tmp0) => f.try_take_expr_variant_try_block((tmp0))?,
            syn::Expr::Tuple(tmp0) => f.try_take_expr_variant_tuple((tmp0))?,
            syn::Expr::Unary(tmp0) => f.try_take_expr_variant_unary((tmp0))?,
            syn::Expr::Unsafe(tmp0) => f.try_take_expr_variant_unsafe((tmp0))?,
            syn::Expr::Verbatim(tmp0) => f.try_take_expr_variant_verbatim((tmp0))?,
            syn::Expr::While(tmp0) => f.try_take_expr_variant_while((tmp0))?,
            syn::Expr::Yield(tmp0) => f.try_take_expr_variant_yield((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_expr_variant_array<F>(
        f: &mut F,
        t: (syn::ExprArray),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_array(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_assign<F>(
        f: &mut F,
        t: (syn::ExprAssign),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_assign(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_async<F>(
        f: &mut F,
        t: (syn::ExprAsync),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_async(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_await<F>(
        f: &mut F,
        t: (syn::ExprAwait),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_await(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_binary<F>(
        f: &mut F,
        t: (syn::ExprBinary),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_binary(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_block<F>(
        f: &mut F,
        t: (syn::ExprBlock),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_block(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_break<F>(
        f: &mut F,
        t: (syn::ExprBreak),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_break(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_call<F>(
        f: &mut F,
        t: (syn::ExprCall),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_call(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_cast<F>(
        f: &mut F,
        t: (syn::ExprCast),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_cast(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_closure<F>(
        f: &mut F,
        t: (syn::ExprClosure),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_closure(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_const<F>(
        f: &mut F,
        t: (syn::ExprConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_const(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_continue<F>(
        f: &mut F,
        t: (syn::ExprContinue),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_continue(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_field<F>(
        f: &mut F,
        t: (syn::ExprField),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_field(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_for_loop<F>(
        f: &mut F,
        t: (syn::ExprForLoop),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_for_loop(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_group<F>(
        f: &mut F,
        t: (syn::ExprGroup),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_group(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_if<F>(
        f: &mut F,
        t: (syn::ExprIf),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_if(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_index<F>(
        f: &mut F,
        t: (syn::ExprIndex),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_index(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_infer<F>(
        f: &mut F,
        t: (syn::ExprInfer),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_infer(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_let<F>(
        f: &mut F,
        t: (syn::ExprLet),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_let(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_lit<F>(
        f: &mut F,
        t: (syn::ExprLit),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_lit(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_loop<F>(
        f: &mut F,
        t: (syn::ExprLoop),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_loop(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_macro<F>(
        f: &mut F,
        t: (syn::ExprMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_macro(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_match<F>(
        f: &mut F,
        t: (syn::ExprMatch),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_match(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_method_call<F>(
        f: &mut F,
        t: (syn::ExprMethodCall),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_method_call(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_paren<F>(
        f: &mut F,
        t: (syn::ExprParen),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_paren(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_path<F>(
        f: &mut F,
        t: (syn::ExprPath),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_path(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_range<F>(
        f: &mut F,
        t: (syn::ExprRange),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_range(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_reference<F>(
        f: &mut F,
        t: (syn::ExprReference),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_reference(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_repeat<F>(
        f: &mut F,
        t: (syn::ExprRepeat),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_repeat(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_return<F>(
        f: &mut F,
        t: (syn::ExprReturn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_return(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_struct<F>(
        f: &mut F,
        t: (syn::ExprStruct),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_struct(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_try<F>(
        f: &mut F,
        t: (syn::ExprTry),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_try(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_try_block<F>(
        f: &mut F,
        t: (syn::ExprTryBlock),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_try_block(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_tuple<F>(
        f: &mut F,
        t: (syn::ExprTuple),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_tuple(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_unary<F>(
        f: &mut F,
        t: (syn::ExprUnary),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_unary(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_unsafe<F>(
        f: &mut F,
        t: (syn::ExprUnsafe),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_unsafe(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_expr_variant_while<F>(
        f: &mut F,
        t: (syn::ExprWhile),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_while(t)?;
        Ok(())
    }
    pub fn try_take_expr_variant_yield<F>(
        f: &mut F,
        t: (syn::ExprYield),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_yield(t)?;
        Ok(())
    }
    pub fn try_take_expr_array<F>(
        f: &mut F,
        t: syn::ExprArray,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        for v in t.elems {
            f.try_take_expr(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_assign<F>(
        f: &mut F,
        t: syn::ExprAssign,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.left)?;
        f.try_take_expr(*t.right)?;
        Ok(())
    }
    pub fn try_take_expr_async<F>(
        f: &mut F,
        t: syn::ExprAsync,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.capture {
            Some(o) => {}
            None => {}
        }
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_expr_await<F>(
        f: &mut F,
        t: syn::ExprAwait,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.base)?;
        Ok(())
    }
    pub fn try_take_expr_binary<F>(
        f: &mut F,
        t: syn::ExprBinary,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.left)?;
        f.try_take_bin_op(t.op)?;
        f.try_take_expr(*t.right)?;
        Ok(())
    }
    pub fn try_take_expr_block<F>(
        f: &mut F,
        t: syn::ExprBlock,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_label(o)?;
            }
            None => {}
        }
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_expr_break<F>(
        f: &mut F,
        t: syn::ExprBreak,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_lifetime(o)?;
            }
            None => {}
        }
        match t.expr {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_call<F>(
        f: &mut F,
        t: syn::ExprCall,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.func)?;
        for v in t.args {
            f.try_take_expr(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_cast<F>(
        f: &mut F,
        t: syn::ExprCast,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        f.try_take_type(*t.ty)?;
        Ok(())
    }
    pub fn try_take_expr_closure<F>(
        f: &mut F,
        t: syn::ExprClosure,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.lifetimes {
            Some(o) => {
                f.try_take_bound_lifetimes(o)?;
            }
            None => {}
        }
        match t.constness {
            Some(o) => {}
            None => {}
        }
        match t.movability {
            Some(o) => {}
            None => {}
        }
        match t.asyncness {
            Some(o) => {}
            None => {}
        }
        match t.capture {
            Some(o) => {}
            None => {}
        }
        for v in t.inputs {
            f.try_take_pat(v)?;
        }
        f.try_take_return_type(t.output)?;
        f.try_take_expr(*t.body)?;
        Ok(())
    }
    pub fn try_take_expr_const<F>(
        f: &mut F,
        t: syn::ExprConst,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_expr_continue<F>(
        f: &mut F,
        t: syn::ExprContinue,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_lifetime(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_field<F>(
        f: &mut F,
        t: syn::ExprField,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.base)?;
        f.try_take_member(t.member)?;
        Ok(())
    }
    pub fn try_take_expr_for_loop<F>(
        f: &mut F,
        t: syn::ExprForLoop,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_label(o)?;
            }
            None => {}
        }
        f.try_take_pat(*t.pat)?;
        f.try_take_expr(*t.expr)?;
        f.try_take_block(t.body)?;
        Ok(())
    }
    pub fn try_take_expr_group<F>(
        f: &mut F,
        t: syn::ExprGroup,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_if<F>(
        f: &mut F,
        t: syn::ExprIf,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.cond)?;
        f.try_take_block(t.then_branch)?;
        match t.else_branch {
            Some(o) => {
                f.try_take_expr(*o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_index<F>(
        f: &mut F,
        t: syn::ExprIndex,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        f.try_take_expr(*t.index)?;
        Ok(())
    }
    pub fn try_take_expr_infer<F>(
        f: &mut F,
        t: syn::ExprInfer,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_let<F>(
        f: &mut F,
        t: syn::ExprLet,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_pat(*t.pat)?;
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_lit<F>(
        f: &mut F,
        t: syn::ExprLit,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_lit(t.lit)?;
        Ok(())
    }
    pub fn try_take_expr_loop<F>(
        f: &mut F,
        t: syn::ExprLoop,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_label(o)?;
            }
            None => {}
        }
        f.try_take_block(t.body)?;
        Ok(())
    }
    pub fn try_take_expr_macro<F>(
        f: &mut F,
        t: syn::ExprMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_macro(t.mac)?;
        Ok(())
    }
    pub fn try_take_expr_match<F>(
        f: &mut F,
        t: syn::ExprMatch,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        for v in t.arms {
            f.try_take_arm(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_method_call<F>(
        f: &mut F,
        t: syn::ExprMethodCall,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.receiver)?;
        match t.turbofish {
            Some(o) => {
                f.try_take_angle_bracketed_generic_arguments(o)?;
            }
            None => {}
        }
        for v in t.args {
            f.try_take_expr(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_paren<F>(
        f: &mut F,
        t: syn::ExprParen,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_path<F>(
        f: &mut F,
        t: syn::ExprPath,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.qself {
            Some(o) => {
                f.try_take_q_self(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        Ok(())
    }
    pub fn try_take_expr_range<F>(
        f: &mut F,
        t: syn::ExprRange,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.start {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        f.try_take_range_limits(t.limits)?;
        match t.end {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_reference<F>(
        f: &mut F,
        t: syn::ExprReference,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_repeat<F>(
        f: &mut F,
        t: syn::ExprRepeat,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        f.try_take_expr(*t.len)?;
        Ok(())
    }
    pub fn try_take_expr_return<F>(
        f: &mut F,
        t: syn::ExprReturn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.expr {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_struct<F>(
        f: &mut F,
        t: syn::ExprStruct,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.qself {
            Some(o) => {
                f.try_take_q_self(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        for v in t.fields {
            f.try_take_field_value(v)?;
        }
        match t.dot2_token {
            Some(o) => {}
            None => {}
        }
        match t.rest {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_expr_try<F>(
        f: &mut F,
        t: syn::ExprTry,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_try_block<F>(
        f: &mut F,
        t: syn::ExprTryBlock,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_expr_tuple<F>(
        f: &mut F,
        t: syn::ExprTuple,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        for v in t.elems {
            f.try_take_expr(v)?;
        }
        Ok(())
    }
    pub fn try_take_expr_unary<F>(
        f: &mut F,
        t: syn::ExprUnary,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_un_op(t.op)?;
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_expr_unsafe<F>(
        f: &mut F,
        t: syn::ExprUnsafe,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_expr_while<F>(
        f: &mut F,
        t: syn::ExprWhile,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.label {
            Some(o) => {
                f.try_take_label(o)?;
            }
            None => {}
        }
        f.try_take_expr(*t.cond)?;
        f.try_take_block(t.body)?;
        Ok(())
    }
    pub fn try_take_expr_yield<F>(
        f: &mut F,
        t: syn::ExprYield,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.expr {
            Some(o) => {
                f.try_take_expr(*o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_field<F>(
        f: &mut F,
        t: syn::Field,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_field_mutability(t.mutability)?;
        match t.ident {
            Some(o) => {}
            None => {}
        }
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_type(t.ty)?;
        Ok(())
    }
    pub fn try_take_field_mutability<F>(
        f: &mut F,
        t: syn::FieldMutability,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_field_pat<F>(
        f: &mut F,
        t: syn::FieldPat,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_member(t.member)?;
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_pat(*t.pat)?;
        Ok(())
    }
    pub fn try_take_field_value<F>(
        f: &mut F,
        t: syn::FieldValue,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_member(t.member)?;
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_expr(t.expr)?;
        Ok(())
    }
    pub fn try_take_fields<F>(
        f: &mut F,
        t: syn::Fields,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Fields::Named(tmp0) => f.try_take_fields_variant_named((tmp0))?,
            syn::Fields::Unnamed(tmp0) => f.try_take_fields_variant_unnamed((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_fields_variant_named<F>(
        f: &mut F,
        t: (syn::FieldsNamed),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_fields_named(t)?;
        Ok(())
    }
    pub fn try_take_fields_variant_unnamed<F>(
        f: &mut F,
        t: (syn::FieldsUnnamed),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_fields_unnamed(t)?;
        Ok(())
    }
    pub fn try_take_fields_named<F>(
        f: &mut F,
        t: syn::FieldsNamed,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.named {
            f.try_take_field(v)?;
        }
        Ok(())
    }
    pub fn try_take_fields_unnamed<F>(
        f: &mut F,
        t: syn::FieldsUnnamed,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.unnamed {
            f.try_take_field(v)?;
        }
        Ok(())
    }
    pub fn try_take_file<F>(f: &mut F, t: syn::File) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.shebang {
            Some(o) => {}
            None => {}
        }
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        for v in t.items {
            f.try_take_item(v)?;
        }
        Ok(())
    }
    pub fn try_take_fn_arg<F>(
        f: &mut F,
        t: syn::FnArg,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::FnArg::Receiver(tmp0) => f.try_take_fn_arg_variant_receiver((tmp0))?,
            syn::FnArg::Typed(tmp0) => f.try_take_fn_arg_variant_typed((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_fn_arg_variant_receiver<F>(
        f: &mut F,
        t: (syn::Receiver),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_receiver(t)?;
        Ok(())
    }
    pub fn try_take_fn_arg_variant_typed<F>(
        f: &mut F,
        t: (syn::PatType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_type(t)?;
        Ok(())
    }
    pub fn try_take_foreign_item<F>(
        f: &mut F,
        t: syn::ForeignItem,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::ForeignItem::Fn(tmp0) => f.try_take_foreign_item_variant_fn((tmp0))?,
            syn::ForeignItem::Static(tmp0) => {
                f.try_take_foreign_item_variant_static((tmp0))?
            }
            syn::ForeignItem::Type(tmp0) => f.try_take_foreign_item_variant_type((tmp0))?,
            syn::ForeignItem::Macro(tmp0) => {
                f.try_take_foreign_item_variant_macro((tmp0))?
            }
            syn::ForeignItem::Verbatim(tmp0) => {
                f.try_take_foreign_item_variant_verbatim((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_foreign_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ForeignItemFn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_foreign_item_fn(t)?;
        Ok(())
    }
    pub fn try_take_foreign_item_variant_static<F>(
        f: &mut F,
        t: (syn::ForeignItemStatic),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_foreign_item_static(t)?;
        Ok(())
    }
    pub fn try_take_foreign_item_variant_type<F>(
        f: &mut F,
        t: (syn::ForeignItemType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_foreign_item_type(t)?;
        Ok(())
    }
    pub fn try_take_foreign_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ForeignItemMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_foreign_item_macro(t)?;
        Ok(())
    }
    pub fn try_take_foreign_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_foreign_item_fn<F>(
        f: &mut F,
        t: syn::ForeignItemFn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_signature(t.sig)?;
        Ok(())
    }
    pub fn try_take_foreign_item_macro<F>(
        f: &mut F,
        t: syn::ForeignItemMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_macro(t.mac)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_foreign_item_static<F>(
        f: &mut F,
        t: syn::ForeignItemStatic,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_static_mutability(t.mutability)?;
        f.try_take_type(*t.ty)?;
        Ok(())
    }
    pub fn try_take_foreign_item_type<F>(
        f: &mut F,
        t: syn::ForeignItemType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        Ok(())
    }
    pub fn try_take_generic_argument<F>(
        f: &mut F,
        t: syn::GenericArgument,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::GenericArgument::Lifetime(tmp0) => {
                f.try_take_generic_argument_variant_lifetime((tmp0))?
            }
            syn::GenericArgument::Type(tmp0) => {
                f.try_take_generic_argument_variant_type((tmp0))?
            }
            syn::GenericArgument::Const(tmp0) => {
                f.try_take_generic_argument_variant_const((tmp0))?
            }
            syn::GenericArgument::AssocType(tmp0) => {
                f.try_take_generic_argument_variant_assoc_type((tmp0))?
            }
            syn::GenericArgument::AssocConst(tmp0) => {
                f.try_take_generic_argument_variant_assoc_const((tmp0))?
            }
            syn::GenericArgument::Constraint(tmp0) => {
                f.try_take_generic_argument_variant_constraint((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_generic_argument_variant_lifetime<F>(
        f: &mut F,
        t: (syn::Lifetime),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lifetime(t)?;
        Ok(())
    }
    pub fn try_take_generic_argument_variant_type<F>(
        f: &mut F,
        t: (syn::Type),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(t)?;
        Ok(())
    }
    pub fn try_take_generic_argument_variant_const<F>(
        f: &mut F,
        t: (syn::Expr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr(t)?;
        Ok(())
    }
    pub fn try_take_generic_argument_variant_assoc_type<F>(
        f: &mut F,
        t: (syn::AssocType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_assoc_type(t)?;
        Ok(())
    }
    pub fn try_take_generic_argument_variant_assoc_const<F>(
        f: &mut F,
        t: (syn::AssocConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_assoc_const(t)?;
        Ok(())
    }
    pub fn try_take_generic_argument_variant_constraint<F>(
        f: &mut F,
        t: (syn::Constraint),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_constraint(t)?;
        Ok(())
    }
    pub fn try_take_generic_param<F>(
        f: &mut F,
        t: syn::GenericParam,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::GenericParam::Lifetime(tmp0) => {
                f.try_take_generic_param_variant_lifetime((tmp0))?
            }
            syn::GenericParam::Type(tmp0) => {
                f.try_take_generic_param_variant_type((tmp0))?
            }
            syn::GenericParam::Const(tmp0) => {
                f.try_take_generic_param_variant_const((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_generic_param_variant_lifetime<F>(
        f: &mut F,
        t: (syn::LifetimeParam),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lifetime_param(t)?;
        Ok(())
    }
    pub fn try_take_generic_param_variant_type<F>(
        f: &mut F,
        t: (syn::TypeParam),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_param(t)?;
        Ok(())
    }
    pub fn try_take_generic_param_variant_const<F>(
        f: &mut F,
        t: (syn::ConstParam),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_const_param(t)?;
        Ok(())
    }
    pub fn try_take_generics<F>(
        f: &mut F,
        t: syn::Generics,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.lt_token {
            Some(o) => {}
            None => {}
        }
        for v in t.params {
            f.try_take_generic_param(v)?;
        }
        match t.gt_token {
            Some(o) => {}
            None => {}
        }
        match t.where_clause {
            Some(o) => {
                f.try_take_where_clause(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_impl_item<F>(
        f: &mut F,
        t: syn::ImplItem,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::ImplItem::Const(tmp0) => f.try_take_impl_item_variant_const((tmp0))?,
            syn::ImplItem::Fn(tmp0) => f.try_take_impl_item_variant_fn((tmp0))?,
            syn::ImplItem::Type(tmp0) => f.try_take_impl_item_variant_type((tmp0))?,
            syn::ImplItem::Macro(tmp0) => f.try_take_impl_item_variant_macro((tmp0))?,
            syn::ImplItem::Verbatim(tmp0) => {
                f.try_take_impl_item_variant_verbatim((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_impl_item_variant_const<F>(
        f: &mut F,
        t: (syn::ImplItemConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_impl_item_const(t)?;
        Ok(())
    }
    pub fn try_take_impl_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ImplItemFn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_impl_item_fn(t)?;
        Ok(())
    }
    pub fn try_take_impl_item_variant_type<F>(
        f: &mut F,
        t: (syn::ImplItemType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_impl_item_type(t)?;
        Ok(())
    }
    pub fn try_take_impl_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ImplItemMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_impl_item_macro(t)?;
        Ok(())
    }
    pub fn try_take_impl_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_impl_item_const<F>(
        f: &mut F,
        t: syn::ImplItemConst,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.defaultness {
            Some(o) => {}
            None => {}
        }
        f.try_take_generics(t.generics)?;
        f.try_take_type(t.ty)?;
        f.try_take_expr(t.expr)?;
        Ok(())
    }
    pub fn try_take_impl_item_fn<F>(
        f: &mut F,
        t: syn::ImplItemFn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.defaultness {
            Some(o) => {}
            None => {}
        }
        f.try_take_signature(t.sig)?;
        f.try_take_block(t.block)?;
        Ok(())
    }
    pub fn try_take_impl_item_macro<F>(
        f: &mut F,
        t: syn::ImplItemMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_macro(t.mac)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_impl_item_type<F>(
        f: &mut F,
        t: syn::ImplItemType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.defaultness {
            Some(o) => {}
            None => {}
        }
        f.try_take_generics(t.generics)?;
        f.try_take_type(t.ty)?;
        Ok(())
    }
    pub fn try_take_impl_restriction<F>(
        f: &mut F,
        t: syn::ImplRestriction,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_index<F>(
        f: &mut F,
        t: syn::Index,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_item<F>(f: &mut F, t: syn::Item) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Item::Const(tmp0) => f.try_take_item_variant_const((tmp0))?,
            syn::Item::Enum(tmp0) => f.try_take_item_variant_enum((tmp0))?,
            syn::Item::ExternCrate(tmp0) => f.try_take_item_variant_extern_crate((tmp0))?,
            syn::Item::Fn(tmp0) => f.try_take_item_variant_fn((tmp0))?,
            syn::Item::ForeignMod(tmp0) => f.try_take_item_variant_foreign_mod((tmp0))?,
            syn::Item::Impl(tmp0) => f.try_take_item_variant_impl((tmp0))?,
            syn::Item::Macro(tmp0) => f.try_take_item_variant_macro((tmp0))?,
            syn::Item::Mod(tmp0) => f.try_take_item_variant_mod((tmp0))?,
            syn::Item::Static(tmp0) => f.try_take_item_variant_static((tmp0))?,
            syn::Item::Struct(tmp0) => f.try_take_item_variant_struct((tmp0))?,
            syn::Item::Trait(tmp0) => f.try_take_item_variant_trait((tmp0))?,
            syn::Item::TraitAlias(tmp0) => f.try_take_item_variant_trait_alias((tmp0))?,
            syn::Item::Type(tmp0) => f.try_take_item_variant_type((tmp0))?,
            syn::Item::Union(tmp0) => f.try_take_item_variant_union((tmp0))?,
            syn::Item::Use(tmp0) => f.try_take_item_variant_use((tmp0))?,
            syn::Item::Verbatim(tmp0) => f.try_take_item_variant_verbatim((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_item_variant_const<F>(
        f: &mut F,
        t: (syn::ItemConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_const(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_enum<F>(
        f: &mut F,
        t: (syn::ItemEnum),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_enum(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_extern_crate<F>(
        f: &mut F,
        t: (syn::ItemExternCrate),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_extern_crate(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ItemFn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_fn(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_foreign_mod<F>(
        f: &mut F,
        t: (syn::ItemForeignMod),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_foreign_mod(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_impl<F>(
        f: &mut F,
        t: (syn::ItemImpl),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_impl(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ItemMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_macro(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_mod<F>(
        f: &mut F,
        t: (syn::ItemMod),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_mod(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_static<F>(
        f: &mut F,
        t: (syn::ItemStatic),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_static(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_struct<F>(
        f: &mut F,
        t: (syn::ItemStruct),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_struct(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_trait<F>(
        f: &mut F,
        t: (syn::ItemTrait),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_trait(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_trait_alias<F>(
        f: &mut F,
        t: (syn::ItemTraitAlias),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_trait_alias(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_type<F>(
        f: &mut F,
        t: (syn::ItemType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_type(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_union<F>(
        f: &mut F,
        t: (syn::ItemUnion),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_union(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_use<F>(
        f: &mut F,
        t: (syn::ItemUse),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item_use(t)?;
        Ok(())
    }
    pub fn try_take_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_item_const<F>(
        f: &mut F,
        t: syn::ItemConst,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        f.try_take_type(*t.ty)?;
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_item_enum<F>(
        f: &mut F,
        t: syn::ItemEnum,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        for v in t.variants {
            f.try_take_variant(v)?;
        }
        Ok(())
    }
    pub fn try_take_item_extern_crate<F>(
        f: &mut F,
        t: syn::ItemExternCrate,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.rename {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_item_fn<F>(
        f: &mut F,
        t: syn::ItemFn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_signature(t.sig)?;
        f.try_take_block(*t.block)?;
        Ok(())
    }
    pub fn try_take_item_foreign_mod<F>(
        f: &mut F,
        t: syn::ItemForeignMod,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        f.try_take_abi(t.abi)?;
        for v in t.items {
            f.try_take_foreign_item(v)?;
        }
        Ok(())
    }
    pub fn try_take_item_impl<F>(
        f: &mut F,
        t: syn::ItemImpl,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.defaultness {
            Some(o) => {}
            None => {}
        }
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        f.try_take_generics(t.generics)?;
        match t.trait_ {
            Some(o) => {
                match o.0 {
                    Some(o) => {}
                    None => {}
                }
                f.try_take_path(o.1)?;
            }
            None => {}
        }
        f.try_take_type(*t.self_ty)?;
        for v in t.items {
            f.try_take_impl_item(v)?;
        }
        Ok(())
    }
    pub fn try_take_item_macro<F>(
        f: &mut F,
        t: syn::ItemMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.ident {
            Some(o) => {}
            None => {}
        }
        f.try_take_macro(t.mac)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_item_mod<F>(
        f: &mut F,
        t: syn::ItemMod,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        match t.content {
            Some(o) => {
                for v in o.1 {
                    f.try_take_item(v)?;
                }
            }
            None => {}
        }
        match t.semi {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_item_static<F>(
        f: &mut F,
        t: syn::ItemStatic,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_static_mutability(t.mutability)?;
        f.try_take_type(*t.ty)?;
        f.try_take_expr(*t.expr)?;
        Ok(())
    }
    pub fn try_take_item_struct<F>(
        f: &mut F,
        t: syn::ItemStruct,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        f.try_take_fields(t.fields)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_item_trait<F>(
        f: &mut F,
        t: syn::ItemTrait,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        match t.auto_token {
            Some(o) => {}
            None => {}
        }
        match t.restriction {
            Some(o) => {
                f.try_take_impl_restriction(o)?;
            }
            None => {}
        }
        f.try_take_generics(t.generics)?;
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        for v in t.supertraits {
            f.try_take_type_param_bound(v)?;
        }
        for v in t.items {
            f.try_take_trait_item(v)?;
        }
        Ok(())
    }
    pub fn try_take_item_trait_alias<F>(
        f: &mut F,
        t: syn::ItemTraitAlias,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        Ok(())
    }
    pub fn try_take_item_type<F>(
        f: &mut F,
        t: syn::ItemType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        f.try_take_type(*t.ty)?;
        Ok(())
    }
    pub fn try_take_item_union<F>(
        f: &mut F,
        t: syn::ItemUnion,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        f.try_take_generics(t.generics)?;
        f.try_take_fields_named(t.fields)?;
        Ok(())
    }
    pub fn try_take_item_use<F>(
        f: &mut F,
        t: syn::ItemUse,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_visibility(t.vis)?;
        match t.leading_colon {
            Some(o) => {}
            None => {}
        }
        f.try_take_use_tree(t.tree)?;
        Ok(())
    }
    pub fn try_take_label<F>(
        f: &mut F,
        t: syn::Label,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lifetime(t.name)?;
        Ok(())
    }
    pub fn try_take_lifetime<F>(
        f: &mut F,
        t: syn::Lifetime,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lifetime_param<F>(
        f: &mut F,
        t: syn::LifetimeParam,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_lifetime(t.lifetime)?;
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        for v in t.bounds {
            f.try_take_lifetime(v)?;
        }
        Ok(())
    }
    pub fn try_take_lit<F>(f: &mut F, t: syn::Lit) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Lit::Str(tmp0) => f.try_take_lit_variant_str((tmp0))?,
            syn::Lit::ByteStr(tmp0) => f.try_take_lit_variant_byte_str((tmp0))?,
            syn::Lit::Byte(tmp0) => f.try_take_lit_variant_byte((tmp0))?,
            syn::Lit::Char(tmp0) => f.try_take_lit_variant_char((tmp0))?,
            syn::Lit::Int(tmp0) => f.try_take_lit_variant_int((tmp0))?,
            syn::Lit::Float(tmp0) => f.try_take_lit_variant_float((tmp0))?,
            syn::Lit::Bool(tmp0) => f.try_take_lit_variant_bool((tmp0))?,
            syn::Lit::Verbatim(tmp0) => f.try_take_lit_variant_verbatim((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_lit_variant_str<F>(
        f: &mut F,
        t: (syn::LitStr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_str(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_byte_str<F>(
        f: &mut F,
        t: (syn::LitByteStr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_byte_str(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_byte<F>(
        f: &mut F,
        t: (syn::LitByte),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_byte(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_char<F>(
        f: &mut F,
        t: (syn::LitChar),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_char(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_int<F>(
        f: &mut F,
        t: (syn::LitInt),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_int(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_float<F>(
        f: &mut F,
        t: (syn::LitFloat),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_float(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_bool<F>(
        f: &mut F,
        t: (syn::LitBool),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lit_bool(t)?;
        Ok(())
    }
    pub fn try_take_lit_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::Literal),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_bool<F>(
        f: &mut F,
        t: syn::LitBool,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_byte<F>(
        f: &mut F,
        t: syn::LitByte,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_byte_str<F>(
        f: &mut F,
        t: syn::LitByteStr,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_char<F>(
        f: &mut F,
        t: syn::LitChar,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_float<F>(
        f: &mut F,
        t: syn::LitFloat,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_int<F>(
        f: &mut F,
        t: syn::LitInt,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_lit_str<F>(
        f: &mut F,
        t: syn::LitStr,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_local<F>(
        f: &mut F,
        t: syn::Local,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_pat(t.pat)?;
        match t.init {
            Some(o) => {
                f.try_take_local_init(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_local_init<F>(
        f: &mut F,
        t: syn::LocalInit,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr(*t.expr)?;
        match t.diverge {
            Some(o) => {
                f.try_take_expr(*o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_macro<F>(
        f: &mut F,
        t: syn::Macro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_path(t.path)?;
        f.try_take_macro_delimiter(t.delimiter)?;
        Ok(())
    }
    pub fn try_take_macro_delimiter<F>(
        f: &mut F,
        t: syn::MacroDelimiter,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::MacroDelimiter::Paren(tmp0) => {
                f.try_take_macro_delimiter_variant_paren((tmp0))?
            }
            syn::MacroDelimiter::Brace(tmp0) => {
                f.try_take_macro_delimiter_variant_brace((tmp0))?
            }
            syn::MacroDelimiter::Bracket(tmp0) => {
                f.try_take_macro_delimiter_variant_bracket((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_macro_delimiter_variant_paren<F>(
        f: &mut F,
        t: (syn::token::Paren),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_macro_delimiter_variant_brace<F>(
        f: &mut F,
        t: (syn::token::Brace),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_macro_delimiter_variant_bracket<F>(
        f: &mut F,
        t: (syn::token::Bracket),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_member<F>(
        f: &mut F,
        t: syn::Member,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Member::Named(tmp0) => f.try_take_member_variant_named((tmp0))?,
            syn::Member::Unnamed(tmp0) => f.try_take_member_variant_unnamed((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_member_variant_named<F>(
        f: &mut F,
        t: (proc_macro2::Ident),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_member_variant_unnamed<F>(
        f: &mut F,
        t: (syn::Index),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_index(t)?;
        Ok(())
    }
    pub fn try_take_meta<F>(f: &mut F, t: syn::Meta) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Meta::Path(tmp0) => f.try_take_meta_variant_path((tmp0))?,
            syn::Meta::List(tmp0) => f.try_take_meta_variant_list((tmp0))?,
            syn::Meta::NameValue(tmp0) => f.try_take_meta_variant_name_value((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_meta_variant_path<F>(
        f: &mut F,
        t: (syn::Path),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_path(t)?;
        Ok(())
    }
    pub fn try_take_meta_variant_list<F>(
        f: &mut F,
        t: (syn::MetaList),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_meta_list(t)?;
        Ok(())
    }
    pub fn try_take_meta_variant_name_value<F>(
        f: &mut F,
        t: (syn::MetaNameValue),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_meta_name_value(t)?;
        Ok(())
    }
    pub fn try_take_meta_list<F>(
        f: &mut F,
        t: syn::MetaList,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_path(t.path)?;
        f.try_take_macro_delimiter(t.delimiter)?;
        Ok(())
    }
    pub fn try_take_meta_name_value<F>(
        f: &mut F,
        t: syn::MetaNameValue,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_path(t.path)?;
        f.try_take_expr(t.value)?;
        Ok(())
    }
    pub fn try_take_parenthesized_generic_arguments<F>(
        f: &mut F,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.inputs {
            f.try_take_type(v)?;
        }
        f.try_take_return_type(t.output)?;
        Ok(())
    }
    pub fn try_take_pat<F>(f: &mut F, t: syn::Pat) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Pat::Const(tmp0) => f.try_take_pat_variant_const((tmp0))?,
            syn::Pat::Ident(tmp0) => f.try_take_pat_variant_ident((tmp0))?,
            syn::Pat::Lit(tmp0) => f.try_take_pat_variant_lit((tmp0))?,
            syn::Pat::Macro(tmp0) => f.try_take_pat_variant_macro((tmp0))?,
            syn::Pat::Or(tmp0) => f.try_take_pat_variant_or((tmp0))?,
            syn::Pat::Paren(tmp0) => f.try_take_pat_variant_paren((tmp0))?,
            syn::Pat::Path(tmp0) => f.try_take_pat_variant_path((tmp0))?,
            syn::Pat::Range(tmp0) => f.try_take_pat_variant_range((tmp0))?,
            syn::Pat::Reference(tmp0) => f.try_take_pat_variant_reference((tmp0))?,
            syn::Pat::Rest(tmp0) => f.try_take_pat_variant_rest((tmp0))?,
            syn::Pat::Slice(tmp0) => f.try_take_pat_variant_slice((tmp0))?,
            syn::Pat::Struct(tmp0) => f.try_take_pat_variant_struct((tmp0))?,
            syn::Pat::Tuple(tmp0) => f.try_take_pat_variant_tuple((tmp0))?,
            syn::Pat::TupleStruct(tmp0) => f.try_take_pat_variant_tuple_struct((tmp0))?,
            syn::Pat::Type(tmp0) => f.try_take_pat_variant_type((tmp0))?,
            syn::Pat::Verbatim(tmp0) => f.try_take_pat_variant_verbatim((tmp0))?,
            syn::Pat::Wild(tmp0) => f.try_take_pat_variant_wild((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_pat_variant_const<F>(
        f: &mut F,
        t: (syn::ExprConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_const(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_ident<F>(
        f: &mut F,
        t: (syn::PatIdent),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_ident(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_lit<F>(
        f: &mut F,
        t: (syn::ExprLit),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_lit(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_macro<F>(
        f: &mut F,
        t: (syn::ExprMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_macro(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_or<F>(
        f: &mut F,
        t: (syn::PatOr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_or(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_paren<F>(
        f: &mut F,
        t: (syn::PatParen),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_paren(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_path<F>(
        f: &mut F,
        t: (syn::ExprPath),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_path(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_range<F>(
        f: &mut F,
        t: (syn::ExprRange),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr_range(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_reference<F>(
        f: &mut F,
        t: (syn::PatReference),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_reference(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_rest<F>(
        f: &mut F,
        t: (syn::PatRest),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_rest(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_slice<F>(
        f: &mut F,
        t: (syn::PatSlice),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_slice(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_struct<F>(
        f: &mut F,
        t: (syn::PatStruct),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_struct(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_tuple<F>(
        f: &mut F,
        t: (syn::PatTuple),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_tuple(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_tuple_struct<F>(
        f: &mut F,
        t: (syn::PatTupleStruct),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_tuple_struct(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_type<F>(
        f: &mut F,
        t: (syn::PatType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_type(t)?;
        Ok(())
    }
    pub fn try_take_pat_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_pat_variant_wild<F>(
        f: &mut F,
        t: (syn::PatWild),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_pat_wild(t)?;
        Ok(())
    }
    pub fn try_take_pat_ident<F>(
        f: &mut F,
        t: syn::PatIdent,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.by_ref {
            Some(o) => {}
            None => {}
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        match t.subpat {
            Some(o) => {
                f.try_take_pat(*o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_pat_or<F>(
        f: &mut F,
        t: syn::PatOr,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.leading_vert {
            Some(o) => {}
            None => {}
        }
        for v in t.cases {
            f.try_take_pat(v)?;
        }
        Ok(())
    }
    pub fn try_take_pat_paren<F>(
        f: &mut F,
        t: syn::PatParen,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_pat(*t.pat)?;
        Ok(())
    }
    pub fn try_take_pat_reference<F>(
        f: &mut F,
        t: syn::PatReference,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        f.try_take_pat(*t.pat)?;
        Ok(())
    }
    pub fn try_take_pat_rest<F>(
        f: &mut F,
        t: syn::PatRest,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        Ok(())
    }
    pub fn try_take_pat_slice<F>(
        f: &mut F,
        t: syn::PatSlice,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        for v in t.elems {
            f.try_take_pat(v)?;
        }
        Ok(())
    }
    pub fn try_take_pat_struct<F>(
        f: &mut F,
        t: syn::PatStruct,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.qself {
            Some(o) => {
                f.try_take_q_self(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        for v in t.fields {
            f.try_take_field_pat(v)?;
        }
        match t.rest {
            Some(o) => {
                f.try_take_pat_rest(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_pat_tuple<F>(
        f: &mut F,
        t: syn::PatTuple,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        for v in t.elems {
            f.try_take_pat(v)?;
        }
        Ok(())
    }
    pub fn try_take_pat_tuple_struct<F>(
        f: &mut F,
        t: syn::PatTupleStruct,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.qself {
            Some(o) => {
                f.try_take_q_self(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        for v in t.elems {
            f.try_take_pat(v)?;
        }
        Ok(())
    }
    pub fn try_take_pat_type<F>(
        f: &mut F,
        t: syn::PatType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_pat(*t.pat)?;
        f.try_take_type(*t.ty)?;
        Ok(())
    }
    pub fn try_take_pat_wild<F>(
        f: &mut F,
        t: syn::PatWild,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        Ok(())
    }
    pub fn try_take_path<F>(f: &mut F, t: syn::Path) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.leading_colon {
            Some(o) => {}
            None => {}
        }
        for v in t.segments {
            f.try_take_path_segment(v)?;
        }
        Ok(())
    }
    pub fn try_take_path_arguments<F>(
        f: &mut F,
        t: syn::PathArguments,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::PathArguments::AngleBracketed(tmp0) => {
                f.try_take_path_arguments_variant_angle_bracketed((tmp0))?
            }
            syn::PathArguments::Parenthesized(tmp0) => {
                f.try_take_path_arguments_variant_parenthesized((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_path_arguments_variant_angle_bracketed<F>(
        f: &mut F,
        t: (syn::AngleBracketedGenericArguments),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_angle_bracketed_generic_arguments(t)?;
        Ok(())
    }
    pub fn try_take_path_arguments_variant_parenthesized<F>(
        f: &mut F,
        t: (syn::ParenthesizedGenericArguments),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_parenthesized_generic_arguments(t)?;
        Ok(())
    }
    pub fn try_take_path_segment<F>(
        f: &mut F,
        t: syn::PathSegment,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_path_arguments(t.arguments)?;
        Ok(())
    }
    pub fn try_take_predicate_lifetime<F>(
        f: &mut F,
        t: syn::PredicateLifetime,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lifetime(t.lifetime)?;
        for v in t.bounds {
            f.try_take_lifetime(v)?;
        }
        Ok(())
    }
    pub fn try_take_predicate_type<F>(
        f: &mut F,
        t: syn::PredicateType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.lifetimes {
            Some(o) => {
                f.try_take_bound_lifetimes(o)?;
            }
            None => {}
        }
        f.try_take_type(t.bounded_ty)?;
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        Ok(())
    }
    pub fn try_take_q_self<F>(
        f: &mut F,
        t: syn::QSelf,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.ty)?;
        match t.as_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_range_limits<F>(
        f: &mut F,
        t: syn::RangeLimits,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::RangeLimits::HalfOpen(tmp0) => {
                f.try_take_range_limits_variant_half_open((tmp0))?
            }
            syn::RangeLimits::Closed(tmp0) => {
                f.try_take_range_limits_variant_closed((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_range_limits_variant_half_open<F>(
        f: &mut F,
        t: (syn::token::DotDot),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_range_limits_variant_closed<F>(
        f: &mut F,
        t: (syn::token::DotDotEq),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_receiver<F>(
        f: &mut F,
        t: syn::Receiver,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.reference {
            Some(o) => {
                match o.1 {
                    Some(o) => {
                        f.try_take_lifetime(o)?;
                    }
                    None => {}
                }
            }
            None => {}
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_type(*t.ty)?;
        Ok(())
    }
    pub fn try_take_return_type<F>(
        f: &mut F,
        t: syn::ReturnType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::ReturnType::Type(tmp0, tmp1) => {
                f.try_take_return_type_variant_type((tmp0, tmp1))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_return_type_variant_type<F>(
        f: &mut F,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.1)?;
        Ok(())
    }
    pub fn try_take_signature<F>(
        f: &mut F,
        t: syn::Signature,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.constness {
            Some(o) => {}
            None => {}
        }
        match t.asyncness {
            Some(o) => {}
            None => {}
        }
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        match t.abi {
            Some(o) => {
                f.try_take_abi(o)?;
            }
            None => {}
        }
        f.try_take_generics(t.generics)?;
        for v in t.inputs {
            f.try_take_fn_arg(v)?;
        }
        match t.variadic {
            Some(o) => {
                f.try_take_variadic(o)?;
            }
            None => {}
        }
        f.try_take_return_type(t.output)?;
        Ok(())
    }
    pub fn try_take_static_mutability<F>(
        f: &mut F,
        t: syn::StaticMutability,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::StaticMutability::Mut(tmp0) => {
                f.try_take_static_mutability_variant_mut((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_static_mutability_variant_mut<F>(
        f: &mut F,
        t: (syn::token::Mut),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_stmt<F>(f: &mut F, t: syn::Stmt) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Stmt::Local(tmp0) => f.try_take_stmt_variant_local((tmp0))?,
            syn::Stmt::Item(tmp0) => f.try_take_stmt_variant_item((tmp0))?,
            syn::Stmt::Expr(tmp0, tmp1) => f.try_take_stmt_variant_expr((tmp0, tmp1))?,
            syn::Stmt::Macro(tmp0) => f.try_take_stmt_variant_macro((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_stmt_variant_local<F>(
        f: &mut F,
        t: (syn::Local),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_local(t)?;
        Ok(())
    }
    pub fn try_take_stmt_variant_item<F>(
        f: &mut F,
        t: (syn::Item),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_item(t)?;
        Ok(())
    }
    pub fn try_take_stmt_variant_expr<F>(
        f: &mut F,
        t: (syn::Expr, Option<syn::token::Semi>),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_expr(t.0)?;
        match t.1 {
            Some(o) => {}
            None => {}
        };
        Ok(())
    }
    pub fn try_take_stmt_variant_macro<F>(
        f: &mut F,
        t: (syn::StmtMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_stmt_macro(t)?;
        Ok(())
    }
    pub fn try_take_stmt_macro<F>(
        f: &mut F,
        t: syn::StmtMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_macro(t.mac)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_trait_bound<F>(
        f: &mut F,
        t: syn::TraitBound,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.paren_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_trait_bound_modifier(t.modifier)?;
        match t.lifetimes {
            Some(o) => {
                f.try_take_bound_lifetimes(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        Ok(())
    }
    pub fn try_take_trait_bound_modifier<F>(
        f: &mut F,
        t: syn::TraitBoundModifier,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::TraitBoundModifier::Maybe(tmp0) => {
                f.try_take_trait_bound_modifier_variant_maybe((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_trait_bound_modifier_variant_maybe<F>(
        f: &mut F,
        t: (syn::token::Question),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_trait_item<F>(
        f: &mut F,
        t: syn::TraitItem,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::TraitItem::Const(tmp0) => f.try_take_trait_item_variant_const((tmp0))?,
            syn::TraitItem::Fn(tmp0) => f.try_take_trait_item_variant_fn((tmp0))?,
            syn::TraitItem::Type(tmp0) => f.try_take_trait_item_variant_type((tmp0))?,
            syn::TraitItem::Macro(tmp0) => f.try_take_trait_item_variant_macro((tmp0))?,
            syn::TraitItem::Verbatim(tmp0) => {
                f.try_take_trait_item_variant_verbatim((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_trait_item_variant_const<F>(
        f: &mut F,
        t: (syn::TraitItemConst),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_trait_item_const(t)?;
        Ok(())
    }
    pub fn try_take_trait_item_variant_fn<F>(
        f: &mut F,
        t: (syn::TraitItemFn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_trait_item_fn(t)?;
        Ok(())
    }
    pub fn try_take_trait_item_variant_type<F>(
        f: &mut F,
        t: (syn::TraitItemType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_trait_item_type(t)?;
        Ok(())
    }
    pub fn try_take_trait_item_variant_macro<F>(
        f: &mut F,
        t: (syn::TraitItemMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_trait_item_macro(t)?;
        Ok(())
    }
    pub fn try_take_trait_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_trait_item_const<F>(
        f: &mut F,
        t: syn::TraitItemConst,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_generics(t.generics)?;
        f.try_take_type(t.ty)?;
        match t.default {
            Some(o) => {
                f.try_take_expr(o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_trait_item_fn<F>(
        f: &mut F,
        t: syn::TraitItemFn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_signature(t.sig)?;
        match t.default {
            Some(o) => {
                f.try_take_block(o)?;
            }
            None => {}
        }
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_trait_item_macro<F>(
        f: &mut F,
        t: syn::TraitItemMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_macro(t.mac)?;
        match t.semi_token {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_trait_item_type<F>(
        f: &mut F,
        t: syn::TraitItemType,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_generics(t.generics)?;
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        match t.default {
            Some(o) => {
                f.try_take_type(o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_type<F>(f: &mut F, t: syn::Type) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Type::Array(tmp0) => f.try_take_type_variant_array((tmp0))?,
            syn::Type::BareFn(tmp0) => f.try_take_type_variant_bare_fn((tmp0))?,
            syn::Type::Group(tmp0) => f.try_take_type_variant_group((tmp0))?,
            syn::Type::ImplTrait(tmp0) => f.try_take_type_variant_impl_trait((tmp0))?,
            syn::Type::Infer(tmp0) => f.try_take_type_variant_infer((tmp0))?,
            syn::Type::Macro(tmp0) => f.try_take_type_variant_macro((tmp0))?,
            syn::Type::Never(tmp0) => f.try_take_type_variant_never((tmp0))?,
            syn::Type::Paren(tmp0) => f.try_take_type_variant_paren((tmp0))?,
            syn::Type::Path(tmp0) => f.try_take_type_variant_path((tmp0))?,
            syn::Type::Ptr(tmp0) => f.try_take_type_variant_ptr((tmp0))?,
            syn::Type::Reference(tmp0) => f.try_take_type_variant_reference((tmp0))?,
            syn::Type::Slice(tmp0) => f.try_take_type_variant_slice((tmp0))?,
            syn::Type::TraitObject(tmp0) => f.try_take_type_variant_trait_object((tmp0))?,
            syn::Type::Tuple(tmp0) => f.try_take_type_variant_tuple((tmp0))?,
            syn::Type::Verbatim(tmp0) => f.try_take_type_variant_verbatim((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_type_variant_array<F>(
        f: &mut F,
        t: (syn::TypeArray),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_array(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_bare_fn<F>(
        f: &mut F,
        t: (syn::TypeBareFn),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_bare_fn(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_group<F>(
        f: &mut F,
        t: (syn::TypeGroup),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_group(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_impl_trait<F>(
        f: &mut F,
        t: (syn::TypeImplTrait),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_impl_trait(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_infer<F>(
        f: &mut F,
        t: (syn::TypeInfer),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_infer(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_macro<F>(
        f: &mut F,
        t: (syn::TypeMacro),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_macro(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_never<F>(
        f: &mut F,
        t: (syn::TypeNever),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_never(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_paren<F>(
        f: &mut F,
        t: (syn::TypeParen),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_paren(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_path<F>(
        f: &mut F,
        t: (syn::TypePath),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_path(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_ptr<F>(
        f: &mut F,
        t: (syn::TypePtr),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_ptr(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_reference<F>(
        f: &mut F,
        t: (syn::TypeReference),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_reference(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_slice<F>(
        f: &mut F,
        t: (syn::TypeSlice),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_slice(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_trait_object<F>(
        f: &mut F,
        t: (syn::TypeTraitObject),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_trait_object(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_tuple<F>(
        f: &mut F,
        t: (syn::TypeTuple),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type_tuple(t)?;
        Ok(())
    }
    pub fn try_take_type_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_type_array<F>(
        f: &mut F,
        t: syn::TypeArray,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.elem)?;
        f.try_take_expr(t.len)?;
        Ok(())
    }
    pub fn try_take_type_bare_fn<F>(
        f: &mut F,
        t: syn::TypeBareFn,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.lifetimes {
            Some(o) => {
                f.try_take_bound_lifetimes(o)?;
            }
            None => {}
        }
        match t.unsafety {
            Some(o) => {}
            None => {}
        }
        match t.abi {
            Some(o) => {
                f.try_take_abi(o)?;
            }
            None => {}
        }
        for v in t.inputs {
            f.try_take_bare_fn_arg(v)?;
        }
        match t.variadic {
            Some(o) => {
                f.try_take_bare_variadic(o)?;
            }
            None => {}
        }
        f.try_take_return_type(t.output)?;
        Ok(())
    }
    pub fn try_take_type_group<F>(
        f: &mut F,
        t: syn::TypeGroup,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.elem)?;
        Ok(())
    }
    pub fn try_take_type_impl_trait<F>(
        f: &mut F,
        t: syn::TypeImplTrait,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        Ok(())
    }
    pub fn try_take_type_infer<F>(
        f: &mut F,
        t: syn::TypeInfer,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_type_macro<F>(
        f: &mut F,
        t: syn::TypeMacro,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_macro(t.mac)?;
        Ok(())
    }
    pub fn try_take_type_never<F>(
        f: &mut F,
        t: syn::TypeNever,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_type_param<F>(
        f: &mut F,
        t: syn::TypeParam,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.colon_token {
            Some(o) => {}
            None => {}
        }
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        match t.eq_token {
            Some(o) => {}
            None => {}
        }
        match t.default {
            Some(o) => {
                f.try_take_type(o)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_type_param_bound<F>(
        f: &mut F,
        t: syn::TypeParamBound,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::TypeParamBound::Trait(tmp0) => {
                f.try_take_type_param_bound_variant_trait((tmp0))?
            }
            syn::TypeParamBound::Lifetime(tmp0) => {
                f.try_take_type_param_bound_variant_lifetime((tmp0))?
            }
            syn::TypeParamBound::Verbatim(tmp0) => {
                f.try_take_type_param_bound_variant_verbatim((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_type_param_bound_variant_trait<F>(
        f: &mut F,
        t: (syn::TraitBound),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_trait_bound(t)?;
        Ok(())
    }
    pub fn try_take_type_param_bound_variant_lifetime<F>(
        f: &mut F,
        t: (syn::Lifetime),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_lifetime(t)?;
        Ok(())
    }
    pub fn try_take_type_param_bound_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_type_paren<F>(
        f: &mut F,
        t: syn::TypeParen,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.elem)?;
        Ok(())
    }
    pub fn try_take_type_path<F>(
        f: &mut F,
        t: syn::TypePath,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.qself {
            Some(o) => {
                f.try_take_q_self(o)?;
            }
            None => {}
        }
        f.try_take_path(t.path)?;
        Ok(())
    }
    pub fn try_take_type_ptr<F>(
        f: &mut F,
        t: syn::TypePtr,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.const_token {
            Some(o) => {}
            None => {}
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        f.try_take_type(*t.elem)?;
        Ok(())
    }
    pub fn try_take_type_reference<F>(
        f: &mut F,
        t: syn::TypeReference,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.lifetime {
            Some(o) => {
                f.try_take_lifetime(o)?;
            }
            None => {}
        }
        match t.mutability {
            Some(o) => {}
            None => {}
        }
        f.try_take_type(*t.elem)?;
        Ok(())
    }
    pub fn try_take_type_slice<F>(
        f: &mut F,
        t: syn::TypeSlice,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_type(*t.elem)?;
        Ok(())
    }
    pub fn try_take_type_trait_object<F>(
        f: &mut F,
        t: syn::TypeTraitObject,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.dyn_token {
            Some(o) => {}
            None => {}
        }
        for v in t.bounds {
            f.try_take_type_param_bound(v)?;
        }
        Ok(())
    }
    pub fn try_take_type_tuple<F>(
        f: &mut F,
        t: syn::TypeTuple,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.elems {
            f.try_take_type(v)?;
        }
        Ok(())
    }
    pub fn try_take_un_op<F>(
        f: &mut F,
        t: syn::UnOp,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::UnOp::Deref(tmp0) => f.try_take_un_op_variant_deref((tmp0))?,
            syn::UnOp::Not(tmp0) => f.try_take_un_op_variant_not((tmp0))?,
            syn::UnOp::Neg(tmp0) => f.try_take_un_op_variant_neg((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_un_op_variant_deref<F>(
        f: &mut F,
        t: (syn::token::Star),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_un_op_variant_not<F>(
        f: &mut F,
        t: (syn::token::Not),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_un_op_variant_neg<F>(
        f: &mut F,
        t: (syn::token::Minus),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_use_glob<F>(
        f: &mut F,
        t: syn::UseGlob,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_use_group<F>(
        f: &mut F,
        t: syn::UseGroup,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.items {
            f.try_take_use_tree(v)?;
        }
        Ok(())
    }
    pub fn try_take_use_name<F>(
        f: &mut F,
        t: syn::UseName,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_use_path<F>(
        f: &mut F,
        t: syn::UsePath,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_tree(*t.tree)?;
        Ok(())
    }
    pub fn try_take_use_rename<F>(
        f: &mut F,
        t: syn::UseRename,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_use_tree<F>(
        f: &mut F,
        t: syn::UseTree,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::UseTree::Path(tmp0) => f.try_take_use_tree_variant_path((tmp0))?,
            syn::UseTree::Name(tmp0) => f.try_take_use_tree_variant_name((tmp0))?,
            syn::UseTree::Rename(tmp0) => f.try_take_use_tree_variant_rename((tmp0))?,
            syn::UseTree::Glob(tmp0) => f.try_take_use_tree_variant_glob((tmp0))?,
            syn::UseTree::Group(tmp0) => f.try_take_use_tree_variant_group((tmp0))?,
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_use_tree_variant_path<F>(
        f: &mut F,
        t: (syn::UsePath),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_path(t)?;
        Ok(())
    }
    pub fn try_take_use_tree_variant_name<F>(
        f: &mut F,
        t: (syn::UseName),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_name(t)?;
        Ok(())
    }
    pub fn try_take_use_tree_variant_rename<F>(
        f: &mut F,
        t: (syn::UseRename),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_rename(t)?;
        Ok(())
    }
    pub fn try_take_use_tree_variant_glob<F>(
        f: &mut F,
        t: (syn::UseGlob),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_glob(t)?;
        Ok(())
    }
    pub fn try_take_use_tree_variant_group<F>(
        f: &mut F,
        t: (syn::UseGroup),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_use_group(t)?;
        Ok(())
    }
    pub fn try_take_variadic<F>(
        f: &mut F,
        t: syn::Variadic,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        match t.pat {
            Some(o) => {
                f.try_take_pat(*o.0)?;
            }
            None => {}
        }
        match t.comma {
            Some(o) => {}
            None => {}
        }
        Ok(())
    }
    pub fn try_take_variant<F>(
        f: &mut F,
        t: syn::Variant,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.attrs {
            f.try_take_attribute(v)?;
        }
        f.try_take_fields(t.fields)?;
        match t.discriminant {
            Some(o) => {
                f.try_take_expr(o.1)?;
            }
            None => {}
        }
        Ok(())
    }
    pub fn try_take_vis_restricted<F>(
        f: &mut F,
        t: syn::VisRestricted,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t.in_token {
            Some(o) => {}
            None => {}
        }
        f.try_take_path(*t.path)?;
        Ok(())
    }
    pub fn try_take_visibility<F>(
        f: &mut F,
        t: syn::Visibility,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::Visibility::Public(tmp0) => {
                f.try_take_visibility_variant_public((tmp0))?
            }
            syn::Visibility::Restricted(tmp0) => {
                f.try_take_visibility_variant_restricted((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_visibility_variant_public<F>(
        f: &mut F,
        t: (syn::token::Pub),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        Ok(())
    }
    pub fn try_take_visibility_variant_restricted<F>(
        f: &mut F,
        t: (syn::VisRestricted),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_vis_restricted(t)?;
        Ok(())
    }
    pub fn try_take_where_clause<F>(
        f: &mut F,
        t: syn::WhereClause,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        for v in t.predicates {
            f.try_take_where_predicate(v)?;
        }
        Ok(())
    }
    pub fn try_take_where_predicate<F>(
        f: &mut F,
        t: syn::WherePredicate,
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        match t {
            syn::WherePredicate::Lifetime(tmp0) => {
                f.try_take_where_predicate_variant_lifetime((tmp0))?
            }
            syn::WherePredicate::Type(tmp0) => {
                f.try_take_where_predicate_variant_type((tmp0))?
            }
            _ => {}
        };
        Ok(())
    }
    pub fn try_take_where_predicate_variant_lifetime<F>(
        f: &mut F,
        t: (syn::PredicateLifetime),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_predicate_lifetime(t)?;
        Ok(())
    }
    pub fn try_take_where_predicate_variant_type<F>(
        f: &mut F,
        t: (syn::PredicateType),
    ) -> Result<(), <F as TryTake>::Error>
    where
        F: TryTake + ?Sized,
    {
        f.try_take_predicate_type(t)?;
        Ok(())
    }
}
