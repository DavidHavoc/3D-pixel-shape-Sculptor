use std::fs::File;
use std::io::{self, Write};
use crate::shapes::VoxelData;

pub fn export_to_obj(voxel_data: &VoxelData) -> io::Result<()> {
    let mut file = File::create("exported_shape.obj")?;
    
    // Write OBJ header
    writeln!(file, "# 3D Voxel Sculptor Export")?;
    writeln!(file, "# Shape: {:?}", voxel_data.shape.shape_type)?;
    writeln!(file, "# Dimensions: {} x {} x {}", 
        voxel_data.shape.width, 
        voxel_data.shape.height, 
        voxel_data.shape.depth)?;
    
    let mut vertex_index = 1;
    
    for &(x, y, z) in &voxel_data.voxels {
        // Adjust coordinates to center the model
        let x_adj = x as f32 - (voxel_data.shape.width as f32 / 2.0);
        let y_adj = y as f32 - (voxel_data.shape.height as f32 / 2.0);
        let z_adj = z as f32 - (voxel_data.shape.depth as f32 / 2.0);
        
        // Each voxel is a 1x1x1 cube
        // Define 8 vertices of the cube
        writeln!(file, "v {} {} {}", x_adj, y_adj, z_adj)?;
        writeln!(file, "v {} {} {}", x_adj + 1.0, y_adj, z_adj)?;
        writeln!(file, "v {} {} {}", x_adj + 1.0, y_adj, z_adj + 1.0)?;
        writeln!(file, "v {} {} {}", x_adj, y_adj, z_adj + 1.0)?;
        writeln!(file, "v {} {} {}", x_adj, y_adj + 1.0, z_adj)?;
        writeln!(file, "v {} {} {}", x_adj + 1.0, y_adj + 1.0, z_adj)?;
        writeln!(file, "v {} {} {}", x_adj + 1.0, y_adj + 1.0, z_adj + 1.0)?;
        writeln!(file, "v {} {} {}", x_adj, y_adj + 1.0, z_adj + 1.0)?;
        
        // Define the 6 faces of the cube (12 triangles)
        // Bottom face
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 1, vertex_index + 2)?;
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 2, vertex_index + 3)?;
        
        // Top face
        writeln!(file, "f {} {} {}", vertex_index + 4, vertex_index + 7, vertex_index + 6)?;
        writeln!(file, "f {} {} {}", vertex_index + 4, vertex_index + 6, vertex_index + 5)?;
        
        // Front face
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 4, vertex_index + 5)?;
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 5, vertex_index + 1)?;
        
        // Back face
        writeln!(file, "f {} {} {}", vertex_index + 3, vertex_index + 2, vertex_index + 6)?;
        writeln!(file, "f {} {} {}", vertex_index + 3, vertex_index + 6, vertex_index + 7)?;
        
        // Left face
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 3, vertex_index + 7)?;
        writeln!(file, "f {} {} {}", vertex_index, vertex_index + 7, vertex_index + 4)?;
        
        // Right face
        writeln!(file, "f {} {} {}", vertex_index + 1, vertex_index + 5, vertex_index + 6)?;
        writeln!(file, "f {} {} {}", vertex_index + 1, vertex_index + 6, vertex_index + 2)?;
        
        vertex_index += 8;
    }
    
    Ok(())
}
