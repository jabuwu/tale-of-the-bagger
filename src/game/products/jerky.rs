use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SecondOrderController, SpineSync2},
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductJerkySystem {
    Spawn,
    Spawned,
    RigUpdate,
}

pub struct ProductJerkyPlugin;

impl Plugin for ProductJerkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_jerky_spawn.label(ProductJerkySystem::Spawn))
            .add_system(product_jerky_spawned.label(ProductJerkySystem::Spawned))
            .add_system(
                product_jerky_rig_update
                    .label(ProductJerkySystem::RigUpdate)
                    .after_spine_sync::<SpineSync2>(),
            );
    }
}

#[derive(Component)]
pub struct ProductJerky;

#[derive(Component)]
pub struct ProductJerkyRig {
    d3: BoneHandle,
    d3_controller: SecondOrderController<Vec2>,
    ears: Vec<BoneHandle>,
    ears_controller: SecondOrderController<Vec2>,
    center: BoneHandle,
}

fn product_jerky_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Jerky {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_jerky.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductJerky);
        }
    }
}

fn product_jerky_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    jerky_query: Query<(Entity, &Spine), With<ProductJerky>>,
) {
    for event in spine_ready_event.iter() {
        if let Some((jerky_entity, jerky_spine)) = jerky_query.get(event.entity).ok() {
            macro_rules! bone {
                ($name:literal) => {
                    jerky_spine.skeleton.find_bone($name).unwrap().handle()
                };
            }
            let d3 = bone!("3d-front");
            let ears = vec![
                bone!("ear-bottom-right"),
                bone!("ear-bottom-left"),
                bone!("ear-top-right"),
                bone!("ear-top-left"),
            ];
            let center = bone!("center");
            commands.entity(jerky_entity).insert(ProductJerkyRig {
                d3,
                d3_controller: SecondOrderController::new(Vec2::ZERO, 1., 0.5, 1.),
                ears,
                ears_controller: SecondOrderController::new(Vec2::ZERO, 1., 0.5, 2.),
                center,
            });
        }
    }
}

fn product_jerky_rig_update(
    mut rig_query: Query<(&mut ProductJerkyRig, &mut Spine, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (mut rig, mut rig_spine, rig_transform) in rig_query.iter_mut() {
        let mut d3 = rig
            .d3_controller
            .update(rig_transform.translation().truncate(), time.delta_seconds());
        d3 = (d3 - rig_transform.translation().truncate()) * 0.08;
        let mut d3_bone = rig.d3.get_mut(&mut rig_spine.skeleton).unwrap();
        let original_position: Vec2 = d3_bone.data().position().into();
        d3_bone.set_position(original_position - d3);

        d3 = (d3 * 0.005).abs();
        let mut center_bone = rig.center.get_mut(&mut rig_spine.skeleton).unwrap();
        let original_scale: Vec2 = center_bone.data().scale().into();
        center_bone.set_scale(original_scale - d3);

        let mut ear = rig
            .ears_controller
            .update(rig_transform.translation().truncate(), time.delta_seconds());
        ear = (ear - rig_transform.translation().truncate()) * 0.1;
        for (i, ear_handle) in rig.ears.iter().enumerate() {
            let mut ear_bone = ear_handle.get_mut(&mut rig_spine.skeleton).unwrap();
            let original_rotation = ear_bone.data().rotation();
            match i {
                0 => {
                    ear_bone.set_rotation(original_rotation + ear.y - ear.x);
                }
                1 => {
                    ear_bone.set_rotation(original_rotation - ear.y - ear.x);
                }
                2 => {
                    ear_bone.set_rotation(original_rotation + ear.y + ear.x);
                }
                3 => {
                    ear_bone.set_rotation(original_rotation - ear.y + ear.x);
                }
                _ => {}
            }
        }
    }
}
