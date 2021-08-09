// library
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

// crate
use crate::{common, extension::is_font};

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

    for common::FileInfo {
        rel_path,
        full_canonical_path,
        extension,
    } in common::get_files(folder_path)
    {
        if is_font(&extension) {
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

pub fn impl_embed_assets(ast: &syn::DeriveInput) -> TokenStream2 {
    common::validate_unit_struct(&ast.data);
    let folder_path = common::get_folder_path(&ast);

    let embedded_impl = embedded(&ast.ident, folder_path);
    quote! {
        #embedded_impl
    }
}
