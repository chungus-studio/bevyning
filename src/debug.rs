use avian2d::prelude::PhysicsGizmos;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct DebugPlugIn;

impl Plugin for DebugPlugIn {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_physics).run_if(input_just_pressed(KeyCode::F1)),
        );
    }
}

fn toggle_physics(mut gcs: ResMut<GizmoConfigStore>) {
    let gc = gcs.config_mut::<PhysicsGizmos>().0;
    gc.enabled = !gc.enabled;
}
