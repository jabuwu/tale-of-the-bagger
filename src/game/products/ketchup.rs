use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SecondOrderController, SpineSync2},
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductKetchupSystem {
    Spawn,
    Spawned,
    RigUpdate,
}

pub struct ProductKetchupPlugin;

impl Plugin for ProductKetchupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_ketchup_spawn.label(ProductKetchupSystem::Spawn))
            .add_system(product_ketchup_spawned.label(ProductKetchupSystem::Spawned))
            .add_system(
                product_ketchup_rig_update
                    .label(ProductKetchupSystem::RigUpdate)
                    .after_spine_sync::<SpineSync2>(),
            );
    }
}

#[derive(Component)]
pub struct ProductKetchup;

#[derive(Component)]
pub struct ProductKetchupRig {
    d3: BoneHandle,
    d3_controller: SecondOrderController<Vec2>,
    center: BoneHandle,
}

fn product_ketchup_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Ketchup {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_ketchup.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductKetchup);
        }
    }
}

fn product_ketchup_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    ketchup_query: Query<(Entity, &Spine), With<ProductKetchup>>,
) {
    for event in spine_ready_event.iter() {
        if let Some((ketchup_entity, ketchup_spine)) = ketchup_query.get(event.entity).ok() {
            macro_rules! bone {
                ($name:literal) => {
                    ketchup_spine.skeleton.find_bone($name).unwrap().handle()
                };
            }
            let d3 = bone!("3d-front");
            let center = bone!("center");
            commands.entity(ketchup_entity).insert(ProductKetchupRig {
                d3,
                d3_controller: SecondOrderController::new(Vec2::ZERO, 1., 1., 1.),
                center,
            });
        }
    }
}

fn product_ketchup_rig_update(
    mut rig_query: Query<(&mut ProductKetchupRig, &mut Spine, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (mut rig, mut rig_spine, rig_transform) in rig_query.iter_mut() {
        let mut d3 = rig
            .d3_controller
            .update(rig_transform.translation().truncate(), time.delta_seconds());
        d3 = (d3 - rig_transform.translation().truncate()) * 0.013;
        let mut d3_bone = rig.d3.get_mut(&mut rig_spine.skeleton).unwrap();
        let original_position: Vec2 = d3_bone.data().position().into();
        d3_bone.set_position(original_position - d3);

        d3 = d3 * 0.01;
        let mut center_bone = rig.center.get_mut(&mut rig_spine.skeleton).unwrap();
        let original_scale: Vec2 = center_bone.data().scale().into();
        center_bone.set_scale(original_scale - d3.abs());
        center_bone.set_rotation(d3.x * -200.);
    }
}
