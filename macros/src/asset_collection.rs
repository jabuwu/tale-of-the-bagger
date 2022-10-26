use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Data, DeriveInput, Error, LitStr, Result,
};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut asset_collection_fields = vec![];
    match input.data {
        Data::Struct(data_struct) => {
            for field in data_struct.fields.iter() {
                let mut attributes = vec![];
                for attr in field.attrs.iter() {
                    if let Some(path_ident) = attr.path.get_ident() {
                        match path_ident.to_string().as_str() {
                            "asset" => {
                                let tokens: TokenStream = attr.tokens.clone().into();
                                attributes.push((
                                    path_ident,
                                    Attribute::AssetAttribute(parse_macro_input!(
                                        tokens as AssetAttribute
                                    )),
                                ));
                            }
                            "spine_asset" => {
                                let tokens: TokenStream = attr.tokens.clone().into();
                                attributes.push((
                                    path_ident,
                                    Attribute::SpineAssetAttribute(parse_macro_input!(
                                        tokens as SpineAssetAttribute
                                    )),
                                ));
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                }
                let mut asset_collection_field =
                    AssetCollectionField::new(field.ident.clone().unwrap());
                for (attribute_ident, attribute) in attributes.iter() {
                    match attribute {
                        Attribute::AssetAttribute(asset_attribute) => {
                            if !matches!(asset_collection_field.path, AssetCollectionPath::None) {
                                return TokenStream::from(
                                    Error::new(
                                        attribute_ident.span(),
                                        "Multiple path attributes found.",
                                    )
                                    .to_compile_error(),
                                );
                            }
                            asset_collection_field.path =
                                AssetCollectionPath::Path(asset_attribute.literal.value());
                        }
                        Attribute::SpineAssetAttribute(asset_attribute) => {
                            if !matches!(asset_collection_field.path, AssetCollectionPath::None) {
                                return TokenStream::from(
                                    Error::new(
                                        attribute_ident.span(),
                                        "Multiple path attributes found.",
                                    )
                                    .to_compile_error(),
                                );
                            }
                            asset_collection_field.path =
                                AssetCollectionPath::Spine(asset_attribute.literal.value());
                        }
                    }
                }
                asset_collection_fields.push(asset_collection_field);
            }
        }
        Data::Enum(data_enum) => {
            return TokenStream::from(
                Error::new(
                    data_enum.enum_token.span,
                    "AssetCollection is not compatible with enums.",
                )
                .to_compile_error(),
            );
        }
        Data::Union(data_union) => {
            return TokenStream::from(
                Error::new(
                    data_union.union_token.span,
                    "AssetCollection is not compatible with unions.",
                )
                .to_compile_error(),
            );
        }
    }

    let mut load_assets = vec![];
    for asset_collection_field in asset_collection_fields.iter() {
        match &asset_collection_field.path {
            AssetCollectionPath::Path(path) => {
                let field_ident = &asset_collection_field.ident;
                load_assets.push(quote! {
                    self.#field_ident = asset_server.load(#path);
                });
            }
            AssetCollectionPath::Spine(spine_path) => {
                let field_ident = &asset_collection_field.ident;
                load_assets.push(quote! {
                    {
                        self.#field_ident = skeletons.add(SkeletonData::new_from_json(
                            asset_server.load(concat!(#spine_path, "/skeleton.json")),
                            asset_server.load(concat!(#spine_path, "/skeleton.atlas")),
                        ));
                    }
                });
            }
            AssetCollectionPath::None => {}
        }
    }

    TokenStream::from(quote! {
        impl crate::common::AssetCollection for #name {
            fn load_assets(&mut self, skeletons: &mut Assets<SkeletonData>, asset_server: &AssetServer) {
                #(#load_assets)*
            }
        }
    })
}

enum Attribute {
    AssetAttribute(AssetAttribute),
    SpineAssetAttribute(SpineAssetAttribute),
}

struct AssetAttribute {
    _paren_token: token::Paren,
    literal: LitStr,
}

impl Parse for AssetAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(AssetAttribute {
            _paren_token: parenthesized!(content in input),
            literal: content.parse()?,
        })
    }
}

struct SpineAssetAttribute {
    _paren_token: token::Paren,
    literal: LitStr,
}

impl Parse for SpineAssetAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(SpineAssetAttribute {
            _paren_token: parenthesized!(content in input),
            literal: content.parse()?,
        })
    }
}

struct AssetCollectionField {
    ident: Ident,
    path: AssetCollectionPath,
}

impl AssetCollectionField {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            path: AssetCollectionPath::None,
        }
    }
}

enum AssetCollectionPath {
    None,
    Path(String),
    Spine(String),
}
