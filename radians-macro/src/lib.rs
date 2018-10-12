#[macro_use]
extern crate quote;
extern crate proc_macro;

use self::proc_macro::TokenStream;
use syn::{Expr, Lit};
use radians::Radians;

#[proc_macro]
pub fn radians(tokens: TokenStream) -> TokenStream {

    let expression: Expr = syn::parse(tokens).unwrap();
    
    let literal = match expression {
        Expr::Lit(expr_lit) => expr_lit.lit,
        _ => panic!("Only literal values are accepted")
    };

    let value = match literal {
        Lit::Float(lit_float) => lit_float.value(),
        _ => panic!("Only floating point values are acepted")
    };

    if Radians::is_in_range(value) {
        let gen = quote! {
            ::radians::Radians::new(#value).unwrap()
        };

        gen.into()
    } else {
        panic!("Only values in the range [0.0; 2π) are accepted")
    }
}
