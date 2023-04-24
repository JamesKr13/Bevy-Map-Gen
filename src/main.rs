use bevy::{pbr::CascadeShadowConfigBuilder,prelude::*,asset::*};
use bevy::render::mesh::{self, PrimitiveTopology,Indices};
use bevy_flycam::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use std::f32::consts::PI;


const CHUNK_SIZE: f32 = 10.;
const TRIANGLE_DENSITY: usize = 8000;


fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    x_pos: f64,
    y_pos: f64,
    z_pos: f64,
    move_cooldown: Timer,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("terrain.png");
    let height_map = generate_height_map(TRIANGLE_DENSITY+1);
    let mut chunk = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new(), height_map: height_map.clone()};
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    chunk.create_plane();
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk.create_mesh()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            ..default()
        }),
        ..default()
    });
    let mut chunk2 = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new(), height_map: height_map.clone()};
    let mut mesh2 = Mesh::new(PrimitiveTopology::TriangleList);
    chunk2.create_plane();
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk2.create_mesh()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            ..default()
        }),
        ..default()
    });
    let mut chunk3 = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new(), height_map: height_map.clone()};
    let mut mesh3 = Mesh::new(PrimitiveTopology::TriangleList);
    chunk3.create_plane();
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk3.create_mesh()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            ..default()
        }),
        ..default()
    });
    let mut chunk3 = ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new(), height_map: height_map.clone()};
    let mut mesh3 = Mesh::new(PrimitiveTopology::TriangleList);
    chunk2.create_plane();
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk3.create_mesh()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        }),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 1.0, 10.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
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
    height_map: Vec<Vec<f64>>
}
impl ChunkPlane {
    fn create_plane(&mut self) {
        let triangle_density = TRIANGLE_DENSITY+1;
        let width = CHUNK_SIZE;
        let height = CHUNK_SIZE;
        let triangle_width = width / triangle_density as f32;
        let triangle_height = height / triangle_density as f32;
        for y in 0..triangle_density {
            for x in 0..triangle_density {
                let x_pos = x as f32 * triangle_width - width / 2.0;
                let y_pos = y as f32 * triangle_height - height / 2.0;
                let z_pos = self.height_map[y][x] as f32;
                let mut square = (0,0);
                if z_pos >= 3. {
                    square.1 = 1;
                }else if z_pos >= 1. {
                    square.0 = 1;
                } else if z_pos >= -1. {
                    square.1 = 1;
                    square.0 = 1;
                }
                let u = ((x&2-1)/2+square.0)  as f32;
                let v = ((y&2-1)/2+square.1) as  f32;
                // let u = ((x&2-1))  as f32;
                // let v = ((y&2-1)) as  f32;
                self.mesh_vertices.push([x_pos, z_pos,y_pos]);
                self.mesh_normals.push([0.0, 0.0, 1.0]);
                self.mesh_uvs.push([u,v]);

                if (x > 0 && y > 0){
                    let current_index = (y * (triangle_density) + x) as u32;
                    let prev_index = current_index - 1;
                    let top_index = current_index-triangle_density as u32;
                    let top_right_index = top_index-1;
                    self.mesh_indices.push(prev_index);
                    self.mesh_indices.push(current_index);
                    self.mesh_indices.push(top_right_index);
                    self.mesh_indices.push(current_index);
                    self.mesh_indices.push(top_index);
                    self.mesh_indices.push(top_right_index);
                }
            }
        }
        println!("Completed");
    }
    fn create_plane2(&mut self) {
        let triangle_density = TRIANGLE_DENSITY+1;
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
                self.mesh_vertices.push([x_pos,self.height_map[y][x] as f32,y_pos]);
                self.mesh_normals.push([0.0, 0.0, 1.0]);
                self.mesh_uvs.push([u,v]);
                if (x > 0 && y > 0){
                    let current_index = (y * (triangle_density) + x) as u32;
                    let prev_index = current_index - 1;
                    let top_index = current_index-triangle_density as u32;
                    let top_right_index = top_index-1;
    
                    self.mesh_indices.push(current_index);
                    self.mesh_indices.push(top_index);
                    self.mesh_indices.push(top_right_index);
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
    let freq_scale = 20.0;
    let amp_scale: f64 = 5.;

    let freq1 = 0.05;
    let freq2 = 0.1;
    let freq3 = 0.3;
    let freq4 = 0.5;
    let freq5 = 1.7;
    let freq6 = 3.0;
    let freq7 = 8.5;
    let freq8 = 7.0;
    let freq9 = 3.0;
    let freq10 = 4.0;

    let amp1 = 0.1 * amp_scale;
    let amp2 = 0.2 * amp_scale;
    let amp3 = 0.5 * amp_scale;
    let amp4 = 0.8 * amp_scale;
    let amp5 = 0.98 * amp_scale;
    let amp6 = 1.2 * amp_scale;
    let amp7 = 1.3 * amp_scale;
    let amp8 = 1.8 * amp_scale;
    let amp9 = 2.0 * amp_scale;
    let amp10 = 2.4 * amp_scale;
    let total_amp: f64 =  ((amp1+amp2+amp3+amp4+amp5+amp6+amp7+amp8+amp9+amp10) as f64);


    let mut seed = rng.gen_range(0..u32::MAX);
    let octave3 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave4 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave5 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave6 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave7 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave8 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave9 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave10 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave2 = Perlin::new(seed);
    let mut seed = rng.gen_range(0..u32::MAX);
    let octave1 = Perlin::new(seed);

    let noise_scale = 20.0;
    for x in 0..n {
        
        let mut row = Vec::with_capacity(n);

        for y in 0..n {
            let octave10_val = 2.*(0.5-abs(0.5-amp10 * octave10.get([
                x as f64 / (freq10 * freq_scale),
                y as f64 / (freq10 * freq_scale),
            ])));
            
            let octave9_val = 2.*(0.5-abs(0.5-amp9 * octave9.get([
                x as f64 / (freq9 * freq_scale),
                y as f64 / (freq9 * freq_scale),
            ])))*octave10_val;
            
            // let octave8_val = 2.*(0.5-abs(0.5-amp8 * octave8.get([
            //     x as f64 / (freq8 * freq_scale),
            //     y as f64 / (freq8 * freq_scale),
            // ])))* (octave10_val+octave9_val);
            
            // let octave7_val = 2.*(0.5-abs(0.5-amp7 * octave7.get([
            //     x as f64 / (freq7 * freq_scale),
            //     y as f64 / (freq7 * freq_scale),
            // ]))) * (octave10_val+octave9_val+octave8_val);
            
            // let octave6_val = 2.*(0.5-abs(0.5-amp6 * octave6.get([
            //     x as f64 / (freq6 * freq_scale),
            //     y as f64 / (freq6 * freq_scale),
            // ]))) * (octave10_val+octave9_val+octave8_val+octave7_val);
            
            // let octave5_val = 2.*(0.5-abs(0.5-amp5 * octave5.get([
            //     x as f64 / (freq5 * freq_scale),
            //     y as f64 / (freq5 * freq_scale),
            // ])))*(octave10_val+octave9_val+octave8_val+octave7_val+octave6_val);
            
            // let octave4_val = 2.*(0.5-abs(0.5-amp4 * octave4.get([
            //     x as f64 / (freq4 * freq_scale),
            //     y as f64 / (freq4 * freq_scale),
            // ])))*(octave10_val+octave9_val+octave8_val+octave7_val+octave6_val+octave5_val);
            
            // let octave3_val = 2.*(0.5-abs(0.5-amp3 * octave3.get([
            //     x as f64 / (freq3 * freq_scale),
            //     y as f64 / (freq3 * freq_scale),
            // ])))*(octave10_val+octave9_val+octave8_val+octave7_val+octave6_val+octave5_val+octave4_val);
            
            // let octave2_val = 2.*(0.5-abs(0.5-amp2 * octave2.get([
            //     x as f64 / (freq2 * freq_scale),
            //     y as f64 / (freq2 * freq_scale),
            // ])))*(octave10_val+octave9_val+octave8_val+octave7_val+octave6_val+octave5_val+octave4_val+octave3_val);
            
            // let octave1_val = 2.*(0.5-abs(0.5-amp1 * octave1.get([
            //     x as f64 / (freq1 * freq_scale),
            //     y as f64 / (freq1 * freq_scale),
            // ])))*(octave10_val+octave9_val+octave8_val+octave7_val+octave6_val+octave5_val+octave4_val+octave3_val+octave2_val);
            
            let height_map_val = ( 
                            //   octave6_val 
                            //   + octave7_val 
                            //   + octave8_val 
                              octave9_val 
                              + octave10_val);
            row.push(height_map_val/total_amp);
        }

        height_map.push(row);
    }
    let kernel_size = 3;

    height_map
}
fn min(one: f32, two: f32) -> f32 {
    match one < two {
        true => one,
        _ => two,
    }
}
fn abs(value: f64) -> f64{
    value.powf(2.).sqrt()   
}