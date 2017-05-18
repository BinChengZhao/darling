use syn;

use {FromMetaItem, FromField, FromVariant, Result};

/// An efficient way of discarding data from an attribute.
///
/// All meta-items, fields, and variants will be successfully read into
/// the `Ignored` struct, with all properties discarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ignored;

impl FromMetaItem for Ignored {
    fn from_meta_item(_: &syn::MetaItem) -> Result<Self> {
        Ok(Self)
    }

    fn from_nested_meta_item(_: &syn::NestedMetaItem) -> Result<Self> {
        Ok(Self)
    }
}

impl FromDeriveInput for Ignored {
    fn from_derive_input(_: &syn::DeriveInput) -> Result<Self> {
        Ok(Self)
    }
}

impl FromField for Ignored {
    fn from_field(_: &syn::Field) -> Result<Self> {
        Ok(Self)
    }
}

impl FromVariant for Ignored {
    fn from_variant(_: &syn::Variant) -> Result<Self> {
        Ok(Self)
    }
}