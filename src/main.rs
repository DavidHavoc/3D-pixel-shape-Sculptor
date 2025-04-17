use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    log::{Level, LogPlugin},
    window::{WindowLevel, PresentMode},
    app::AppExit,  // Added this import for AppExit
};
use bevy_egui::{EguiContexts, EguiPlugin};

mod shapes;
mod ui;
mod export;

use shapes::{VoxelData, Shape, ShapeType, generate_shape};
use ui::UiState;

fn main() {
    // Configure logging to see what's happening
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_app=info".to_string(),
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Voxel Sculptor".to_string(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_level: WindowLevel::Normal,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(UiState::default())
        .insert_resource(SafetyCheck::default())
        .add_systems(Startup, (setup, initialize_default_shape))
        .add_systems(Update, (ui_system, rotate_camera, update_voxels, handle_keyboard_input))
        .run();
}

// Safety resource to ensure we don't crash on empty voxel data
#[derive(Resource, Default)]
struct SafetyCheck {
    initialized: bool,
}

fn setup(
    mut commands: Commands,
) {
    info!("Setting up the application...");
    
    // Set up the 3D camera with a safe distance
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 15.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add an ambient light to ensure shapes are visible
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    
    info!("Setup complete!");
}

// Initialize a default shape to prevent empty data issues
fn initialize_default_shape(
    mut commands: Commands,
) {
    info!("Initializing default shape...");
    
    let default_shape = Shape {
        shape_type: ShapeType::Cube,
        width: 5,
        height: 5,
        depth: 5,
    };
    
    let voxel_data = VoxelData {
        shape: default_shape.clone(),
        voxels: generate_shape(&default_shape),
    };
    
    commands.insert_resource(voxel_data);
    
    info!("Default shape initialized!");
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut voxel_data: ResMut<VoxelData>,
    mut safety: ResMut<SafetyCheck>,
) {
    // Mark as initialized once UI is rendered
    safety.initialized = true;
    ui::draw_ui(&mut contexts, &mut ui_state, &mut voxel_data);
}

// Add keyboard handling for ESC to quit
fn handle_keyboard_input(
    input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        info!("Escape key pressed, exiting application...");
        exit.send(AppExit);
    }
}

fn rotate_camera(
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if query.is_empty() {
        return; // Skip if no camera found
    }
    
    let mut camera_transform = query.single_mut();
    
    if mouse_input.pressed(MouseButton::Right) {
        for event in mouse_motion_events.read() {
            let sensitivity = 0.005;
            
            // Rotate around the Y axis for horizontal mouse movement
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_rotation_y(-event.delta.x * sensitivity)
            );
            
            // Rotate around the local X axis for vertical mouse movement
            let right = camera_transform.rotation.mul_vec3(Vec3::X);
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(right, -event.delta.y * sensitivity)
            );
        }
    }
}

fn update_voxels(
    mut commands: Commands,
    voxel_data: Res<VoxelData>,
    safety: Res<SafetyCheck>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxels: Query<Entity, With<VoxelInstance>>,
) {
    // Only proceed if initialization is complete and data has changed
    if !safety.initialized || !voxel_data.is_changed() {
        return;
    }
    
    // Remove old voxels
    for entity in voxels.iter() {
        commands.entity(entity).despawn();
    }

    // Skip if no voxels to draw
    if voxel_data.voxels.is_empty() {
        return;
    }

    // Create pink material
    let pink_material = materials.add(StandardMaterial {
        base_color: Color::hex("AC1754").unwrap(),
        perceptual_roughness: 0.9,
        ..default()
    });

    // Create box mesh for voxels
    let voxel_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.95 }));

    // Spawn new voxels based on the shape data
    for voxel in voxel_data.voxels.iter() {
        commands.spawn((
            PbrBundle {
                mesh: voxel_mesh.clone(),
                material: pink_material.clone(),
                transform: Transform::from_xyz(
                    voxel.0 as f32 - (voxel_data.shape.width as f32 / 2.0),
                    voxel.1 as f32 - (voxel_data.shape.height as f32 / 2.0),
                    voxel.2 as f32 - (voxel_data.shape.depth as f32 / 2.0),
                ),
                ..default()
            },
            VoxelInstance,
        ));
    }
}

// Marker component for voxel instances
#[derive(Component)]
struct VoxelInstance;
