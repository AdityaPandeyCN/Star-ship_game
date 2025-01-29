// Import necessary modules from the Bevy game engine
use bevy::prelude::*;
// Import the PrimaryWindow type for window handling
use bevy::window::PrimaryWindow;
// Import the inspector plugin for debugging and visualization
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Define the Starship component with reflection capabilities for the inspector
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Starship {
    rotation_speed: f32,  // How fast the ship can rotate
}

// Define the Velocity component to track ship's movement
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Velocity {
    x: f32,  // Horizontal velocity
    y: f32,  // Vertical velocity
}

// Define the Engine component to handle ship's propulsion
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Engine {
    fuel: f32,    // Amount of fuel remaining
    thrust: f32,  // Power of the engine
}

// Setup system that runs when the game starts
fn setup(
    mut commands: Commands,                                    // For spawning entities
    asset_server: Res<AssetServer>,                           // For loading assets
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,        // For handling sprite sheets
) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load the ship sprite sheet
    let texture_handle = asset_server.load("ship.png");
    // Create a texture atlas from the sprite sheet (2x2 grid of 32x32 pixel sprites)
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2, None, None);
    // Add the texture atlas to the game's assets
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn the player's ship
    commands
        .spawn((
            // Visual components for the ship
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 0.0),         // Ship size
                    translation: Vec3::new(0.0, 0.0, 0.0),    // Starting position (center)
                    ..default()
                },
                ..default()
            },
            // Add ship components with initial values
            Starship { rotation_speed: 1.0 },
            Velocity { x: 0.0, y: 0.0 },
            Engine {
                fuel: 100.0,
                thrust: 100.0,
            },
        ));
}

// System to handle ship rotation
fn rotate_ship_system(
    time: Res<Time>,                                          // For frame-independent movement
    mut query: Query<(&mut Transform, &Starship)>,            // Get ship position and properties
    keyboard_input: Res<Input<KeyCode>>,                      // For keyboard input
) {
    // Get the ship's transform and properties
    let (mut transform, starship) = query.single_mut();

    // Rotate left when Left arrow is pressed
    if keyboard_input.pressed(KeyCode::Left) {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * starship.rotation_speed);
    } 
    // Rotate right when Right arrow is pressed
    else if keyboard_input.pressed(KeyCode::Right) {
        transform.rotation *=
            Quat::from_rotation_z(time.delta_seconds() * -starship.rotation_speed);
    }
}

// System to handle ship movement
fn velocity_system(
    time: Res<Time>,                                          // For frame-independent movement
    mut query: Query<(&mut Transform, &Velocity)>,            // Get position and velocity
    window_query: Query<&Window, With<PrimaryWindow>>,        // Get window dimensions
) {
    // Get the game window
    let window = window_query.single();
    let w = window.width();
    let h = window.height();
    
    // Update position for each entity with Transform and Velocity
    for (mut transform, velocity) in query.iter_mut() {
        // Apply velocity to position
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        // Wrap around screen horizontally
        if transform.translation.x < -w / 2.0 {
            transform.translation.x += w;
        } else if transform.translation.x > w / 2.0 {
            transform.translation.x -= w;
        }

        // Wrap around screen vertically
        if transform.translation.y < -h / 2.0 {
            transform.translation.y += h;
        } else if transform.translation.y > h / 2.0 {
            transform.translation.y -= h;
        }
    }
}

// System to handle ship engine and thrust
fn engine_system(
    time: Res<Time>,                                          // For frame-independent movement
    mut query: Query<(&mut Velocity, &Transform, &mut Engine)>, // Get velocity, position, and engine
    keyboard_input: Res<Input<KeyCode>>,                      // For keyboard input
) {
    // Update each entity with Velocity, Transform, and Engine
    for (mut velocity, transform, mut engine) in query.iter_mut() {
        // Apply thrust when Up arrow is pressed and there's fuel
        if keyboard_input.pressed(KeyCode::Up) && engine.fuel > 0.0 {
            // Get ship's rotation angle
            let (_, _, z) = transform.rotation.to_euler(EulerRot::YXZ);
            // Apply thrust in the direction the ship is facing
            velocity.x -= engine.thrust * time.delta_seconds() * z.sin();
            velocity.y += engine.thrust * time.delta_seconds() * z.cos();
            // Consume fuel
            engine.fuel -= engine.thrust * time.delta_seconds();
            // Ensure fuel stays within valid range
            engine.fuel = engine.fuel.clamp(0.0, 1000.0);
        }
    }
}

// Main function where the game starts
fn main() {
    App::new()
        // Add default plugins and the inspector plugin
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        // Set background color to black
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // Register component types for the inspector
        .register_type::<Starship>()
        .register_type::<Engine>()
        .register_type::<Velocity>()
        // Add the setup system to run at startup
        .add_systems(Startup, setup)
        // Add game systems to run every frame
        .add_systems(Update, (
            rotate_ship_system,
            velocity_system,
            engine_system,
        ))
        // Start the game
        .run();
}