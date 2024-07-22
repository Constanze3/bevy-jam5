use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureViewDescriptor, TextureViewDimension,
    },
};

/// Usage
///
/// 1. Add the `CustomSkyboxPlugin` to the app.
///
/// 2. Load skyboxes with `load_cubemap` from a specified folder.
/// Each side of the cube is a separate image named px, nx, py, ny, pz, nz.
///
/// 3. Assign the CustomSkybox component to cameras specifying a brightness
/// and the cubemap to use.
///
/// 4. Run the `apply_custom_skyboxes` system after the components have been assigned.
///
/// 5. Yay, skybox!
///
pub struct CustomSkyboxPlugin;

impl Plugin for CustomSkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            create_skyboxes.run_if(resource_exists::<SkyboxTextures>),
        );
    }
}

#[derive(Component)]
pub struct CustomSkybox {
    pub cubemap: CubemapHandle,
    pub brightness: f32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct CubemapHandle {
    px: Handle<Image>,
    nx: Handle<Image>,
    py: Handle<Image>,
    ny: Handle<Image>,
    pz: Handle<Image>,
    nz: Handle<Image>,
}

impl CubemapHandle {
    fn as_vec(&self) -> Vec<Handle<Image>> {
        return vec![
            self.px.clone(),
            self.nx.clone(),
            self.py.clone(),
            self.ny.clone(),
            self.pz.clone(),
            self.nz.clone(),
        ];
    }
}

pub fn load_cubemap(path: impl Into<String>, asset_server: Res<AssetServer>) -> CubemapHandle {
    let path = path.into();

    CubemapHandle {
        px: asset_server.load(format!("{path}/px.png")),
        nx: asset_server.load(format!("{path}/nx.png")),
        py: asset_server.load(format!("{path}/py.png")),
        ny: asset_server.load(format!("{path}/ny.png")),
        pz: asset_server.load(format!("{path}/pz.png")),
        nz: asset_server.load(format!("{path}/nz.png")),
    }
}

// not map because I assume there aren't many skybox targets
#[derive(Resource)]
pub struct SkyboxTextures(Vec<SkyboxTexture>);

pub struct SkyboxTexture {
    cubemap: CubemapHandle,
    targets: Vec<SkyboxTextureTarget>,
}

pub struct SkyboxTextureTarget {
    entity: Entity,
    brightness: f32,
}

pub fn apply_custom_skyboxes(
    custom_skyboxes: Query<(Entity, &CustomSkybox)>,
    mut commands: Commands,
    skybox_textures: Option<ResMut<SkyboxTextures>>,
) {
    let mut textures: Vec<SkyboxTexture> = Vec::new();

    'blk: for (entity, custom_skybox) in custom_skyboxes.iter() {
        let target = SkyboxTextureTarget {
            entity,
            brightness: custom_skybox.brightness,
        };

        for texture in textures.iter_mut() {
            if texture.cubemap == custom_skybox.cubemap {
                texture.targets.push(target);
                continue 'blk;
            }
        }

        textures.push(SkyboxTexture {
            cubemap: custom_skybox.cubemap.clone(),
            targets: vec![target],
        });

        commands.entity(entity).remove::<CustomSkybox>();
    }

    if let Some(mut skybox_textures_resource) = skybox_textures {
        skybox_textures_resource.0.extend(textures);
    } else {
        commands.insert_resource(SkyboxTextures(textures));
    }
}

fn create_skyboxes(
    assets: Res<AssetServer>,
    mut skybox_textures: ResMut<SkyboxTextures>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if skybox_textures.0.len() == 0 {
        commands.remove_resource::<SkyboxTextures>();
        return;
    }

    let mut to_be_removed: Vec<usize> = Vec::new();

    'blk: for (i, skybox_texture) in skybox_textures.0.iter_mut().enumerate() {
        for handle in skybox_texture.cubemap.as_vec() {
            if assets.get_load_state(&handle).unwrap() != LoadState::Loaded {
                continue 'blk;
            }
        }

        let textures: Vec<Image> = skybox_texture
            .cubemap
            .as_vec()
            .into_iter()
            .map(|x| images.get(&x).unwrap().clone())
            .collect();

        let width = textures[0].width();
        let height = textures[0].height();
        let array_layers = 6;

        let texture_descriptor = textures[0].texture_descriptor.clone();

        let mut data: Vec<u8> = vec![0; (width * height * 4 * array_layers) as usize];
        for (i, image) in textures.into_iter().enumerate() {
            let offset = (i as u32 * width * height * 4) as usize;

            let start = offset;
            let end = offset + image.data.len();

            data[start..end].copy_from_slice(&image.data);
        }

        let image = Image {
            data,
            texture_descriptor: TextureDescriptor {
                size: Extent3d {
                    width,
                    height,
                    depth_or_array_layers: array_layers,
                },
                ..texture_descriptor
            },
            texture_view_descriptor: Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            }),
            ..default()
        };

        let cubemap_handle = images.add(image);

        for target in skybox_texture.targets.iter() {
            let mut commands = commands.entity(target.entity);

            commands.insert(Skybox {
                image: cubemap_handle.clone(),
                brightness: target.brightness,
            });
        }

        to_be_removed.push(i);
    }

    for i in to_be_removed {
        skybox_textures.0.remove(i);
    }
}
