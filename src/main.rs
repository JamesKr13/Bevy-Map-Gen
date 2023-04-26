use bevy::{pbr::CascadeShadowConfigBuilder,prelude::*,asset::*,winit::WinitSettings};
use bevy::render::mesh::{self, PrimitiveTopology,Indices};
use bevy_flycam::prelude::*;
use noise::{NoiseFn, Perlin, Seedable,OpenSimplex};
use rand::Rng;
use std::f32::consts::PI;
use bevy_egui::{EguiPlugin,egui,EguiContexts};
// use bevy::egui::{self,Ui};

const CHUNK_SIZE: f32 = 40.;
const TRIANGLE_ROOT_DENSITY: usize = 2000;
const FREQ_MAX: f32 = 100.;
const ALT_MAX: f32 = 25.;
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    MenuNotInUse,
    MenuInUse
}
enum Enum {
    First, Second, Third
}
#[derive(Resource, Default)]
struct NoiseOctaves {
    octaves_freq:Vec<f32>,
    octaves_alit:Vec<f32>,
    selections: Vec<usize>,
    power_ex: f32,
    
}
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_state::<GameState>()
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugin(EguiPlugin)
        .insert_resource(NoiseOctaves {octaves_freq: vec!(0.1), octaves_alit: vec!(0.1), selections: vec!(0), power_ex: 1.0})
        .insert_resource(ChunkPlane {mesh_vertices: Vec::new(),mesh_normals: Vec::new(), mesh_uvs: Vec::new(), mesh_indices: Vec::new(), height_map: Vec::new(),chunk_seeds: vec!(0)})
        .add_system(ui_system)
        // .add_system(await_key)
        .add_system(update_plane)
        .add_startup_system(setup)
        .run();
}
// fn change_state_menu(mut state: ResMut<State<GameState>>) {
//    state.set(GameState::MenuInUse);
// }
fn ui_system(mut noise_octaves: ResMut<NoiseOctaves>, mut contexts: EguiContexts, mut plane: ResMut<ChunkPlane>) {
    let mut rng = rand::thread_rng();
    egui::Window::new("Noise").show(contexts.ctx_mut(), |ui| {
        if ui.button("Add Noise").clicked() {
            plane.chunk_seeds.push(rng.gen_range(0..u32::MAX));
            noise_octaves.octaves_freq.push(0.1);
            noise_octaves.selections.push(0);
            noise_octaves.octaves_alit.push(0.1);
        }
        if ui.button("Remove Noise").clicked() {
            let len = noise_octaves.octaves_alit.len();
            plane.chunk_seeds.remove(len-1);
            noise_octaves.octaves_freq.remove(len-1);
            noise_octaves.octaves_alit.remove(len-1);
            noise_octaves.selections.remove(len-1);
        }
        ui.add(egui::Slider::new(&mut noise_octaves.power_ex, 0.0..=5.0));
        ui.label("Power");
        let noise_len = noise_octaves.selections.len();
        for index in 0..noise_len {
            ui.label(format!("Noise {}",index+1));
            ui.label("Frequency");
            ui.add(egui::Slider::new(&mut noise_octaves.octaves_freq[index],0.0..=FREQ_MAX));
            ui.label("Amplitude");
            ui.add(egui::Slider::new(&mut noise_octaves.octaves_alit[index],0.0..=ALT_MAX));
            egui::ComboBox::new(format!("Noise {}",index+1),"Select Ridgenoise")
            .selected_text(format!("{}", match noise_octaves.selections[index] {
                0 => "None".to_string(),
                _ => format!("Noise {}",noise_octaves.selections[index])
            }))
            .show_ui(
                ui,
                |ui| {
                    ui.selectable_value(&mut noise_octaves.selections[index],0, format!("None"));
                    if index != 0 {
                        for combo_index in 0..=index-1 {
                            ui.selectable_value(&mut noise_octaves.selections[index],combo_index+1, format!("Noise {}",combo_index+1));
                        }
                    }
                    
                }
            );
        }
        
    });
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk: ResMut<ChunkPlane>,
) {
    let texture_handle = asset_server.load("rock2.png");
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    chunk.create_plane();
    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk.create_mesh()),
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
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
#[derive(Resource, Default)]
struct ChunkPlane {
    mesh_vertices: Vec<[f32;3]>,
    mesh_normals: Vec<[f32;3]>,
    mesh_uvs: Vec<[f32;2]>,
    mesh_indices: Vec<u32>,
    height_map: Vec<Vec<f64>>,
    chunk_seeds: Vec<u32>,
}
impl ChunkPlane {
    fn create_plane(&mut self) {
        let width = CHUNK_SIZE;
        let height = CHUNK_SIZE;
        let triangle_width = width / TRIANGLE_ROOT_DENSITY as f32;
        let triangle_height = height / TRIANGLE_ROOT_DENSITY as f32;
        for y in 0..TRIANGLE_ROOT_DENSITY+1 {
            for x in 0..TRIANGLE_ROOT_DENSITY+1 {
                let x_pos = x as f32 * triangle_width - width / 2.0;
                let y_pos = y as f32 * triangle_height - height / 2.0;
                // let z_pos = self.height_map[y][x] as f32;
                // let mut square = (0,0);
                // if z_pos >= 3. {
                //     square.1 = 1;
                // }else if z_pos >= 1. {
                //     square.0 = 1;
                // } else if z_pos >= -1. {
                //     square.1 = 1;
                //     square.0 = 1;
                // }
                // let u = ((x&2-1)/2+square.0)  as f32;
                // let v = ((y&2-1)/2+square.1) as  f32;
                let u = ((x&2-1))  as f32;
                let v = ((y&2-1)) as  f32;
                self.mesh_vertices.push([x_pos, 0.0,y_pos]);
                self.mesh_normals.push([0.0, 0.0, 1.0]);
                self.mesh_uvs.push([u,v]);

                if (x > 0 && y > 0){
                    let current_index = (y * (TRIANGLE_ROOT_DENSITY+1) + x) as u32;
                    let prev_index = current_index - 1;
                    let top_index = current_index-(TRIANGLE_ROOT_DENSITY+1) as u32;
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
    fn create_mesh(&mut self) -> Mesh{
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.mesh_vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.mesh_normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.mesh_uvs.clone());
        mesh.set_indices(Some(Indices::U32(self.mesh_indices.clone())));
        println!("Mesh Returned");
        mesh
    }
}
fn update_plane(
    mut query: Query<(&Transform, &Handle<Mesh>)>,
    mut assets: ResMut<Assets<Mesh>>, 
    mut plane: ResMut<ChunkPlane>,
    noise_octaves: Res<NoiseOctaves>
) {
    let (transform, mut handle) = query.get_single_mut().expect("");
    let mut mesh = assets.get_mut(handle);
    let seed_number = plane.chunk_seeds.len();
    let mut temp_noise: Vec<OpenSimplex> = Vec::with_capacity(seed_number);
    for seed in &plane.chunk_seeds {
        temp_noise.push(OpenSimplex::new(*seed))
    }
    let test = OpenSimplex::new(0);
    if mesh.is_some() {
        let mesh_unwrapped = mesh.unwrap();
        let mut positions = &mesh_unwrapped.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
        let mut temporary = Vec::new();
        for vertex in positions.as_float3().expect("Mesh Not found: Update_Plane()") {
            let mut temp = [vertex[0],0.0,vertex[2]];
            let mut all_values: Vec<f32> = Vec::new();
            for noise_index in 0..temp_noise.len() {
                let mut freq = noise_octaves.octaves_freq[noise_index] as f32;
                let mut alt = noise_octaves.octaves_alit[noise_index] as f64;
                if freq == 0.0 {
                    freq = 0.001;
                }
                if alt == 0.0 {
                    alt = 0.001;
                }
                let ridgenoise_index: usize = noise_octaves.selections[noise_index];
                let value: f64 = alt*ridgenoise(temp_noise[noise_index].get([(temp[0]/freq) as f64,(temp[2]/freq) as f64]));
                all_values.push(value as f32);
                temp[1] += value as f32 * all_values[ridgenoise_index];
        
            }
            if temp[1] >= 0.0 {
                temp[1] = temp[1].powf(noise_octaves.power_ex);
            } else {
                temp[1] = temp[1]/100.;
            }
            temporary.push(temp);
        }
        mesh_unwrapped.insert_attribute(Mesh::ATTRIBUTE_POSITION, temporary);
    }
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
fn ridgenoise(noise: f64) -> f64 {
    2. * (0.5 - abs(0.5 - noise))
  }