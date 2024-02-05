use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Rotation(pub Quat);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Force(pub Vec3);


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(Update, (
            apply_force,
        ));
    }
}

fn apply_force (
    mut query: Query<(&mut Position, &mut Velocity, &mut Force, &Mass)>,
    time: Res<Time>,
) {
    for (mut position, mut velocity, mut force, mass) in query.iter_mut() {
        let acceleration = force.0 / mass.0;
        velocity.0 += acceleration * time.delta_seconds();
        if velocity.0.length() <= 5.0 {
            velocity.0 = Vec3::ZERO;
            continue;
        }
        position.0 += velocity.0 * time.delta_seconds();
        force.0 = Vec3::ZERO;
    }
}

#[derive(Bundle)]
pub struct Physics {
    pub position: Position,
    pub rotation: Rotation,
    pub mass: Mass,
    pub velocity: Velocity,
    pub force: Force,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            position: Position(Vec3::ZERO),
            rotation: Rotation(Quat::IDENTITY),
            mass: Mass(1.0),
            velocity: Velocity(Vec3::ZERO),
            force: Force(Vec3::ZERO),
        }
    }
}