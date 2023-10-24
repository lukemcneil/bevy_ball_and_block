use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::Slider;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .insert_resource(Config::default())
        .add_systems(Startup, setup)
        .add_systems(Update, config_ui_system)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Block;

fn setup(mut commands: Commands, config: Res<Config>) {
    commands.spawn((Camera2dBundle {
        camera: Camera::default(),
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },));

    spawn_ball_and_block(commands, config);
}

fn spawn_ball_and_block(mut commands: Commands, config: Res<Config>) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            config.ball_starting_x,
            config.block_height,
            0.0,
        )),
        RigidBody::Dynamic,
        Collider::ball(config.ball_radius),
        Friction::coefficient(0.0),
        GravityScale(0.0),
        Name::from("Ball"),
        Velocity {
            linvel: Vec2 {
                x: config.ball_speed,
                y: 0.0,
            },
            angvel: 0.0,
        },
        Restitution {
            coefficient: config.elasticity,
            combine_rule: CoefficientCombineRule::Average,
        },
        ColliderMassProperties::Mass(config.ball_mass),
        Ball,
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(config.block_width, config.block_height),
        Friction::coefficient(0.0),
        GravityScale(0.0),
        Name::from("Block"),
        Velocity::default(),
        Restitution {
            coefficient: config.elasticity,
            combine_rule: CoefficientCombineRule::Average,
        },
        ColliderMassProperties::MassProperties(MassProperties {
            local_center_of_mass: Vec2::ZERO,
            mass: config.block_mass,
            principal_inertia: (1.0 / 12.0)
                * (config.block_mass)
                * (config.block_height * 2.0).powi(2),
        }),
        Block,
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Config {
    ball_speed: f32,
    ball_radius: f32,
    ball_starting_x: f32,
    ball_mass: f32,
    elasticity: f32,
    block_height: f32,
    block_width: f32,
    block_mass: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ball_speed: 50.0,
            ball_radius: 1.0,
            ball_starting_x: -100.0,
            ball_mass: 1.0,
            elasticity: 1.0,
            block_height: 150.0,
            block_width: 1.0,
            block_mass: 1.0,
        }
    }
}

fn config_ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ball: Query<Entity, With<Ball>>,
    mut block: Query<Entity, (With<Block>, Without<Ball>)>,
    mut config: ResMut<Config>,
) {
    bevy_inspector_egui::egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.add(Slider::new(&mut config.ball_speed, 10.0..=200.0).text("ball speed"));
        ui.add(Slider::new(&mut config.ball_radius, 1.0..=20.0).text("ball radius"));
        ui.add(Slider::new(&mut config.ball_starting_x, -200.0..=-50.0).text("ball starting x"));
        ui.add(Slider::new(&mut config.ball_mass, 0.1..=50.0).text("ball mass"));
        ui.add(Slider::new(&mut config.elasticity, 0.0..=1.0).text("elasticity"));
        ui.add(Slider::new(&mut config.block_height, 10.0..=300.0).text("block height"));
        ui.add(Slider::new(&mut config.block_width, 0.1..=10.0).text("block width"));
        ui.add(Slider::new(&mut config.block_mass, 0.1..=50.0).text("block mass"));
        if ui.button("Reset").clicked() {
            let ball = ball.single_mut();
            commands.entity(ball).despawn();
            let block = block.single_mut();
            commands.entity(block).despawn();

            spawn_ball_and_block(commands, config.into());
        }
    });
}
