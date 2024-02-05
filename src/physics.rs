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
    mut query: Query<(&mut Position, &mut Velocity, &mut Force, &Mass, &mut Rotation)>,
    time: Res<Time>,
) {
    for (mut position, mut velocity, mut force, mass, mut direction) in query.iter_mut() {
        let acceleration = force.0 / mass.0;
        velocity.0 += acceleration * time.delta_seconds();

        if velocity.0.length() <= 5.0 {
            velocity.0 = Vec3::ZERO;
            continue;
        }
        let next_dir = Quat::from_rotation_y(velocity.0.x.atan2(velocity.0.z));

        direction.0 = if direction.0.angle_between(next_dir) < 1.57 {
            direction.0.slerp(next_dir, 0.1)
        } else {
            direction.0.slerp(next_dir.mul_quat(Quat::from_rotation_y(3.14)), 0.1)
        };
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