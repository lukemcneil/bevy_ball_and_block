use std::f32::consts::PI;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui::Slider};
use bevy_rapier3d::prelude::*;

use crate::car::{Tire, VehicleConfig};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            config_ui_system.run_if(input_toggle_active(true, KeyCode::Escape)),
        );
    }
}

fn config_ui_system(
    mut contexts: EguiContexts,
    mut vehicle_configs: Query<(Entity, &mut VehicleConfig, &Name, &Children)>,
    mut tires: Query<(&mut Tire, &Name)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (vehicle_entity, mut vehicle_config, vehicle_name, vehicle_children) in &mut vehicle_configs
    {
        bevy_inspector_egui::egui::Window::new(format!("{} Settings", vehicle_name)).show(
            contexts.ctx_mut(),
            |ui| {
                ui.add(Slider::new(&mut vehicle_config.max_speed, 5.0..=200.0).text("max speed"));
                ui.add(Slider::new(&mut vehicle_config.max_force, 50.0..=1000.0).text("max force"));
                ui.add(
                    Slider::new(&mut vehicle_config.spring_offset, 0.0..=10.0)
                        .text("suspension height"),
                );
                ui.add(
                    Slider::new(&mut vehicle_config.spring_power, 0.0..=500.0)
                        .text("suspension power"),
                );
                ui.add(Slider::new(&mut vehicle_config.shock, 0.0..=100.0).text("shock viscosity"));
                let height_slider =
                    ui.add(Slider::new(&mut vehicle_config.height, 0.1..=10.0).text("height"));
                let width_slider =
                    ui.add(Slider::new(&mut vehicle_config.width, 0.1..=10.0).text("width"));
                let length_slider =
                    ui.add(Slider::new(&mut vehicle_config.length, 0.1..=10.0).text("length"));
                if height_slider.changed() || width_slider.changed() || length_slider.changed() {
                    commands.entity(vehicle_entity).insert((
                        Collider::cuboid(
                            vehicle_config.length,
                            vehicle_config.height,
                            vehicle_config.width,
                        ),
                        meshes.add(Mesh::from(shape::Box {
                            min_x: -vehicle_config.length,
                            max_x: vehicle_config.length,
                            min_y: -vehicle_config.height,
                            max_y: vehicle_config.height,
                            min_z: -vehicle_config.width,
                            max_z: vehicle_config.width,
                        })),
                    ));
                };
                ui.add(
                    Slider::new(&mut vehicle_config.turn_radius, 0.0..=(PI / 4.0))
                        .text("turn radius"),
                );

                for child in vehicle_children {
                    let tire = tires.get_mut(*child);
                    if let Ok((mut tire, tire_name)) = tire {
                        ui.collapsing(tire_name.as_str(), |ui| {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut tire.connected_to_engine, "spins");
                                ui.checkbox(&mut tire.turns, "turns");
                                ui.add(Slider::new(&mut tire.grip, 0.0..=1.0).text("grip"));
                            });
                        });
                    }
                }
            },
        );
    }
}
