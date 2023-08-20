use bevy::prelude::*;

pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_scale);
    }
}

#[derive(Component)]
pub struct AnimScale {
    scale: f32,
    direction: f32,
}

impl Default for AnimScale {
    fn default() -> Self {
        Self {
            scale: 1.0,
            direction: 1.0,
        }
    }
}

fn animate_scale(time: Res<Time>, mut query: Query<(&mut Transform, &mut AnimScale)>) {
    query.for_each_mut(|(mut transform, mut anim)| {
        anim.scale += anim.direction * time.delta_seconds();

        if anim.direction < 0.0 {
            if anim.scale < 0.8 {
                anim.scale = 0.8;
                anim.direction = 1.0;
            }
        } else {
            if anim.scale > 1.2 {
                anim.scale = 1.2;
                anim.direction = -1.0;
            }
        }

        transform.scale = Vec3::new(anim.scale, anim.scale, 1.0);
    });
}
