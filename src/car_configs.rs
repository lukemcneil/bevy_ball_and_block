use bevy::prelude::*;

use crate::car::VehicleConfig;

const CAR_LENGTH: f32 = 5.31114 / 2.0;
#[allow(dead_code)]
pub const CAR_CONFIG: VehicleConfig = VehicleConfig {
    height: 1.452024 / 2.0,
    width: 2.02946 / 2.0,
    length: CAR_LENGTH,
    wheelbase: 3.11912 / 2.0,
    wheel_offset: 0.0,
    spring_offset: 1.252926,
    spring_power: 300.0,
    shock: 45.0,
    max_speed: 50.0,
    max_force: 100.0,
    turn_radius: 0.45811518324607,
    anchor_point: Vec3 {
        x: -CAR_LENGTH - 0.787,
        y: -0.7,
        z: 0.0,
    },
    scale: 1.0,
    starting_tire_grip: 0.7,
};

const TRAILER_LENGTH: f32 = 7.8768 / 2.0;
const TRAILER_WIDTH: f32 = 2.159 / 2.0;
#[allow(dead_code)]
pub const TRAILER_CONFIG: VehicleConfig = VehicleConfig {
    height: 0.18234 / 2.0,
    width: TRAILER_WIDTH,
    length: TRAILER_LENGTH,
    wheelbase: 1.0 / 2.0,
    wheel_offset: -1.0,
    spring_offset: 1.0,
    spring_power: 21.0,
    shock: 5.0,
    max_speed: 0.0,
    max_force: 0.0,
    turn_radius: 0.0,
    anchor_point: Vec3 {
        x: TRAILER_LENGTH + TRAILER_WIDTH,
        y: -(0.18234 / 2.0),
        z: 0.0,
    },
    scale: 1.0,
    starting_tire_grip: 0.7,
};

const DRIFTER_LENGTH: f32 = 3.31114 / 2.0;
pub const DRIFTER_CONFIG: VehicleConfig = VehicleConfig {
    height: 1.252024 / 2.0,
    width: 2.02946 / 2.0,
    length: DRIFTER_LENGTH,
    wheelbase: 3.11912 / 2.0,
    wheel_offset: 0.0,
    spring_offset: 1.252926,
    spring_power: 300.0,
    shock: 45.0,
    max_speed: 50.0,
    max_force: 160.0,
    turn_radius: 0.45811518324607,
    anchor_point: Vec3 {
        x: -DRIFTER_LENGTH * 1.1,
        y: -0.7,
        z: 0.0,
    },
    scale: 1.0,
    starting_tire_grip: 0.03,
};

const DRIFTER_TRAILER_LENGTH: f32 = 2.8768 / 2.0;
const DRIFTER_TRAILER_WIDTH: f32 = 2.159 / 2.0;
pub const DRIFTER_TRAILER_CONFIG: VehicleConfig = VehicleConfig {
    height: 0.18234 / 2.0,
    width: DRIFTER_TRAILER_WIDTH,
    length: DRIFTER_TRAILER_LENGTH,
    wheelbase: 1.0 / 2.0,
    wheel_offset: 0.0,
    spring_offset: 1.0,
    spring_power: 15.0,
    shock: 3.0,
    max_speed: 0.0,
    max_force: 0.0,
    turn_radius: 0.0,
    anchor_point: Vec3 {
        x: DRIFTER_TRAILER_LENGTH + DRIFTER_TRAILER_WIDTH,
        y: -(0.18234 / 2.0),
        z: 0.0,
    },
    scale: 1.0,
    starting_tire_grip: 0.03,
};
