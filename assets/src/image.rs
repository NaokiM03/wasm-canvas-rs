// library
use image::{DynamicImage, RgbaImage};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde::Serialize;

// crate
use crate::{common, extension::is_image};

#[derive(Serialize)]
struct Image {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
    let dynamic_image: DynamicImage =
        image::open(full_canonical_path).expect("Could not open png file.");
    let image_bytes = dynamic_image.as_bytes().to_vec();
    let rgba_image: RgbaImage = dynamic_image.to_rgba8();
    let image = Image {
        bytes: image_bytes,
        width: rgba_image.width() as usize,
        height: rgba_image.height() as usize,
    };
    let json_str = serde_json::to_string(&image).unwrap();

    quote! {
        #rel_path => {
            Some(std::borrow::Cow::from(#json_str))
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
        if is_image(&extension) {
            match_values.push(embed_file(&rel_path, &full_canonical_path));
            list_values.push(rel_path);
        }
    }

    let handle_prefix = TokenStream2::new();

    quote! {
        impl #ident {
            pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, str>> {
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
