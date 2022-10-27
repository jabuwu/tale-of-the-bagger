use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_spine::{prelude::*, SpineSynchronizerSystem};
use lerp::Lerp;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Transform2System {
    TransformPropagate,
}

pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(spine_attach_transform2)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_transform2
                    .label(Transform2System::TransformPropagate)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_transform2_depth.after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Transform2 {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub pixel_perfect: bool,
}

impl Default for Transform2 {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            pixel_perfect: true,
        }
    }
}

impl Transform2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn with_rotation(self, rotation: f32) -> Self {
        Self { rotation, ..self }
    }

    pub fn with_scale(self, scale: Vec2) -> Self {
        Self { scale, ..self }
    }

    pub fn without_pixel_perfect(self) -> Self {
        Self {
            pixel_perfect: false,
            ..self
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum DepthLayer {
    Inherit(f32),
    Background(f32),
    Foreground(f32),
    Camera,
}

impl Default for DepthLayer {
    fn default() -> Self {
        Self::Inherit(0.)
    }
}

impl DepthLayer {
    pub fn depth_f32(&self) -> f32 {
        match *self {
            DepthLayer::Inherit(depth) => 0.0_f32.lerp(0.01, depth),
            DepthLayer::Background(depth) => 0.1_f32.lerp(0.19, depth),
            DepthLayer::Foreground(depth) => 0.2_f32.lerp(0.29, depth),
            DepthLayer::Camera => 1.0,
        }
    }
}

fn spine_attach_transform2(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
) {
    for event in spine_ready_event.iter() {
        for (_, bone_entity) in event.bones.iter() {
            commands.entity(*bone_entity).insert(Transform2::default());
        }
    }
}

fn update_transform2(mut query: Query<(Option<&Transform2>, Option<&DepthLayer>, &mut Transform)>) {
    for (transform2, depth_layer, mut transform) in query.iter_mut() {
        if let Some(transform2) = transform2 {
            transform.translation.x = transform2.translation.x;
            transform.translation.y = transform2.translation.y;
            transform.scale = Vec3::new(transform2.scale.x, transform2.scale.y, 1.0);
            transform.rotation = Quat::from_rotation_z(transform2.rotation);
        }
        if let Some(depth_layer) = depth_layer {
            transform.translation.z = depth_layer.depth_f32();
        }
    }
}

fn update_transform2_depth(
    mut query: Query<(
        Option<&Transform2>,
        Option<&DepthLayer>,
        &mut GlobalTransform,
    )>,
) {
    for (transform2, depth_layer, mut transform) in query.iter_mut() {
        if let Some(transform2) = transform2 {
            if transform2.pixel_perfect {
                transform.translation_mut().x = transform.translation_mut().x.round();
                transform.translation_mut().y = transform.translation_mut().y.round();
            }
        }
        if let Some(depth_layer) = depth_layer {
            if !matches!(depth_layer, DepthLayer::Inherit(..)) {
                transform.translation_mut().z = depth_layer.depth_f32();
            }
        }
    }
}

pub struct SpineSynchronizer2Plugin<T: Component> {
    _marker: PhantomData<T>,
}

impl<T: Component> Default for SpineSynchronizer2Plugin<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T: Component> Plugin for SpineSynchronizer2Plugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(
            spine_sync_entities_2::<T>
                .label(SpineSynchronizerSystem::<T>::SyncEntities)
                .after(SpineSystem::Update),
        )
        .add_system(
            spine_sync_bones_2::<T>
                .label(SpineSynchronizerSystem::<T>::SyncBones)
                .after(SpineSynchronizerSystem::<T>::SyncEntities),
        )
        .add_system(
            spine_sync_entities_applied_2::<T>
                .label(SpineSynchronizerSystem::<T>::SyncEntitiesApplied)
                .after(SpineSynchronizerSystem::<T>::SyncBones)
                .before(SpineSystem::Render),
        );
    }
}

pub fn spine_sync_entities_2<S: Component>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    spine_query: Query<&Spine, With<S>>,
) {
    for (mut bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(spine) = spine_query.get(bone.spine_entity) {
            if let Some(bone) = bone.handle.get(&spine.skeleton) {
                bone_transform.translation.x = bone.x();
                bone_transform.translation.y = bone.y();
                bone_transform.rotation = bone.rotation().to_radians();
                bone_transform.scale.x = bone.scale_x();
                bone_transform.scale.y = bone.scale_y();
            }
        }
    }
}

pub fn spine_sync_bones_2<S: Component>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    mut spine_query: Query<&mut Spine, With<S>>,
) {
    for (bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(mut spine) = spine_query.get_mut(bone.spine_entity) {
            if let Some(mut bone) = bone.handle.get_mut(&mut spine.skeleton) {
                bone.set_x(bone_transform.translation.x);
                bone.set_y(bone_transform.translation.y);
                bone.set_rotation(bone_transform.rotation.to_degrees());
                bone.set_scale_x(bone_transform.scale.x);
                bone.set_scale_y(bone_transform.scale.y);
            }
        }
    }
    for mut spine in spine_query.iter_mut() {
        spine.0.skeleton.update_world_transform();
    }
}

pub fn spine_sync_entities_applied_2<S: Component>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    spine_query: Query<&Spine, With<S>>,
) {
    for (mut bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(spine) = spine_query.get(bone.spine_entity) {
            if let Some(bone) = bone.handle.get(&spine.skeleton) {
                bone_transform.translation.x = bone.applied_x();
                bone_transform.translation.y = bone.applied_y();
                bone_transform.rotation = bone.applied_rotation().to_radians();
                bone_transform.scale.x = bone.applied_scale_x();
                bone_transform.scale.y = bone.applied_scale_y();
            }
        }
    }
}

#[derive(Component)]
pub struct SpineSync2;

pub type SpineSync2System = SpineSynchronizerSystem<SpineSync2>;
pub type SpineSync2Plugin = SpineSynchronizer2Plugin<SpineSync2>;
