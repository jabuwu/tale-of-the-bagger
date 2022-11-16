use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    ProductAk47Plugin, ProductAntifreezePlugin, ProductAvocadoPlugin, ProductBaconPlugin,
    ProductBatteriesPlugin, ProductBeansPlugin, ProductBeerPlugin, ProductBleachPlugin,
    ProductBoilingWaterPlugin, ProductBowlingBallPlugin, ProductBreadPlugin, ProductChickenPlugin,
    ProductCinderPlugin, ProductCocainePlugin, ProductCoffeePlugin, ProductEggsPlugin,
    ProductGoodStuffPlugin, ProductHeartPlugin, ProductIceCreamPlugin, ProductIcePlugin,
    ProductJerkyPlugin, ProductKatanaPlugin, ProductKetchupPlugin, ProductMilkPlugin,
    ProductPlatePlugin, ProductRocketFuelPlugin, ProductSkullPlugin, ProductSoupPlugin,
    ProductTacoPlugin, ProductTorchPlugin, ProductWatermelonPlugin, ProductXxxPlugin,
};

pub struct ProductPlugins;

impl PluginGroup for ProductPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(ProductAk47Plugin);
        group = group.add(ProductAntifreezePlugin);
        group = group.add(ProductAvocadoPlugin);
        group = group.add(ProductBaconPlugin);
        group = group.add(ProductBatteriesPlugin);
        group = group.add(ProductBeansPlugin);
        group = group.add(ProductBeerPlugin);
        group = group.add(ProductBleachPlugin);
        group = group.add(ProductBoilingWaterPlugin);
        group = group.add(ProductBowlingBallPlugin);
        group = group.add(ProductBreadPlugin);
        group = group.add(ProductChickenPlugin);
        group = group.add(ProductCinderPlugin);
        group = group.add(ProductCocainePlugin);
        group = group.add(ProductCoffeePlugin);
        group = group.add(ProductEggsPlugin);
        group = group.add(ProductGoodStuffPlugin);
        group = group.add(ProductHeartPlugin);
        group = group.add(ProductIcePlugin);
        group = group.add(ProductIceCreamPlugin);
        group = group.add(ProductJerkyPlugin);
        group = group.add(ProductKatanaPlugin);
        group = group.add(ProductKetchupPlugin);
        group = group.add(ProductMilkPlugin);
        group = group.add(ProductPlatePlugin);
        group = group.add(ProductRocketFuelPlugin);
        group = group.add(ProductSkullPlugin);
        group = group.add(ProductSoupPlugin);
        group = group.add(ProductTacoPlugin);
        group = group.add(ProductTorchPlugin);
        group = group.add(ProductWatermelonPlugin);
        group = group.add(ProductXxxPlugin);
        group
    }
}
