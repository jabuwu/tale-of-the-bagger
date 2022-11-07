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
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(ProductAk47Plugin);
        group.add(ProductAntifreezePlugin);
        group.add(ProductAvocadoPlugin);
        group.add(ProductBaconPlugin);
        group.add(ProductBatteriesPlugin);
        group.add(ProductBeansPlugin);
        group.add(ProductBeerPlugin);
        group.add(ProductBleachPlugin);
        group.add(ProductBoilingWaterPlugin);
        group.add(ProductBowlingBallPlugin);
        group.add(ProductBreadPlugin);
        group.add(ProductChickenPlugin);
        group.add(ProductCinderPlugin);
        group.add(ProductCocainePlugin);
        group.add(ProductCoffeePlugin);
        group.add(ProductEggsPlugin);
        group.add(ProductGoodStuffPlugin);
        group.add(ProductHeartPlugin);
        group.add(ProductIcePlugin);
        group.add(ProductIceCreamPlugin);
        group.add(ProductJerkyPlugin);
        group.add(ProductKatanaPlugin);
        group.add(ProductKetchupPlugin);
        group.add(ProductMilkPlugin);
        group.add(ProductPlatePlugin);
        group.add(ProductRocketFuelPlugin);
        group.add(ProductSkullPlugin);
        group.add(ProductSoupPlugin);
        group.add(ProductTacoPlugin);
        group.add(ProductTorchPlugin);
        group.add(ProductWatermelonPlugin);
        group.add(ProductXxxPlugin);
    }
}
