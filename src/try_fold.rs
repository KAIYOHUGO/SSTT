// Codegen file with version `0.1.0`
// !!!Don't modify this file manually!!!
// --------------------



pub use tmp::*;
#[allow(unused_parens, unused_variables, unreachable_patterns)]
mod tmp {
    /// `fn try_fold_T(&mut self, T) -> Result<T, Self::Error>`
    ///
    /// It can fold syntax tree and report error
    pub trait TryFold {
        type Error;
        fn try_fold_abi(&mut self, t: syn::Abi) -> Result<syn::Abi, Self::Error> {
            try_fold_abi(self, t)
        }
        fn try_fold_angle_bracketed_generic_arguments(
            &mut self,
            t: syn::AngleBracketedGenericArguments,
        ) -> Result<syn::AngleBracketedGenericArguments, Self::Error> {
            try_fold_angle_bracketed_generic_arguments(self, t)
        }
        fn try_fold_arm(&mut self, t: syn::Arm) -> Result<syn::Arm, Self::Error> {
            try_fold_arm(self, t)
        }
        fn try_fold_assoc_const(
            &mut self,
            t: syn::AssocConst,
        ) -> Result<syn::AssocConst, Self::Error> {
            try_fold_assoc_const(self, t)
        }
        fn try_fold_assoc_type(
            &mut self,
            t: syn::AssocType,
        ) -> Result<syn::AssocType, Self::Error> {
            try_fold_assoc_type(self, t)
        }
        fn try_fold_attr_style(
            &mut self,
            t: syn::AttrStyle,
        ) -> Result<syn::AttrStyle, Self::Error> {
            try_fold_attr_style(self, t)
        }
        fn try_fold_attr_style_variant_inner(
            &mut self,
            t: (syn::token::Not),
        ) -> Result<syn::AttrStyle, Self::Error> {
            try_fold_attr_style_variant_inner(self, t)
        }
        fn try_fold_attribute(
            &mut self,
            t: syn::Attribute,
        ) -> Result<syn::Attribute, Self::Error> {
            try_fold_attribute(self, t)
        }
        fn try_fold_bare_fn_arg(
            &mut self,
            t: syn::BareFnArg,
        ) -> Result<syn::BareFnArg, Self::Error> {
            try_fold_bare_fn_arg(self, t)
        }
        fn try_fold_bare_variadic(
            &mut self,
            t: syn::BareVariadic,
        ) -> Result<syn::BareVariadic, Self::Error> {
            try_fold_bare_variadic(self, t)
        }
        fn try_fold_bin_op(&mut self, t: syn::BinOp) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op(self, t)
        }
        fn try_fold_bin_op_variant_add(
            &mut self,
            t: (syn::token::Plus),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_add(self, t)
        }
        fn try_fold_bin_op_variant_sub(
            &mut self,
            t: (syn::token::Minus),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_sub(self, t)
        }
        fn try_fold_bin_op_variant_mul(
            &mut self,
            t: (syn::token::Star),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_mul(self, t)
        }
        fn try_fold_bin_op_variant_div(
            &mut self,
            t: (syn::token::Slash),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_div(self, t)
        }
        fn try_fold_bin_op_variant_rem(
            &mut self,
            t: (syn::token::Percent),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_rem(self, t)
        }
        fn try_fold_bin_op_variant_and(
            &mut self,
            t: (syn::token::AndAnd),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_and(self, t)
        }
        fn try_fold_bin_op_variant_or(
            &mut self,
            t: (syn::token::OrOr),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_or(self, t)
        }
        fn try_fold_bin_op_variant_bit_xor(
            &mut self,
            t: (syn::token::Caret),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_xor(self, t)
        }
        fn try_fold_bin_op_variant_bit_and(
            &mut self,
            t: (syn::token::And),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_and(self, t)
        }
        fn try_fold_bin_op_variant_bit_or(
            &mut self,
            t: (syn::token::Or),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_or(self, t)
        }
        fn try_fold_bin_op_variant_shl(
            &mut self,
            t: (syn::token::Shl),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_shl(self, t)
        }
        fn try_fold_bin_op_variant_shr(
            &mut self,
            t: (syn::token::Shr),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_shr(self, t)
        }
        fn try_fold_bin_op_variant_eq(
            &mut self,
            t: (syn::token::EqEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_eq(self, t)
        }
        fn try_fold_bin_op_variant_lt(
            &mut self,
            t: (syn::token::Lt),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_lt(self, t)
        }
        fn try_fold_bin_op_variant_le(
            &mut self,
            t: (syn::token::Le),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_le(self, t)
        }
        fn try_fold_bin_op_variant_ne(
            &mut self,
            t: (syn::token::Ne),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_ne(self, t)
        }
        fn try_fold_bin_op_variant_ge(
            &mut self,
            t: (syn::token::Ge),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_ge(self, t)
        }
        fn try_fold_bin_op_variant_gt(
            &mut self,
            t: (syn::token::Gt),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_gt(self, t)
        }
        fn try_fold_bin_op_variant_add_assign(
            &mut self,
            t: (syn::token::PlusEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_add_assign(self, t)
        }
        fn try_fold_bin_op_variant_sub_assign(
            &mut self,
            t: (syn::token::MinusEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_sub_assign(self, t)
        }
        fn try_fold_bin_op_variant_mul_assign(
            &mut self,
            t: (syn::token::StarEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_mul_assign(self, t)
        }
        fn try_fold_bin_op_variant_div_assign(
            &mut self,
            t: (syn::token::SlashEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_div_assign(self, t)
        }
        fn try_fold_bin_op_variant_rem_assign(
            &mut self,
            t: (syn::token::PercentEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_rem_assign(self, t)
        }
        fn try_fold_bin_op_variant_bit_xor_assign(
            &mut self,
            t: (syn::token::CaretEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_xor_assign(self, t)
        }
        fn try_fold_bin_op_variant_bit_and_assign(
            &mut self,
            t: (syn::token::AndEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_and_assign(self, t)
        }
        fn try_fold_bin_op_variant_bit_or_assign(
            &mut self,
            t: (syn::token::OrEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_bit_or_assign(self, t)
        }
        fn try_fold_bin_op_variant_shl_assign(
            &mut self,
            t: (syn::token::ShlEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_shl_assign(self, t)
        }
        fn try_fold_bin_op_variant_shr_assign(
            &mut self,
            t: (syn::token::ShrEq),
        ) -> Result<syn::BinOp, Self::Error> {
            try_fold_bin_op_variant_shr_assign(self, t)
        }
        fn try_fold_block(&mut self, t: syn::Block) -> Result<syn::Block, Self::Error> {
            try_fold_block(self, t)
        }
        fn try_fold_bound_lifetimes(
            &mut self,
            t: syn::BoundLifetimes,
        ) -> Result<syn::BoundLifetimes, Self::Error> {
            try_fold_bound_lifetimes(self, t)
        }
        fn try_fold_const_param(
            &mut self,
            t: syn::ConstParam,
        ) -> Result<syn::ConstParam, Self::Error> {
            try_fold_const_param(self, t)
        }
        fn try_fold_constraint(
            &mut self,
            t: syn::Constraint,
        ) -> Result<syn::Constraint, Self::Error> {
            try_fold_constraint(self, t)
        }
        fn try_fold_data(&mut self, t: syn::Data) -> Result<syn::Data, Self::Error> {
            try_fold_data(self, t)
        }
        fn try_fold_data_variant_struct(
            &mut self,
            t: (syn::DataStruct),
        ) -> Result<syn::Data, Self::Error> {
            try_fold_data_variant_struct(self, t)
        }
        fn try_fold_data_variant_enum(
            &mut self,
            t: (syn::DataEnum),
        ) -> Result<syn::Data, Self::Error> {
            try_fold_data_variant_enum(self, t)
        }
        fn try_fold_data_variant_union(
            &mut self,
            t: (syn::DataUnion),
        ) -> Result<syn::Data, Self::Error> {
            try_fold_data_variant_union(self, t)
        }
        fn try_fold_data_enum(
            &mut self,
            t: syn::DataEnum,
        ) -> Result<syn::DataEnum, Self::Error> {
            try_fold_data_enum(self, t)
        }
        fn try_fold_data_struct(
            &mut self,
            t: syn::DataStruct,
        ) -> Result<syn::DataStruct, Self::Error> {
            try_fold_data_struct(self, t)
        }
        fn try_fold_data_union(
            &mut self,
            t: syn::DataUnion,
        ) -> Result<syn::DataUnion, Self::Error> {
            try_fold_data_union(self, t)
        }
        fn try_fold_derive_input(
            &mut self,
            t: syn::DeriveInput,
        ) -> Result<syn::DeriveInput, Self::Error> {
            try_fold_derive_input(self, t)
        }
        fn try_fold_expr(&mut self, t: syn::Expr) -> Result<syn::Expr, Self::Error> {
            try_fold_expr(self, t)
        }
        fn try_fold_expr_variant_array(
            &mut self,
            t: (syn::ExprArray),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_array(self, t)
        }
        fn try_fold_expr_variant_assign(
            &mut self,
            t: (syn::ExprAssign),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_assign(self, t)
        }
        fn try_fold_expr_variant_async(
            &mut self,
            t: (syn::ExprAsync),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_async(self, t)
        }
        fn try_fold_expr_variant_await(
            &mut self,
            t: (syn::ExprAwait),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_await(self, t)
        }
        fn try_fold_expr_variant_binary(
            &mut self,
            t: (syn::ExprBinary),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_binary(self, t)
        }
        fn try_fold_expr_variant_block(
            &mut self,
            t: (syn::ExprBlock),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_block(self, t)
        }
        fn try_fold_expr_variant_break(
            &mut self,
            t: (syn::ExprBreak),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_break(self, t)
        }
        fn try_fold_expr_variant_call(
            &mut self,
            t: (syn::ExprCall),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_call(self, t)
        }
        fn try_fold_expr_variant_cast(
            &mut self,
            t: (syn::ExprCast),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_cast(self, t)
        }
        fn try_fold_expr_variant_closure(
            &mut self,
            t: (syn::ExprClosure),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_closure(self, t)
        }
        fn try_fold_expr_variant_const(
            &mut self,
            t: (syn::ExprConst),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_const(self, t)
        }
        fn try_fold_expr_variant_continue(
            &mut self,
            t: (syn::ExprContinue),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_continue(self, t)
        }
        fn try_fold_expr_variant_field(
            &mut self,
            t: (syn::ExprField),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_field(self, t)
        }
        fn try_fold_expr_variant_for_loop(
            &mut self,
            t: (syn::ExprForLoop),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_for_loop(self, t)
        }
        fn try_fold_expr_variant_group(
            &mut self,
            t: (syn::ExprGroup),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_group(self, t)
        }
        fn try_fold_expr_variant_if(
            &mut self,
            t: (syn::ExprIf),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_if(self, t)
        }
        fn try_fold_expr_variant_index(
            &mut self,
            t: (syn::ExprIndex),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_index(self, t)
        }
        fn try_fold_expr_variant_infer(
            &mut self,
            t: (syn::ExprInfer),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_infer(self, t)
        }
        fn try_fold_expr_variant_let(
            &mut self,
            t: (syn::ExprLet),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_let(self, t)
        }
        fn try_fold_expr_variant_lit(
            &mut self,
            t: (syn::ExprLit),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_lit(self, t)
        }
        fn try_fold_expr_variant_loop(
            &mut self,
            t: (syn::ExprLoop),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_loop(self, t)
        }
        fn try_fold_expr_variant_macro(
            &mut self,
            t: (syn::ExprMacro),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_macro(self, t)
        }
        fn try_fold_expr_variant_match(
            &mut self,
            t: (syn::ExprMatch),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_match(self, t)
        }
        fn try_fold_expr_variant_method_call(
            &mut self,
            t: (syn::ExprMethodCall),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_method_call(self, t)
        }
        fn try_fold_expr_variant_paren(
            &mut self,
            t: (syn::ExprParen),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_paren(self, t)
        }
        fn try_fold_expr_variant_path(
            &mut self,
            t: (syn::ExprPath),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_path(self, t)
        }
        fn try_fold_expr_variant_range(
            &mut self,
            t: (syn::ExprRange),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_range(self, t)
        }
        fn try_fold_expr_variant_reference(
            &mut self,
            t: (syn::ExprReference),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_reference(self, t)
        }
        fn try_fold_expr_variant_repeat(
            &mut self,
            t: (syn::ExprRepeat),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_repeat(self, t)
        }
        fn try_fold_expr_variant_return(
            &mut self,
            t: (syn::ExprReturn),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_return(self, t)
        }
        fn try_fold_expr_variant_struct(
            &mut self,
            t: (syn::ExprStruct),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_struct(self, t)
        }
        fn try_fold_expr_variant_try(
            &mut self,
            t: (syn::ExprTry),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_try(self, t)
        }
        fn try_fold_expr_variant_try_block(
            &mut self,
            t: (syn::ExprTryBlock),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_try_block(self, t)
        }
        fn try_fold_expr_variant_tuple(
            &mut self,
            t: (syn::ExprTuple),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_tuple(self, t)
        }
        fn try_fold_expr_variant_unary(
            &mut self,
            t: (syn::ExprUnary),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_unary(self, t)
        }
        fn try_fold_expr_variant_unsafe(
            &mut self,
            t: (syn::ExprUnsafe),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_unsafe(self, t)
        }
        fn try_fold_expr_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_verbatim(self, t)
        }
        fn try_fold_expr_variant_while(
            &mut self,
            t: (syn::ExprWhile),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_while(self, t)
        }
        fn try_fold_expr_variant_yield(
            &mut self,
            t: (syn::ExprYield),
        ) -> Result<syn::Expr, Self::Error> {
            try_fold_expr_variant_yield(self, t)
        }
        fn try_fold_expr_array(
            &mut self,
            t: syn::ExprArray,
        ) -> Result<syn::ExprArray, Self::Error> {
            try_fold_expr_array(self, t)
        }
        fn try_fold_expr_assign(
            &mut self,
            t: syn::ExprAssign,
        ) -> Result<syn::ExprAssign, Self::Error> {
            try_fold_expr_assign(self, t)
        }
        fn try_fold_expr_async(
            &mut self,
            t: syn::ExprAsync,
        ) -> Result<syn::ExprAsync, Self::Error> {
            try_fold_expr_async(self, t)
        }
        fn try_fold_expr_await(
            &mut self,
            t: syn::ExprAwait,
        ) -> Result<syn::ExprAwait, Self::Error> {
            try_fold_expr_await(self, t)
        }
        fn try_fold_expr_binary(
            &mut self,
            t: syn::ExprBinary,
        ) -> Result<syn::ExprBinary, Self::Error> {
            try_fold_expr_binary(self, t)
        }
        fn try_fold_expr_block(
            &mut self,
            t: syn::ExprBlock,
        ) -> Result<syn::ExprBlock, Self::Error> {
            try_fold_expr_block(self, t)
        }
        fn try_fold_expr_break(
            &mut self,
            t: syn::ExprBreak,
        ) -> Result<syn::ExprBreak, Self::Error> {
            try_fold_expr_break(self, t)
        }
        fn try_fold_expr_call(
            &mut self,
            t: syn::ExprCall,
        ) -> Result<syn::ExprCall, Self::Error> {
            try_fold_expr_call(self, t)
        }
        fn try_fold_expr_cast(
            &mut self,
            t: syn::ExprCast,
        ) -> Result<syn::ExprCast, Self::Error> {
            try_fold_expr_cast(self, t)
        }
        fn try_fold_expr_closure(
            &mut self,
            t: syn::ExprClosure,
        ) -> Result<syn::ExprClosure, Self::Error> {
            try_fold_expr_closure(self, t)
        }
        fn try_fold_expr_const(
            &mut self,
            t: syn::ExprConst,
        ) -> Result<syn::ExprConst, Self::Error> {
            try_fold_expr_const(self, t)
        }
        fn try_fold_expr_continue(
            &mut self,
            t: syn::ExprContinue,
        ) -> Result<syn::ExprContinue, Self::Error> {
            try_fold_expr_continue(self, t)
        }
        fn try_fold_expr_field(
            &mut self,
            t: syn::ExprField,
        ) -> Result<syn::ExprField, Self::Error> {
            try_fold_expr_field(self, t)
        }
        fn try_fold_expr_for_loop(
            &mut self,
            t: syn::ExprForLoop,
        ) -> Result<syn::ExprForLoop, Self::Error> {
            try_fold_expr_for_loop(self, t)
        }
        fn try_fold_expr_group(
            &mut self,
            t: syn::ExprGroup,
        ) -> Result<syn::ExprGroup, Self::Error> {
            try_fold_expr_group(self, t)
        }
        fn try_fold_expr_if(
            &mut self,
            t: syn::ExprIf,
        ) -> Result<syn::ExprIf, Self::Error> {
            try_fold_expr_if(self, t)
        }
        fn try_fold_expr_index(
            &mut self,
            t: syn::ExprIndex,
        ) -> Result<syn::ExprIndex, Self::Error> {
            try_fold_expr_index(self, t)
        }
        fn try_fold_expr_infer(
            &mut self,
            t: syn::ExprInfer,
        ) -> Result<syn::ExprInfer, Self::Error> {
            try_fold_expr_infer(self, t)
        }
        fn try_fold_expr_let(
            &mut self,
            t: syn::ExprLet,
        ) -> Result<syn::ExprLet, Self::Error> {
            try_fold_expr_let(self, t)
        }
        fn try_fold_expr_lit(
            &mut self,
            t: syn::ExprLit,
        ) -> Result<syn::ExprLit, Self::Error> {
            try_fold_expr_lit(self, t)
        }
        fn try_fold_expr_loop(
            &mut self,
            t: syn::ExprLoop,
        ) -> Result<syn::ExprLoop, Self::Error> {
            try_fold_expr_loop(self, t)
        }
        fn try_fold_expr_macro(
            &mut self,
            t: syn::ExprMacro,
        ) -> Result<syn::ExprMacro, Self::Error> {
            try_fold_expr_macro(self, t)
        }
        fn try_fold_expr_match(
            &mut self,
            t: syn::ExprMatch,
        ) -> Result<syn::ExprMatch, Self::Error> {
            try_fold_expr_match(self, t)
        }
        fn try_fold_expr_method_call(
            &mut self,
            t: syn::ExprMethodCall,
        ) -> Result<syn::ExprMethodCall, Self::Error> {
            try_fold_expr_method_call(self, t)
        }
        fn try_fold_expr_paren(
            &mut self,
            t: syn::ExprParen,
        ) -> Result<syn::ExprParen, Self::Error> {
            try_fold_expr_paren(self, t)
        }
        fn try_fold_expr_path(
            &mut self,
            t: syn::ExprPath,
        ) -> Result<syn::ExprPath, Self::Error> {
            try_fold_expr_path(self, t)
        }
        fn try_fold_expr_range(
            &mut self,
            t: syn::ExprRange,
        ) -> Result<syn::ExprRange, Self::Error> {
            try_fold_expr_range(self, t)
        }
        fn try_fold_expr_reference(
            &mut self,
            t: syn::ExprReference,
        ) -> Result<syn::ExprReference, Self::Error> {
            try_fold_expr_reference(self, t)
        }
        fn try_fold_expr_repeat(
            &mut self,
            t: syn::ExprRepeat,
        ) -> Result<syn::ExprRepeat, Self::Error> {
            try_fold_expr_repeat(self, t)
        }
        fn try_fold_expr_return(
            &mut self,
            t: syn::ExprReturn,
        ) -> Result<syn::ExprReturn, Self::Error> {
            try_fold_expr_return(self, t)
        }
        fn try_fold_expr_struct(
            &mut self,
            t: syn::ExprStruct,
        ) -> Result<syn::ExprStruct, Self::Error> {
            try_fold_expr_struct(self, t)
        }
        fn try_fold_expr_try(
            &mut self,
            t: syn::ExprTry,
        ) -> Result<syn::ExprTry, Self::Error> {
            try_fold_expr_try(self, t)
        }
        fn try_fold_expr_try_block(
            &mut self,
            t: syn::ExprTryBlock,
        ) -> Result<syn::ExprTryBlock, Self::Error> {
            try_fold_expr_try_block(self, t)
        }
        fn try_fold_expr_tuple(
            &mut self,
            t: syn::ExprTuple,
        ) -> Result<syn::ExprTuple, Self::Error> {
            try_fold_expr_tuple(self, t)
        }
        fn try_fold_expr_unary(
            &mut self,
            t: syn::ExprUnary,
        ) -> Result<syn::ExprUnary, Self::Error> {
            try_fold_expr_unary(self, t)
        }
        fn try_fold_expr_unsafe(
            &mut self,
            t: syn::ExprUnsafe,
        ) -> Result<syn::ExprUnsafe, Self::Error> {
            try_fold_expr_unsafe(self, t)
        }
        fn try_fold_expr_while(
            &mut self,
            t: syn::ExprWhile,
        ) -> Result<syn::ExprWhile, Self::Error> {
            try_fold_expr_while(self, t)
        }
        fn try_fold_expr_yield(
            &mut self,
            t: syn::ExprYield,
        ) -> Result<syn::ExprYield, Self::Error> {
            try_fold_expr_yield(self, t)
        }
        fn try_fold_field(&mut self, t: syn::Field) -> Result<syn::Field, Self::Error> {
            try_fold_field(self, t)
        }
        fn try_fold_field_mutability(
            &mut self,
            t: syn::FieldMutability,
        ) -> Result<syn::FieldMutability, Self::Error> {
            try_fold_field_mutability(self, t)
        }
        fn try_fold_field_pat(
            &mut self,
            t: syn::FieldPat,
        ) -> Result<syn::FieldPat, Self::Error> {
            try_fold_field_pat(self, t)
        }
        fn try_fold_field_value(
            &mut self,
            t: syn::FieldValue,
        ) -> Result<syn::FieldValue, Self::Error> {
            try_fold_field_value(self, t)
        }
        fn try_fold_fields(
            &mut self,
            t: syn::Fields,
        ) -> Result<syn::Fields, Self::Error> {
            try_fold_fields(self, t)
        }
        fn try_fold_fields_variant_named(
            &mut self,
            t: (syn::FieldsNamed),
        ) -> Result<syn::Fields, Self::Error> {
            try_fold_fields_variant_named(self, t)
        }
        fn try_fold_fields_variant_unnamed(
            &mut self,
            t: (syn::FieldsUnnamed),
        ) -> Result<syn::Fields, Self::Error> {
            try_fold_fields_variant_unnamed(self, t)
        }
        fn try_fold_fields_named(
            &mut self,
            t: syn::FieldsNamed,
        ) -> Result<syn::FieldsNamed, Self::Error> {
            try_fold_fields_named(self, t)
        }
        fn try_fold_fields_unnamed(
            &mut self,
            t: syn::FieldsUnnamed,
        ) -> Result<syn::FieldsUnnamed, Self::Error> {
            try_fold_fields_unnamed(self, t)
        }
        fn try_fold_file(&mut self, t: syn::File) -> Result<syn::File, Self::Error> {
            try_fold_file(self, t)
        }
        fn try_fold_fn_arg(&mut self, t: syn::FnArg) -> Result<syn::FnArg, Self::Error> {
            try_fold_fn_arg(self, t)
        }
        fn try_fold_fn_arg_variant_receiver(
            &mut self,
            t: (syn::Receiver),
        ) -> Result<syn::FnArg, Self::Error> {
            try_fold_fn_arg_variant_receiver(self, t)
        }
        fn try_fold_fn_arg_variant_typed(
            &mut self,
            t: (syn::PatType),
        ) -> Result<syn::FnArg, Self::Error> {
            try_fold_fn_arg_variant_typed(self, t)
        }
        fn try_fold_foreign_item(
            &mut self,
            t: syn::ForeignItem,
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item(self, t)
        }
        fn try_fold_foreign_item_variant_fn(
            &mut self,
            t: (syn::ForeignItemFn),
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item_variant_fn(self, t)
        }
        fn try_fold_foreign_item_variant_static(
            &mut self,
            t: (syn::ForeignItemStatic),
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item_variant_static(self, t)
        }
        fn try_fold_foreign_item_variant_type(
            &mut self,
            t: (syn::ForeignItemType),
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item_variant_type(self, t)
        }
        fn try_fold_foreign_item_variant_macro(
            &mut self,
            t: (syn::ForeignItemMacro),
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item_variant_macro(self, t)
        }
        fn try_fold_foreign_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::ForeignItem, Self::Error> {
            try_fold_foreign_item_variant_verbatim(self, t)
        }
        fn try_fold_foreign_item_fn(
            &mut self,
            t: syn::ForeignItemFn,
        ) -> Result<syn::ForeignItemFn, Self::Error> {
            try_fold_foreign_item_fn(self, t)
        }
        fn try_fold_foreign_item_macro(
            &mut self,
            t: syn::ForeignItemMacro,
        ) -> Result<syn::ForeignItemMacro, Self::Error> {
            try_fold_foreign_item_macro(self, t)
        }
        fn try_fold_foreign_item_static(
            &mut self,
            t: syn::ForeignItemStatic,
        ) -> Result<syn::ForeignItemStatic, Self::Error> {
            try_fold_foreign_item_static(self, t)
        }
        fn try_fold_foreign_item_type(
            &mut self,
            t: syn::ForeignItemType,
        ) -> Result<syn::ForeignItemType, Self::Error> {
            try_fold_foreign_item_type(self, t)
        }
        fn try_fold_generic_argument(
            &mut self,
            t: syn::GenericArgument,
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument(self, t)
        }
        fn try_fold_generic_argument_variant_lifetime(
            &mut self,
            t: (syn::Lifetime),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_lifetime(self, t)
        }
        fn try_fold_generic_argument_variant_type(
            &mut self,
            t: (syn::Type),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_type(self, t)
        }
        fn try_fold_generic_argument_variant_const(
            &mut self,
            t: (syn::Expr),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_const(self, t)
        }
        fn try_fold_generic_argument_variant_assoc_type(
            &mut self,
            t: (syn::AssocType),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_assoc_type(self, t)
        }
        fn try_fold_generic_argument_variant_assoc_const(
            &mut self,
            t: (syn::AssocConst),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_assoc_const(self, t)
        }
        fn try_fold_generic_argument_variant_constraint(
            &mut self,
            t: (syn::Constraint),
        ) -> Result<syn::GenericArgument, Self::Error> {
            try_fold_generic_argument_variant_constraint(self, t)
        }
        fn try_fold_generic_param(
            &mut self,
            t: syn::GenericParam,
        ) -> Result<syn::GenericParam, Self::Error> {
            try_fold_generic_param(self, t)
        }
        fn try_fold_generic_param_variant_lifetime(
            &mut self,
            t: (syn::LifetimeParam),
        ) -> Result<syn::GenericParam, Self::Error> {
            try_fold_generic_param_variant_lifetime(self, t)
        }
        fn try_fold_generic_param_variant_type(
            &mut self,
            t: (syn::TypeParam),
        ) -> Result<syn::GenericParam, Self::Error> {
            try_fold_generic_param_variant_type(self, t)
        }
        fn try_fold_generic_param_variant_const(
            &mut self,
            t: (syn::ConstParam),
        ) -> Result<syn::GenericParam, Self::Error> {
            try_fold_generic_param_variant_const(self, t)
        }
        fn try_fold_generics(
            &mut self,
            t: syn::Generics,
        ) -> Result<syn::Generics, Self::Error> {
            try_fold_generics(self, t)
        }
        fn try_fold_impl_item(
            &mut self,
            t: syn::ImplItem,
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item(self, t)
        }
        fn try_fold_impl_item_variant_const(
            &mut self,
            t: (syn::ImplItemConst),
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item_variant_const(self, t)
        }
        fn try_fold_impl_item_variant_fn(
            &mut self,
            t: (syn::ImplItemFn),
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item_variant_fn(self, t)
        }
        fn try_fold_impl_item_variant_type(
            &mut self,
            t: (syn::ImplItemType),
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item_variant_type(self, t)
        }
        fn try_fold_impl_item_variant_macro(
            &mut self,
            t: (syn::ImplItemMacro),
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item_variant_macro(self, t)
        }
        fn try_fold_impl_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::ImplItem, Self::Error> {
            try_fold_impl_item_variant_verbatim(self, t)
        }
        fn try_fold_impl_item_const(
            &mut self,
            t: syn::ImplItemConst,
        ) -> Result<syn::ImplItemConst, Self::Error> {
            try_fold_impl_item_const(self, t)
        }
        fn try_fold_impl_item_fn(
            &mut self,
            t: syn::ImplItemFn,
        ) -> Result<syn::ImplItemFn, Self::Error> {
            try_fold_impl_item_fn(self, t)
        }
        fn try_fold_impl_item_macro(
            &mut self,
            t: syn::ImplItemMacro,
        ) -> Result<syn::ImplItemMacro, Self::Error> {
            try_fold_impl_item_macro(self, t)
        }
        fn try_fold_impl_item_type(
            &mut self,
            t: syn::ImplItemType,
        ) -> Result<syn::ImplItemType, Self::Error> {
            try_fold_impl_item_type(self, t)
        }
        fn try_fold_impl_restriction(
            &mut self,
            t: syn::ImplRestriction,
        ) -> Result<syn::ImplRestriction, Self::Error> {
            try_fold_impl_restriction(self, t)
        }
        fn try_fold_index(&mut self, t: syn::Index) -> Result<syn::Index, Self::Error> {
            try_fold_index(self, t)
        }
        fn try_fold_item(&mut self, t: syn::Item) -> Result<syn::Item, Self::Error> {
            try_fold_item(self, t)
        }
        fn try_fold_item_variant_const(
            &mut self,
            t: (syn::ItemConst),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_const(self, t)
        }
        fn try_fold_item_variant_enum(
            &mut self,
            t: (syn::ItemEnum),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_enum(self, t)
        }
        fn try_fold_item_variant_extern_crate(
            &mut self,
            t: (syn::ItemExternCrate),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_extern_crate(self, t)
        }
        fn try_fold_item_variant_fn(
            &mut self,
            t: (syn::ItemFn),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_fn(self, t)
        }
        fn try_fold_item_variant_foreign_mod(
            &mut self,
            t: (syn::ItemForeignMod),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_foreign_mod(self, t)
        }
        fn try_fold_item_variant_impl(
            &mut self,
            t: (syn::ItemImpl),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_impl(self, t)
        }
        fn try_fold_item_variant_macro(
            &mut self,
            t: (syn::ItemMacro),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_macro(self, t)
        }
        fn try_fold_item_variant_mod(
            &mut self,
            t: (syn::ItemMod),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_mod(self, t)
        }
        fn try_fold_item_variant_static(
            &mut self,
            t: (syn::ItemStatic),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_static(self, t)
        }
        fn try_fold_item_variant_struct(
            &mut self,
            t: (syn::ItemStruct),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_struct(self, t)
        }
        fn try_fold_item_variant_trait(
            &mut self,
            t: (syn::ItemTrait),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_trait(self, t)
        }
        fn try_fold_item_variant_trait_alias(
            &mut self,
            t: (syn::ItemTraitAlias),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_trait_alias(self, t)
        }
        fn try_fold_item_variant_type(
            &mut self,
            t: (syn::ItemType),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_type(self, t)
        }
        fn try_fold_item_variant_union(
            &mut self,
            t: (syn::ItemUnion),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_union(self, t)
        }
        fn try_fold_item_variant_use(
            &mut self,
            t: (syn::ItemUse),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_use(self, t)
        }
        fn try_fold_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::Item, Self::Error> {
            try_fold_item_variant_verbatim(self, t)
        }
        fn try_fold_item_const(
            &mut self,
            t: syn::ItemConst,
        ) -> Result<syn::ItemConst, Self::Error> {
            try_fold_item_const(self, t)
        }
        fn try_fold_item_enum(
            &mut self,
            t: syn::ItemEnum,
        ) -> Result<syn::ItemEnum, Self::Error> {
            try_fold_item_enum(self, t)
        }
        fn try_fold_item_extern_crate(
            &mut self,
            t: syn::ItemExternCrate,
        ) -> Result<syn::ItemExternCrate, Self::Error> {
            try_fold_item_extern_crate(self, t)
        }
        fn try_fold_item_fn(
            &mut self,
            t: syn::ItemFn,
        ) -> Result<syn::ItemFn, Self::Error> {
            try_fold_item_fn(self, t)
        }
        fn try_fold_item_foreign_mod(
            &mut self,
            t: syn::ItemForeignMod,
        ) -> Result<syn::ItemForeignMod, Self::Error> {
            try_fold_item_foreign_mod(self, t)
        }
        fn try_fold_item_impl(
            &mut self,
            t: syn::ItemImpl,
        ) -> Result<syn::ItemImpl, Self::Error> {
            try_fold_item_impl(self, t)
        }
        fn try_fold_item_macro(
            &mut self,
            t: syn::ItemMacro,
        ) -> Result<syn::ItemMacro, Self::Error> {
            try_fold_item_macro(self, t)
        }
        fn try_fold_item_mod(
            &mut self,
            t: syn::ItemMod,
        ) -> Result<syn::ItemMod, Self::Error> {
            try_fold_item_mod(self, t)
        }
        fn try_fold_item_static(
            &mut self,
            t: syn::ItemStatic,
        ) -> Result<syn::ItemStatic, Self::Error> {
            try_fold_item_static(self, t)
        }
        fn try_fold_item_struct(
            &mut self,
            t: syn::ItemStruct,
        ) -> Result<syn::ItemStruct, Self::Error> {
            try_fold_item_struct(self, t)
        }
        fn try_fold_item_trait(
            &mut self,
            t: syn::ItemTrait,
        ) -> Result<syn::ItemTrait, Self::Error> {
            try_fold_item_trait(self, t)
        }
        fn try_fold_item_trait_alias(
            &mut self,
            t: syn::ItemTraitAlias,
        ) -> Result<syn::ItemTraitAlias, Self::Error> {
            try_fold_item_trait_alias(self, t)
        }
        fn try_fold_item_type(
            &mut self,
            t: syn::ItemType,
        ) -> Result<syn::ItemType, Self::Error> {
            try_fold_item_type(self, t)
        }
        fn try_fold_item_union(
            &mut self,
            t: syn::ItemUnion,
        ) -> Result<syn::ItemUnion, Self::Error> {
            try_fold_item_union(self, t)
        }
        fn try_fold_item_use(
            &mut self,
            t: syn::ItemUse,
        ) -> Result<syn::ItemUse, Self::Error> {
            try_fold_item_use(self, t)
        }
        fn try_fold_label(&mut self, t: syn::Label) -> Result<syn::Label, Self::Error> {
            try_fold_label(self, t)
        }
        fn try_fold_lifetime(
            &mut self,
            t: syn::Lifetime,
        ) -> Result<syn::Lifetime, Self::Error> {
            try_fold_lifetime(self, t)
        }
        fn try_fold_lifetime_param(
            &mut self,
            t: syn::LifetimeParam,
        ) -> Result<syn::LifetimeParam, Self::Error> {
            try_fold_lifetime_param(self, t)
        }
        fn try_fold_lit(&mut self, t: syn::Lit) -> Result<syn::Lit, Self::Error> {
            try_fold_lit(self, t)
        }
        fn try_fold_lit_variant_str(
            &mut self,
            t: (syn::LitStr),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_str(self, t)
        }
        fn try_fold_lit_variant_byte_str(
            &mut self,
            t: (syn::LitByteStr),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_byte_str(self, t)
        }
        fn try_fold_lit_variant_byte(
            &mut self,
            t: (syn::LitByte),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_byte(self, t)
        }
        fn try_fold_lit_variant_char(
            &mut self,
            t: (syn::LitChar),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_char(self, t)
        }
        fn try_fold_lit_variant_int(
            &mut self,
            t: (syn::LitInt),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_int(self, t)
        }
        fn try_fold_lit_variant_float(
            &mut self,
            t: (syn::LitFloat),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_float(self, t)
        }
        fn try_fold_lit_variant_bool(
            &mut self,
            t: (syn::LitBool),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_bool(self, t)
        }
        fn try_fold_lit_variant_verbatim(
            &mut self,
            t: (proc_macro2::Literal),
        ) -> Result<syn::Lit, Self::Error> {
            try_fold_lit_variant_verbatim(self, t)
        }
        fn try_fold_lit_bool(
            &mut self,
            t: syn::LitBool,
        ) -> Result<syn::LitBool, Self::Error> {
            try_fold_lit_bool(self, t)
        }
        fn try_fold_lit_byte(
            &mut self,
            t: syn::LitByte,
        ) -> Result<syn::LitByte, Self::Error> {
            try_fold_lit_byte(self, t)
        }
        fn try_fold_lit_byte_str(
            &mut self,
            t: syn::LitByteStr,
        ) -> Result<syn::LitByteStr, Self::Error> {
            try_fold_lit_byte_str(self, t)
        }
        fn try_fold_lit_char(
            &mut self,
            t: syn::LitChar,
        ) -> Result<syn::LitChar, Self::Error> {
            try_fold_lit_char(self, t)
        }
        fn try_fold_lit_float(
            &mut self,
            t: syn::LitFloat,
        ) -> Result<syn::LitFloat, Self::Error> {
            try_fold_lit_float(self, t)
        }
        fn try_fold_lit_int(
            &mut self,
            t: syn::LitInt,
        ) -> Result<syn::LitInt, Self::Error> {
            try_fold_lit_int(self, t)
        }
        fn try_fold_lit_str(
            &mut self,
            t: syn::LitStr,
        ) -> Result<syn::LitStr, Self::Error> {
            try_fold_lit_str(self, t)
        }
        fn try_fold_local(&mut self, t: syn::Local) -> Result<syn::Local, Self::Error> {
            try_fold_local(self, t)
        }
        fn try_fold_local_init(
            &mut self,
            t: syn::LocalInit,
        ) -> Result<syn::LocalInit, Self::Error> {
            try_fold_local_init(self, t)
        }
        fn try_fold_macro(&mut self, t: syn::Macro) -> Result<syn::Macro, Self::Error> {
            try_fold_macro(self, t)
        }
        fn try_fold_macro_delimiter(
            &mut self,
            t: syn::MacroDelimiter,
        ) -> Result<syn::MacroDelimiter, Self::Error> {
            try_fold_macro_delimiter(self, t)
        }
        fn try_fold_macro_delimiter_variant_paren(
            &mut self,
            t: (syn::token::Paren),
        ) -> Result<syn::MacroDelimiter, Self::Error> {
            try_fold_macro_delimiter_variant_paren(self, t)
        }
        fn try_fold_macro_delimiter_variant_brace(
            &mut self,
            t: (syn::token::Brace),
        ) -> Result<syn::MacroDelimiter, Self::Error> {
            try_fold_macro_delimiter_variant_brace(self, t)
        }
        fn try_fold_macro_delimiter_variant_bracket(
            &mut self,
            t: (syn::token::Bracket),
        ) -> Result<syn::MacroDelimiter, Self::Error> {
            try_fold_macro_delimiter_variant_bracket(self, t)
        }
        fn try_fold_member(
            &mut self,
            t: syn::Member,
        ) -> Result<syn::Member, Self::Error> {
            try_fold_member(self, t)
        }
        fn try_fold_member_variant_named(
            &mut self,
            t: (proc_macro2::Ident),
        ) -> Result<syn::Member, Self::Error> {
            try_fold_member_variant_named(self, t)
        }
        fn try_fold_member_variant_unnamed(
            &mut self,
            t: (syn::Index),
        ) -> Result<syn::Member, Self::Error> {
            try_fold_member_variant_unnamed(self, t)
        }
        fn try_fold_meta(&mut self, t: syn::Meta) -> Result<syn::Meta, Self::Error> {
            try_fold_meta(self, t)
        }
        fn try_fold_meta_variant_path(
            &mut self,
            t: (syn::Path),
        ) -> Result<syn::Meta, Self::Error> {
            try_fold_meta_variant_path(self, t)
        }
        fn try_fold_meta_variant_list(
            &mut self,
            t: (syn::MetaList),
        ) -> Result<syn::Meta, Self::Error> {
            try_fold_meta_variant_list(self, t)
        }
        fn try_fold_meta_variant_name_value(
            &mut self,
            t: (syn::MetaNameValue),
        ) -> Result<syn::Meta, Self::Error> {
            try_fold_meta_variant_name_value(self, t)
        }
        fn try_fold_meta_list(
            &mut self,
            t: syn::MetaList,
        ) -> Result<syn::MetaList, Self::Error> {
            try_fold_meta_list(self, t)
        }
        fn try_fold_meta_name_value(
            &mut self,
            t: syn::MetaNameValue,
        ) -> Result<syn::MetaNameValue, Self::Error> {
            try_fold_meta_name_value(self, t)
        }
        fn try_fold_parenthesized_generic_arguments(
            &mut self,
            t: syn::ParenthesizedGenericArguments,
        ) -> Result<syn::ParenthesizedGenericArguments, Self::Error> {
            try_fold_parenthesized_generic_arguments(self, t)
        }
        fn try_fold_pat(&mut self, t: syn::Pat) -> Result<syn::Pat, Self::Error> {
            try_fold_pat(self, t)
        }
        fn try_fold_pat_variant_const(
            &mut self,
            t: (syn::ExprConst),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_const(self, t)
        }
        fn try_fold_pat_variant_ident(
            &mut self,
            t: (syn::PatIdent),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_ident(self, t)
        }
        fn try_fold_pat_variant_lit(
            &mut self,
            t: (syn::ExprLit),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_lit(self, t)
        }
        fn try_fold_pat_variant_macro(
            &mut self,
            t: (syn::ExprMacro),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_macro(self, t)
        }
        fn try_fold_pat_variant_or(
            &mut self,
            t: (syn::PatOr),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_or(self, t)
        }
        fn try_fold_pat_variant_paren(
            &mut self,
            t: (syn::PatParen),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_paren(self, t)
        }
        fn try_fold_pat_variant_path(
            &mut self,
            t: (syn::ExprPath),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_path(self, t)
        }
        fn try_fold_pat_variant_range(
            &mut self,
            t: (syn::ExprRange),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_range(self, t)
        }
        fn try_fold_pat_variant_reference(
            &mut self,
            t: (syn::PatReference),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_reference(self, t)
        }
        fn try_fold_pat_variant_rest(
            &mut self,
            t: (syn::PatRest),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_rest(self, t)
        }
        fn try_fold_pat_variant_slice(
            &mut self,
            t: (syn::PatSlice),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_slice(self, t)
        }
        fn try_fold_pat_variant_struct(
            &mut self,
            t: (syn::PatStruct),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_struct(self, t)
        }
        fn try_fold_pat_variant_tuple(
            &mut self,
            t: (syn::PatTuple),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_tuple(self, t)
        }
        fn try_fold_pat_variant_tuple_struct(
            &mut self,
            t: (syn::PatTupleStruct),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_tuple_struct(self, t)
        }
        fn try_fold_pat_variant_type(
            &mut self,
            t: (syn::PatType),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_type(self, t)
        }
        fn try_fold_pat_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_verbatim(self, t)
        }
        fn try_fold_pat_variant_wild(
            &mut self,
            t: (syn::PatWild),
        ) -> Result<syn::Pat, Self::Error> {
            try_fold_pat_variant_wild(self, t)
        }
        fn try_fold_pat_ident(
            &mut self,
            t: syn::PatIdent,
        ) -> Result<syn::PatIdent, Self::Error> {
            try_fold_pat_ident(self, t)
        }
        fn try_fold_pat_or(&mut self, t: syn::PatOr) -> Result<syn::PatOr, Self::Error> {
            try_fold_pat_or(self, t)
        }
        fn try_fold_pat_paren(
            &mut self,
            t: syn::PatParen,
        ) -> Result<syn::PatParen, Self::Error> {
            try_fold_pat_paren(self, t)
        }
        fn try_fold_pat_reference(
            &mut self,
            t: syn::PatReference,
        ) -> Result<syn::PatReference, Self::Error> {
            try_fold_pat_reference(self, t)
        }
        fn try_fold_pat_rest(
            &mut self,
            t: syn::PatRest,
        ) -> Result<syn::PatRest, Self::Error> {
            try_fold_pat_rest(self, t)
        }
        fn try_fold_pat_slice(
            &mut self,
            t: syn::PatSlice,
        ) -> Result<syn::PatSlice, Self::Error> {
            try_fold_pat_slice(self, t)
        }
        fn try_fold_pat_struct(
            &mut self,
            t: syn::PatStruct,
        ) -> Result<syn::PatStruct, Self::Error> {
            try_fold_pat_struct(self, t)
        }
        fn try_fold_pat_tuple(
            &mut self,
            t: syn::PatTuple,
        ) -> Result<syn::PatTuple, Self::Error> {
            try_fold_pat_tuple(self, t)
        }
        fn try_fold_pat_tuple_struct(
            &mut self,
            t: syn::PatTupleStruct,
        ) -> Result<syn::PatTupleStruct, Self::Error> {
            try_fold_pat_tuple_struct(self, t)
        }
        fn try_fold_pat_type(
            &mut self,
            t: syn::PatType,
        ) -> Result<syn::PatType, Self::Error> {
            try_fold_pat_type(self, t)
        }
        fn try_fold_pat_wild(
            &mut self,
            t: syn::PatWild,
        ) -> Result<syn::PatWild, Self::Error> {
            try_fold_pat_wild(self, t)
        }
        fn try_fold_path(&mut self, t: syn::Path) -> Result<syn::Path, Self::Error> {
            try_fold_path(self, t)
        }
        fn try_fold_path_arguments(
            &mut self,
            t: syn::PathArguments,
        ) -> Result<syn::PathArguments, Self::Error> {
            try_fold_path_arguments(self, t)
        }
        fn try_fold_path_arguments_variant_angle_bracketed(
            &mut self,
            t: (syn::AngleBracketedGenericArguments),
        ) -> Result<syn::PathArguments, Self::Error> {
            try_fold_path_arguments_variant_angle_bracketed(self, t)
        }
        fn try_fold_path_arguments_variant_parenthesized(
            &mut self,
            t: (syn::ParenthesizedGenericArguments),
        ) -> Result<syn::PathArguments, Self::Error> {
            try_fold_path_arguments_variant_parenthesized(self, t)
        }
        fn try_fold_path_segment(
            &mut self,
            t: syn::PathSegment,
        ) -> Result<syn::PathSegment, Self::Error> {
            try_fold_path_segment(self, t)
        }
        fn try_fold_predicate_lifetime(
            &mut self,
            t: syn::PredicateLifetime,
        ) -> Result<syn::PredicateLifetime, Self::Error> {
            try_fold_predicate_lifetime(self, t)
        }
        fn try_fold_predicate_type(
            &mut self,
            t: syn::PredicateType,
        ) -> Result<syn::PredicateType, Self::Error> {
            try_fold_predicate_type(self, t)
        }
        fn try_fold_q_self(&mut self, t: syn::QSelf) -> Result<syn::QSelf, Self::Error> {
            try_fold_q_self(self, t)
        }
        fn try_fold_range_limits(
            &mut self,
            t: syn::RangeLimits,
        ) -> Result<syn::RangeLimits, Self::Error> {
            try_fold_range_limits(self, t)
        }
        fn try_fold_range_limits_variant_half_open(
            &mut self,
            t: (syn::token::DotDot),
        ) -> Result<syn::RangeLimits, Self::Error> {
            try_fold_range_limits_variant_half_open(self, t)
        }
        fn try_fold_range_limits_variant_closed(
            &mut self,
            t: (syn::token::DotDotEq),
        ) -> Result<syn::RangeLimits, Self::Error> {
            try_fold_range_limits_variant_closed(self, t)
        }
        fn try_fold_receiver(
            &mut self,
            t: syn::Receiver,
        ) -> Result<syn::Receiver, Self::Error> {
            try_fold_receiver(self, t)
        }
        fn try_fold_return_type(
            &mut self,
            t: syn::ReturnType,
        ) -> Result<syn::ReturnType, Self::Error> {
            try_fold_return_type(self, t)
        }
        fn try_fold_return_type_variant_type(
            &mut self,
            t: (syn::token::RArrow, Box<syn::Type>),
        ) -> Result<syn::ReturnType, Self::Error> {
            try_fold_return_type_variant_type(self, t)
        }
        fn try_fold_signature(
            &mut self,
            t: syn::Signature,
        ) -> Result<syn::Signature, Self::Error> {
            try_fold_signature(self, t)
        }
        fn try_fold_static_mutability(
            &mut self,
            t: syn::StaticMutability,
        ) -> Result<syn::StaticMutability, Self::Error> {
            try_fold_static_mutability(self, t)
        }
        fn try_fold_static_mutability_variant_mut(
            &mut self,
            t: (syn::token::Mut),
        ) -> Result<syn::StaticMutability, Self::Error> {
            try_fold_static_mutability_variant_mut(self, t)
        }
        fn try_fold_stmt(&mut self, t: syn::Stmt) -> Result<syn::Stmt, Self::Error> {
            try_fold_stmt(self, t)
        }
        fn try_fold_stmt_variant_local(
            &mut self,
            t: (syn::Local),
        ) -> Result<syn::Stmt, Self::Error> {
            try_fold_stmt_variant_local(self, t)
        }
        fn try_fold_stmt_variant_item(
            &mut self,
            t: (syn::Item),
        ) -> Result<syn::Stmt, Self::Error> {
            try_fold_stmt_variant_item(self, t)
        }
        fn try_fold_stmt_variant_expr(
            &mut self,
            t: (syn::Expr, Option<syn::token::Semi>),
        ) -> Result<syn::Stmt, Self::Error> {
            try_fold_stmt_variant_expr(self, t)
        }
        fn try_fold_stmt_variant_macro(
            &mut self,
            t: (syn::StmtMacro),
        ) -> Result<syn::Stmt, Self::Error> {
            try_fold_stmt_variant_macro(self, t)
        }
        fn try_fold_stmt_macro(
            &mut self,
            t: syn::StmtMacro,
        ) -> Result<syn::StmtMacro, Self::Error> {
            try_fold_stmt_macro(self, t)
        }
        fn try_fold_trait_bound(
            &mut self,
            t: syn::TraitBound,
        ) -> Result<syn::TraitBound, Self::Error> {
            try_fold_trait_bound(self, t)
        }
        fn try_fold_trait_bound_modifier(
            &mut self,
            t: syn::TraitBoundModifier,
        ) -> Result<syn::TraitBoundModifier, Self::Error> {
            try_fold_trait_bound_modifier(self, t)
        }
        fn try_fold_trait_bound_modifier_variant_maybe(
            &mut self,
            t: (syn::token::Question),
        ) -> Result<syn::TraitBoundModifier, Self::Error> {
            try_fold_trait_bound_modifier_variant_maybe(self, t)
        }
        fn try_fold_trait_item(
            &mut self,
            t: syn::TraitItem,
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item(self, t)
        }
        fn try_fold_trait_item_variant_const(
            &mut self,
            t: (syn::TraitItemConst),
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item_variant_const(self, t)
        }
        fn try_fold_trait_item_variant_fn(
            &mut self,
            t: (syn::TraitItemFn),
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item_variant_fn(self, t)
        }
        fn try_fold_trait_item_variant_type(
            &mut self,
            t: (syn::TraitItemType),
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item_variant_type(self, t)
        }
        fn try_fold_trait_item_variant_macro(
            &mut self,
            t: (syn::TraitItemMacro),
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item_variant_macro(self, t)
        }
        fn try_fold_trait_item_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::TraitItem, Self::Error> {
            try_fold_trait_item_variant_verbatim(self, t)
        }
        fn try_fold_trait_item_const(
            &mut self,
            t: syn::TraitItemConst,
        ) -> Result<syn::TraitItemConst, Self::Error> {
            try_fold_trait_item_const(self, t)
        }
        fn try_fold_trait_item_fn(
            &mut self,
            t: syn::TraitItemFn,
        ) -> Result<syn::TraitItemFn, Self::Error> {
            try_fold_trait_item_fn(self, t)
        }
        fn try_fold_trait_item_macro(
            &mut self,
            t: syn::TraitItemMacro,
        ) -> Result<syn::TraitItemMacro, Self::Error> {
            try_fold_trait_item_macro(self, t)
        }
        fn try_fold_trait_item_type(
            &mut self,
            t: syn::TraitItemType,
        ) -> Result<syn::TraitItemType, Self::Error> {
            try_fold_trait_item_type(self, t)
        }
        fn try_fold_type(&mut self, t: syn::Type) -> Result<syn::Type, Self::Error> {
            try_fold_type(self, t)
        }
        fn try_fold_type_variant_array(
            &mut self,
            t: (syn::TypeArray),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_array(self, t)
        }
        fn try_fold_type_variant_bare_fn(
            &mut self,
            t: (syn::TypeBareFn),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_bare_fn(self, t)
        }
        fn try_fold_type_variant_group(
            &mut self,
            t: (syn::TypeGroup),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_group(self, t)
        }
        fn try_fold_type_variant_impl_trait(
            &mut self,
            t: (syn::TypeImplTrait),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_impl_trait(self, t)
        }
        fn try_fold_type_variant_infer(
            &mut self,
            t: (syn::TypeInfer),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_infer(self, t)
        }
        fn try_fold_type_variant_macro(
            &mut self,
            t: (syn::TypeMacro),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_macro(self, t)
        }
        fn try_fold_type_variant_never(
            &mut self,
            t: (syn::TypeNever),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_never(self, t)
        }
        fn try_fold_type_variant_paren(
            &mut self,
            t: (syn::TypeParen),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_paren(self, t)
        }
        fn try_fold_type_variant_path(
            &mut self,
            t: (syn::TypePath),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_path(self, t)
        }
        fn try_fold_type_variant_ptr(
            &mut self,
            t: (syn::TypePtr),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_ptr(self, t)
        }
        fn try_fold_type_variant_reference(
            &mut self,
            t: (syn::TypeReference),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_reference(self, t)
        }
        fn try_fold_type_variant_slice(
            &mut self,
            t: (syn::TypeSlice),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_slice(self, t)
        }
        fn try_fold_type_variant_trait_object(
            &mut self,
            t: (syn::TypeTraitObject),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_trait_object(self, t)
        }
        fn try_fold_type_variant_tuple(
            &mut self,
            t: (syn::TypeTuple),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_tuple(self, t)
        }
        fn try_fold_type_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::Type, Self::Error> {
            try_fold_type_variant_verbatim(self, t)
        }
        fn try_fold_type_array(
            &mut self,
            t: syn::TypeArray,
        ) -> Result<syn::TypeArray, Self::Error> {
            try_fold_type_array(self, t)
        }
        fn try_fold_type_bare_fn(
            &mut self,
            t: syn::TypeBareFn,
        ) -> Result<syn::TypeBareFn, Self::Error> {
            try_fold_type_bare_fn(self, t)
        }
        fn try_fold_type_group(
            &mut self,
            t: syn::TypeGroup,
        ) -> Result<syn::TypeGroup, Self::Error> {
            try_fold_type_group(self, t)
        }
        fn try_fold_type_impl_trait(
            &mut self,
            t: syn::TypeImplTrait,
        ) -> Result<syn::TypeImplTrait, Self::Error> {
            try_fold_type_impl_trait(self, t)
        }
        fn try_fold_type_infer(
            &mut self,
            t: syn::TypeInfer,
        ) -> Result<syn::TypeInfer, Self::Error> {
            try_fold_type_infer(self, t)
        }
        fn try_fold_type_macro(
            &mut self,
            t: syn::TypeMacro,
        ) -> Result<syn::TypeMacro, Self::Error> {
            try_fold_type_macro(self, t)
        }
        fn try_fold_type_never(
            &mut self,
            t: syn::TypeNever,
        ) -> Result<syn::TypeNever, Self::Error> {
            try_fold_type_never(self, t)
        }
        fn try_fold_type_param(
            &mut self,
            t: syn::TypeParam,
        ) -> Result<syn::TypeParam, Self::Error> {
            try_fold_type_param(self, t)
        }
        fn try_fold_type_param_bound(
            &mut self,
            t: syn::TypeParamBound,
        ) -> Result<syn::TypeParamBound, Self::Error> {
            try_fold_type_param_bound(self, t)
        }
        fn try_fold_type_param_bound_variant_trait(
            &mut self,
            t: (syn::TraitBound),
        ) -> Result<syn::TypeParamBound, Self::Error> {
            try_fold_type_param_bound_variant_trait(self, t)
        }
        fn try_fold_type_param_bound_variant_lifetime(
            &mut self,
            t: (syn::Lifetime),
        ) -> Result<syn::TypeParamBound, Self::Error> {
            try_fold_type_param_bound_variant_lifetime(self, t)
        }
        fn try_fold_type_param_bound_variant_verbatim(
            &mut self,
            t: (proc_macro2::TokenStream),
        ) -> Result<syn::TypeParamBound, Self::Error> {
            try_fold_type_param_bound_variant_verbatim(self, t)
        }
        fn try_fold_type_paren(
            &mut self,
            t: syn::TypeParen,
        ) -> Result<syn::TypeParen, Self::Error> {
            try_fold_type_paren(self, t)
        }
        fn try_fold_type_path(
            &mut self,
            t: syn::TypePath,
        ) -> Result<syn::TypePath, Self::Error> {
            try_fold_type_path(self, t)
        }
        fn try_fold_type_ptr(
            &mut self,
            t: syn::TypePtr,
        ) -> Result<syn::TypePtr, Self::Error> {
            try_fold_type_ptr(self, t)
        }
        fn try_fold_type_reference(
            &mut self,
            t: syn::TypeReference,
        ) -> Result<syn::TypeReference, Self::Error> {
            try_fold_type_reference(self, t)
        }
        fn try_fold_type_slice(
            &mut self,
            t: syn::TypeSlice,
        ) -> Result<syn::TypeSlice, Self::Error> {
            try_fold_type_slice(self, t)
        }
        fn try_fold_type_trait_object(
            &mut self,
            t: syn::TypeTraitObject,
        ) -> Result<syn::TypeTraitObject, Self::Error> {
            try_fold_type_trait_object(self, t)
        }
        fn try_fold_type_tuple(
            &mut self,
            t: syn::TypeTuple,
        ) -> Result<syn::TypeTuple, Self::Error> {
            try_fold_type_tuple(self, t)
        }
        fn try_fold_un_op(&mut self, t: syn::UnOp) -> Result<syn::UnOp, Self::Error> {
            try_fold_un_op(self, t)
        }
        fn try_fold_un_op_variant_deref(
            &mut self,
            t: (syn::token::Star),
        ) -> Result<syn::UnOp, Self::Error> {
            try_fold_un_op_variant_deref(self, t)
        }
        fn try_fold_un_op_variant_not(
            &mut self,
            t: (syn::token::Not),
        ) -> Result<syn::UnOp, Self::Error> {
            try_fold_un_op_variant_not(self, t)
        }
        fn try_fold_un_op_variant_neg(
            &mut self,
            t: (syn::token::Minus),
        ) -> Result<syn::UnOp, Self::Error> {
            try_fold_un_op_variant_neg(self, t)
        }
        fn try_fold_use_glob(
            &mut self,
            t: syn::UseGlob,
        ) -> Result<syn::UseGlob, Self::Error> {
            try_fold_use_glob(self, t)
        }
        fn try_fold_use_group(
            &mut self,
            t: syn::UseGroup,
        ) -> Result<syn::UseGroup, Self::Error> {
            try_fold_use_group(self, t)
        }
        fn try_fold_use_name(
            &mut self,
            t: syn::UseName,
        ) -> Result<syn::UseName, Self::Error> {
            try_fold_use_name(self, t)
        }
        fn try_fold_use_path(
            &mut self,
            t: syn::UsePath,
        ) -> Result<syn::UsePath, Self::Error> {
            try_fold_use_path(self, t)
        }
        fn try_fold_use_rename(
            &mut self,
            t: syn::UseRename,
        ) -> Result<syn::UseRename, Self::Error> {
            try_fold_use_rename(self, t)
        }
        fn try_fold_use_tree(
            &mut self,
            t: syn::UseTree,
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree(self, t)
        }
        fn try_fold_use_tree_variant_path(
            &mut self,
            t: (syn::UsePath),
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree_variant_path(self, t)
        }
        fn try_fold_use_tree_variant_name(
            &mut self,
            t: (syn::UseName),
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree_variant_name(self, t)
        }
        fn try_fold_use_tree_variant_rename(
            &mut self,
            t: (syn::UseRename),
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree_variant_rename(self, t)
        }
        fn try_fold_use_tree_variant_glob(
            &mut self,
            t: (syn::UseGlob),
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree_variant_glob(self, t)
        }
        fn try_fold_use_tree_variant_group(
            &mut self,
            t: (syn::UseGroup),
        ) -> Result<syn::UseTree, Self::Error> {
            try_fold_use_tree_variant_group(self, t)
        }
        fn try_fold_variadic(
            &mut self,
            t: syn::Variadic,
        ) -> Result<syn::Variadic, Self::Error> {
            try_fold_variadic(self, t)
        }
        fn try_fold_variant(
            &mut self,
            t: syn::Variant,
        ) -> Result<syn::Variant, Self::Error> {
            try_fold_variant(self, t)
        }
        fn try_fold_vis_restricted(
            &mut self,
            t: syn::VisRestricted,
        ) -> Result<syn::VisRestricted, Self::Error> {
            try_fold_vis_restricted(self, t)
        }
        fn try_fold_visibility(
            &mut self,
            t: syn::Visibility,
        ) -> Result<syn::Visibility, Self::Error> {
            try_fold_visibility(self, t)
        }
        fn try_fold_visibility_variant_public(
            &mut self,
            t: (syn::token::Pub),
        ) -> Result<syn::Visibility, Self::Error> {
            try_fold_visibility_variant_public(self, t)
        }
        fn try_fold_visibility_variant_restricted(
            &mut self,
            t: (syn::VisRestricted),
        ) -> Result<syn::Visibility, Self::Error> {
            try_fold_visibility_variant_restricted(self, t)
        }
        fn try_fold_where_clause(
            &mut self,
            t: syn::WhereClause,
        ) -> Result<syn::WhereClause, Self::Error> {
            try_fold_where_clause(self, t)
        }
        fn try_fold_where_predicate(
            &mut self,
            t: syn::WherePredicate,
        ) -> Result<syn::WherePredicate, Self::Error> {
            try_fold_where_predicate(self, t)
        }
        fn try_fold_where_predicate_variant_lifetime(
            &mut self,
            t: (syn::PredicateLifetime),
        ) -> Result<syn::WherePredicate, Self::Error> {
            try_fold_where_predicate_variant_lifetime(self, t)
        }
        fn try_fold_where_predicate_variant_type(
            &mut self,
            t: (syn::PredicateType),
        ) -> Result<syn::WherePredicate, Self::Error> {
            try_fold_where_predicate_variant_type(self, t)
        }
    }
    pub fn try_fold_abi<F>(
        f: &mut F,
        t: syn::Abi,
    ) -> Result<syn::Abi, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.extern_token = t.extern_token;
        t
            .name = match t.name {
            Some(o) => Some(f.try_fold_lit_str(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_angle_bracketed_generic_arguments<F>(
        f: &mut F,
        t: syn::AngleBracketedGenericArguments,
    ) -> Result<syn::AngleBracketedGenericArguments, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .colon2_token = match t.colon2_token {
            Some(o) => Some(o),
            None => None,
        };
        t.lt_token = t.lt_token;
        t
            .args = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.args {
                tmp.push(f.try_fold_generic_argument(v)?);
            }
            tmp
        };
        t.gt_token = t.gt_token;
        Ok(t)
    }
    pub fn try_fold_arm<F>(
        f: &mut F,
        t: syn::Arm,
    ) -> Result<syn::Arm, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.pat = f.try_fold_pat(t.pat)?;
        t
            .guard = match t.guard {
            Some(o) => Some((o.0, Box::new(f.try_fold_expr(*o.1)?))),
            None => None,
        };
        t.fat_arrow_token = t.fat_arrow_token;
        t.body = Box::new(f.try_fold_expr(*t.body)?);
        t
            .comma = match t.comma {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_assoc_const<F>(
        f: &mut F,
        t: syn::AssocConst,
    ) -> Result<syn::AssocConst, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t
            .generics = match t.generics {
            Some(o) => Some(f.try_fold_angle_bracketed_generic_arguments(o)?),
            None => None,
        };
        t.eq_token = t.eq_token;
        t.value = f.try_fold_expr(t.value)?;
        Ok(t)
    }
    pub fn try_fold_assoc_type<F>(
        f: &mut F,
        t: syn::AssocType,
    ) -> Result<syn::AssocType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t
            .generics = match t.generics {
            Some(o) => Some(f.try_fold_angle_bracketed_generic_arguments(o)?),
            None => None,
        };
        t.eq_token = t.eq_token;
        t.ty = f.try_fold_type(t.ty)?;
        Ok(t)
    }
    pub fn try_fold_attr_style<F>(
        f: &mut F,
        t: syn::AttrStyle,
    ) -> Result<syn::AttrStyle, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::AttrStyle::Inner(tmp0) => f.try_fold_attr_style_variant_inner((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_attr_style_variant_inner<F>(
        f: &mut F,
        t: (syn::token::Not),
    ) -> Result<syn::AttrStyle, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::AttrStyle::Inner(t))
    }
    pub fn try_fold_attribute<F>(
        f: &mut F,
        t: syn::Attribute,
    ) -> Result<syn::Attribute, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.pound_token = t.pound_token;
        t.style = f.try_fold_attr_style(t.style)?;
        t.bracket_token = t.bracket_token;
        t.meta = f.try_fold_meta(t.meta)?;
        Ok(t)
    }
    pub fn try_fold_bare_fn_arg<F>(
        f: &mut F,
        t: syn::BareFnArg,
    ) -> Result<syn::BareFnArg, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .name = match t.name {
            Some(o) => Some((o.0, o.1)),
            None => None,
        };
        t.ty = f.try_fold_type(t.ty)?;
        Ok(t)
    }
    pub fn try_fold_bare_variadic<F>(
        f: &mut F,
        t: syn::BareVariadic,
    ) -> Result<syn::BareVariadic, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .name = match t.name {
            Some(o) => Some((o.0, o.1)),
            None => None,
        };
        t.dots = t.dots;
        t
            .comma = match t.comma {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_bin_op<F>(
        f: &mut F,
        t: syn::BinOp,
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::BinOp::Add(tmp0) => f.try_fold_bin_op_variant_add((tmp0))?,
            syn::BinOp::Sub(tmp0) => f.try_fold_bin_op_variant_sub((tmp0))?,
            syn::BinOp::Mul(tmp0) => f.try_fold_bin_op_variant_mul((tmp0))?,
            syn::BinOp::Div(tmp0) => f.try_fold_bin_op_variant_div((tmp0))?,
            syn::BinOp::Rem(tmp0) => f.try_fold_bin_op_variant_rem((tmp0))?,
            syn::BinOp::And(tmp0) => f.try_fold_bin_op_variant_and((tmp0))?,
            syn::BinOp::Or(tmp0) => f.try_fold_bin_op_variant_or((tmp0))?,
            syn::BinOp::BitXor(tmp0) => f.try_fold_bin_op_variant_bit_xor((tmp0))?,
            syn::BinOp::BitAnd(tmp0) => f.try_fold_bin_op_variant_bit_and((tmp0))?,
            syn::BinOp::BitOr(tmp0) => f.try_fold_bin_op_variant_bit_or((tmp0))?,
            syn::BinOp::Shl(tmp0) => f.try_fold_bin_op_variant_shl((tmp0))?,
            syn::BinOp::Shr(tmp0) => f.try_fold_bin_op_variant_shr((tmp0))?,
            syn::BinOp::Eq(tmp0) => f.try_fold_bin_op_variant_eq((tmp0))?,
            syn::BinOp::Lt(tmp0) => f.try_fold_bin_op_variant_lt((tmp0))?,
            syn::BinOp::Le(tmp0) => f.try_fold_bin_op_variant_le((tmp0))?,
            syn::BinOp::Ne(tmp0) => f.try_fold_bin_op_variant_ne((tmp0))?,
            syn::BinOp::Ge(tmp0) => f.try_fold_bin_op_variant_ge((tmp0))?,
            syn::BinOp::Gt(tmp0) => f.try_fold_bin_op_variant_gt((tmp0))?,
            syn::BinOp::AddAssign(tmp0) => f.try_fold_bin_op_variant_add_assign((tmp0))?,
            syn::BinOp::SubAssign(tmp0) => f.try_fold_bin_op_variant_sub_assign((tmp0))?,
            syn::BinOp::MulAssign(tmp0) => f.try_fold_bin_op_variant_mul_assign((tmp0))?,
            syn::BinOp::DivAssign(tmp0) => f.try_fold_bin_op_variant_div_assign((tmp0))?,
            syn::BinOp::RemAssign(tmp0) => f.try_fold_bin_op_variant_rem_assign((tmp0))?,
            syn::BinOp::BitXorAssign(tmp0) => {
                f.try_fold_bin_op_variant_bit_xor_assign((tmp0))?
            }
            syn::BinOp::BitAndAssign(tmp0) => {
                f.try_fold_bin_op_variant_bit_and_assign((tmp0))?
            }
            syn::BinOp::BitOrAssign(tmp0) => {
                f.try_fold_bin_op_variant_bit_or_assign((tmp0))?
            }
            syn::BinOp::ShlAssign(tmp0) => f.try_fold_bin_op_variant_shl_assign((tmp0))?,
            syn::BinOp::ShrAssign(tmp0) => f.try_fold_bin_op_variant_shr_assign((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_bin_op_variant_add<F>(
        f: &mut F,
        t: (syn::token::Plus),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Add(t))
    }
    pub fn try_fold_bin_op_variant_sub<F>(
        f: &mut F,
        t: (syn::token::Minus),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Sub(t))
    }
    pub fn try_fold_bin_op_variant_mul<F>(
        f: &mut F,
        t: (syn::token::Star),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Mul(t))
    }
    pub fn try_fold_bin_op_variant_div<F>(
        f: &mut F,
        t: (syn::token::Slash),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Div(t))
    }
    pub fn try_fold_bin_op_variant_rem<F>(
        f: &mut F,
        t: (syn::token::Percent),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Rem(t))
    }
    pub fn try_fold_bin_op_variant_and<F>(
        f: &mut F,
        t: (syn::token::AndAnd),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::And(t))
    }
    pub fn try_fold_bin_op_variant_or<F>(
        f: &mut F,
        t: (syn::token::OrOr),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Or(t))
    }
    pub fn try_fold_bin_op_variant_bit_xor<F>(
        f: &mut F,
        t: (syn::token::Caret),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitXor(t))
    }
    pub fn try_fold_bin_op_variant_bit_and<F>(
        f: &mut F,
        t: (syn::token::And),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitAnd(t))
    }
    pub fn try_fold_bin_op_variant_bit_or<F>(
        f: &mut F,
        t: (syn::token::Or),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitOr(t))
    }
    pub fn try_fold_bin_op_variant_shl<F>(
        f: &mut F,
        t: (syn::token::Shl),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Shl(t))
    }
    pub fn try_fold_bin_op_variant_shr<F>(
        f: &mut F,
        t: (syn::token::Shr),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Shr(t))
    }
    pub fn try_fold_bin_op_variant_eq<F>(
        f: &mut F,
        t: (syn::token::EqEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Eq(t))
    }
    pub fn try_fold_bin_op_variant_lt<F>(
        f: &mut F,
        t: (syn::token::Lt),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Lt(t))
    }
    pub fn try_fold_bin_op_variant_le<F>(
        f: &mut F,
        t: (syn::token::Le),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Le(t))
    }
    pub fn try_fold_bin_op_variant_ne<F>(
        f: &mut F,
        t: (syn::token::Ne),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Ne(t))
    }
    pub fn try_fold_bin_op_variant_ge<F>(
        f: &mut F,
        t: (syn::token::Ge),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Ge(t))
    }
    pub fn try_fold_bin_op_variant_gt<F>(
        f: &mut F,
        t: (syn::token::Gt),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::Gt(t))
    }
    pub fn try_fold_bin_op_variant_add_assign<F>(
        f: &mut F,
        t: (syn::token::PlusEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::AddAssign(t))
    }
    pub fn try_fold_bin_op_variant_sub_assign<F>(
        f: &mut F,
        t: (syn::token::MinusEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::SubAssign(t))
    }
    pub fn try_fold_bin_op_variant_mul_assign<F>(
        f: &mut F,
        t: (syn::token::StarEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::MulAssign(t))
    }
    pub fn try_fold_bin_op_variant_div_assign<F>(
        f: &mut F,
        t: (syn::token::SlashEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::DivAssign(t))
    }
    pub fn try_fold_bin_op_variant_rem_assign<F>(
        f: &mut F,
        t: (syn::token::PercentEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::RemAssign(t))
    }
    pub fn try_fold_bin_op_variant_bit_xor_assign<F>(
        f: &mut F,
        t: (syn::token::CaretEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitXorAssign(t))
    }
    pub fn try_fold_bin_op_variant_bit_and_assign<F>(
        f: &mut F,
        t: (syn::token::AndEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitAndAssign(t))
    }
    pub fn try_fold_bin_op_variant_bit_or_assign<F>(
        f: &mut F,
        t: (syn::token::OrEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::BitOrAssign(t))
    }
    pub fn try_fold_bin_op_variant_shl_assign<F>(
        f: &mut F,
        t: (syn::token::ShlEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::ShlAssign(t))
    }
    pub fn try_fold_bin_op_variant_shr_assign<F>(
        f: &mut F,
        t: (syn::token::ShrEq),
    ) -> Result<syn::BinOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::BinOp::ShrAssign(t))
    }
    pub fn try_fold_block<F>(
        f: &mut F,
        t: syn::Block,
    ) -> Result<syn::Block, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.brace_token = t.brace_token;
        t
            .stmts = {
            let mut tmp = vec![];
            for v in t.stmts {
                tmp.push(f.try_fold_stmt(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_bound_lifetimes<F>(
        f: &mut F,
        t: syn::BoundLifetimes,
    ) -> Result<syn::BoundLifetimes, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.for_token = t.for_token;
        t.lt_token = t.lt_token;
        t
            .lifetimes = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.lifetimes {
                tmp.push(f.try_fold_generic_param(v)?);
            }
            tmp
        };
        t.gt_token = t.gt_token;
        Ok(t)
    }
    pub fn try_fold_const_param<F>(
        f: &mut F,
        t: syn::ConstParam,
    ) -> Result<syn::ConstParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = f.try_fold_type(t.ty)?;
        t
            .eq_token = match t.eq_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .default = match t.default {
            Some(o) => Some(f.try_fold_expr(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_constraint<F>(
        f: &mut F,
        t: syn::Constraint,
    ) -> Result<syn::Constraint, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t
            .generics = match t.generics {
            Some(o) => Some(f.try_fold_angle_bracketed_generic_arguments(o)?),
            None => None,
        };
        t.colon_token = t.colon_token;
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_data<F>(
        f: &mut F,
        t: syn::Data,
    ) -> Result<syn::Data, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Data::Struct(tmp0) => f.try_fold_data_variant_struct((tmp0))?,
            syn::Data::Enum(tmp0) => f.try_fold_data_variant_enum((tmp0))?,
            syn::Data::Union(tmp0) => f.try_fold_data_variant_union((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_data_variant_struct<F>(
        f: &mut F,
        t: (syn::DataStruct),
    ) -> Result<syn::Data, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_data_struct(t)?);
        Ok(syn::Data::Struct(t))
    }
    pub fn try_fold_data_variant_enum<F>(
        f: &mut F,
        t: (syn::DataEnum),
    ) -> Result<syn::Data, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_data_enum(t)?);
        Ok(syn::Data::Enum(t))
    }
    pub fn try_fold_data_variant_union<F>(
        f: &mut F,
        t: (syn::DataUnion),
    ) -> Result<syn::Data, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_data_union(t)?);
        Ok(syn::Data::Union(t))
    }
    pub fn try_fold_data_enum<F>(
        f: &mut F,
        t: syn::DataEnum,
    ) -> Result<syn::DataEnum, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.enum_token = t.enum_token;
        t.brace_token = t.brace_token;
        t
            .variants = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.variants {
                tmp.push(f.try_fold_variant(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_data_struct<F>(
        f: &mut F,
        t: syn::DataStruct,
    ) -> Result<syn::DataStruct, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.struct_token = t.struct_token;
        t.fields = f.try_fold_fields(t.fields)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_data_union<F>(
        f: &mut F,
        t: syn::DataUnion,
    ) -> Result<syn::DataUnion, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.union_token = t.union_token;
        t.fields = f.try_fold_fields_named(t.fields)?;
        Ok(t)
    }
    pub fn try_fold_derive_input<F>(
        f: &mut F,
        t: syn::DeriveInput,
    ) -> Result<syn::DeriveInput, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.data = f.try_fold_data(t.data)?;
        Ok(t)
    }
    pub fn try_fold_expr<F>(
        f: &mut F,
        t: syn::Expr,
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Expr::Array(tmp0) => f.try_fold_expr_variant_array((tmp0))?,
            syn::Expr::Assign(tmp0) => f.try_fold_expr_variant_assign((tmp0))?,
            syn::Expr::Async(tmp0) => f.try_fold_expr_variant_async((tmp0))?,
            syn::Expr::Await(tmp0) => f.try_fold_expr_variant_await((tmp0))?,
            syn::Expr::Binary(tmp0) => f.try_fold_expr_variant_binary((tmp0))?,
            syn::Expr::Block(tmp0) => f.try_fold_expr_variant_block((tmp0))?,
            syn::Expr::Break(tmp0) => f.try_fold_expr_variant_break((tmp0))?,
            syn::Expr::Call(tmp0) => f.try_fold_expr_variant_call((tmp0))?,
            syn::Expr::Cast(tmp0) => f.try_fold_expr_variant_cast((tmp0))?,
            syn::Expr::Closure(tmp0) => f.try_fold_expr_variant_closure((tmp0))?,
            syn::Expr::Const(tmp0) => f.try_fold_expr_variant_const((tmp0))?,
            syn::Expr::Continue(tmp0) => f.try_fold_expr_variant_continue((tmp0))?,
            syn::Expr::Field(tmp0) => f.try_fold_expr_variant_field((tmp0))?,
            syn::Expr::ForLoop(tmp0) => f.try_fold_expr_variant_for_loop((tmp0))?,
            syn::Expr::Group(tmp0) => f.try_fold_expr_variant_group((tmp0))?,
            syn::Expr::If(tmp0) => f.try_fold_expr_variant_if((tmp0))?,
            syn::Expr::Index(tmp0) => f.try_fold_expr_variant_index((tmp0))?,
            syn::Expr::Infer(tmp0) => f.try_fold_expr_variant_infer((tmp0))?,
            syn::Expr::Let(tmp0) => f.try_fold_expr_variant_let((tmp0))?,
            syn::Expr::Lit(tmp0) => f.try_fold_expr_variant_lit((tmp0))?,
            syn::Expr::Loop(tmp0) => f.try_fold_expr_variant_loop((tmp0))?,
            syn::Expr::Macro(tmp0) => f.try_fold_expr_variant_macro((tmp0))?,
            syn::Expr::Match(tmp0) => f.try_fold_expr_variant_match((tmp0))?,
            syn::Expr::MethodCall(tmp0) => f.try_fold_expr_variant_method_call((tmp0))?,
            syn::Expr::Paren(tmp0) => f.try_fold_expr_variant_paren((tmp0))?,
            syn::Expr::Path(tmp0) => f.try_fold_expr_variant_path((tmp0))?,
            syn::Expr::Range(tmp0) => f.try_fold_expr_variant_range((tmp0))?,
            syn::Expr::Reference(tmp0) => f.try_fold_expr_variant_reference((tmp0))?,
            syn::Expr::Repeat(tmp0) => f.try_fold_expr_variant_repeat((tmp0))?,
            syn::Expr::Return(tmp0) => f.try_fold_expr_variant_return((tmp0))?,
            syn::Expr::Struct(tmp0) => f.try_fold_expr_variant_struct((tmp0))?,
            syn::Expr::Try(tmp0) => f.try_fold_expr_variant_try((tmp0))?,
            syn::Expr::TryBlock(tmp0) => f.try_fold_expr_variant_try_block((tmp0))?,
            syn::Expr::Tuple(tmp0) => f.try_fold_expr_variant_tuple((tmp0))?,
            syn::Expr::Unary(tmp0) => f.try_fold_expr_variant_unary((tmp0))?,
            syn::Expr::Unsafe(tmp0) => f.try_fold_expr_variant_unsafe((tmp0))?,
            syn::Expr::Verbatim(tmp0) => f.try_fold_expr_variant_verbatim((tmp0))?,
            syn::Expr::While(tmp0) => f.try_fold_expr_variant_while((tmp0))?,
            syn::Expr::Yield(tmp0) => f.try_fold_expr_variant_yield((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_expr_variant_array<F>(
        f: &mut F,
        t: (syn::ExprArray),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_array(t)?);
        Ok(syn::Expr::Array(t))
    }
    pub fn try_fold_expr_variant_assign<F>(
        f: &mut F,
        t: (syn::ExprAssign),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_assign(t)?);
        Ok(syn::Expr::Assign(t))
    }
    pub fn try_fold_expr_variant_async<F>(
        f: &mut F,
        t: (syn::ExprAsync),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_async(t)?);
        Ok(syn::Expr::Async(t))
    }
    pub fn try_fold_expr_variant_await<F>(
        f: &mut F,
        t: (syn::ExprAwait),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_await(t)?);
        Ok(syn::Expr::Await(t))
    }
    pub fn try_fold_expr_variant_binary<F>(
        f: &mut F,
        t: (syn::ExprBinary),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_binary(t)?);
        Ok(syn::Expr::Binary(t))
    }
    pub fn try_fold_expr_variant_block<F>(
        f: &mut F,
        t: (syn::ExprBlock),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_block(t)?);
        Ok(syn::Expr::Block(t))
    }
    pub fn try_fold_expr_variant_break<F>(
        f: &mut F,
        t: (syn::ExprBreak),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_break(t)?);
        Ok(syn::Expr::Break(t))
    }
    pub fn try_fold_expr_variant_call<F>(
        f: &mut F,
        t: (syn::ExprCall),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_call(t)?);
        Ok(syn::Expr::Call(t))
    }
    pub fn try_fold_expr_variant_cast<F>(
        f: &mut F,
        t: (syn::ExprCast),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_cast(t)?);
        Ok(syn::Expr::Cast(t))
    }
    pub fn try_fold_expr_variant_closure<F>(
        f: &mut F,
        t: (syn::ExprClosure),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_closure(t)?);
        Ok(syn::Expr::Closure(t))
    }
    pub fn try_fold_expr_variant_const<F>(
        f: &mut F,
        t: (syn::ExprConst),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_const(t)?);
        Ok(syn::Expr::Const(t))
    }
    pub fn try_fold_expr_variant_continue<F>(
        f: &mut F,
        t: (syn::ExprContinue),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_continue(t)?);
        Ok(syn::Expr::Continue(t))
    }
    pub fn try_fold_expr_variant_field<F>(
        f: &mut F,
        t: (syn::ExprField),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_field(t)?);
        Ok(syn::Expr::Field(t))
    }
    pub fn try_fold_expr_variant_for_loop<F>(
        f: &mut F,
        t: (syn::ExprForLoop),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_for_loop(t)?);
        Ok(syn::Expr::ForLoop(t))
    }
    pub fn try_fold_expr_variant_group<F>(
        f: &mut F,
        t: (syn::ExprGroup),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_group(t)?);
        Ok(syn::Expr::Group(t))
    }
    pub fn try_fold_expr_variant_if<F>(
        f: &mut F,
        t: (syn::ExprIf),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_if(t)?);
        Ok(syn::Expr::If(t))
    }
    pub fn try_fold_expr_variant_index<F>(
        f: &mut F,
        t: (syn::ExprIndex),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_index(t)?);
        Ok(syn::Expr::Index(t))
    }
    pub fn try_fold_expr_variant_infer<F>(
        f: &mut F,
        t: (syn::ExprInfer),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_infer(t)?);
        Ok(syn::Expr::Infer(t))
    }
    pub fn try_fold_expr_variant_let<F>(
        f: &mut F,
        t: (syn::ExprLet),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_let(t)?);
        Ok(syn::Expr::Let(t))
    }
    pub fn try_fold_expr_variant_lit<F>(
        f: &mut F,
        t: (syn::ExprLit),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_lit(t)?);
        Ok(syn::Expr::Lit(t))
    }
    pub fn try_fold_expr_variant_loop<F>(
        f: &mut F,
        t: (syn::ExprLoop),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_loop(t)?);
        Ok(syn::Expr::Loop(t))
    }
    pub fn try_fold_expr_variant_macro<F>(
        f: &mut F,
        t: (syn::ExprMacro),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_macro(t)?);
        Ok(syn::Expr::Macro(t))
    }
    pub fn try_fold_expr_variant_match<F>(
        f: &mut F,
        t: (syn::ExprMatch),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_match(t)?);
        Ok(syn::Expr::Match(t))
    }
    pub fn try_fold_expr_variant_method_call<F>(
        f: &mut F,
        t: (syn::ExprMethodCall),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_method_call(t)?);
        Ok(syn::Expr::MethodCall(t))
    }
    pub fn try_fold_expr_variant_paren<F>(
        f: &mut F,
        t: (syn::ExprParen),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_paren(t)?);
        Ok(syn::Expr::Paren(t))
    }
    pub fn try_fold_expr_variant_path<F>(
        f: &mut F,
        t: (syn::ExprPath),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_path(t)?);
        Ok(syn::Expr::Path(t))
    }
    pub fn try_fold_expr_variant_range<F>(
        f: &mut F,
        t: (syn::ExprRange),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_range(t)?);
        Ok(syn::Expr::Range(t))
    }
    pub fn try_fold_expr_variant_reference<F>(
        f: &mut F,
        t: (syn::ExprReference),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_reference(t)?);
        Ok(syn::Expr::Reference(t))
    }
    pub fn try_fold_expr_variant_repeat<F>(
        f: &mut F,
        t: (syn::ExprRepeat),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_repeat(t)?);
        Ok(syn::Expr::Repeat(t))
    }
    pub fn try_fold_expr_variant_return<F>(
        f: &mut F,
        t: (syn::ExprReturn),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_return(t)?);
        Ok(syn::Expr::Return(t))
    }
    pub fn try_fold_expr_variant_struct<F>(
        f: &mut F,
        t: (syn::ExprStruct),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_struct(t)?);
        Ok(syn::Expr::Struct(t))
    }
    pub fn try_fold_expr_variant_try<F>(
        f: &mut F,
        t: (syn::ExprTry),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_try(t)?);
        Ok(syn::Expr::Try(t))
    }
    pub fn try_fold_expr_variant_try_block<F>(
        f: &mut F,
        t: (syn::ExprTryBlock),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_try_block(t)?);
        Ok(syn::Expr::TryBlock(t))
    }
    pub fn try_fold_expr_variant_tuple<F>(
        f: &mut F,
        t: (syn::ExprTuple),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_tuple(t)?);
        Ok(syn::Expr::Tuple(t))
    }
    pub fn try_fold_expr_variant_unary<F>(
        f: &mut F,
        t: (syn::ExprUnary),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_unary(t)?);
        Ok(syn::Expr::Unary(t))
    }
    pub fn try_fold_expr_variant_unsafe<F>(
        f: &mut F,
        t: (syn::ExprUnsafe),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_unsafe(t)?);
        Ok(syn::Expr::Unsafe(t))
    }
    pub fn try_fold_expr_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Expr::Verbatim(t))
    }
    pub fn try_fold_expr_variant_while<F>(
        f: &mut F,
        t: (syn::ExprWhile),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_while(t)?);
        Ok(syn::Expr::While(t))
    }
    pub fn try_fold_expr_variant_yield<F>(
        f: &mut F,
        t: (syn::ExprYield),
    ) -> Result<syn::Expr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_yield(t)?);
        Ok(syn::Expr::Yield(t))
    }
    pub fn try_fold_expr_array<F>(
        f: &mut F,
        t: syn::ExprArray,
    ) -> Result<syn::ExprArray, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_expr(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_expr_assign<F>(
        f: &mut F,
        t: syn::ExprAssign,
    ) -> Result<syn::ExprAssign, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.left = Box::new(f.try_fold_expr(*t.left)?);
        t.eq_token = t.eq_token;
        t.right = Box::new(f.try_fold_expr(*t.right)?);
        Ok(t)
    }
    pub fn try_fold_expr_async<F>(
        f: &mut F,
        t: syn::ExprAsync,
    ) -> Result<syn::ExprAsync, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.async_token = t.async_token;
        t
            .capture = match t.capture {
            Some(o) => Some(o),
            None => None,
        };
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_expr_await<F>(
        f: &mut F,
        t: syn::ExprAwait,
    ) -> Result<syn::ExprAwait, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.base = Box::new(f.try_fold_expr(*t.base)?);
        t.dot_token = t.dot_token;
        t.await_token = t.await_token;
        Ok(t)
    }
    pub fn try_fold_expr_binary<F>(
        f: &mut F,
        t: syn::ExprBinary,
    ) -> Result<syn::ExprBinary, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.left = Box::new(f.try_fold_expr(*t.left)?);
        t.op = f.try_fold_bin_op(t.op)?;
        t.right = Box::new(f.try_fold_expr(*t.right)?);
        Ok(t)
    }
    pub fn try_fold_expr_block<F>(
        f: &mut F,
        t: syn::ExprBlock,
    ) -> Result<syn::ExprBlock, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_label(o)?),
            None => None,
        };
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_expr_break<F>(
        f: &mut F,
        t: syn::ExprBreak,
    ) -> Result<syn::ExprBreak, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.break_token = t.break_token;
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_lifetime(o)?),
            None => None,
        };
        t
            .expr = match t.expr {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_call<F>(
        f: &mut F,
        t: syn::ExprCall,
    ) -> Result<syn::ExprCall, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.func = Box::new(f.try_fold_expr(*t.func)?);
        t.paren_token = t.paren_token;
        t
            .args = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.args {
                tmp.push(f.try_fold_expr(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_expr_cast<F>(
        f: &mut F,
        t: syn::ExprCast,
    ) -> Result<syn::ExprCast, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.as_token = t.as_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        Ok(t)
    }
    pub fn try_fold_expr_closure<F>(
        f: &mut F,
        t: syn::ExprClosure,
    ) -> Result<syn::ExprClosure, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .lifetimes = match t.lifetimes {
            Some(o) => Some(f.try_fold_bound_lifetimes(o)?),
            None => None,
        };
        t
            .constness = match t.constness {
            Some(o) => Some(o),
            None => None,
        };
        t
            .movability = match t.movability {
            Some(o) => Some(o),
            None => None,
        };
        t
            .asyncness = match t.asyncness {
            Some(o) => Some(o),
            None => None,
        };
        t
            .capture = match t.capture {
            Some(o) => Some(o),
            None => None,
        };
        t.or1_token = t.or1_token;
        t
            .inputs = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.inputs {
                tmp.push(f.try_fold_pat(v)?);
            }
            tmp
        };
        t.or2_token = t.or2_token;
        t.output = f.try_fold_return_type(t.output)?;
        t.body = Box::new(f.try_fold_expr(*t.body)?);
        Ok(t)
    }
    pub fn try_fold_expr_const<F>(
        f: &mut F,
        t: syn::ExprConst,
    ) -> Result<syn::ExprConst, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.const_token = t.const_token;
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_expr_continue<F>(
        f: &mut F,
        t: syn::ExprContinue,
    ) -> Result<syn::ExprContinue, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.continue_token = t.continue_token;
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_lifetime(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_field<F>(
        f: &mut F,
        t: syn::ExprField,
    ) -> Result<syn::ExprField, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.base = Box::new(f.try_fold_expr(*t.base)?);
        t.dot_token = t.dot_token;
        t.member = f.try_fold_member(t.member)?;
        Ok(t)
    }
    pub fn try_fold_expr_for_loop<F>(
        f: &mut F,
        t: syn::ExprForLoop,
    ) -> Result<syn::ExprForLoop, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_label(o)?),
            None => None,
        };
        t.for_token = t.for_token;
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        t.in_token = t.in_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.body = f.try_fold_block(t.body)?;
        Ok(t)
    }
    pub fn try_fold_expr_group<F>(
        f: &mut F,
        t: syn::ExprGroup,
    ) -> Result<syn::ExprGroup, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.group_token = t.group_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        Ok(t)
    }
    pub fn try_fold_expr_if<F>(
        f: &mut F,
        t: syn::ExprIf,
    ) -> Result<syn::ExprIf, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.if_token = t.if_token;
        t.cond = Box::new(f.try_fold_expr(*t.cond)?);
        t.then_branch = f.try_fold_block(t.then_branch)?;
        t
            .else_branch = match t.else_branch {
            Some(o) => Some((o.0, Box::new(f.try_fold_expr(*o.1)?))),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_index<F>(
        f: &mut F,
        t: syn::ExprIndex,
    ) -> Result<syn::ExprIndex, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.bracket_token = t.bracket_token;
        t.index = Box::new(f.try_fold_expr(*t.index)?);
        Ok(t)
    }
    pub fn try_fold_expr_infer<F>(
        f: &mut F,
        t: syn::ExprInfer,
    ) -> Result<syn::ExprInfer, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.underscore_token = t.underscore_token;
        Ok(t)
    }
    pub fn try_fold_expr_let<F>(
        f: &mut F,
        t: syn::ExprLet,
    ) -> Result<syn::ExprLet, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.let_token = t.let_token;
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        t.eq_token = t.eq_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        Ok(t)
    }
    pub fn try_fold_expr_lit<F>(
        f: &mut F,
        t: syn::ExprLit,
    ) -> Result<syn::ExprLit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.lit = f.try_fold_lit(t.lit)?;
        Ok(t)
    }
    pub fn try_fold_expr_loop<F>(
        f: &mut F,
        t: syn::ExprLoop,
    ) -> Result<syn::ExprLoop, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_label(o)?),
            None => None,
        };
        t.loop_token = t.loop_token;
        t.body = f.try_fold_block(t.body)?;
        Ok(t)
    }
    pub fn try_fold_expr_macro<F>(
        f: &mut F,
        t: syn::ExprMacro,
    ) -> Result<syn::ExprMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.mac = f.try_fold_macro(t.mac)?;
        Ok(t)
    }
    pub fn try_fold_expr_match<F>(
        f: &mut F,
        t: syn::ExprMatch,
    ) -> Result<syn::ExprMatch, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.match_token = t.match_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.brace_token = t.brace_token;
        t
            .arms = {
            let mut tmp = vec![];
            for v in t.arms {
                tmp.push(f.try_fold_arm(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_expr_method_call<F>(
        f: &mut F,
        t: syn::ExprMethodCall,
    ) -> Result<syn::ExprMethodCall, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.receiver = Box::new(f.try_fold_expr(*t.receiver)?);
        t.dot_token = t.dot_token;
        t.method = t.method;
        t
            .turbofish = match t.turbofish {
            Some(o) => Some(f.try_fold_angle_bracketed_generic_arguments(o)?),
            None => None,
        };
        t.paren_token = t.paren_token;
        t
            .args = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.args {
                tmp.push(f.try_fold_expr(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_expr_paren<F>(
        f: &mut F,
        t: syn::ExprParen,
    ) -> Result<syn::ExprParen, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        Ok(t)
    }
    pub fn try_fold_expr_path<F>(
        f: &mut F,
        t: syn::ExprPath,
    ) -> Result<syn::ExprPath, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .qself = match t.qself {
            Some(o) => Some(f.try_fold_q_self(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        Ok(t)
    }
    pub fn try_fold_expr_range<F>(
        f: &mut F,
        t: syn::ExprRange,
    ) -> Result<syn::ExprRange, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .start = match t.start {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        t.limits = f.try_fold_range_limits(t.limits)?;
        t
            .end = match t.end {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_reference<F>(
        f: &mut F,
        t: syn::ExprReference,
    ) -> Result<syn::ExprReference, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.and_token = t.and_token;
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        Ok(t)
    }
    pub fn try_fold_expr_repeat<F>(
        f: &mut F,
        t: syn::ExprRepeat,
    ) -> Result<syn::ExprRepeat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.semi_token = t.semi_token;
        t.len = Box::new(f.try_fold_expr(*t.len)?);
        Ok(t)
    }
    pub fn try_fold_expr_return<F>(
        f: &mut F,
        t: syn::ExprReturn,
    ) -> Result<syn::ExprReturn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.return_token = t.return_token;
        t
            .expr = match t.expr {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_struct<F>(
        f: &mut F,
        t: syn::ExprStruct,
    ) -> Result<syn::ExprStruct, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .qself = match t.qself {
            Some(o) => Some(f.try_fold_q_self(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        t.brace_token = t.brace_token;
        t
            .fields = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.fields {
                tmp.push(f.try_fold_field_value(v)?);
            }
            tmp
        };
        t
            .dot2_token = match t.dot2_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .rest = match t.rest {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_expr_try<F>(
        f: &mut F,
        t: syn::ExprTry,
    ) -> Result<syn::ExprTry, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.question_token = t.question_token;
        Ok(t)
    }
    pub fn try_fold_expr_try_block<F>(
        f: &mut F,
        t: syn::ExprTryBlock,
    ) -> Result<syn::ExprTryBlock, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.try_token = t.try_token;
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_expr_tuple<F>(
        f: &mut F,
        t: syn::ExprTuple,
    ) -> Result<syn::ExprTuple, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_expr(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_expr_unary<F>(
        f: &mut F,
        t: syn::ExprUnary,
    ) -> Result<syn::ExprUnary, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.op = f.try_fold_un_op(t.op)?;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        Ok(t)
    }
    pub fn try_fold_expr_unsafe<F>(
        f: &mut F,
        t: syn::ExprUnsafe,
    ) -> Result<syn::ExprUnsafe, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.unsafe_token = t.unsafe_token;
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_expr_while<F>(
        f: &mut F,
        t: syn::ExprWhile,
    ) -> Result<syn::ExprWhile, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .label = match t.label {
            Some(o) => Some(f.try_fold_label(o)?),
            None => None,
        };
        t.while_token = t.while_token;
        t.cond = Box::new(f.try_fold_expr(*t.cond)?);
        t.body = f.try_fold_block(t.body)?;
        Ok(t)
    }
    pub fn try_fold_expr_yield<F>(
        f: &mut F,
        t: syn::ExprYield,
    ) -> Result<syn::ExprYield, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.yield_token = t.yield_token;
        t
            .expr = match t.expr {
            Some(o) => Some(Box::new(f.try_fold_expr(*o)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_field<F>(
        f: &mut F,
        t: syn::Field,
    ) -> Result<syn::Field, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.mutability = f.try_fold_field_mutability(t.mutability)?;
        t
            .ident = match t.ident {
            Some(o) => Some(o),
            None => None,
        };
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t.ty = f.try_fold_type(t.ty)?;
        Ok(t)
    }
    pub fn try_fold_field_mutability<F>(
        f: &mut F,
        t: syn::FieldMutability,
    ) -> Result<syn::FieldMutability, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_field_pat<F>(
        f: &mut F,
        t: syn::FieldPat,
    ) -> Result<syn::FieldPat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.member = f.try_fold_member(t.member)?;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        Ok(t)
    }
    pub fn try_fold_field_value<F>(
        f: &mut F,
        t: syn::FieldValue,
    ) -> Result<syn::FieldValue, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.member = f.try_fold_member(t.member)?;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t.expr = f.try_fold_expr(t.expr)?;
        Ok(t)
    }
    pub fn try_fold_fields<F>(
        f: &mut F,
        t: syn::Fields,
    ) -> Result<syn::Fields, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Fields::Named(tmp0) => f.try_fold_fields_variant_named((tmp0))?,
            syn::Fields::Unnamed(tmp0) => f.try_fold_fields_variant_unnamed((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_fields_variant_named<F>(
        f: &mut F,
        t: (syn::FieldsNamed),
    ) -> Result<syn::Fields, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_fields_named(t)?);
        Ok(syn::Fields::Named(t))
    }
    pub fn try_fold_fields_variant_unnamed<F>(
        f: &mut F,
        t: (syn::FieldsUnnamed),
    ) -> Result<syn::Fields, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_fields_unnamed(t)?);
        Ok(syn::Fields::Unnamed(t))
    }
    pub fn try_fold_fields_named<F>(
        f: &mut F,
        t: syn::FieldsNamed,
    ) -> Result<syn::FieldsNamed, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.brace_token = t.brace_token;
        t
            .named = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.named {
                tmp.push(f.try_fold_field(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_fields_unnamed<F>(
        f: &mut F,
        t: syn::FieldsUnnamed,
    ) -> Result<syn::FieldsUnnamed, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.paren_token = t.paren_token;
        t
            .unnamed = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.unnamed {
                tmp.push(f.try_fold_field(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_file<F>(
        f: &mut F,
        t: syn::File,
    ) -> Result<syn::File, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .shebang = match t.shebang {
            Some(o) => Some(o),
            None => None,
        };
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(f.try_fold_item(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_fn_arg<F>(
        f: &mut F,
        t: syn::FnArg,
    ) -> Result<syn::FnArg, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::FnArg::Receiver(tmp0) => f.try_fold_fn_arg_variant_receiver((tmp0))?,
            syn::FnArg::Typed(tmp0) => f.try_fold_fn_arg_variant_typed((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_fn_arg_variant_receiver<F>(
        f: &mut F,
        t: (syn::Receiver),
    ) -> Result<syn::FnArg, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_receiver(t)?);
        Ok(syn::FnArg::Receiver(t))
    }
    pub fn try_fold_fn_arg_variant_typed<F>(
        f: &mut F,
        t: (syn::PatType),
    ) -> Result<syn::FnArg, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_type(t)?);
        Ok(syn::FnArg::Typed(t))
    }
    pub fn try_fold_foreign_item<F>(
        f: &mut F,
        t: syn::ForeignItem,
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::ForeignItem::Fn(tmp0) => f.try_fold_foreign_item_variant_fn((tmp0))?,
            syn::ForeignItem::Static(tmp0) => {
                f.try_fold_foreign_item_variant_static((tmp0))?
            }
            syn::ForeignItem::Type(tmp0) => f.try_fold_foreign_item_variant_type((tmp0))?,
            syn::ForeignItem::Macro(tmp0) => {
                f.try_fold_foreign_item_variant_macro((tmp0))?
            }
            syn::ForeignItem::Verbatim(tmp0) => {
                f.try_fold_foreign_item_variant_verbatim((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_foreign_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ForeignItemFn),
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_foreign_item_fn(t)?);
        Ok(syn::ForeignItem::Fn(t))
    }
    pub fn try_fold_foreign_item_variant_static<F>(
        f: &mut F,
        t: (syn::ForeignItemStatic),
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_foreign_item_static(t)?);
        Ok(syn::ForeignItem::Static(t))
    }
    pub fn try_fold_foreign_item_variant_type<F>(
        f: &mut F,
        t: (syn::ForeignItemType),
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_foreign_item_type(t)?);
        Ok(syn::ForeignItem::Type(t))
    }
    pub fn try_fold_foreign_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ForeignItemMacro),
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_foreign_item_macro(t)?);
        Ok(syn::ForeignItem::Macro(t))
    }
    pub fn try_fold_foreign_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::ForeignItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::ForeignItem::Verbatim(t))
    }
    pub fn try_fold_foreign_item_fn<F>(
        f: &mut F,
        t: syn::ForeignItemFn,
    ) -> Result<syn::ForeignItemFn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.sig = f.try_fold_signature(t.sig)?;
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_foreign_item_macro<F>(
        f: &mut F,
        t: syn::ForeignItemMacro,
    ) -> Result<syn::ForeignItemMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.mac = f.try_fold_macro(t.mac)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_foreign_item_static<F>(
        f: &mut F,
        t: syn::ForeignItemStatic,
    ) -> Result<syn::ForeignItemStatic, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.static_token = t.static_token;
        t.mutability = f.try_fold_static_mutability(t.mutability)?;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_foreign_item_type<F>(
        f: &mut F,
        t: syn::ForeignItemType,
    ) -> Result<syn::ForeignItemType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_generic_argument<F>(
        f: &mut F,
        t: syn::GenericArgument,
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::GenericArgument::Lifetime(tmp0) => {
                f.try_fold_generic_argument_variant_lifetime((tmp0))?
            }
            syn::GenericArgument::Type(tmp0) => {
                f.try_fold_generic_argument_variant_type((tmp0))?
            }
            syn::GenericArgument::Const(tmp0) => {
                f.try_fold_generic_argument_variant_const((tmp0))?
            }
            syn::GenericArgument::AssocType(tmp0) => {
                f.try_fold_generic_argument_variant_assoc_type((tmp0))?
            }
            syn::GenericArgument::AssocConst(tmp0) => {
                f.try_fold_generic_argument_variant_assoc_const((tmp0))?
            }
            syn::GenericArgument::Constraint(tmp0) => {
                f.try_fold_generic_argument_variant_constraint((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_generic_argument_variant_lifetime<F>(
        f: &mut F,
        t: (syn::Lifetime),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lifetime(t)?);
        Ok(syn::GenericArgument::Lifetime(t))
    }
    pub fn try_fold_generic_argument_variant_type<F>(
        f: &mut F,
        t: (syn::Type),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type(t)?);
        Ok(syn::GenericArgument::Type(t))
    }
    pub fn try_fold_generic_argument_variant_const<F>(
        f: &mut F,
        t: (syn::Expr),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr(t)?);
        Ok(syn::GenericArgument::Const(t))
    }
    pub fn try_fold_generic_argument_variant_assoc_type<F>(
        f: &mut F,
        t: (syn::AssocType),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_assoc_type(t)?);
        Ok(syn::GenericArgument::AssocType(t))
    }
    pub fn try_fold_generic_argument_variant_assoc_const<F>(
        f: &mut F,
        t: (syn::AssocConst),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_assoc_const(t)?);
        Ok(syn::GenericArgument::AssocConst(t))
    }
    pub fn try_fold_generic_argument_variant_constraint<F>(
        f: &mut F,
        t: (syn::Constraint),
    ) -> Result<syn::GenericArgument, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_constraint(t)?);
        Ok(syn::GenericArgument::Constraint(t))
    }
    pub fn try_fold_generic_param<F>(
        f: &mut F,
        t: syn::GenericParam,
    ) -> Result<syn::GenericParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::GenericParam::Lifetime(tmp0) => {
                f.try_fold_generic_param_variant_lifetime((tmp0))?
            }
            syn::GenericParam::Type(tmp0) => {
                f.try_fold_generic_param_variant_type((tmp0))?
            }
            syn::GenericParam::Const(tmp0) => {
                f.try_fold_generic_param_variant_const((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_generic_param_variant_lifetime<F>(
        f: &mut F,
        t: (syn::LifetimeParam),
    ) -> Result<syn::GenericParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lifetime_param(t)?);
        Ok(syn::GenericParam::Lifetime(t))
    }
    pub fn try_fold_generic_param_variant_type<F>(
        f: &mut F,
        t: (syn::TypeParam),
    ) -> Result<syn::GenericParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_param(t)?);
        Ok(syn::GenericParam::Type(t))
    }
    pub fn try_fold_generic_param_variant_const<F>(
        f: &mut F,
        t: (syn::ConstParam),
    ) -> Result<syn::GenericParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_const_param(t)?);
        Ok(syn::GenericParam::Const(t))
    }
    pub fn try_fold_generics<F>(
        f: &mut F,
        t: syn::Generics,
    ) -> Result<syn::Generics, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .lt_token = match t.lt_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .params = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.params {
                tmp.push(f.try_fold_generic_param(v)?);
            }
            tmp
        };
        t
            .gt_token = match t.gt_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .where_clause = match t.where_clause {
            Some(o) => Some(f.try_fold_where_clause(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_impl_item<F>(
        f: &mut F,
        t: syn::ImplItem,
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::ImplItem::Const(tmp0) => f.try_fold_impl_item_variant_const((tmp0))?,
            syn::ImplItem::Fn(tmp0) => f.try_fold_impl_item_variant_fn((tmp0))?,
            syn::ImplItem::Type(tmp0) => f.try_fold_impl_item_variant_type((tmp0))?,
            syn::ImplItem::Macro(tmp0) => f.try_fold_impl_item_variant_macro((tmp0))?,
            syn::ImplItem::Verbatim(tmp0) => {
                f.try_fold_impl_item_variant_verbatim((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_impl_item_variant_const<F>(
        f: &mut F,
        t: (syn::ImplItemConst),
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_impl_item_const(t)?);
        Ok(syn::ImplItem::Const(t))
    }
    pub fn try_fold_impl_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ImplItemFn),
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_impl_item_fn(t)?);
        Ok(syn::ImplItem::Fn(t))
    }
    pub fn try_fold_impl_item_variant_type<F>(
        f: &mut F,
        t: (syn::ImplItemType),
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_impl_item_type(t)?);
        Ok(syn::ImplItem::Type(t))
    }
    pub fn try_fold_impl_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ImplItemMacro),
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_impl_item_macro(t)?);
        Ok(syn::ImplItem::Macro(t))
    }
    pub fn try_fold_impl_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::ImplItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::ImplItem::Verbatim(t))
    }
    pub fn try_fold_impl_item_const<F>(
        f: &mut F,
        t: syn::ImplItemConst,
    ) -> Result<syn::ImplItemConst, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t
            .defaultness = match t.defaultness {
            Some(o) => Some(o),
            None => None,
        };
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.colon_token = t.colon_token;
        t.ty = f.try_fold_type(t.ty)?;
        t.eq_token = t.eq_token;
        t.expr = f.try_fold_expr(t.expr)?;
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_impl_item_fn<F>(
        f: &mut F,
        t: syn::ImplItemFn,
    ) -> Result<syn::ImplItemFn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t
            .defaultness = match t.defaultness {
            Some(o) => Some(o),
            None => None,
        };
        t.sig = f.try_fold_signature(t.sig)?;
        t.block = f.try_fold_block(t.block)?;
        Ok(t)
    }
    pub fn try_fold_impl_item_macro<F>(
        f: &mut F,
        t: syn::ImplItemMacro,
    ) -> Result<syn::ImplItemMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.mac = f.try_fold_macro(t.mac)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_impl_item_type<F>(
        f: &mut F,
        t: syn::ImplItemType,
    ) -> Result<syn::ImplItemType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t
            .defaultness = match t.defaultness {
            Some(o) => Some(o),
            None => None,
        };
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.eq_token = t.eq_token;
        t.ty = f.try_fold_type(t.ty)?;
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_impl_restriction<F>(
        f: &mut F,
        t: syn::ImplRestriction,
    ) -> Result<syn::ImplRestriction, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_index<F>(
        f: &mut F,
        t: syn::Index,
    ) -> Result<syn::Index, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.index = t.index;
        t.span = t.span;
        Ok(t)
    }
    pub fn try_fold_item<F>(
        f: &mut F,
        t: syn::Item,
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Item::Const(tmp0) => f.try_fold_item_variant_const((tmp0))?,
            syn::Item::Enum(tmp0) => f.try_fold_item_variant_enum((tmp0))?,
            syn::Item::ExternCrate(tmp0) => f.try_fold_item_variant_extern_crate((tmp0))?,
            syn::Item::Fn(tmp0) => f.try_fold_item_variant_fn((tmp0))?,
            syn::Item::ForeignMod(tmp0) => f.try_fold_item_variant_foreign_mod((tmp0))?,
            syn::Item::Impl(tmp0) => f.try_fold_item_variant_impl((tmp0))?,
            syn::Item::Macro(tmp0) => f.try_fold_item_variant_macro((tmp0))?,
            syn::Item::Mod(tmp0) => f.try_fold_item_variant_mod((tmp0))?,
            syn::Item::Static(tmp0) => f.try_fold_item_variant_static((tmp0))?,
            syn::Item::Struct(tmp0) => f.try_fold_item_variant_struct((tmp0))?,
            syn::Item::Trait(tmp0) => f.try_fold_item_variant_trait((tmp0))?,
            syn::Item::TraitAlias(tmp0) => f.try_fold_item_variant_trait_alias((tmp0))?,
            syn::Item::Type(tmp0) => f.try_fold_item_variant_type((tmp0))?,
            syn::Item::Union(tmp0) => f.try_fold_item_variant_union((tmp0))?,
            syn::Item::Use(tmp0) => f.try_fold_item_variant_use((tmp0))?,
            syn::Item::Verbatim(tmp0) => f.try_fold_item_variant_verbatim((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_item_variant_const<F>(
        f: &mut F,
        t: (syn::ItemConst),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_const(t)?);
        Ok(syn::Item::Const(t))
    }
    pub fn try_fold_item_variant_enum<F>(
        f: &mut F,
        t: (syn::ItemEnum),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_enum(t)?);
        Ok(syn::Item::Enum(t))
    }
    pub fn try_fold_item_variant_extern_crate<F>(
        f: &mut F,
        t: (syn::ItemExternCrate),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_extern_crate(t)?);
        Ok(syn::Item::ExternCrate(t))
    }
    pub fn try_fold_item_variant_fn<F>(
        f: &mut F,
        t: (syn::ItemFn),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_fn(t)?);
        Ok(syn::Item::Fn(t))
    }
    pub fn try_fold_item_variant_foreign_mod<F>(
        f: &mut F,
        t: (syn::ItemForeignMod),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_foreign_mod(t)?);
        Ok(syn::Item::ForeignMod(t))
    }
    pub fn try_fold_item_variant_impl<F>(
        f: &mut F,
        t: (syn::ItemImpl),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_impl(t)?);
        Ok(syn::Item::Impl(t))
    }
    pub fn try_fold_item_variant_macro<F>(
        f: &mut F,
        t: (syn::ItemMacro),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_macro(t)?);
        Ok(syn::Item::Macro(t))
    }
    pub fn try_fold_item_variant_mod<F>(
        f: &mut F,
        t: (syn::ItemMod),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_mod(t)?);
        Ok(syn::Item::Mod(t))
    }
    pub fn try_fold_item_variant_static<F>(
        f: &mut F,
        t: (syn::ItemStatic),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_static(t)?);
        Ok(syn::Item::Static(t))
    }
    pub fn try_fold_item_variant_struct<F>(
        f: &mut F,
        t: (syn::ItemStruct),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_struct(t)?);
        Ok(syn::Item::Struct(t))
    }
    pub fn try_fold_item_variant_trait<F>(
        f: &mut F,
        t: (syn::ItemTrait),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_trait(t)?);
        Ok(syn::Item::Trait(t))
    }
    pub fn try_fold_item_variant_trait_alias<F>(
        f: &mut F,
        t: (syn::ItemTraitAlias),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_trait_alias(t)?);
        Ok(syn::Item::TraitAlias(t))
    }
    pub fn try_fold_item_variant_type<F>(
        f: &mut F,
        t: (syn::ItemType),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_type(t)?);
        Ok(syn::Item::Type(t))
    }
    pub fn try_fold_item_variant_union<F>(
        f: &mut F,
        t: (syn::ItemUnion),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_union(t)?);
        Ok(syn::Item::Union(t))
    }
    pub fn try_fold_item_variant_use<F>(
        f: &mut F,
        t: (syn::ItemUse),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item_use(t)?);
        Ok(syn::Item::Use(t))
    }
    pub fn try_fold_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::Item, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Item::Verbatim(t))
    }
    pub fn try_fold_item_const<F>(
        f: &mut F,
        t: syn::ItemConst,
    ) -> Result<syn::ItemConst, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.colon_token = t.colon_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        t.eq_token = t.eq_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_item_enum<F>(
        f: &mut F,
        t: syn::ItemEnum,
    ) -> Result<syn::ItemEnum, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.enum_token = t.enum_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.brace_token = t.brace_token;
        t
            .variants = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.variants {
                tmp.push(f.try_fold_variant(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_item_extern_crate<F>(
        f: &mut F,
        t: syn::ItemExternCrate,
    ) -> Result<syn::ItemExternCrate, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.extern_token = t.extern_token;
        t.crate_token = t.crate_token;
        t.ident = t.ident;
        t
            .rename = match t.rename {
            Some(o) => Some((o.0, o.1)),
            None => None,
        };
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_item_fn<F>(
        f: &mut F,
        t: syn::ItemFn,
    ) -> Result<syn::ItemFn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.sig = f.try_fold_signature(t.sig)?;
        t.block = Box::new(f.try_fold_block(*t.block)?);
        Ok(t)
    }
    pub fn try_fold_item_foreign_mod<F>(
        f: &mut F,
        t: syn::ItemForeignMod,
    ) -> Result<syn::ItemForeignMod, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t.abi = f.try_fold_abi(t.abi)?;
        t.brace_token = t.brace_token;
        t
            .items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(f.try_fold_foreign_item(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_item_impl<F>(
        f: &mut F,
        t: syn::ItemImpl,
    ) -> Result<syn::ItemImpl, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .defaultness = match t.defaultness {
            Some(o) => Some(o),
            None => None,
        };
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t.impl_token = t.impl_token;
        t.generics = f.try_fold_generics(t.generics)?;
        t
            .trait_ = match t.trait_ {
            Some(o) => {
                Some((
                    match o.0 {
                        Some(o) => Some(o),
                        None => None,
                    },
                    f.try_fold_path(o.1)?,
                    o.2,
                ))
            }
            None => None,
        };
        t.self_ty = Box::new(f.try_fold_type(*t.self_ty)?);
        t.brace_token = t.brace_token;
        t
            .items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(f.try_fold_impl_item(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_item_macro<F>(
        f: &mut F,
        t: syn::ItemMacro,
    ) -> Result<syn::ItemMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .ident = match t.ident {
            Some(o) => Some(o),
            None => None,
        };
        t.mac = f.try_fold_macro(t.mac)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_item_mod<F>(
        f: &mut F,
        t: syn::ItemMod,
    ) -> Result<syn::ItemMod, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t.mod_token = t.mod_token;
        t.ident = t.ident;
        t
            .content = match t.content {
            Some(o) => {
                Some((
                    o.0,
                    {
                        let mut tmp = vec![];
                        for v in o.1 {
                            tmp.push(f.try_fold_item(v)?);
                        }
                        tmp
                    },
                ))
            }
            None => None,
        };
        t
            .semi = match t.semi {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_item_static<F>(
        f: &mut F,
        t: syn::ItemStatic,
    ) -> Result<syn::ItemStatic, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.static_token = t.static_token;
        t.mutability = f.try_fold_static_mutability(t.mutability)?;
        t.ident = t.ident;
        t.colon_token = t.colon_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        t.eq_token = t.eq_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_item_struct<F>(
        f: &mut F,
        t: syn::ItemStruct,
    ) -> Result<syn::ItemStruct, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.struct_token = t.struct_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.fields = f.try_fold_fields(t.fields)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_item_trait<F>(
        f: &mut F,
        t: syn::ItemTrait,
    ) -> Result<syn::ItemTrait, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t
            .auto_token = match t.auto_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .restriction = match t.restriction {
            Some(o) => Some(f.try_fold_impl_restriction(o)?),
            None => None,
        };
        t.trait_token = t.trait_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .supertraits = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.supertraits {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        t.brace_token = t.brace_token;
        t
            .items = {
            let mut tmp = vec![];
            for v in t.items {
                tmp.push(f.try_fold_trait_item(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_item_trait_alias<F>(
        f: &mut F,
        t: syn::ItemTraitAlias,
    ) -> Result<syn::ItemTraitAlias, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.trait_token = t.trait_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.eq_token = t.eq_token;
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_item_type<F>(
        f: &mut F,
        t: syn::ItemType,
    ) -> Result<syn::ItemType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.eq_token = t.eq_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_item_union<F>(
        f: &mut F,
        t: syn::ItemUnion,
    ) -> Result<syn::ItemUnion, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.union_token = t.union_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.fields = f.try_fold_fields_named(t.fields)?;
        Ok(t)
    }
    pub fn try_fold_item_use<F>(
        f: &mut F,
        t: syn::ItemUse,
    ) -> Result<syn::ItemUse, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.vis = f.try_fold_visibility(t.vis)?;
        t.use_token = t.use_token;
        t
            .leading_colon = match t.leading_colon {
            Some(o) => Some(o),
            None => None,
        };
        t.tree = f.try_fold_use_tree(t.tree)?;
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_label<F>(
        f: &mut F,
        t: syn::Label,
    ) -> Result<syn::Label, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.name = f.try_fold_lifetime(t.name)?;
        t.colon_token = t.colon_token;
        Ok(t)
    }
    pub fn try_fold_lifetime<F>(
        f: &mut F,
        t: syn::Lifetime,
    ) -> Result<syn::Lifetime, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.apostrophe = t.apostrophe;
        t.ident = t.ident;
        Ok(t)
    }
    pub fn try_fold_lifetime_param<F>(
        f: &mut F,
        t: syn::LifetimeParam,
    ) -> Result<syn::LifetimeParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.lifetime = f.try_fold_lifetime(t.lifetime)?;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_lifetime(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_lit<F>(
        f: &mut F,
        t: syn::Lit,
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Lit::Str(tmp0) => f.try_fold_lit_variant_str((tmp0))?,
            syn::Lit::ByteStr(tmp0) => f.try_fold_lit_variant_byte_str((tmp0))?,
            syn::Lit::Byte(tmp0) => f.try_fold_lit_variant_byte((tmp0))?,
            syn::Lit::Char(tmp0) => f.try_fold_lit_variant_char((tmp0))?,
            syn::Lit::Int(tmp0) => f.try_fold_lit_variant_int((tmp0))?,
            syn::Lit::Float(tmp0) => f.try_fold_lit_variant_float((tmp0))?,
            syn::Lit::Bool(tmp0) => f.try_fold_lit_variant_bool((tmp0))?,
            syn::Lit::Verbatim(tmp0) => f.try_fold_lit_variant_verbatim((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_lit_variant_str<F>(
        f: &mut F,
        t: (syn::LitStr),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_str(t)?);
        Ok(syn::Lit::Str(t))
    }
    pub fn try_fold_lit_variant_byte_str<F>(
        f: &mut F,
        t: (syn::LitByteStr),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_byte_str(t)?);
        Ok(syn::Lit::ByteStr(t))
    }
    pub fn try_fold_lit_variant_byte<F>(
        f: &mut F,
        t: (syn::LitByte),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_byte(t)?);
        Ok(syn::Lit::Byte(t))
    }
    pub fn try_fold_lit_variant_char<F>(
        f: &mut F,
        t: (syn::LitChar),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_char(t)?);
        Ok(syn::Lit::Char(t))
    }
    pub fn try_fold_lit_variant_int<F>(
        f: &mut F,
        t: (syn::LitInt),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_int(t)?);
        Ok(syn::Lit::Int(t))
    }
    pub fn try_fold_lit_variant_float<F>(
        f: &mut F,
        t: (syn::LitFloat),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_float(t)?);
        Ok(syn::Lit::Float(t))
    }
    pub fn try_fold_lit_variant_bool<F>(
        f: &mut F,
        t: (syn::LitBool),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lit_bool(t)?);
        Ok(syn::Lit::Bool(t))
    }
    pub fn try_fold_lit_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::Literal),
    ) -> Result<syn::Lit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Lit::Verbatim(t))
    }
    pub fn try_fold_lit_bool<F>(
        f: &mut F,
        t: syn::LitBool,
    ) -> Result<syn::LitBool, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.value = t.value;
        t.span = t.span;
        Ok(t)
    }
    pub fn try_fold_lit_byte<F>(
        f: &mut F,
        t: syn::LitByte,
    ) -> Result<syn::LitByte, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_lit_byte_str<F>(
        f: &mut F,
        t: syn::LitByteStr,
    ) -> Result<syn::LitByteStr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_lit_char<F>(
        f: &mut F,
        t: syn::LitChar,
    ) -> Result<syn::LitChar, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_lit_float<F>(
        f: &mut F,
        t: syn::LitFloat,
    ) -> Result<syn::LitFloat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_lit_int<F>(
        f: &mut F,
        t: syn::LitInt,
    ) -> Result<syn::LitInt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_lit_str<F>(
        f: &mut F,
        t: syn::LitStr,
    ) -> Result<syn::LitStr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        Ok(t)
    }
    pub fn try_fold_local<F>(
        f: &mut F,
        t: syn::Local,
    ) -> Result<syn::Local, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.let_token = t.let_token;
        t.pat = f.try_fold_pat(t.pat)?;
        t
            .init = match t.init {
            Some(o) => Some(f.try_fold_local_init(o)?),
            None => None,
        };
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_local_init<F>(
        f: &mut F,
        t: syn::LocalInit,
    ) -> Result<syn::LocalInit, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.eq_token = t.eq_token;
        t.expr = Box::new(f.try_fold_expr(*t.expr)?);
        t
            .diverge = match t.diverge {
            Some(o) => Some((o.0, Box::new(f.try_fold_expr(*o.1)?))),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_macro<F>(
        f: &mut F,
        t: syn::Macro,
    ) -> Result<syn::Macro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.path = f.try_fold_path(t.path)?;
        t.bang_token = t.bang_token;
        t.delimiter = f.try_fold_macro_delimiter(t.delimiter)?;
        t.tokens = t.tokens;
        Ok(t)
    }
    pub fn try_fold_macro_delimiter<F>(
        f: &mut F,
        t: syn::MacroDelimiter,
    ) -> Result<syn::MacroDelimiter, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::MacroDelimiter::Paren(tmp0) => {
                f.try_fold_macro_delimiter_variant_paren((tmp0))?
            }
            syn::MacroDelimiter::Brace(tmp0) => {
                f.try_fold_macro_delimiter_variant_brace((tmp0))?
            }
            syn::MacroDelimiter::Bracket(tmp0) => {
                f.try_fold_macro_delimiter_variant_bracket((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_macro_delimiter_variant_paren<F>(
        f: &mut F,
        t: (syn::token::Paren),
    ) -> Result<syn::MacroDelimiter, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::MacroDelimiter::Paren(t))
    }
    pub fn try_fold_macro_delimiter_variant_brace<F>(
        f: &mut F,
        t: (syn::token::Brace),
    ) -> Result<syn::MacroDelimiter, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::MacroDelimiter::Brace(t))
    }
    pub fn try_fold_macro_delimiter_variant_bracket<F>(
        f: &mut F,
        t: (syn::token::Bracket),
    ) -> Result<syn::MacroDelimiter, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::MacroDelimiter::Bracket(t))
    }
    pub fn try_fold_member<F>(
        f: &mut F,
        t: syn::Member,
    ) -> Result<syn::Member, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Member::Named(tmp0) => f.try_fold_member_variant_named((tmp0))?,
            syn::Member::Unnamed(tmp0) => f.try_fold_member_variant_unnamed((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_member_variant_named<F>(
        f: &mut F,
        t: (proc_macro2::Ident),
    ) -> Result<syn::Member, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Member::Named(t))
    }
    pub fn try_fold_member_variant_unnamed<F>(
        f: &mut F,
        t: (syn::Index),
    ) -> Result<syn::Member, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_index(t)?);
        Ok(syn::Member::Unnamed(t))
    }
    pub fn try_fold_meta<F>(
        f: &mut F,
        t: syn::Meta,
    ) -> Result<syn::Meta, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Meta::Path(tmp0) => f.try_fold_meta_variant_path((tmp0))?,
            syn::Meta::List(tmp0) => f.try_fold_meta_variant_list((tmp0))?,
            syn::Meta::NameValue(tmp0) => f.try_fold_meta_variant_name_value((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_meta_variant_path<F>(
        f: &mut F,
        t: (syn::Path),
    ) -> Result<syn::Meta, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_path(t)?);
        Ok(syn::Meta::Path(t))
    }
    pub fn try_fold_meta_variant_list<F>(
        f: &mut F,
        t: (syn::MetaList),
    ) -> Result<syn::Meta, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_meta_list(t)?);
        Ok(syn::Meta::List(t))
    }
    pub fn try_fold_meta_variant_name_value<F>(
        f: &mut F,
        t: (syn::MetaNameValue),
    ) -> Result<syn::Meta, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_meta_name_value(t)?);
        Ok(syn::Meta::NameValue(t))
    }
    pub fn try_fold_meta_list<F>(
        f: &mut F,
        t: syn::MetaList,
    ) -> Result<syn::MetaList, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.path = f.try_fold_path(t.path)?;
        t.delimiter = f.try_fold_macro_delimiter(t.delimiter)?;
        t.tokens = t.tokens;
        Ok(t)
    }
    pub fn try_fold_meta_name_value<F>(
        f: &mut F,
        t: syn::MetaNameValue,
    ) -> Result<syn::MetaNameValue, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.path = f.try_fold_path(t.path)?;
        t.eq_token = t.eq_token;
        t.value = f.try_fold_expr(t.value)?;
        Ok(t)
    }
    pub fn try_fold_parenthesized_generic_arguments<F>(
        f: &mut F,
        t: syn::ParenthesizedGenericArguments,
    ) -> Result<syn::ParenthesizedGenericArguments, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.paren_token = t.paren_token;
        t
            .inputs = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.inputs {
                tmp.push(f.try_fold_type(v)?);
            }
            tmp
        };
        t.output = f.try_fold_return_type(t.output)?;
        Ok(t)
    }
    pub fn try_fold_pat<F>(
        f: &mut F,
        t: syn::Pat,
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Pat::Const(tmp0) => f.try_fold_pat_variant_const((tmp0))?,
            syn::Pat::Ident(tmp0) => f.try_fold_pat_variant_ident((tmp0))?,
            syn::Pat::Lit(tmp0) => f.try_fold_pat_variant_lit((tmp0))?,
            syn::Pat::Macro(tmp0) => f.try_fold_pat_variant_macro((tmp0))?,
            syn::Pat::Or(tmp0) => f.try_fold_pat_variant_or((tmp0))?,
            syn::Pat::Paren(tmp0) => f.try_fold_pat_variant_paren((tmp0))?,
            syn::Pat::Path(tmp0) => f.try_fold_pat_variant_path((tmp0))?,
            syn::Pat::Range(tmp0) => f.try_fold_pat_variant_range((tmp0))?,
            syn::Pat::Reference(tmp0) => f.try_fold_pat_variant_reference((tmp0))?,
            syn::Pat::Rest(tmp0) => f.try_fold_pat_variant_rest((tmp0))?,
            syn::Pat::Slice(tmp0) => f.try_fold_pat_variant_slice((tmp0))?,
            syn::Pat::Struct(tmp0) => f.try_fold_pat_variant_struct((tmp0))?,
            syn::Pat::Tuple(tmp0) => f.try_fold_pat_variant_tuple((tmp0))?,
            syn::Pat::TupleStruct(tmp0) => f.try_fold_pat_variant_tuple_struct((tmp0))?,
            syn::Pat::Type(tmp0) => f.try_fold_pat_variant_type((tmp0))?,
            syn::Pat::Verbatim(tmp0) => f.try_fold_pat_variant_verbatim((tmp0))?,
            syn::Pat::Wild(tmp0) => f.try_fold_pat_variant_wild((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_pat_variant_const<F>(
        f: &mut F,
        t: (syn::ExprConst),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_const(t)?);
        Ok(syn::Pat::Const(t))
    }
    pub fn try_fold_pat_variant_ident<F>(
        f: &mut F,
        t: (syn::PatIdent),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_ident(t)?);
        Ok(syn::Pat::Ident(t))
    }
    pub fn try_fold_pat_variant_lit<F>(
        f: &mut F,
        t: (syn::ExprLit),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_lit(t)?);
        Ok(syn::Pat::Lit(t))
    }
    pub fn try_fold_pat_variant_macro<F>(
        f: &mut F,
        t: (syn::ExprMacro),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_macro(t)?);
        Ok(syn::Pat::Macro(t))
    }
    pub fn try_fold_pat_variant_or<F>(
        f: &mut F,
        t: (syn::PatOr),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_or(t)?);
        Ok(syn::Pat::Or(t))
    }
    pub fn try_fold_pat_variant_paren<F>(
        f: &mut F,
        t: (syn::PatParen),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_paren(t)?);
        Ok(syn::Pat::Paren(t))
    }
    pub fn try_fold_pat_variant_path<F>(
        f: &mut F,
        t: (syn::ExprPath),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_path(t)?);
        Ok(syn::Pat::Path(t))
    }
    pub fn try_fold_pat_variant_range<F>(
        f: &mut F,
        t: (syn::ExprRange),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_expr_range(t)?);
        Ok(syn::Pat::Range(t))
    }
    pub fn try_fold_pat_variant_reference<F>(
        f: &mut F,
        t: (syn::PatReference),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_reference(t)?);
        Ok(syn::Pat::Reference(t))
    }
    pub fn try_fold_pat_variant_rest<F>(
        f: &mut F,
        t: (syn::PatRest),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_rest(t)?);
        Ok(syn::Pat::Rest(t))
    }
    pub fn try_fold_pat_variant_slice<F>(
        f: &mut F,
        t: (syn::PatSlice),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_slice(t)?);
        Ok(syn::Pat::Slice(t))
    }
    pub fn try_fold_pat_variant_struct<F>(
        f: &mut F,
        t: (syn::PatStruct),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_struct(t)?);
        Ok(syn::Pat::Struct(t))
    }
    pub fn try_fold_pat_variant_tuple<F>(
        f: &mut F,
        t: (syn::PatTuple),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_tuple(t)?);
        Ok(syn::Pat::Tuple(t))
    }
    pub fn try_fold_pat_variant_tuple_struct<F>(
        f: &mut F,
        t: (syn::PatTupleStruct),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_tuple_struct(t)?);
        Ok(syn::Pat::TupleStruct(t))
    }
    pub fn try_fold_pat_variant_type<F>(
        f: &mut F,
        t: (syn::PatType),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_type(t)?);
        Ok(syn::Pat::Type(t))
    }
    pub fn try_fold_pat_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Pat::Verbatim(t))
    }
    pub fn try_fold_pat_variant_wild<F>(
        f: &mut F,
        t: (syn::PatWild),
    ) -> Result<syn::Pat, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_pat_wild(t)?);
        Ok(syn::Pat::Wild(t))
    }
    pub fn try_fold_pat_ident<F>(
        f: &mut F,
        t: syn::PatIdent,
    ) -> Result<syn::PatIdent, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .by_ref = match t.by_ref {
            Some(o) => Some(o),
            None => None,
        };
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.ident = t.ident;
        t
            .subpat = match t.subpat {
            Some(o) => Some((o.0, Box::new(f.try_fold_pat(*o.1)?))),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_pat_or<F>(
        f: &mut F,
        t: syn::PatOr,
    ) -> Result<syn::PatOr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .leading_vert = match t.leading_vert {
            Some(o) => Some(o),
            None => None,
        };
        t
            .cases = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.cases {
                tmp.push(f.try_fold_pat(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_pat_paren<F>(
        f: &mut F,
        t: syn::PatParen,
    ) -> Result<syn::PatParen, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        Ok(t)
    }
    pub fn try_fold_pat_reference<F>(
        f: &mut F,
        t: syn::PatReference,
    ) -> Result<syn::PatReference, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.and_token = t.and_token;
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        Ok(t)
    }
    pub fn try_fold_pat_rest<F>(
        f: &mut F,
        t: syn::PatRest,
    ) -> Result<syn::PatRest, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.dot2_token = t.dot2_token;
        Ok(t)
    }
    pub fn try_fold_pat_slice<F>(
        f: &mut F,
        t: syn::PatSlice,
    ) -> Result<syn::PatSlice, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.bracket_token = t.bracket_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_pat(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_pat_struct<F>(
        f: &mut F,
        t: syn::PatStruct,
    ) -> Result<syn::PatStruct, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .qself = match t.qself {
            Some(o) => Some(f.try_fold_q_self(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        t.brace_token = t.brace_token;
        t
            .fields = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.fields {
                tmp.push(f.try_fold_field_pat(v)?);
            }
            tmp
        };
        t
            .rest = match t.rest {
            Some(o) => Some(f.try_fold_pat_rest(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_pat_tuple<F>(
        f: &mut F,
        t: syn::PatTuple,
    ) -> Result<syn::PatTuple, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.paren_token = t.paren_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_pat(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_pat_tuple_struct<F>(
        f: &mut F,
        t: syn::PatTupleStruct,
    ) -> Result<syn::PatTupleStruct, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .qself = match t.qself {
            Some(o) => Some(f.try_fold_q_self(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        t.paren_token = t.paren_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_pat(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_pat_type<F>(
        f: &mut F,
        t: syn::PatType,
    ) -> Result<syn::PatType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.pat = Box::new(f.try_fold_pat(*t.pat)?);
        t.colon_token = t.colon_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        Ok(t)
    }
    pub fn try_fold_pat_wild<F>(
        f: &mut F,
        t: syn::PatWild,
    ) -> Result<syn::PatWild, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.underscore_token = t.underscore_token;
        Ok(t)
    }
    pub fn try_fold_path<F>(
        f: &mut F,
        t: syn::Path,
    ) -> Result<syn::Path, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .leading_colon = match t.leading_colon {
            Some(o) => Some(o),
            None => None,
        };
        t
            .segments = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.segments {
                tmp.push(f.try_fold_path_segment(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_path_arguments<F>(
        f: &mut F,
        t: syn::PathArguments,
    ) -> Result<syn::PathArguments, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::PathArguments::AngleBracketed(tmp0) => {
                f.try_fold_path_arguments_variant_angle_bracketed((tmp0))?
            }
            syn::PathArguments::Parenthesized(tmp0) => {
                f.try_fold_path_arguments_variant_parenthesized((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_path_arguments_variant_angle_bracketed<F>(
        f: &mut F,
        t: (syn::AngleBracketedGenericArguments),
    ) -> Result<syn::PathArguments, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_angle_bracketed_generic_arguments(t)?);
        Ok(syn::PathArguments::AngleBracketed(t))
    }
    pub fn try_fold_path_arguments_variant_parenthesized<F>(
        f: &mut F,
        t: (syn::ParenthesizedGenericArguments),
    ) -> Result<syn::PathArguments, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_parenthesized_generic_arguments(t)?);
        Ok(syn::PathArguments::Parenthesized(t))
    }
    pub fn try_fold_path_segment<F>(
        f: &mut F,
        t: syn::PathSegment,
    ) -> Result<syn::PathSegment, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t.arguments = f.try_fold_path_arguments(t.arguments)?;
        Ok(t)
    }
    pub fn try_fold_predicate_lifetime<F>(
        f: &mut F,
        t: syn::PredicateLifetime,
    ) -> Result<syn::PredicateLifetime, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.lifetime = f.try_fold_lifetime(t.lifetime)?;
        t.colon_token = t.colon_token;
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_lifetime(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_predicate_type<F>(
        f: &mut F,
        t: syn::PredicateType,
    ) -> Result<syn::PredicateType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .lifetimes = match t.lifetimes {
            Some(o) => Some(f.try_fold_bound_lifetimes(o)?),
            None => None,
        };
        t.bounded_ty = f.try_fold_type(t.bounded_ty)?;
        t.colon_token = t.colon_token;
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_q_self<F>(
        f: &mut F,
        t: syn::QSelf,
    ) -> Result<syn::QSelf, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.lt_token = t.lt_token;
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        t.position = t.position;
        t
            .as_token = match t.as_token {
            Some(o) => Some(o),
            None => None,
        };
        t.gt_token = t.gt_token;
        Ok(t)
    }
    pub fn try_fold_range_limits<F>(
        f: &mut F,
        t: syn::RangeLimits,
    ) -> Result<syn::RangeLimits, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::RangeLimits::HalfOpen(tmp0) => {
                f.try_fold_range_limits_variant_half_open((tmp0))?
            }
            syn::RangeLimits::Closed(tmp0) => {
                f.try_fold_range_limits_variant_closed((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_range_limits_variant_half_open<F>(
        f: &mut F,
        t: (syn::token::DotDot),
    ) -> Result<syn::RangeLimits, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::RangeLimits::HalfOpen(t))
    }
    pub fn try_fold_range_limits_variant_closed<F>(
        f: &mut F,
        t: (syn::token::DotDotEq),
    ) -> Result<syn::RangeLimits, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::RangeLimits::Closed(t))
    }
    pub fn try_fold_receiver<F>(
        f: &mut F,
        t: syn::Receiver,
    ) -> Result<syn::Receiver, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .reference = match t.reference {
            Some(o) => {
                Some((
                    o.0,
                    match o.1 {
                        Some(o) => Some(f.try_fold_lifetime(o)?),
                        None => None,
                    },
                ))
            }
            None => None,
        };
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.self_token = t.self_token;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t.ty = Box::new(f.try_fold_type(*t.ty)?);
        Ok(t)
    }
    pub fn try_fold_return_type<F>(
        f: &mut F,
        t: syn::ReturnType,
    ) -> Result<syn::ReturnType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::ReturnType::Type(tmp0, tmp1) => {
                f.try_fold_return_type_variant_type((tmp0, tmp1))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_return_type_variant_type<F>(
        f: &mut F,
        t: (syn::token::RArrow, Box<syn::Type>),
    ) -> Result<syn::ReturnType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t.0, Box::new(f.try_fold_type(*t.1)?));
        Ok(syn::ReturnType::Type(t.0, t.1))
    }
    pub fn try_fold_signature<F>(
        f: &mut F,
        t: syn::Signature,
    ) -> Result<syn::Signature, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .constness = match t.constness {
            Some(o) => Some(o),
            None => None,
        };
        t
            .asyncness = match t.asyncness {
            Some(o) => Some(o),
            None => None,
        };
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t
            .abi = match t.abi {
            Some(o) => Some(f.try_fold_abi(o)?),
            None => None,
        };
        t.fn_token = t.fn_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.paren_token = t.paren_token;
        t
            .inputs = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.inputs {
                tmp.push(f.try_fold_fn_arg(v)?);
            }
            tmp
        };
        t
            .variadic = match t.variadic {
            Some(o) => Some(f.try_fold_variadic(o)?),
            None => None,
        };
        t.output = f.try_fold_return_type(t.output)?;
        Ok(t)
    }
    pub fn try_fold_static_mutability<F>(
        f: &mut F,
        t: syn::StaticMutability,
    ) -> Result<syn::StaticMutability, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::StaticMutability::Mut(tmp0) => {
                f.try_fold_static_mutability_variant_mut((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_static_mutability_variant_mut<F>(
        f: &mut F,
        t: (syn::token::Mut),
    ) -> Result<syn::StaticMutability, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::StaticMutability::Mut(t))
    }
    pub fn try_fold_stmt<F>(
        f: &mut F,
        t: syn::Stmt,
    ) -> Result<syn::Stmt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Stmt::Local(tmp0) => f.try_fold_stmt_variant_local((tmp0))?,
            syn::Stmt::Item(tmp0) => f.try_fold_stmt_variant_item((tmp0))?,
            syn::Stmt::Expr(tmp0, tmp1) => f.try_fold_stmt_variant_expr((tmp0, tmp1))?,
            syn::Stmt::Macro(tmp0) => f.try_fold_stmt_variant_macro((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_stmt_variant_local<F>(
        f: &mut F,
        t: (syn::Local),
    ) -> Result<syn::Stmt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_local(t)?);
        Ok(syn::Stmt::Local(t))
    }
    pub fn try_fold_stmt_variant_item<F>(
        f: &mut F,
        t: (syn::Item),
    ) -> Result<syn::Stmt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_item(t)?);
        Ok(syn::Stmt::Item(t))
    }
    pub fn try_fold_stmt_variant_expr<F>(
        f: &mut F,
        t: (syn::Expr, Option<syn::token::Semi>),
    ) -> Result<syn::Stmt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (
            f.try_fold_expr(t.0)?,
            match t.1 {
                Some(o) => Some(o),
                None => None,
            },
        );
        Ok(syn::Stmt::Expr(t.0, t.1))
    }
    pub fn try_fold_stmt_variant_macro<F>(
        f: &mut F,
        t: (syn::StmtMacro),
    ) -> Result<syn::Stmt, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_stmt_macro(t)?);
        Ok(syn::Stmt::Macro(t))
    }
    pub fn try_fold_stmt_macro<F>(
        f: &mut F,
        t: syn::StmtMacro,
    ) -> Result<syn::StmtMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.mac = f.try_fold_macro(t.mac)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_trait_bound<F>(
        f: &mut F,
        t: syn::TraitBound,
    ) -> Result<syn::TraitBound, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .paren_token = match t.paren_token {
            Some(o) => Some(o),
            None => None,
        };
        t.modifier = f.try_fold_trait_bound_modifier(t.modifier)?;
        t
            .lifetimes = match t.lifetimes {
            Some(o) => Some(f.try_fold_bound_lifetimes(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        Ok(t)
    }
    pub fn try_fold_trait_bound_modifier<F>(
        f: &mut F,
        t: syn::TraitBoundModifier,
    ) -> Result<syn::TraitBoundModifier, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::TraitBoundModifier::Maybe(tmp0) => {
                f.try_fold_trait_bound_modifier_variant_maybe((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_trait_bound_modifier_variant_maybe<F>(
        f: &mut F,
        t: (syn::token::Question),
    ) -> Result<syn::TraitBoundModifier, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::TraitBoundModifier::Maybe(t))
    }
    pub fn try_fold_trait_item<F>(
        f: &mut F,
        t: syn::TraitItem,
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::TraitItem::Const(tmp0) => f.try_fold_trait_item_variant_const((tmp0))?,
            syn::TraitItem::Fn(tmp0) => f.try_fold_trait_item_variant_fn((tmp0))?,
            syn::TraitItem::Type(tmp0) => f.try_fold_trait_item_variant_type((tmp0))?,
            syn::TraitItem::Macro(tmp0) => f.try_fold_trait_item_variant_macro((tmp0))?,
            syn::TraitItem::Verbatim(tmp0) => {
                f.try_fold_trait_item_variant_verbatim((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_trait_item_variant_const<F>(
        f: &mut F,
        t: (syn::TraitItemConst),
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_trait_item_const(t)?);
        Ok(syn::TraitItem::Const(t))
    }
    pub fn try_fold_trait_item_variant_fn<F>(
        f: &mut F,
        t: (syn::TraitItemFn),
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_trait_item_fn(t)?);
        Ok(syn::TraitItem::Fn(t))
    }
    pub fn try_fold_trait_item_variant_type<F>(
        f: &mut F,
        t: (syn::TraitItemType),
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_trait_item_type(t)?);
        Ok(syn::TraitItem::Type(t))
    }
    pub fn try_fold_trait_item_variant_macro<F>(
        f: &mut F,
        t: (syn::TraitItemMacro),
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_trait_item_macro(t)?);
        Ok(syn::TraitItem::Macro(t))
    }
    pub fn try_fold_trait_item_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::TraitItem, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::TraitItem::Verbatim(t))
    }
    pub fn try_fold_trait_item_const<F>(
        f: &mut F,
        t: syn::TraitItemConst,
    ) -> Result<syn::TraitItemConst, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.const_token = t.const_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t.colon_token = t.colon_token;
        t.ty = f.try_fold_type(t.ty)?;
        t
            .default = match t.default {
            Some(o) => Some((o.0, f.try_fold_expr(o.1)?)),
            None => None,
        };
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_trait_item_fn<F>(
        f: &mut F,
        t: syn::TraitItemFn,
    ) -> Result<syn::TraitItemFn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.sig = f.try_fold_signature(t.sig)?;
        t
            .default = match t.default {
            Some(o) => Some(f.try_fold_block(o)?),
            None => None,
        };
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_trait_item_macro<F>(
        f: &mut F,
        t: syn::TraitItemMacro,
    ) -> Result<syn::TraitItemMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.mac = f.try_fold_macro(t.mac)?;
        t
            .semi_token = match t.semi_token {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_trait_item_type<F>(
        f: &mut F,
        t: syn::TraitItemType,
    ) -> Result<syn::TraitItemType, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.type_token = t.type_token;
        t.ident = t.ident;
        t.generics = f.try_fold_generics(t.generics)?;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        t
            .default = match t.default {
            Some(o) => Some((o.0, f.try_fold_type(o.1)?)),
            None => None,
        };
        t.semi_token = t.semi_token;
        Ok(t)
    }
    pub fn try_fold_type<F>(
        f: &mut F,
        t: syn::Type,
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Type::Array(tmp0) => f.try_fold_type_variant_array((tmp0))?,
            syn::Type::BareFn(tmp0) => f.try_fold_type_variant_bare_fn((tmp0))?,
            syn::Type::Group(tmp0) => f.try_fold_type_variant_group((tmp0))?,
            syn::Type::ImplTrait(tmp0) => f.try_fold_type_variant_impl_trait((tmp0))?,
            syn::Type::Infer(tmp0) => f.try_fold_type_variant_infer((tmp0))?,
            syn::Type::Macro(tmp0) => f.try_fold_type_variant_macro((tmp0))?,
            syn::Type::Never(tmp0) => f.try_fold_type_variant_never((tmp0))?,
            syn::Type::Paren(tmp0) => f.try_fold_type_variant_paren((tmp0))?,
            syn::Type::Path(tmp0) => f.try_fold_type_variant_path((tmp0))?,
            syn::Type::Ptr(tmp0) => f.try_fold_type_variant_ptr((tmp0))?,
            syn::Type::Reference(tmp0) => f.try_fold_type_variant_reference((tmp0))?,
            syn::Type::Slice(tmp0) => f.try_fold_type_variant_slice((tmp0))?,
            syn::Type::TraitObject(tmp0) => f.try_fold_type_variant_trait_object((tmp0))?,
            syn::Type::Tuple(tmp0) => f.try_fold_type_variant_tuple((tmp0))?,
            syn::Type::Verbatim(tmp0) => f.try_fold_type_variant_verbatim((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_type_variant_array<F>(
        f: &mut F,
        t: (syn::TypeArray),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_array(t)?);
        Ok(syn::Type::Array(t))
    }
    pub fn try_fold_type_variant_bare_fn<F>(
        f: &mut F,
        t: (syn::TypeBareFn),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_bare_fn(t)?);
        Ok(syn::Type::BareFn(t))
    }
    pub fn try_fold_type_variant_group<F>(
        f: &mut F,
        t: (syn::TypeGroup),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_group(t)?);
        Ok(syn::Type::Group(t))
    }
    pub fn try_fold_type_variant_impl_trait<F>(
        f: &mut F,
        t: (syn::TypeImplTrait),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_impl_trait(t)?);
        Ok(syn::Type::ImplTrait(t))
    }
    pub fn try_fold_type_variant_infer<F>(
        f: &mut F,
        t: (syn::TypeInfer),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_infer(t)?);
        Ok(syn::Type::Infer(t))
    }
    pub fn try_fold_type_variant_macro<F>(
        f: &mut F,
        t: (syn::TypeMacro),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_macro(t)?);
        Ok(syn::Type::Macro(t))
    }
    pub fn try_fold_type_variant_never<F>(
        f: &mut F,
        t: (syn::TypeNever),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_never(t)?);
        Ok(syn::Type::Never(t))
    }
    pub fn try_fold_type_variant_paren<F>(
        f: &mut F,
        t: (syn::TypeParen),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_paren(t)?);
        Ok(syn::Type::Paren(t))
    }
    pub fn try_fold_type_variant_path<F>(
        f: &mut F,
        t: (syn::TypePath),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_path(t)?);
        Ok(syn::Type::Path(t))
    }
    pub fn try_fold_type_variant_ptr<F>(
        f: &mut F,
        t: (syn::TypePtr),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_ptr(t)?);
        Ok(syn::Type::Ptr(t))
    }
    pub fn try_fold_type_variant_reference<F>(
        f: &mut F,
        t: (syn::TypeReference),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_reference(t)?);
        Ok(syn::Type::Reference(t))
    }
    pub fn try_fold_type_variant_slice<F>(
        f: &mut F,
        t: (syn::TypeSlice),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_slice(t)?);
        Ok(syn::Type::Slice(t))
    }
    pub fn try_fold_type_variant_trait_object<F>(
        f: &mut F,
        t: (syn::TypeTraitObject),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_trait_object(t)?);
        Ok(syn::Type::TraitObject(t))
    }
    pub fn try_fold_type_variant_tuple<F>(
        f: &mut F,
        t: (syn::TypeTuple),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_type_tuple(t)?);
        Ok(syn::Type::Tuple(t))
    }
    pub fn try_fold_type_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::Type, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Type::Verbatim(t))
    }
    pub fn try_fold_type_array<F>(
        f: &mut F,
        t: syn::TypeArray,
    ) -> Result<syn::TypeArray, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.bracket_token = t.bracket_token;
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        t.semi_token = t.semi_token;
        t.len = f.try_fold_expr(t.len)?;
        Ok(t)
    }
    pub fn try_fold_type_bare_fn<F>(
        f: &mut F,
        t: syn::TypeBareFn,
    ) -> Result<syn::TypeBareFn, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .lifetimes = match t.lifetimes {
            Some(o) => Some(f.try_fold_bound_lifetimes(o)?),
            None => None,
        };
        t
            .unsafety = match t.unsafety {
            Some(o) => Some(o),
            None => None,
        };
        t
            .abi = match t.abi {
            Some(o) => Some(f.try_fold_abi(o)?),
            None => None,
        };
        t.fn_token = t.fn_token;
        t.paren_token = t.paren_token;
        t
            .inputs = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.inputs {
                tmp.push(f.try_fold_bare_fn_arg(v)?);
            }
            tmp
        };
        t
            .variadic = match t.variadic {
            Some(o) => Some(f.try_fold_bare_variadic(o)?),
            None => None,
        };
        t.output = f.try_fold_return_type(t.output)?;
        Ok(t)
    }
    pub fn try_fold_type_group<F>(
        f: &mut F,
        t: syn::TypeGroup,
    ) -> Result<syn::TypeGroup, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.group_token = t.group_token;
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        Ok(t)
    }
    pub fn try_fold_type_impl_trait<F>(
        f: &mut F,
        t: syn::TypeImplTrait,
    ) -> Result<syn::TypeImplTrait, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.impl_token = t.impl_token;
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_type_infer<F>(
        f: &mut F,
        t: syn::TypeInfer,
    ) -> Result<syn::TypeInfer, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.underscore_token = t.underscore_token;
        Ok(t)
    }
    pub fn try_fold_type_macro<F>(
        f: &mut F,
        t: syn::TypeMacro,
    ) -> Result<syn::TypeMacro, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.mac = f.try_fold_macro(t.mac)?;
        Ok(t)
    }
    pub fn try_fold_type_never<F>(
        f: &mut F,
        t: syn::TypeNever,
    ) -> Result<syn::TypeNever, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.bang_token = t.bang_token;
        Ok(t)
    }
    pub fn try_fold_type_param<F>(
        f: &mut F,
        t: syn::TypeParam,
    ) -> Result<syn::TypeParam, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.ident = t.ident;
        t
            .colon_token = match t.colon_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        t
            .eq_token = match t.eq_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .default = match t.default {
            Some(o) => Some(f.try_fold_type(o)?),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_type_param_bound<F>(
        f: &mut F,
        t: syn::TypeParamBound,
    ) -> Result<syn::TypeParamBound, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::TypeParamBound::Trait(tmp0) => {
                f.try_fold_type_param_bound_variant_trait((tmp0))?
            }
            syn::TypeParamBound::Lifetime(tmp0) => {
                f.try_fold_type_param_bound_variant_lifetime((tmp0))?
            }
            syn::TypeParamBound::Verbatim(tmp0) => {
                f.try_fold_type_param_bound_variant_verbatim((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_type_param_bound_variant_trait<F>(
        f: &mut F,
        t: (syn::TraitBound),
    ) -> Result<syn::TypeParamBound, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_trait_bound(t)?);
        Ok(syn::TypeParamBound::Trait(t))
    }
    pub fn try_fold_type_param_bound_variant_lifetime<F>(
        f: &mut F,
        t: (syn::Lifetime),
    ) -> Result<syn::TypeParamBound, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_lifetime(t)?);
        Ok(syn::TypeParamBound::Lifetime(t))
    }
    pub fn try_fold_type_param_bound_variant_verbatim<F>(
        f: &mut F,
        t: (proc_macro2::TokenStream),
    ) -> Result<syn::TypeParamBound, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::TypeParamBound::Verbatim(t))
    }
    pub fn try_fold_type_paren<F>(
        f: &mut F,
        t: syn::TypeParen,
    ) -> Result<syn::TypeParen, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.paren_token = t.paren_token;
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        Ok(t)
    }
    pub fn try_fold_type_path<F>(
        f: &mut F,
        t: syn::TypePath,
    ) -> Result<syn::TypePath, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .qself = match t.qself {
            Some(o) => Some(f.try_fold_q_self(o)?),
            None => None,
        };
        t.path = f.try_fold_path(t.path)?;
        Ok(t)
    }
    pub fn try_fold_type_ptr<F>(
        f: &mut F,
        t: syn::TypePtr,
    ) -> Result<syn::TypePtr, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.star_token = t.star_token;
        t
            .const_token = match t.const_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        Ok(t)
    }
    pub fn try_fold_type_reference<F>(
        f: &mut F,
        t: syn::TypeReference,
    ) -> Result<syn::TypeReference, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.and_token = t.and_token;
        t
            .lifetime = match t.lifetime {
            Some(o) => Some(f.try_fold_lifetime(o)?),
            None => None,
        };
        t
            .mutability = match t.mutability {
            Some(o) => Some(o),
            None => None,
        };
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        Ok(t)
    }
    pub fn try_fold_type_slice<F>(
        f: &mut F,
        t: syn::TypeSlice,
    ) -> Result<syn::TypeSlice, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.bracket_token = t.bracket_token;
        t.elem = Box::new(f.try_fold_type(*t.elem)?);
        Ok(t)
    }
    pub fn try_fold_type_trait_object<F>(
        f: &mut F,
        t: syn::TypeTraitObject,
    ) -> Result<syn::TypeTraitObject, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .dyn_token = match t.dyn_token {
            Some(o) => Some(o),
            None => None,
        };
        t
            .bounds = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.bounds {
                tmp.push(f.try_fold_type_param_bound(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_type_tuple<F>(
        f: &mut F,
        t: syn::TypeTuple,
    ) -> Result<syn::TypeTuple, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.paren_token = t.paren_token;
        t
            .elems = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.elems {
                tmp.push(f.try_fold_type(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_un_op<F>(
        f: &mut F,
        t: syn::UnOp,
    ) -> Result<syn::UnOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::UnOp::Deref(tmp0) => f.try_fold_un_op_variant_deref((tmp0))?,
            syn::UnOp::Not(tmp0) => f.try_fold_un_op_variant_not((tmp0))?,
            syn::UnOp::Neg(tmp0) => f.try_fold_un_op_variant_neg((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_un_op_variant_deref<F>(
        f: &mut F,
        t: (syn::token::Star),
    ) -> Result<syn::UnOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::UnOp::Deref(t))
    }
    pub fn try_fold_un_op_variant_not<F>(
        f: &mut F,
        t: (syn::token::Not),
    ) -> Result<syn::UnOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::UnOp::Not(t))
    }
    pub fn try_fold_un_op_variant_neg<F>(
        f: &mut F,
        t: (syn::token::Minus),
    ) -> Result<syn::UnOp, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::UnOp::Neg(t))
    }
    pub fn try_fold_use_glob<F>(
        f: &mut F,
        t: syn::UseGlob,
    ) -> Result<syn::UseGlob, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.star_token = t.star_token;
        Ok(t)
    }
    pub fn try_fold_use_group<F>(
        f: &mut F,
        t: syn::UseGroup,
    ) -> Result<syn::UseGroup, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.brace_token = t.brace_token;
        t
            .items = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.items {
                tmp.push(f.try_fold_use_tree(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_use_name<F>(
        f: &mut F,
        t: syn::UseName,
    ) -> Result<syn::UseName, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        Ok(t)
    }
    pub fn try_fold_use_path<F>(
        f: &mut F,
        t: syn::UsePath,
    ) -> Result<syn::UsePath, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t.colon2_token = t.colon2_token;
        t.tree = Box::new(f.try_fold_use_tree(*t.tree)?);
        Ok(t)
    }
    pub fn try_fold_use_rename<F>(
        f: &mut F,
        t: syn::UseRename,
    ) -> Result<syn::UseRename, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.ident = t.ident;
        t.as_token = t.as_token;
        t.rename = t.rename;
        Ok(t)
    }
    pub fn try_fold_use_tree<F>(
        f: &mut F,
        t: syn::UseTree,
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::UseTree::Path(tmp0) => f.try_fold_use_tree_variant_path((tmp0))?,
            syn::UseTree::Name(tmp0) => f.try_fold_use_tree_variant_name((tmp0))?,
            syn::UseTree::Rename(tmp0) => f.try_fold_use_tree_variant_rename((tmp0))?,
            syn::UseTree::Glob(tmp0) => f.try_fold_use_tree_variant_glob((tmp0))?,
            syn::UseTree::Group(tmp0) => f.try_fold_use_tree_variant_group((tmp0))?,
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_use_tree_variant_path<F>(
        f: &mut F,
        t: (syn::UsePath),
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_use_path(t)?);
        Ok(syn::UseTree::Path(t))
    }
    pub fn try_fold_use_tree_variant_name<F>(
        f: &mut F,
        t: (syn::UseName),
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_use_name(t)?);
        Ok(syn::UseTree::Name(t))
    }
    pub fn try_fold_use_tree_variant_rename<F>(
        f: &mut F,
        t: (syn::UseRename),
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_use_rename(t)?);
        Ok(syn::UseTree::Rename(t))
    }
    pub fn try_fold_use_tree_variant_glob<F>(
        f: &mut F,
        t: (syn::UseGlob),
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_use_glob(t)?);
        Ok(syn::UseTree::Glob(t))
    }
    pub fn try_fold_use_tree_variant_group<F>(
        f: &mut F,
        t: (syn::UseGroup),
    ) -> Result<syn::UseTree, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_use_group(t)?);
        Ok(syn::UseTree::Group(t))
    }
    pub fn try_fold_variadic<F>(
        f: &mut F,
        t: syn::Variadic,
    ) -> Result<syn::Variadic, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t
            .pat = match t.pat {
            Some(o) => Some((Box::new(f.try_fold_pat(*o.0)?), o.1)),
            None => None,
        };
        t.dots = t.dots;
        t
            .comma = match t.comma {
            Some(o) => Some(o),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_variant<F>(
        f: &mut F,
        t: syn::Variant,
    ) -> Result<syn::Variant, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t
            .attrs = {
            let mut tmp = vec![];
            for v in t.attrs {
                tmp.push(f.try_fold_attribute(v)?);
            }
            tmp
        };
        t.ident = t.ident;
        t.fields = f.try_fold_fields(t.fields)?;
        t
            .discriminant = match t.discriminant {
            Some(o) => Some((o.0, f.try_fold_expr(o.1)?)),
            None => None,
        };
        Ok(t)
    }
    pub fn try_fold_vis_restricted<F>(
        f: &mut F,
        t: syn::VisRestricted,
    ) -> Result<syn::VisRestricted, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.pub_token = t.pub_token;
        t.paren_token = t.paren_token;
        t
            .in_token = match t.in_token {
            Some(o) => Some(o),
            None => None,
        };
        t.path = Box::new(f.try_fold_path(*t.path)?);
        Ok(t)
    }
    pub fn try_fold_visibility<F>(
        f: &mut F,
        t: syn::Visibility,
    ) -> Result<syn::Visibility, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::Visibility::Public(tmp0) => {
                f.try_fold_visibility_variant_public((tmp0))?
            }
            syn::Visibility::Restricted(tmp0) => {
                f.try_fold_visibility_variant_restricted((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_visibility_variant_public<F>(
        f: &mut F,
        t: (syn::token::Pub),
    ) -> Result<syn::Visibility, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (t);
        Ok(syn::Visibility::Public(t))
    }
    pub fn try_fold_visibility_variant_restricted<F>(
        f: &mut F,
        t: (syn::VisRestricted),
    ) -> Result<syn::Visibility, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_vis_restricted(t)?);
        Ok(syn::Visibility::Restricted(t))
    }
    pub fn try_fold_where_clause<F>(
        f: &mut F,
        t: syn::WhereClause,
    ) -> Result<syn::WhereClause, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let mut t = t;
        t.where_token = t.where_token;
        t
            .predicates = {
            let mut tmp = syn::punctuated::Punctuated::new();
            for v in t.predicates {
                tmp.push(f.try_fold_where_predicate(v)?);
            }
            tmp
        };
        Ok(t)
    }
    pub fn try_fold_where_predicate<F>(
        f: &mut F,
        t: syn::WherePredicate,
    ) -> Result<syn::WherePredicate, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = match t {
            syn::WherePredicate::Lifetime(tmp0) => {
                f.try_fold_where_predicate_variant_lifetime((tmp0))?
            }
            syn::WherePredicate::Type(tmp0) => {
                f.try_fold_where_predicate_variant_type((tmp0))?
            }
            t => t,
        };
        Ok(t)
    }
    pub fn try_fold_where_predicate_variant_lifetime<F>(
        f: &mut F,
        t: (syn::PredicateLifetime),
    ) -> Result<syn::WherePredicate, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_predicate_lifetime(t)?);
        Ok(syn::WherePredicate::Lifetime(t))
    }
    pub fn try_fold_where_predicate_variant_type<F>(
        f: &mut F,
        t: (syn::PredicateType),
    ) -> Result<syn::WherePredicate, <F as TryFold>::Error>
    where
        F: TryFold + ?Sized,
    {
        let t = (f.try_fold_predicate_type(t)?);
        Ok(syn::WherePredicate::Type(t))
    }
}
