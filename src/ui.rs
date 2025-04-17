use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::shapes::{ShapeType, Shape, VoxelData, generate_shape};
use crate::export;

#[derive(Debug, Resource)]
pub struct UiState {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub shape_type: ShapeType,
    pub generate: bool,
    pub export: bool,
    pub error_message: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            width: 5,
            height: 5,
            depth: 5,
            shape_type: ShapeType::Cube,
            generate: true,
            export: false,
            error_message: None,
        }
    }
}

pub fn draw_ui(
    contexts: &mut EguiContexts,
    ui_state: &mut UiState,
    voxel_data: &mut VoxelData,
) {
    let ctx = contexts.ctx_mut();
    
    egui::SidePanel::left("control_panel").default_width(250.0).show(ctx, |ui| {
        ui.heading("3D Voxel Sculptor");
        ui.separator();
        
        ui.label("Dimensions (1-32):");
        ui.add(egui::Slider::new(&mut ui_state.width, 1..=32).text("Width"));
        ui.add(egui::Slider::new(&mut ui_state.height, 1..=32).text("Height"));
        ui.add(egui::Slider::new(&mut ui_state.depth, 1..=32).text("Depth"));

        ui.separator();
        ui.label("Shape Type:");
        
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", ui_state.shape_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::Circle, "Circle");
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::Sphere, "Sphere");
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::Cube, "Cube");
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::SquarePyramid, "Square Pyramid");
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::Cone, "Cone");
                ui.selectable_value(&mut ui_state.shape_type, ShapeType::Cylinder, "Cylinder");
            });
        
        ui.separator();
        
        if ui.button("Generate Shape").clicked() {
            ui_state.generate = true;
        }
        
        if ui.button("Export as OBJ").clicked() {
            ui_state.export = true;
        }
        
        ui.separator();
        ui.label("Controls:");
        ui.label("• Right-click + drag to rotate");
        ui.label("• Mouse wheel to zoom");
        ui.label("• ESC to exit");
        
        if let Some(error) = &ui_state.error_message {
            ui.separator();
            ui.colored_label(egui::Color32::RED, error);
        }
    });
    
    if ui_state.generate {
        ui_state.error_message = None;
        
        let shape = Shape {
            shape_type: ui_state.shape_type,
            width: ui_state.width,
            height: ui_state.height,
            depth: ui_state.depth,
        };
        
        voxel_data.shape = shape.clone();
        voxel_data.voxels = generate_shape(&shape);
        
        if voxel_data.voxels.is_empty() {
            ui_state.error_message = Some("Failed to generate shape: no voxels created".to_string());
        }
        
        ui_state.generate = false;
    }
    
    if ui_state.export {
        match export::export_to_obj(&voxel_data) {
            Ok(_) => {
                egui::Window::new("Export Successful")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Shape exported successfully to 'exported_shape.obj'");
                        if ui.button("Close").clicked() {
                            ui_state.export = false;
                        }
                    });
            },
            Err(e) => {
                egui::Window::new("Export Failed")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(format!("Failed to export: {}", e));
                        if ui.button("Close").clicked() {
                            ui_state.export = false;
                        }
                    });
            }
        }
    }
}
