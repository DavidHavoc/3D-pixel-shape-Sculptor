use bevy::{
    input::mouse::MouseButton, // Keep specific MouseButton if needed, but prelude usually covers it
    input::keyboard::KeyCode, // Keep specific KeyCode if needed
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use core::fmt; // REMOVE THIS LINE
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

// --- Constants ---
const PINK_COLOR_HEX: &str = "AC1754";
const MIN_DIMENSION: u32 = 1;
const MAX_DIMENSION: u32 = 32;

// --- Resources ---

// Stores the user's input from the GUI
#[derive(Resource, Debug, Clone)]
struct UserInput {
    width: u32,
    depth: u32,
    height: u32,
    shape: GeometricShape,
    // Flag to trigger regeneration when inputs change
    needs_regeneration: bool,
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            width: 8,
            depth: 8,
            height: 8,
            shape: GeometricShape::Cube,
            needs_regeneration: true, // Regenerate on startup
        }
    }
}

// Holds the handle for the pink material
#[derive(Resource)]
struct VoxelMaterial(Handle<StandardMaterial>);

// Holds the handle for the cube mesh
#[derive(Resource)]
struct VoxelMesh(Handle<Mesh>);

// --- Components ---

// Marker component for voxel entities, used for cleanup
#[derive(Component)]
struct Voxel;

// --- Enums ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter)]
enum GeometricShape {
    Cube,
    Sphere,
    Cylinder,
    Cone,
    SquarePyramid,
}

// --- Systems ---

// Setup function to initialize resources and scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Parse the pink color from hex
    let pink_color_bytes = hex::decode(PINK_COLOR_HEX).expect("Invalid hex color");
    let pink_color = Color::rgb_u8(
        pink_color_bytes[0],
        pink_color_bytes[1],
        pink_color_bytes[2],
    );

    // Create and store the voxel material
    let material_handle = materials.add(StandardMaterial {
        base_color: pink_color,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });
    commands.insert_resource(VoxelMaterial(material_handle));

    // Create and store the unit cube mesh
    let mesh_handle = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    commands.insert_resource(VoxelMesh(mesh_handle));

  
    // Spawn camera entity
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 35.0)
                       .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // Add PanOrbitCamera component to the same entity
        PanOrbitCamera {
             button_orbit: MouseButton::Left, 
             button_pan: MouseButton::Right,
             modifier_orbit: Some(KeyCode::ShiftLeft), 
             zoom_sensitivity: 0.2,
            ..default()
         },
    ));
 


    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });

    // Add a directional light source
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(15.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Initialize user input resource
    commands.insert_resource(UserInput::default());
}


fn ui_system(mut contexts: EguiContexts, mut user_input: ResMut<UserInput>) {
    egui::Window::new("Sculptor Controls").show(contexts.ctx_mut(), |ui| {
        let mut changed = false;

        ui.heading("Dimensions");

        // Width Slider
        let mut current_width = user_input.width;
        ui.add(egui::Slider::new(&mut current_width, MIN_DIMENSION..=MAX_DIMENSION).text("Width"));
        if current_width != user_input.width {
            user_input.width = current_width;
            changed = true;
        }

        // Depth Slider
        let mut current_depth = user_input.depth;
        ui.add(egui::Slider::new(&mut current_depth, MIN_DIMENSION..=MAX_DIMENSION).text("Depth"));
        if current_depth != user_input.depth {
            user_input.depth = current_depth;
            changed = true;
        }

        // Height Slider
        let mut current_height = user_input.height;
        ui.add(egui::Slider::new(&mut current_height, MIN_DIMENSION..=MAX_DIMENSION).text("Height"));
        if current_height != user_input.height {
            user_input.height = current_height;
            changed = true;
        }

        ui.separator();
        ui.heading("Shape");

        // Shape Dropdown (Combo Box)
        let selected_shape_label = user_input.shape.to_string();
        egui::ComboBox::from_label("Select Shape")
            .selected_text(selected_shape_label)
            .show_ui(ui, |ui| {
                for shape in GeometricShape::iter() {
                    if ui
                        .selectable_value(&mut user_input.shape, shape, shape.to_string())
                        .clicked()
                    {
                        changed = true;
                    }
                }
            });

        if changed {
            user_input.needs_regeneration = true;
        }
    });
}

