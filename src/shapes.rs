use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Circle,
    Sphere,
    Cube,
    SquarePyramid,
    Cone,
    Cylinder,
}

impl Default for ShapeType {
    fn default() -> Self {
        ShapeType::Cube
    }
}

impl std::fmt::Display for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeType::Circle => write!(f, "Circle"),
            ShapeType::Sphere => write!(f, "Sphere"),
            ShapeType::Cube => write!(f, "Cube"),
            ShapeType::SquarePyramid => write!(f, "Square Pyramid"),
            ShapeType::Cone => write!(f, "Cone"),
            ShapeType::Cylinder => write!(f, "Cylinder"),
        }
    }
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct VoxelData {
    pub shape: Shape,
    pub voxels: Vec<(i32, i32, i32)>,
}

pub fn generate_shape(shape: &Shape) -> Vec<(i32, i32, i32)> {
    let mut voxels = Vec::new();
    
    match shape.shape_type {
        ShapeType::Cube => generate_cube(shape, &mut voxels),
        ShapeType::Sphere => generate_sphere(shape, &mut voxels),
        ShapeType::Circle => generate_circle(shape, &mut voxels),
        ShapeType::SquarePyramid => generate_square_pyramid(shape, &mut voxels),
        ShapeType::Cone => generate_cone(shape, &mut voxels),
        ShapeType::Cylinder => generate_cylinder(shape, &mut voxels),
    }
    
    voxels
}

fn generate_cube(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    for x in 0..shape.width {
        for y in 0..shape.height {
            for z in 0..shape.depth {
                voxels.push((x as i32, y as i32, z as i32));
            }
        }
    }
}

fn generate_sphere(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    let center_x = (shape.width as f32 - 1.0) / 2.0;
    let center_y = (shape.height as f32 - 1.0) / 2.0;
    let center_z = (shape.depth as f32 - 1.0) / 2.0;
    
    let radius_x = shape.width as f32 / 2.0;
    let radius_y = shape.height as f32 / 2.0;
    let radius_z = shape.depth as f32 / 2.0;
    
    for x in 0..shape.width {
        for y in 0..shape.height {
            for z in 0..shape.depth {
                let dx = (x as f32 - center_x) / radius_x;
                let dy = (y as f32 - center_y) / radius_y;
                let dz = (z as f32 - center_z) / radius_z;
                
                if dx*dx + dy*dy + dz*dz <= 1.0 {
                    voxels.push((x as i32, y as i32, z as i32));
                }
            }
        }
    }
}

fn generate_circle(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    let center_x = (shape.width as f32 - 1.0) / 2.0;
    let center_z = (shape.depth as f32 - 1.0) / 2.0;
    
    let radius_x = shape.width as f32 / 2.0;
    let radius_z = shape.depth as f32 / 2.0;
    
    for x in 0..shape.width {
        for y in 0..1.min(shape.height) {
            for z in 0..shape.depth {
                let dx = (x as f32 - center_x) / radius_x;
                let dz = (z as f32 - center_z) / radius_z;
                
                if dx*dx + dz*dz <= 1.0 {
                    voxels.push((x as i32, y as i32, z as i32));
                }
            }
        }
    }
}

fn generate_cylinder(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    let center_x = (shape.width as f32 - 1.0) / 2.0;
    let center_z = (shape.depth as f32 - 1.0) / 2.0;
    
    let radius_x = shape.width as f32 / 2.0;
    let radius_z = shape.depth as f32 / 2.0;
    
    for x in 0..shape.width {
        for y in 0..shape.height {
            for z in 0..shape.depth {
                let dx = (x as f32 - center_x) / radius_x;
                let dz = (z as f32 - center_z) / radius_z;
                
                if dx*dx + dz*dz <= 1.0 {
                    voxels.push((x as i32, y as i32, z as i32));
                }
            }
        }
    }
}

fn generate_cone(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    let center_x = (shape.width as f32 - 1.0) / 2.0;
    let center_z = (shape.depth as f32 - 1.0) / 2.0;
    
    let radius_x = shape.width as f32 / 2.0;
    let radius_z = shape.depth as f32 / 2.0;
    
    for x in 0..shape.width {
        for y in 0..shape.height {
            for z in 0..shape.depth {
                let dx = (x as f32 - center_x) / radius_x;
                let dz = (z as f32 - center_z) / radius_z;
                
                let height_factor = 1.0 - (y as f32 / shape.height as f32);
                
                if dx*dx + dz*dz <= height_factor * height_factor {
                    voxels.push((x as i32, y as i32, z as i32));
                }
            }
        }
    }
}

fn generate_square_pyramid(shape: &Shape, voxels: &mut Vec<(i32, i32, i32)>) {
    for x in 0..shape.width {
        for y in 0..shape.height {
            for z in 0..shape.depth {
                let height_factor = 1.0 - (y as f32 / shape.height as f32);
                let width_bound = (shape.width as f32 * height_factor / 2.0).ceil() as u32;
                let depth_bound = (shape.depth as f32 * height_factor / 2.0).ceil() as u32;
                
                let center_x = shape.width / 2;
                let center_z = shape.depth / 2;
                
                if x >= center_x.saturating_sub(width_bound) && x < center_x + width_bound &&
                   z >= center_z.saturating_sub(depth_bound) && z < center_z + depth_bound {
                    voxels.push((x as i32, y as i32, z as i32));
                }
            }
        }
    }
}
