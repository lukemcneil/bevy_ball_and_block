use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::Slider;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
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
        .add_systems(Update, (config_ui_system, keyboard_events, draw_axis))
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Block;

fn setup(mut commands: Commands, config: ResMut<Config>) {
    commands.spawn((Camera2dBundle {
        camera: Camera::default(),
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },));

    spawn_ball_and_block(&mut commands, &config);
}

fn spawn_ball_and_block(commands: &mut Commands, config: &ResMut<Config>) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(
            config.ball_starting_x,
            config.block_height * config.ball_starting_y,
            0.0,
        )),
        RigidBody::Dynamic,
        Collider::ball(config.ball_radius),
        Friction::coefficient(config.friction),
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
        Friction::coefficient(config.friction),
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
            principal_inertia: config.moment_of_inertia(),
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
    ball_starting_y: f32,
    ball_mass: f32,
    block_height: f32,
    block_width: f32,
    block_mass: f32,
    elasticity: f32,
    friction: f32,
}

impl Config {
    fn moment_of_inertia(&self) -> f32 {
        (1.0 / 12.0) * (self.block_mass) * (self.block_height * 2.0).powi(2)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ball_speed: 100.0,
            ball_radius: 3.0,
            ball_starting_x: -100.0,
            ball_starting_y: 1.0,
            ball_mass: 1.0,
            block_height: 100.0,
            block_width: 1.0,
            block_mass: 1.0,
            elasticity: 1.0,
            friction: 0.0,
        }
    }
}

fn config_ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ball: Query<(Entity, &Velocity), With<Ball>>,
    mut block: Query<(Entity, &Velocity), (With<Block>, Without<Ball>)>,
    mut config: ResMut<Config>,
) {
    let (ball_entity, ball_velocity) = ball.single_mut();
    let (block_entity, block_velocity) = block.single_mut();
    bevy_inspector_egui::egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        if ui
            .add(Slider::new(&mut config.ball_speed, 10.0..=200.0).text("ball speed"))
            .union(ui.add(Slider::new(&mut config.ball_radius, 1.0..=20.0).text("ball radius")))
            .union(ui.add(
                Slider::new(&mut config.ball_starting_x, -200.0..=-10.0).text("ball starting x"),
            ))
            .union(
                ui.add(Slider::new(&mut config.ball_starting_y, 0.0..=1.0).text("ball starting y")),
            )
            .union(ui.add(Slider::new(&mut config.ball_mass, 0.1..=50.0).text("ball mass")))
            .union(ui.add(Slider::new(&mut config.block_height, 10.0..=300.0).text("block height")))
            .union(ui.add(Slider::new(&mut config.block_width, 0.1..=10.0).text("block width")))
            .union(ui.add(Slider::new(&mut config.block_mass, 0.1..=50.0).text("block mass")))
            .union(ui.add(Slider::new(&mut config.elasticity, 0.0..=1.0).text("elasticity")))
            .union(ui.add(Slider::new(&mut config.friction, 0.0..=1.0).text("friction")))
            .changed
        {
            commands.entity(ball_entity).despawn();
            commands.entity(block_entity).despawn();
            spawn_ball_and_block(&mut commands, &config);
        }
        if ui.button("Restart Simulation (press space)").clicked() {
            commands.entity(ball_entity).despawn();
            commands.entity(block_entity).despawn();
            spawn_ball_and_block(&mut commands, &config);
        }
        if ui.button("Reset Settings").clicked() {
            *config = Config::default();
            commands.entity(ball_entity).despawn();
            commands.entity(block_entity).despawn();
            spawn_ball_and_block(&mut commands, &config);
        }
    });

    bevy_inspector_egui::egui::Window::new("Stats").show(contexts.ctx_mut(), |ui| {
        ui.group(|ui| {
            ui.heading("Velocity");
            ui.label(format!("ball linear: {:.2}", ball_velocity.linvel.x));
            ui.label(format!("ball angular: {:.2}", ball_velocity.angvel));
            ui.label(format!("block linear: {:.2}", block_velocity.linvel.x));
            ui.label(format!("block angular: {:.2}", block_velocity.angvel));
        });
        ui.group(|ui| {
            ui.heading("Linear Momentum");
            let ball_momentum = config.ball_mass * ball_velocity.linvel.x;
            ui.label(format!("ball: {:.2}", ball_momentum));
            let block_momentum = config.block_mass * block_velocity.linvel.x;
            ui.label(format!("block: {:.2}", block_momentum));
            ui.label(format!("total: {:.2}", ball_momentum + block_momentum));
        });
        ui.group(|ui| {
            ui.heading("Angular Momentum");
            let ball_angular_momentum = (config.block_height * config.ball_starting_y)
                * config.ball_mass
                * ball_velocity.linvel.x;
            ui.label(format!("ball: {:.2}", ball_angular_momentum));
            let block_angular_momentum = config.moment_of_inertia() * block_velocity.angvel.abs();
            ui.label(format!("block: {:.2}", block_angular_momentum));
            ui.label(format!(
                "total: {:.2}",
                ball_angular_momentum + block_angular_momentum
            ));
        });
        ui.group(|ui| {
            ui.heading("Energy");
            let ball_linear_energy = (config.ball_mass / 2.0) * ball_velocity.linvel.x.powi(2);
            ui.label(format!("ball linear: {:.2}", ball_linear_energy));
            let block_linear_energy = (config.block_mass / 2.0) * block_velocity.linvel.x.powi(2);
            ui.label(format!(
                "block linear: {:.2}",
                (config.block_mass / 2.0) * block_velocity.linvel.x.powi(2)
            ));
            let block_angular_energy =
                (config.moment_of_inertia() / 2.0) * block_velocity.angvel.powi(2);
            ui.label(format!("block angular: {:.2}", block_angular_energy));
            ui.label(format!(
                "total: {:.2}",
                ball_linear_energy + block_linear_energy + block_angular_energy
            ));
        });
    });
}

fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
    mut commands: Commands,
    mut ball: Query<Entity, With<Ball>>,
    mut block: Query<Entity, (With<Block>, Without<Ball>)>,
    config: ResMut<Config>,
) {
    let ball_entity = ball.single_mut();
    let block_entity = block.single_mut();
    for ev in key_evr.iter() {
        if let ButtonState::Pressed = ev.state {
            if let Some(KeyCode::Space) = ev.key_code {
                commands.entity(ball_entity).despawn();
                commands.entity(block_entity).despawn();
                spawn_ball_and_block(&mut commands, &config);
            }
        }
    }
}

fn draw_axis(mut gizmos: Gizmos) {
    gizmos.line_2d(1000.0 * Vec2::NEG_X, 1000.0 * Vec2::X, Color::BLACK);
}
