use std::f32::consts::PI;

use bevy::{
    input::{common_conditions::input_toggle_active, gamepad::GamepadEvent},
    prelude::*,
    utils::HashMap,
};
use bevy_rapier3d::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddForce>()
            .add_systems(
                Update,
                (
                    turn_tires,
                    calculate_tire_distances_to_ground,
                    calculate_tire_acceleration_and_braking_forces
                        .after(calculate_tire_distances_to_ground),
                    calculate_tire_turning_forces.after(calculate_tire_distances_to_ground),
                    calculate_tire_suspension_forces.after(calculate_tire_distances_to_ground),
                    calculate_tire_friction.after(calculate_tire_distances_to_ground),
                    move_tires_to_ground.after(calculate_tire_distances_to_ground),
                    reset_car,
                    (
                        sum_all_forces,
                        draw_tire_force_gizmos.run_if(input_toggle_active(true, KeyCode::L)),
                    )
                        .after(calculate_tire_acceleration_and_braking_forces)
                        .after(calculate_tire_turning_forces)
                        .after(calculate_tire_suspension_forces)
                        .after(calculate_tire_friction),
                ),
            )
            .register_type::<Car>()
            .register_type::<Drivable>()
            .register_type::<Tire>()
            .register_type::<CameraPosition>()
            .register_type::<VehicleConfig>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Car;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Drivable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tire {
    pub connected_to_engine: bool,
    pub turns: bool,
    pub grip: f32,
    pub distance_to_ground: Option<f32>,
}