fn generate_shape_system(
    mut commands: Commands,
    mut user_input: ResMut<UserInput>,
    voxel_query: Query<Entity, With<Voxel>>,
    voxel_material: Res<VoxelMaterial>,
    voxel_mesh: Res<VoxelMesh>,
) {
    if !user_input.needs_regeneration {
        return;
    }

    println!("Regenerating shape: {:?}", *user_input);

    // 1. Despawn existing voxels
    for entity in voxel_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // 2. Get parameters
    let w = user_input.width as f32;
    let d = user_input.depth as f32;
    let h = user_input.height as f32;

    let center_x = w / 2.0 - 0.5;
    let center_y = h / 2.0 - 0.5;
    let center_z = d / 2.0 - 0.5;

    let radius_x = w / 2.0;
    let radius_y = h / 2.0; //sphere uses it as radius
    let radius_z = d / 2.0;

    // 3. Iterate through potential voxel grid positions
    for y_idx in 0..user_input.height {
        for z_idx in 0..user_input.depth {
            for x_idx in 0..user_input.width {
                // Voxel center coordinates (relative to grid origin 0,0,0)
                let vx = x_idx as f32 + 0.5;
                let vy = y_idx as f32 + 0.5;
                let vz = z_idx as f32 + 0.5;

                // Voxel position in world space (relative to center of bounding box)
                let pos_x = x_idx as f32 - center_x;
                let pos_y = y_idx as f32 - center_y;
                let pos_z = z_idx as f32 - center_z;

                let mut should_spawn = false;
                match user_input.shape {
                    GeometricShape::Cube => {
                        should_spawn = true;
                    }
                    GeometricShape::Sphere => {
                        let norm_x = if radius_x > 0.0 { (vx - w / 2.0) / radius_x } else { 0.0 };
                        let norm_y = if radius_y > 0.0 { (vy - h / 2.0) / radius_y } else { 0.0 };
                        let norm_z = if radius_z > 0.0 { (vz - d / 2.0) / radius_z } else { 0.0 };
                        if norm_x.powi(2) + norm_y.powi(2) + norm_z.powi(2) <= 1.0 {
                            should_spawn = true;
                        }
                    }
                    GeometricShape::Cylinder => {
                        // Y-axis aligned cylinder
                        let norm_x = if radius_x > 0.0 { (vx - w / 2.0) / radius_x } else { 0.0 };
                        let norm_z = if radius_z > 0.0 { (vz - d / 2.0) / radius_z } else { 0.0 };
                        if norm_x.powi(2) + norm_z.powi(2) <= 1.0 && y_idx < user_input.height {
                            should_spawn = true;
                        }
                    }
                    GeometricShape::Cone => {
                        // Y-axis aligned cone, apex pointing up (+Y)
                        if h <= 0.0 { continue; }
                        let scale_factor = (1.0 - (vy / h)).max(0.0); // Ensure scale factor is not negative

                        let scaled_radius_x = radius_x * scale_factor;
                        let scaled_radius_z = radius_z * scale_factor;

                        let norm_x = if scaled_radius_x > 0.01 { (vx - w/2.0) / scaled_radius_x } else { 0.0 };
                        let norm_z = if scaled_radius_z > 0.01 { (vz - d/2.0) / scaled_radius_z } else { 0.0 };

                        // Check within base ellipse at this height and within overall height
                        if norm_x.powi(2) + norm_z.powi(2) <= 1.0 && y_idx < user_input.height {
                             should_spawn = true;
                        }
                    }
                    GeometricShape::SquarePyramid => {
                        // Y-axis aligned pyramid, apex pointing up (+Y)
                        if h <= 0.0 { continue; }
                        let scale_factor = (1.0 - (vy / h)).max(0.0); // Ensure scale factor is not negative

                        let max_dist_x = radius_x * scale_factor;
                        let max_dist_z = radius_z * scale_factor;

                         // Check within bounding box at this height and within overall height
                        if (vx - w / 2.0).abs() <= max_dist_x && (vz - d / 2.0).abs() <= max_dist_z && y_idx < user_input.height {
                             should_spawn = true;
                        }
                    }
                }

                // 5. Spawn the voxel entity if needed
                if should_spawn {
                    commands.spawn((
                        PbrBundle {
                            mesh: voxel_mesh.0.clone(),
                            material: voxel_material.0.clone(),
                            transform: Transform::from_xyz(pos_x, pos_y, pos_z),
                            ..default()
                        },
                        Voxel,
                    ));
                }
            }
        }
    }

    // Reset the flag
    user_input.needs_regeneration = false;
}




fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Shape Sculptor".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .add_systems(Update, generate_shape_system)
        .run();
}