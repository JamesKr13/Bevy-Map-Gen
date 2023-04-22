use bevy::{pbr::CascadeShadowConfigBuilder,prelude::*};
use bevy::render::mesh::{self, PrimitiveTopology,Indices};
use bevy_flycam::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use std::f32::consts::PI;

const CHUNK_SIZE: f32 = 500.;
const TRIANGLE_DENSITY: usize = 5000;
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}
use bevy::prelude::*;

use bevy::prelude::*;

fn create_rect_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let positions = vec![
        [-half_width, -half_height, 0.0],
        [half_width, -half_height, 0.0],
        [half_width, half_height, 0.0],
        [-half_width, half_height, 0.0],
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 4];

    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new()};
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    chunk.create_plane();
    mesh = chunk.create_mesh();

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0., 0., 255.).into()),
        ..default()
    });
    let mut chunk2 = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new()};
    let mut mesh2 = Mesh::new(PrimitiveTopology::TriangleList);
    chunk2.create_plane2();
    mesh2 = chunk2.create_mesh();

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh2),
        material: materials.add(Color::rgb(255., 165., 0.).into()),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(40.0, 1.0, 40.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
struct ChunkPlane {
    mesh_vertices: Vec<[f32;3]>,
    mesh_normals: Vec<[f32;3]>,
    mesh_uvs: Vec<[f32;2]>,
    mesh_indices: Vec<u32>,
}
impl ChunkPlane {
    fn create_plane(&mut self) {
        let triangle_density = TRIANGLE_DENSITY;
        let height_map = generate_height_map(triangle_density);
        let width = CHUNK_SIZE;
        let height = CHUNK_SIZE;
        let triangle_width = width / triangle_density as f32;
        let triangle_height = height / triangle_density as f32;
        for y in 0..triangle_density {
            println!("1");
            for x in 0..triangle_density {
                let x_pos = x as f32 * triangle_width - width / 2.0;
                let y_pos = y as f32 * triangle_height - height / 2.0;
    
                let u = x as f32 / triangle_density as f32;
                let v = y as f32 / triangle_density as f32;
                self.mesh_vertices.push([x_pos,height_map[y][x] as f32,y_pos]);
                self.mesh_normals.push([0.0, 0.0, 1.0]);
                self.mesh_uvs.push([u,v]);

                if (x > 0 && y > 0) && !(x == triangle_density || y == triangle_density-1 ){
                    let current_index = (y * (triangle_density + 1) + x) as u32;
                    let prev_index = current_index - 1;
                    let top_index = prev_index - (triangle_density + 1) as u32;
                    let top_right_index = current_index - (triangle_density + 1) as u32;
    
                    // self.mesh_indices.push(top_right_index);
                    // self.mesh_indices.push(top_index);
                    // self.mesh_indices.push(prev_index);
    
                    self.mesh_indices.push(prev_index);
                    self.mesh_indices.push(current_index);
                    self.mesh_indices.push(top_right_index);
                }
            }
        }
        println!("Completed");
    }
    fn create_plane2(&mut self) {
        let triangle_density = TRIANGLE_DENSITY;
        let height_map = generate_height_map(triangle_density);
        let width = CHUNK_SIZE;
        let height = CHUNK_SIZE;
        let triangle_width = width / triangle_density as f32;
        let triangle_height = height / triangle_density as f32;
        for y in 0..triangle_density {
            for x in 0..triangle_density {
                let x_pos = x as f32 * triangle_width - width / 2.0;
                let y_pos = y as f32 * triangle_height - height / 2.0;
    
                let u = x as f32 / triangle_density as f32;
                let v = y as f32 / triangle_density as f32;
                self.mesh_vertices.push([x_pos,height_map[y][x] as f32,y_pos]);
                self.mesh_normals.push([0.0, 0.0, 1.0]);
                self.mesh_uvs.push([u,v]);
                if (x > 0 && y > 0) && !(x == triangle_density || y == triangle_density-1 ){
                    let current_index = (y * (triangle_density + 1) + x) as u32;
                    let prev_index = current_index - 1;
                    let top_index = prev_index - (triangle_density + 1) as u32;
                    let top_right_index = current_index - (triangle_density + 1) as u32;
    
                    self.mesh_indices.push(top_right_index);
                    self.mesh_indices.push(top_index);
                    self.mesh_indices.push(prev_index);

                    // self.mesh_indices.push(prev_index);
                    // self.mesh_indices.push(top_right_index);
                    // self.mesh_indices.push(top_index);
    
                    // self.mesh_indices.push(prev_index);
                    // self.mesh_indices.push(current_index);
                    // self.mesh_indices.push(top_right_index);
                    if x == triangle_density-1 || y == triangle_density-1 {
                        println!{"{},{},{},{}",current_index,prev_index,top_index,top_right_index};
                    } 
                }
            }
        }
        println!("Completed");
    }
    fn create_mesh(&mut self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.mesh_vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.mesh_normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.mesh_uvs.clone());
        mesh.set_indices(Some(Indices::U32(self.mesh_indices.clone())));
        println!("Mesh Returned");
        mesh
    }
}

fn generate_height_map(n: usize) -> Vec<Vec<f64>> {
    let mut height_map = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    let mut seed = rng.gen_range(0..u32::MAX);
    let noise = Perlin::new(1);

    for x in 0..n {
        let mut row = Vec::with_capacity(n);

        for y in 0..n {
            let value = noise.get([x as f64 / 10.0, y as f64 / 10.0]);
            row.push(value);
        }

        height_map.push(row);
    }

    height_map
}
fn min(one: f32, two: f32) -> f32 {
    match one < two {
        true => one,
        _ => two,
    }
}