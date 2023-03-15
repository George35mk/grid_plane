use bevy::{
    prelude::*, 
    render::{mesh, render_resource::PrimitiveTopology}
};

#[derive(Debug, Copy, Clone)]
pub enum GridAxis { Xy, Yz, Zx }

pub struct GridPlanePlugin {
    /// the grid axis
    pub grid_axis: GridAxis,

    /// the grid size
    pub size: i32,

    /// the grid spacing
    pub spacing: f32,

    /// the grid x axis colors
    pub x_axis_color: Color,

    /// the grid y axis colors
    pub y_axis_color: Color,

    /// the grid z axis colors
    pub z_axis_color: Color,

    /// the grid minor line color. For example every 1 unit
    pub minor_line_color: Color,

    // the major line color. For example every 10 units
    pub major_line_color: Color,
}

impl Plugin for GridPlanePlugin {
    fn build(&self, app: &mut App) {
        let state = GridOptions {
            grid_axis: self.grid_axis,
            size: self.size,
            spacing: self.spacing,
            x_axis_color: self.x_axis_color,
            y_axis_color: self.y_axis_color,
            z_axis_color: self.z_axis_color,
            minor_line_color: self.minor_line_color,
            major_line_color: self.major_line_color,
        };

        app.insert_resource(state).add_startup_system(setup);
    }
}

#[derive(Resource)]
struct GridOptions {
    grid_axis: GridAxis,
    size: i32, 
    spacing: f32,
    x_axis_color: Color,
    y_axis_color: Color,
    z_axis_color: Color,
    minor_line_color: Color,
    major_line_color: Color,
}

impl Default for GridPlanePlugin {
    fn default() -> GridPlanePlugin {
        GridPlanePlugin {
            grid_axis:  GridAxis::Zx,
            size: 100,
            spacing: 1.,
            x_axis_color: Color::hsla(0.0, 1.0, 0.45, 1.0),
            y_axis_color: Color::hsla(137.0, 1.0, 0.45, 1.0),
            z_axis_color: Color::hsla(213.0, 1.0, 0.45, 1.0),
            minor_line_color: Color::Rgba { red: 0.05, green: 0.05, blue: 0.05, alpha: 1.0 },
            major_line_color: Color::Rgba { red: 0.2, green: 0.2, blue: 0.2, alpha: 1.0 },
        }
    }
}

fn setup(
    state: ResMut<GridOptions>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) { 
    let grid_mesh = create_grid_mesh(
        state.size, 
        state.spacing, 
        state.grid_axis, 
        state.x_axis_color,
        state.y_axis_color,
        state.z_axis_color,
        state.minor_line_color,
        state.major_line_color,
    );
    
    let line_material = materials.add(StandardMaterial {
            unlit: true,
            alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(grid_mesh),
        material: line_material.clone(),
        transform: Transform { 
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation:Quat::default(),
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 }
        },
        ..default()
    });
}

fn create_grid_mesh(
    size: i32, spacing: f32, grid_axis: GridAxis, 
    x_color: Color, y_color: Color, z_color: Color, 
    minor_line_color: Color, major_line_color: Color,
) -> Mesh {
    
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);

    // Attributes
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Draw the horizontal grid lines
    for y in 0..=size {

        let (start, end) = compute_y(y, spacing, size, grid_axis);

        // line color on the axis
        let mut color = match grid_axis {
            GridAxis::Xy => if y == size / 2 { x_color } else { minor_line_color },
            GridAxis::Yz => if y == size / 2 { z_color } else { minor_line_color },
            GridAxis::Zx => if y == size / 2 { x_color } else { minor_line_color },
        };

        // Line color every 10 units
        if y != size / 2 && y % 10 == 0 { color = major_line_color; }
        
        vertices.push(start.into());
        vertices.push(end.into());
        indices.push(indices.len() as u32);
        indices.push(indices.len() as u32);
        colors.push(color.into());        
        colors.push(color.into());
    }

    // Draw the vertical grid lines
    for x in 0..=size {

        let (start, end) = compute_x(x, spacing, size, grid_axis);

        // line color on the axis
        let mut color = match grid_axis {
            GridAxis::Xy => if x == size / 2 { y_color } else { minor_line_color },
            GridAxis::Yz => if x == size / 2 { y_color } else { minor_line_color },
            GridAxis::Zx => if x == size / 2 { z_color } else { minor_line_color },
        };

        // Line color every 10 units
        if x != size / 2 && x % 10 == 0 { color = major_line_color; }

        vertices.push(start.into());
        vertices.push(end.into());
        indices.push(indices.len() as u32);
        indices.push(indices.len() as u32);
        colors.push(color.into());        
        colors.push(color.into());
    }

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; n]);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; n]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    mesh
}

/// Computes the horizontal grid line start and end vectors.
fn compute_y(y: i32, spacing: f32, size: i32, axis: GridAxis) -> (Vec3, Vec3) {

    let y_pos: f32;
    let start: Vec3;
    let end: Vec3;

    match axis {
        GridAxis::Xy => {
            y_pos = (y as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(-size as f32 * spacing * 0.5, y_pos, 0.0);
            end = Vec3::new(size as f32 * spacing * 0.5, y_pos, 0.0);

            (start, end)
        }

        GridAxis::Yz => {
            y_pos = (y as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(0.0, y_pos, -size as f32 * spacing * 0.5);
            end = Vec3::new(0.0, y_pos, size as f32 * spacing * 0.5);
            
            (start, end)
        }

        GridAxis::Zx => {
            y_pos = (y as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(-size as f32 * spacing * 0.5, 0.0, y_pos);
            end = Vec3::new(size as f32 * spacing * 0.5, 0.0, y_pos);

            (start, end)
        }
    }

}


/// Computes the vertical grid line start and end vectors.
fn compute_x(x: i32, spacing: f32, size: i32, axis: GridAxis) -> (Vec3, Vec3) {

    let x_pos: f32;
    let start: Vec3;
    let end: Vec3;

    match axis {
        GridAxis::Xy => {
            x_pos = (x as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(x_pos, -size as f32 * spacing * 0.5, 0.0);
            end = Vec3::new(x_pos, size as f32 * spacing * 0.5, 0.0);

            (start, end)
        }

        GridAxis::Yz => {
            x_pos = (x as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(0.0, -size as f32 * spacing * 0.5, x_pos);
            end = Vec3::new(0.0, size as f32 * spacing * 0.5, x_pos);

            (start, end)
        }

        GridAxis::Zx => {
            x_pos = (x as f32 * spacing) - (size as f32 * 0.5 * spacing);
            start = Vec3::new(x_pos, 0.0, -size as f32 * spacing * 0.5);
            end = Vec3::new(x_pos, 0.0, size as f32 * spacing * 0.5);

            (start, end)
        }
    }

}
