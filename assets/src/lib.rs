#![recursion_limit = "1024"]
#![forbid(unsafe_code)]

extern crate proc_macro;
extern crate quote;

// library
use proc_macro::TokenStream;
use syn::DeriveInput;

// crate
mod common;
mod extension;
mod font;
mod image;

#[proc_macro_derive(AssetCommon, attributes(folder))]
pub fn derive_common_assets(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = common::impl_embed_assets(&ast);
    gen.into()
}

#[proc_macro_derive(AssetFont, attributes(folder))]
pub fn derive_font_assets(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = font::impl_embed_assets(&ast);
    gen.into()
}

#[proc_macro_derive(AssetImage, attributes(folder))]
pub fn derive_image_assets(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = image::impl_embed_assets(&ast);
    gen.into()
}
