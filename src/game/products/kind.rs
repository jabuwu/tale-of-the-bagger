use bevy::prelude::*;
use bitmask_enum::bitmask;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::AssetLibrary;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum ProductKind {
    Ak47,
    Antifreeze,
    Avocado,
    Bacon,
    Batteries,
    Beans,
    Beer,
    Bleach,
    BoilingWater,
    BowlingBall,
    Bread,
    Chicken,
    Cinder,
    Cocaine,
    Coffee,
    Eggs,
    GoodStuff,
    Heart,
    Ice,
    IceCream,
    Jerky,
    Katana,
    Ketchup,
    Milk,
    Plate,
    RocketFuel,
    Skull,
    Soup,
    Taco,
    Torch,
    Watermelon,
    Xxx,
}

impl ProductKind {
    pub fn attributes(&self) -> ProductAttributes {
        match *self {
            ProductKind::Ak47 => ProductAttributes::Heavy | ProductAttributes::Illicit,
            ProductKind::Antifreeze => ProductAttributes::Cold | ProductAttributes::Heavy,
            ProductKind::Avocado => ProductAttributes::Fresh,
            ProductKind::Bacon => ProductAttributes::Cold | ProductAttributes::Meat,
            ProductKind::Batteries => ProductAttributes::Fragile | ProductAttributes::Toxic,
            ProductKind::Beans => ProductAttributes::none(),
            ProductKind::Beer => ProductAttributes::Cold | ProductAttributes::Fragile,
            ProductKind::Bleach => ProductAttributes::Toxic,
            ProductKind::BoilingWater => ProductAttributes::Hot | ProductAttributes::Heavy,
            ProductKind::BowlingBall => ProductAttributes::Heavy,
            ProductKind::Bread => ProductAttributes::Fresh | ProductAttributes::Fragile,
            ProductKind::Chicken => ProductAttributes::Hot | ProductAttributes::Meat,
            ProductKind::Cinder => ProductAttributes::Heavy,
            ProductKind::Cocaine => ProductAttributes::Illicit,
            ProductKind::Coffee => ProductAttributes::Hot | ProductAttributes::Fragile,
            ProductKind::Eggs => ProductAttributes::Fresh | ProductAttributes::Fragile,
            ProductKind::GoodStuff => ProductAttributes::Illicit,
            ProductKind::Heart => ProductAttributes::Fresh | ProductAttributes::Illicit,
            ProductKind::Ice => ProductAttributes::Cold | ProductAttributes::Heavy,
            ProductKind::IceCream => ProductAttributes::Cold,
            ProductKind::Jerky => ProductAttributes::Meat,
            ProductKind::Katana => ProductAttributes::Illicit,
            ProductKind::Ketchup => ProductAttributes::none(),
            ProductKind::Milk => ProductAttributes::Cold | ProductAttributes::Fresh,
            ProductKind::Plate => ProductAttributes::Fragile,
            ProductKind::RocketFuel => ProductAttributes::Heavy | ProductAttributes::Toxic,
            ProductKind::Skull => ProductAttributes::Fragile | ProductAttributes::Illicit,
            ProductKind::Soup => ProductAttributes::Hot,
            ProductKind::Taco => ProductAttributes::Hot | ProductAttributes::Fresh,
            ProductKind::Torch => ProductAttributes::Hot | ProductAttributes::Toxic,
            ProductKind::Watermelon => ProductAttributes::Fresh | ProductAttributes::Heavy,
            ProductKind::Xxx => ProductAttributes::Hot | ProductAttributes::Illicit,
        }
    }

    pub fn compatible(&self, other: ProductKind) -> bool {
        self.attributes().compatible(other.attributes())
    }

    pub fn weight(&self) -> ProductWeight {
        self.attributes().weight()
    }

    pub fn valid_stack(products: &[ProductKind]) -> bool {
        let mut max_weight = ProductWeight::Heavy;
        for (i, product) in products.iter().enumerate() {
            let weight = product.weight();
            if weight > max_weight {
                return false;
            } else if weight < max_weight {
                max_weight = weight;
            }
            for (_, other_product) in products.iter().enumerate().take_while(|(j, _)| *j < i) {
                if !product.compatible(*other_product) {
                    return false;
                }
            }
        }
        true
    }
}

