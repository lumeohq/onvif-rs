use proc_macro2::TokenStream;
use quote;
use syn::spanned::Spanned;

enum Type {
    Simple(syn::Path),
    String(syn::Path),
    Struct(syn::Path),
    Vec(syn::Path, syn::Path),
}

pub fn from_to_string(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let struct_name = &ast.ident;

            let field_path = extract_field_path(&data_struct).expect("Bad field count or type");

            let ty = Type::from_path(&field_path);

            let from = from_str(&ty);

            let to = to_string(&ty);

            quote! {
                impl #struct_name {
                    pub fn from_str(s: &str) -> Result<#struct_name, String> {
                        use std::str::FromStr;

                        Ok(#struct_name(#from))
                    }

                    pub fn to_string(&self) -> String {
                        use itertools::Itertools;

                        #to
                    }
                }
            }
        }
        _ => unimplemented!("Implemented only for structs"),
    }
}

fn from_str(ty: &Type) -> TokenStream {
    match ty {
        Type::String(_) => quote! { s.to_string() },
        Type::Simple(ty) => quote! { #ty::from_str(s).map_err(|e| e.to_string())? },
        Type::Struct(ty) => quote! { #ty::from_str(s)? },
        Type::Vec(_, subtype) => match Type::from_path(&subtype) {
            Type::String(subtype) | Type::Struct(subtype) | Type::Simple(subtype) => quote! {
                s.split_whitespace()
                    .filter_map(|s| #subtype::from_str(s).ok())
                    .collect()
            },
            _ => syn::Error::new(subtype.span(), "Not implemented for this subtype")
                .to_compile_error(),
        },
    }
}

fn to_string(ty: &Type) -> TokenStream {
    match ty {
        Type::String(_) => quote! { self.0.clone() },
        Type::Simple(_) | Type::Struct(_) => quote! { self.0.to_string() },
        Type::Vec(_, subtype) => match Type::from_path(&subtype) {
            Type::String(_) | Type::Simple(_) | Type::Struct(_) => quote! {
                self.0
                    .iter()
                    .map(|x| x.to_string())
                    .join(" ")
            },
            _ => syn::Error::new(subtype.span(), "Not implemented for this subtype")
                .to_compile_error(),
        },
    }
}

impl Type {
    pub fn from_path(path: &syn::Path) -> Type {
        match path
            .segments
            .last()
            .expect("Empty type")
            .ident
            .to_string()
            .as_str()
        {
            "bool" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f32"
            | "f64" => Type::Simple(path.clone()),
            "String" => Type::String(path.clone()),
            "Vec" => Type::Vec(
                path.clone(),
                extract_subtype(path.segments.last().expect("Missing subtype"))
                    .expect("Vec subtype not found")
                    .clone(),
            ),
            _ => Type::Struct(path.clone()),
        }
    }
}

pub fn serde(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let struct_name_literal = &ast.ident.to_string();

    quote! {
        impl YaSerialize for #struct_name {
            fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
                utils::yaserde::serialize(self, #struct_name_literal, writer, |s| s.to_string())
            }
        }

        impl YaDeserialize for #struct_name {
            fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
                utils::yaserde::deserialize(reader, |s| #struct_name::from_str(s))
            }
        }
    }
}

fn extract_field_path(data_struct: &syn::DataStruct) -> Option<&syn::Path> {
    if let syn::Fields::Unnamed(fields) = &data_struct.fields {
        if let Some(field) = fields.unnamed.first() {
            if let syn::Type::Path(path) = &field.ty {
                return Some(&path.path);
            }
        }
    }

    None
}

fn extract_subtype(path: &syn::PathSegment) -> Option<&syn::Path> {
    if let syn::PathArguments::AngleBracketed(args) = &path.arguments {
        if let Some(arg) = args.args.last() {
            if let syn::GenericArgument::Type(ty) = arg {
                if let syn::Type::Path(path) = ty {
                    return Some(&path.path);
                }
            }
        }
    }

    None
}
