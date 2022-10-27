use bevy::prelude::*;

pub enum CollisionShape {
    None,
    Point,
    Aabb { half_extents: Vec2 },
}

impl CollisionShape {
    pub fn colliding(
        &self,
        translation: Vec2,
        other: &CollisionShape,
        other_translation: Vec2,
    ) -> bool {
        match *self {
            CollisionShape::None => false,
            CollisionShape::Point => {
                let point = Point::new(translation);
                match *other {
                    CollisionShape::None => false,
                    CollisionShape::Point => point.colliding_point(&Point::new(other_translation)),
                    CollisionShape::Aabb {
                        half_extents: other_half_extents,
                    } => point.colliding_aabb(&Aabb::new(other_translation, other_half_extents)),
                }
            }
            CollisionShape::Aabb { half_extents } => {
                let aabb = Aabb::new(translation, half_extents);
                match *other {
                    CollisionShape::None => false,
                    CollisionShape::Point => aabb.colliding_point(&Point::new(other_translation)),
                    CollisionShape::Aabb {
                        half_extents: other_half_extents,
                    } => aabb.colliding_aabb(&Aabb::new(other_translation, other_half_extents)),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub translation: Vec2,
}

impl Point {
    pub fn new(translation: Vec2) -> Self {
        Self { translation }
    }

    pub fn colliding_point(&self, other: &Point) -> bool {
        self.translation == other.translation
    }

    pub fn colliding_aabb(&self, aabb: &Aabb) -> bool {
        let difference = self.translation - aabb.translation;
        difference.x.abs() <= aabb.half_extents.x && difference.y.abs() <= aabb.half_extents.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Aabb {
    pub translation: Vec2,
    pub half_extents: Vec2,
}

impl Aabb {
    pub fn new(translation: Vec2, half_extents: Vec2) -> Self {
        Self {
            translation,
            half_extents,
        }
    }

    pub fn new_from_vertices(vertices: &[Vec2]) -> Option<Self> {
        if vertices.len() > 0 {
            let mut bottom_left = vertices[0];
            let mut top_right = vertices[0];
            for vertex in vertices.iter().skip(1) {
                bottom_left = bottom_left.min(*vertex);
                top_right = top_right.max(*vertex);
            }
            let translation = (top_right + bottom_left) / 2.;
            let half_extents = (top_right - bottom_left) / 2.;
            Some(Self {
                translation,
                half_extents,
            })
        } else {
            None
        }
    }

    pub fn colliding_point(&self, point: &Point) -> bool {
        point.colliding_aabb(self)
    }

    pub fn colliding_aabb(&self, _other: &Aabb) -> bool {
        todo!()
    }
}