macro_rules! product_attributes {
    ($($body:tt,)*) => {
        #[bitmask(u16)]
        pub enum ProductAttributes {
            $($body,)*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
        pub enum ProductAttribute {
            $($body,)*
        }

        impl ProductAttribute {
            pub fn bitwise(&self) -> ProductAttributes {
                match *self {
                    $(
                        ProductAttribute::$body => ProductAttributes::$body,
                    )*
                }
            }
        }
    };
}

product_attributes! {
    Cold,
    Hot,
    Fresh,
    Meat,
    Fragile,
    Heavy,
    Toxic,
    Illicit,
}

impl ProductAttributes {
    pub fn enums(&self) -> impl Iterator<Item = ProductAttribute> {
        let copy = *self;
        ProductAttribute::iter().filter(move |product| copy.contains(product.bitwise()))
    }

    pub fn compatible(&self, other: ProductAttributes) -> bool {
        for attribute in self.enums() {
            for other_attribute in other.enums() {
                if !attribute.compatible(other_attribute) {
                    return false;
                }
            }
        }
        true
    }

    pub fn weight(&self) -> ProductWeight {
        ProductWeight::determine(*self)
    }
}

impl ProductAttribute {
    pub fn icon(&self, asset_library: &AssetLibrary) -> Handle<Image> {
        match *self {
            Self::Cold => asset_library.textures.icon_cold.clone(),
            Self::Hot => asset_library.textures.icon_hot.clone(),
            Self::Fresh => asset_library.textures.icon_fresh.clone(),
            Self::Meat => asset_library.textures.icon_meat.clone(),
            Self::Fragile => asset_library.textures.icon_fragile.clone(),
            Self::Heavy => asset_library.textures.icon_heavy.clone(),
            Self::Toxic => asset_library.textures.icon_toxic.clone(),
            Self::Illicit => asset_library.textures.icon_illicit.clone(),
        }
    }

    pub fn compatible(&self, other: ProductAttribute) -> bool {
        if self.bitwise() <= other.bitwise() {
            if *self == ProductAttribute::Cold && other == ProductAttribute::Hot {
                return false;
            }
            if *self == ProductAttribute::Fresh && other == ProductAttribute::Meat {
                return false;
            }
            if *self == ProductAttribute::Fresh && other == ProductAttribute::Toxic {
                return false;
            }
            if *self == ProductAttribute::Meat && other == ProductAttribute::Toxic {
                return false;
            }
            if *self == ProductAttribute::Illicit && other == ProductAttribute::Illicit {
                return false;
            }
            true
        } else {
            other.compatible(*self)
        }
    }

    pub fn weight(&self) -> Option<ProductWeight> {
        match *self {
            ProductAttribute::Heavy => Some(ProductWeight::Heavy),
            ProductAttribute::Fragile => Some(ProductWeight::Light),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductWeight {
    Light,
    Normal,
    Heavy,
}

impl ProductWeight {
    pub fn determine(attributes: ProductAttributes) -> Self {
        let mut weight = None;
        for attribute in attributes.enums() {
            if let Some(attribute_weight) = attribute.weight() {
                if let Some(current_weight) = weight {
                    if attribute_weight > current_weight {
                        weight = Some(attribute_weight);
                    }
                } else {
                    weight = Some(attribute_weight);
                }
            }
        }
        weight.unwrap_or(ProductWeight::Normal)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{ProductAttribute, ProductAttributes, ProductKind, ProductWeight};

    #[test]
    fn compatible() {
        assert!(ProductAttribute::Cold.compatible(ProductAttribute::Cold));
        assert!(ProductAttribute::Hot.compatible(ProductAttribute::Hot));
        assert!(ProductAttribute::Hot.compatible(ProductAttribute::Toxic));
        assert!(ProductAttribute::Meat.compatible(ProductAttribute::Heavy));
        assert!(ProductAttribute::Meat.compatible(ProductAttribute::Fragile));

        assert!(ProductKind::Jerky.compatible(ProductKind::Beans));
        assert!(ProductKind::Soup.compatible(ProductKind::Chicken));
    }

    #[test]
    fn incompatible() {
        assert!(!ProductAttribute::Cold.compatible(ProductAttribute::Hot));
        assert!(!ProductAttribute::Hot.compatible(ProductAttribute::Cold));

        assert!(!ProductAttribute::Fresh.compatible(ProductAttribute::Meat));
        assert!(!ProductAttribute::Meat.compatible(ProductAttribute::Fresh));

        assert!(!ProductAttribute::Fresh.compatible(ProductAttribute::Toxic));
        assert!(!ProductAttribute::Toxic.compatible(ProductAttribute::Fresh));

        assert!(!ProductAttribute::Meat.compatible(ProductAttribute::Toxic));
        assert!(!ProductAttribute::Toxic.compatible(ProductAttribute::Meat));

        assert!(!ProductAttribute::Illicit.compatible(ProductAttribute::Illicit));

        assert!(!ProductKind::Soup.compatible(ProductKind::IceCream));
        assert!(!ProductKind::Ak47.compatible(ProductKind::Katana));
    }

    #[test]
    fn weights() {
        let heavy_item = ProductWeight::determine(ProductAttributes::Heavy);
        let normal_item = ProductWeight::determine(ProductAttributes::none());
        let light_item = ProductWeight::determine(ProductAttributes::Fragile);

        assert_eq!(heavy_item, ProductWeight::Heavy);
        assert_eq!(normal_item, ProductWeight::Normal);
        assert_eq!(light_item, ProductWeight::Light);

        assert!(heavy_item > normal_item);
        assert!(normal_item > light_item);

        assert_eq!(ProductKind::Cinder.weight(), ProductWeight::Heavy);
        assert_eq!(ProductKind::Jerky.weight(), ProductWeight::Normal);
        assert_eq!(ProductKind::Bread.weight(), ProductWeight::Light);
    }

    #[test]
    fn valid_stack() {
        assert!(ProductKind::valid_stack(&[]));
        assert!(ProductKind::valid_stack(&[
            ProductKind::Cinder,
            ProductKind::Jerky,
            ProductKind::Plate,
        ]));
        assert!(ProductKind::valid_stack(&[
            ProductKind::Beans,
            ProductKind::Eggs,
            ProductKind::Plate,
        ]));
    }

    #[test]
    fn invalid_stack() {
        assert!(!ProductKind::valid_stack(&[
            ProductKind::Jerky,
            ProductKind::Jerky,
            ProductKind::Cinder,
        ]));
        assert!(!ProductKind::valid_stack(&[
            ProductKind::Beans,
            ProductKind::Plate,
            ProductKind::Jerky,
        ]));
        assert!(!ProductKind::valid_stack(&[
            ProductKind::Soup,
            ProductKind::IceCream,
        ]));
        assert!(!ProductKind::valid_stack(&[
            ProductKind::Watermelon,
            ProductKind::Batteries,
        ]));
        assert!(!ProductKind::valid_stack(&[
            ProductKind::Ak47,
            ProductKind::Katana,
        ]));
    }
}