impl Default for Tire {
    fn default() -> Self {
        Self {
            connected_to_engine: false,
            turns: false,
            grip: 0.7,
            distance_to_ground: None,
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraPosition;

#[derive(Component, Default, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct VehicleConfig {
    pub height: f32,
    pub width: f32,
    pub length: f32,
    pub wheelbase: f32,
    pub wheel_offset: f32,
    pub spring_offset: f32,
    pub spring_power: f32,
    pub shock: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub turn_radius: f32,
    pub anchor_point: Vec3,
    pub scale: f32,
    pub starting_tire_grip: f32,
}

#[derive(Bundle, Default)]
struct DrivableBundle {
    rigidbody: RigidBody,
    collider: Collider,
    drivable: Drivable,
    read_mass_properties: ReadMassProperties,
    velocity: Velocity,
    external_force: ExternalForce,
    name: Name,
    friction: Friction,
    vehicle_config: VehicleConfig,
}

#[derive(Bundle, Default)]
struct TireBundle {
    transform_bundle: TransformBundle,
    tire: Tire,
    name: Name,
    visibility: VisibilityBundle,
}

pub fn spawn_vehicle(
    commands: &mut Commands,
    vehicle_config: VehicleConfig,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    texture_handle: Handle<Image>,
    tire_material: Handle<StandardMaterial>,
    name: &str,
    is_car: bool,
) -> Entity {
    commands
        .spawn((
            DrivableBundle {
                collider: Collider::cuboid(
                    vehicle_config.length,
                    vehicle_config.height,
                    vehicle_config.width,
                ),
                name: Name::from(name),
                friction: Friction::coefficient(0.5),
                vehicle_config,
                ..default()
            },
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -vehicle_config.length,
                    max_x: vehicle_config.length,
                    min_y: -vehicle_config.height,
                    max_y: vehicle_config.height,
                    min_z: -vehicle_config.width,
                    max_z: vehicle_config.width,
                })),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture_handle.clone()),
                    unlit: false,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    if is_car {
                        vehicle_config.length + vehicle_config.anchor_point.x
                    } else {
                        -(vehicle_config.length + vehicle_config.anchor_point.x)
                    },
                    vehicle_config.height,
                    0.,
                ),
                global_transform: default(),
                ..default()
            },
        ))
        .with_children(|child_builder| {
            let tire_mesh = meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.5,
                height: 0.5,
                ..default()
            }));
            let get_tire_material_mesh_bundle = || MaterialMeshBundle {
                mesh: tire_mesh.clone(),
                material: tire_material.clone(),
                transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0)),
                ..default()
            };
            child_builder
                .spawn(TireBundle {
                    transform_bundle: TransformBundle::from(Transform::from_xyz(
                        vehicle_config.wheelbase + vehicle_config.wheel_offset,
                        -vehicle_config.height / 6.0,
                        vehicle_config.width * 1.1,
                    )),
                    tire: if is_car {
                        Tire {
                            connected_to_engine: true,
                            turns: true,
                            grip: vehicle_config.starting_tire_grip,
                            ..default()
                        }
                    } else {
                        Tire {
                            grip: vehicle_config.starting_tire_grip,
                            ..default()
                        }
                    },
                    name: Name::from("Tire Front Right"),
                    ..default()
                })
                .with_children(|tire_child_builder| {
                    tire_child_builder.spawn(get_tire_material_mesh_bundle());
                });
            child_builder
                .spawn(TireBundle {
                    transform_bundle: TransformBundle::from(Transform::from_xyz(
                        vehicle_config.wheelbase + vehicle_config.wheel_offset,
                        -vehicle_config.height / 6.0,
                        -vehicle_config.width * 1.1,
                    )),
                    tire: if is_car {
                        Tire {
                            connected_to_engine: true,
                            turns: true,
                            grip: vehicle_config.starting_tire_grip,
                            ..default()
                        }
                    } else {
                        Tire {
                            grip: vehicle_config.starting_tire_grip,
                            ..default()
                        }
                    },
                    name: Name::from("Tire Front Left"),
                    ..default()
                })
                .with_children(|tire_child_builder| {
                    tire_child_builder.spawn(get_tire_material_mesh_bundle());
                });
            child_builder
                .spawn(TireBundle {
                    transform_bundle: TransformBundle::from(Transform::from_xyz(
                        -vehicle_config.wheelbase + vehicle_config.wheel_offset,
                        -vehicle_config.height / 6.0,
                        vehicle_config.width * 1.1,
                    )),
                    name: Name::from("Tire Back Right"),
                    tire: Tire {
                        grip: vehicle_config.starting_tire_grip,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tire_child_builder| {
                    tire_child_builder.spawn(get_tire_material_mesh_bundle());
                });

            child_builder
                .spawn(TireBundle {
                    transform_bundle: TransformBundle::from(Transform::from_xyz(
                        -vehicle_config.wheelbase + vehicle_config.wheel_offset,
                        -vehicle_config.height / 6.0,
                        -vehicle_config.width * 1.1,
                    )),
                    name: Name::from("Tire Back Left"),
                    tire: Tire {
                        grip: vehicle_config.starting_tire_grip,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tire_child_builder| {
                    tire_child_builder.spawn(get_tire_material_mesh_bundle());
                });

            if is_car {
                child_builder.spawn((
                    TransformBundle::from(
                        Transform::from_xyz(-40.0, 40.0, 0.0)
                            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                    ),
                    CameraPosition,
                    Name::from("Camera Desired Position"),
                ));

                child_builder.spawn(SpotLightBundle {
                    spot_light: SpotLight {
                        color: Color::rgb(225.0 / 255.0, 208.0 / 255.0, 182.0 / 255.0),
                        intensity: 10000.0,
                        range: 200.0,
                        shadows_enabled: true,
                        outer_angle: 0.5,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        vehicle_config.length,
                        0.0,
                        -vehicle_config.width,
                    )
                    .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0)),
                    ..default()
                });
                child_builder.spawn(SpotLightBundle {
                    spot_light: SpotLight {
                        color: Color::rgb(225.0 / 255.0, 208.0 / 255.0, 182.0 / 255.0),
                        intensity: 10000.0,
                        range: 200.0,
                        shadows_enabled: true,
                        outer_angle: 0.5,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        vehicle_config.length,
                        0.0,
                        vehicle_config.width,
                    )
                    .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0)),
                    ..default()
                });
            }

            child_builder.spawn(PbrBundle {
                mesh: meshes.add(shape::Cube { size: 0.25 }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    emissive: Color::RED,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    -vehicle_config.length,
                    vehicle_config.height / 3.0,
                    vehicle_config.width - 0.125,
                )
                .with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 2.0)),
                ..default()
            });
            child_builder.spawn(PbrBundle {
                mesh: meshes.add(shape::Cube { size: 0.25 }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    emissive: Color::RED,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    -vehicle_config.length,
                    vehicle_config.height / 3.0,
                    -vehicle_config.width + 0.125,
                )
                .with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 2.0)),
                ..default()
            });
        })
        .id()
}

