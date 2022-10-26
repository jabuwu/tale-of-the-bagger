use proc_macro::TokenStream;

#[proc_macro_derive(AssetCollection, attributes(asset, spine_asset))]
pub fn derive_asset_collection(input: TokenStream) -> TokenStream {
    asset_collection::derive(input)
}

mod asset_collection;
