// library
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Fields, Lit, Meta, MetaNameValue};

// std
use std::path::Path;

// crate
use crate::extension::is_common;

pub struct FileInfo {
    pub rel_path: String,
    pub full_canonical_path: String,
    pub extension: String,
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .to_str()
        .expect("Path does not have a string representation")
        .to_owned()
}

pub fn get_files(folder_path: String) -> impl Iterator<Item = FileInfo> {
    walkdir::WalkDir::new(&folder_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| x.file_type().is_file())
        .map(move |x| {
            let rel_path = path_to_str(x.path().strip_prefix(&folder_path).unwrap());
            let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
                rel_path.replace('\\', "/")
            } else {
                rel_path
            };

            let full_canonical_path =
                path_to_str(std::fs::canonicalize(x.path()).expect("Could not get canonical path"));

            let extension = x
                .path()
                .extension()
                .expect("Could not get file extension")
                .to_str()
                .unwrap()
                .to_string();

            FileInfo {
                rel_path,
                full_canonical_path,
                extension,
            }
        })
}

fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
    quote! {
        #rel_path => {
            let bytes = &include_bytes!(#full_canonical_path)[..];
            Some(std::borrow::Cow::from(bytes))
        },
    }
}

fn embedded(ident: &syn::Ident, folder_path: String) -> TokenStream2 {
    let mut match_values = Vec::<TokenStream2>::new();
    let mut list_values = Vec::<String>::new();

    for FileInfo {
        rel_path,
        full_canonical_path,
        extension,
    } in get_files(folder_path)
    {
        if is_common(&extension) {
            match_values.push(embed_file(&rel_path, &full_canonical_path));
            list_values.push(rel_path);
        }
    }

    let handle_prefix = TokenStream2::new();

    quote! {
        impl #ident {
            pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
                #handle_prefix
                match file_path.replace("\\", "/").as_str() {
                    #(#match_values)*
                    _ => None,
                }
            }
        }
    }
}

/// Find a `name = "value"` attribute from the derive input
fn find_attribute_value(ast: &syn::DeriveInput, attr_name: &str) -> Option<String> {
    ast.attrs
        .iter()
        .find(|value| value.path.is_ident(attr_name))
        .and_then(|attr| attr.parse_meta().ok())
        .and_then(|meta| match meta {
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(val), ..
            }) => Some(val.value()),
            _ => None,
        })
}

pub fn validate_unit_struct(data: &Data) {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unit => {}
            _ => panic!("EmbedAssets can only be derived for unit structs"),
        },
        _ => panic!("EmbedAssets can only be derived for unit structs"),
    };
}

pub fn get_folder_path(ast: &syn::DeriveInput) -> String {
    let folder_name = find_attribute_value(ast, "folder").expect(
        "#[derive(EmbedAssets)] should contain one attribute like this #[folder = \"assets\"]",
    );

    let folder_path = std::env::current_dir()
        .unwrap()
        .join(folder_name)
        .to_str()
        .unwrap()
        .to_owned();

    if !Path::new(&folder_path).exists() {
        panic!(format!(
            "#[derive(EmbedAssets)] folder '{}' does not exist. current-work-directory is: '{}'",
            folder_path,
            std::env::current_dir().unwrap().to_str().unwrap()
        ));
    };

    return folder_path;
}

pub fn impl_embed_assets(ast: &syn::DeriveInput) -> TokenStream2 {
    validate_unit_struct(&ast.data);
    let folder_path = get_folder_path(&ast);

    let embedded_impl = embedded(&ast.ident, folder_path);
    quote! {
        #embedded_impl
    }
}
