extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod tuple;

#[proc_macro_derive(UtilsTupleSerDe)]
pub fn tuple_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let from_str = tuple::from_str(&ast);
    let display = tuple::display(&ast);
    let serde = tuple::serde(&ast);

    let ts = quote! {
        #from_str
        #display
        #serde
    };

    ts.into()
}