#[derive(Event)]
struct AddForce {
    force: Vec3,
    point: Vec3,
    entity: Entity,
}

fn reset_car(
    keys: Res<Input<KeyCode>>,
    mut drivables: Query<
        (
            &mut Transform,
            &VehicleConfig,
            &mut Velocity,
            Option<&Car>,
            &mut ExternalForce,
        ),
        With<Drivable>,
    >,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    let mut should_respawn = keys.just_pressed(KeyCode::R);
    for ev in gamepad_evr.iter() {
        match ev {
            GamepadEvent::Button(button_ev) => {
                if button_ev.value == 0.0 {
                    continue;
                }
                match button_ev.button_type {
                    GamepadButtonType::Start => {
                        should_respawn = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    for (
        mut drivable_transform,
        drivable_config,
        mut drivable_velocity,
        maybe_car,
        mut external_force,
    ) in &mut drivables
    {
        let reseted_tranform = Transform::from_xyz(
            match maybe_car {
                Some(_) => drivable_config.length + drivable_config.anchor_point.x,
                None => -drivable_config.length - drivable_config.anchor_point.x,
            },
            drivable_config.height,
            0.,
        );

        if should_respawn {
            external_force.force = Vec3::ZERO;
            external_force.torque = Vec3::ZERO;
            drivable_velocity.linvel = Vec3::ZERO;
            drivable_velocity.angvel = Vec3::ZERO;
            drivable_transform.translation = reseted_tranform.translation;
            drivable_transform.rotation = reseted_tranform.rotation;
        }
    }
}

fn calculate_tire_distances_to_ground(
    mut tires: Query<(&GlobalTransform, &Parent, &mut Tire)>,
    drivables: Query<&VehicleConfig, With<Drivable>>,
    rapier_context: Res<RapierContext>,
) {
    for (tire_transform, parent, mut tire) in &mut tires {
        let parent_config = drivables.get(parent.get()).unwrap();
        let hit = rapier_context.cast_ray(
            tire_transform.translation(),
            tire_transform.down(),
            parent_config.spring_offset,
            false,
            QueryFilter::only_fixed(),
        );
        if let Some((_, hit_distance)) = hit {
            tire.distance_to_ground = Some(hit_distance);
        } else {
            tire.distance_to_ground = None;
        }
    }
}

fn calculate_tire_suspension_forces(
    tires: Query<(&GlobalTransform, &Parent, &Tire)>,
    drivables: Query<(Entity, &Velocity, &Transform, &VehicleConfig), With<Drivable>>,
    mut add_forces: EventWriter<AddForce>,
) {
    for (tire_transform, parent, tire) in &tires {
        let (parent_entity, parent_velocity, parent_transform, parent_config) =
            drivables.get(parent.get()).unwrap();
        if let Some(hit_distance) = tire.distance_to_ground {
            let spring_direction = tire_transform.up();
            let tire_velocity = parent_velocity.linear_velocity_at_point(
                tire_transform.translation(),
                parent_transform.translation,
            );
            let offset = parent_config.spring_offset - hit_distance;
            let velocity = spring_direction.dot(tire_velocity);
            let force = (offset * parent_config.spring_power) - (velocity * parent_config.shock);
            add_forces.send(AddForce {
                force: spring_direction * force,
                point: tire_transform.translation(),
                entity: parent_entity,
            });
        }
    }
}

fn lookup_power(velocity: Velocity, max_speed: f32, max_force: f32) -> f32 {
    let speed_ratio = velocity.linvel.length() / max_speed;
    let lookup = if speed_ratio < 0.0 {
        0.5
    } else if speed_ratio >= 0.0 && speed_ratio < 0.4 {
        -(-0.5 * speed_ratio + 0.3).log(10.0)
    } else if speed_ratio >= 0.4 && speed_ratio <= 0.698 {
        1.0
    } else if speed_ratio > 0.698 && speed_ratio <= 1.0 {
        (-5.0 * speed_ratio + 6.0).log(10.0) + 0.6
    } else {
        0.0
    };
    max_force * lookup
}

fn calculate_tire_acceleration_and_braking_forces(
    keys: Res<Input<KeyCode>>,
    tires: Query<(&GlobalTransform, &Parent, &Tire)>,
    drivables: Query<(Entity, &Velocity, &VehicleConfig), With<Drivable>>,
    mut add_forces: EventWriter<AddForce>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut left_trigger: Local<f32>,
    mut right_trigger: Local<f32>,
) {
    for (tire_transform, parent, tire) in &tires {
        let (parent_entity, parent_velocity, parent_config) = drivables.get(parent.get()).unwrap();
        let force_at_tire = tire_transform
            .compute_transform()
            .rotation
            .mul_vec3(Vec3::new(
                lookup_power(
                    *parent_velocity,
                    parent_config.max_speed,
                    parent_config.max_force,
                ),
                0.0,
                0.0,
            ));
        let mut multiplier = if keys.pressed(KeyCode::W) {
            1.0
        } else if keys.pressed(KeyCode::S) {
            -1.0
        } else {
            0.0
        };
        for ev in gamepad_evr.iter() {
            match ev {
                GamepadEvent::Button(button_ev) => match button_ev.button_type {
                    GamepadButtonType::RightTrigger2 => {
                        *right_trigger = button_ev.value;
                    }
                    GamepadButtonType::LeftTrigger2 => {
                        *left_trigger = -button_ev.value;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        if *left_trigger != 0.0 {
            multiplier = *left_trigger;
        } else if *right_trigger != 0.0 {
            multiplier = *right_trigger;
        }
        if tire.distance_to_ground.is_some() && tire.connected_to_engine {
            add_forces.send(AddForce {
                force: multiplier * force_at_tire,
                point: tire_transform.translation(),
                entity: parent_entity,
            });
        }
    }
}

fn calculate_tire_friction(
    tires: Query<(&GlobalTransform, &Parent, &Tire)>,
    drivables: Query<(Entity, &Velocity, &Transform, &ReadMassProperties), With<Drivable>>,
    mut add_forces: EventWriter<AddForce>,
) {
    let coefficient_of_friction = 0.5;
    for (tire_transform, parent, tire) in &tires {
        let (parent_entity, parent_velocity, parent_transform, ReadMassProperties(mass_properties)) =
            drivables.get(parent.get()).unwrap();
        if tire.distance_to_ground.is_some() && tire.connected_to_engine {
            if parent_velocity.linvel.length() > 0.0 {
                let tire_velocity = parent_velocity.linear_velocity_at_point(
                    tire_transform.translation(),
                    parent_transform.translation,
                );
                let multiplier = if tire_velocity.dot(tire_transform.right()) < 0.0 {
                    1.0
                } else {
                    -1.0
                };
                add_forces.send(AddForce {
                    force: multiplier
                        * tire_transform
                            .compute_transform()
                            .rotation
                            .mul_vec3(Vec3::new(
                                (mass_properties.mass / 4.0) * coefficient_of_friction * 9.81,
                                0.0,
                                0.0,
                            )),
                    point: tire_transform.translation(),
                    entity: parent_entity,
                });
            }
        }
    }
}

fn turn_tires(
    drivables: Query<&VehicleConfig, With<Drivable>>,
    keys: Res<Input<KeyCode>>,
    mut tires: Query<(&mut Transform, &Tire, &Parent)>,
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
) {
    for (mut tire_transform, tire, parent) in &mut tires {
        let parent_config = drivables.get(parent.get()).unwrap();
        let mut multiplier = if keys.pressed(KeyCode::D) {
            -1.0
        } else if keys.pressed(KeyCode::A) {
            1.0
        } else {
            0.0
        };

        for gamepad in gamepads.iter() {
            let axis_lx = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            };
            if let Some(x) = axes.get(axis_lx) {
                if x != 0.0 {
                    multiplier = -x;
                }
            }
        }

        if tire.turns {
            tire_transform.rotation =
                Quat::from_axis_angle(Vec3::Y, multiplier * parent_config.turn_radius);
        }
    }
}

fn calculate_tire_turning_forces(
    drivables: Query<(Entity, &Transform, &Velocity, &ReadMassProperties), With<Drivable>>,
    tires: Query<(&Tire, &GlobalTransform, &Parent)>,
    mut add_forces: EventWriter<AddForce>,
) {
    for (tire, tire_transform, parent) in &tires {
        let (parent_entity, parent_transform, parent_velocity, ReadMassProperties(car_mass)) =
            drivables.get(parent.get()).unwrap();
        if tire.distance_to_ground.is_some() {
            let steering_direction = tire_transform.compute_transform().forward();
            let tire_velocity = parent_velocity.linear_velocity_at_point(
                tire_transform.translation(),
                parent_transform.translation,
            );
            let steering_velocity = steering_direction.dot(tire_velocity);
            let desired_velocity_change = -steering_velocity * tire.grip;
            let desired_acceleration = desired_velocity_change * 60.0;
            add_forces.send(AddForce {
                force: steering_direction * desired_acceleration * (car_mass.mass / 4.0),
                point: tire_transform.translation(),
                entity: parent_entity,
            });
        }
    }
}

fn sum_all_forces(
    mut add_forces: EventReader<AddForce>,
    mut drivables: Query<(Entity, &Transform, &mut ExternalForce), With<Drivable>>,
) {
    let mut final_forces = HashMap::new();
    for (entity, _, _) in &drivables {
        final_forces.insert(entity, ExternalForce::default());
    }

    for AddForce {
        force,
        point,
        entity,
    } in add_forces.iter()
    {
        let (_, transform, _) = drivables.get(entity.clone()).unwrap();
        let force_on_entity = ExternalForce::at_point(*force, *point, transform.translation);
        *final_forces.get_mut(entity).unwrap() += force_on_entity;
    }

    // set the external forces on the entity to the calculated final_force
    for (entity, _, mut external_force) in &mut drivables {
        let final_force = final_forces.get(&entity).unwrap();
        external_force.force = final_force.force;
        external_force.torque = final_force.torque;
    }
}

fn draw_tire_force_gizmos(mut add_forces: EventReader<AddForce>, mut gizmos: Gizmos) {
    let scale_factor = 0.04;
    for AddForce {
        force,
        point,
        entity: _,
    } in add_forces.iter()
    {
        gizmos.ray(*point, *force * scale_factor, Color::BLUE);
    }
}

fn move_tires_to_ground(
    tires: Query<(&Parent, &Children, &Tire)>,
    drivables: Query<&VehicleConfig, With<Drivable>>,
    mut transforms: Query<&mut Transform>,
) {
    for (tire_parent, tire_child, tire) in &tires {
        let parent_config = drivables.get(tire_parent.get()).unwrap();
        for child in tire_child.iter() {
            let mut x = transforms.get_mut(*child).unwrap();
            if let Some(hit_distance) = tire.distance_to_ground {
                x.translation.y = -hit_distance + 0.5;
            } else {
                x.translation.y = -parent_config.spring_offset + 0.5;
            }
        }
    }
}
