use bevy::{
    asset::LoadState,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureViewDescriptor, TextureViewDimension,
    },
};

/// Usage
///
/// 1: Add the `CustomSkyboxPlugin` to the app.
///
/// 2: Query the `CubemapFactory` resource and call `load_from_folder` on it.
///
///
pub struct CubemapFactoryPlugin;

impl Plugin for CubemapFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CubemapFactory::default())
            .add_systems(Update, create_cubemap.run_if(factory_non_empty));
    }
}

fn factory_non_empty(factory: Res<CubemapFactory>) -> bool {
    !factory.0.is_empty()
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
    pub fn load(path: impl Into<String>, asset_server: Res<AssetServer>) -> Self {
        let path = path.into();

        Self {
            px: asset_server.load(format!("{path}/px.png")),
            nx: asset_server.load(format!("{path}/nx.png")),
            py: asset_server.load(format!("{path}/py.png")),
            ny: asset_server.load(format!("{path}/ny.png")),
            pz: asset_server.load(format!("{path}/pz.png")),
            nz: asset_server.load(format!("{path}/nz.png")),
        }
    }

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

#[derive(Resource, Default)]
pub struct CubemapFactory(Vec<(Handle<Image>, CubemapHandle)>);

impl CubemapFactory {
    pub fn create_from_cubemap_handle(
        &mut self,
        cubemap_handle: CubemapHandle,
        images: Res<Assets<Image>>,
    ) -> Handle<Image> {
        let handle = images.reserve_handle();
        self.0.push((handle.clone(), cubemap_handle));

        return handle;
    }

    pub fn load_from_folder(
        &mut self,
        path: impl Into<String>,
        asset_server: Res<AssetServer>,
        images: Res<Assets<Image>>,
    ) -> Handle<Image> {
        let cubemap_handle = CubemapHandle::load(path, asset_server);
        return self.create_from_cubemap_handle(cubemap_handle, images);
    }
}

fn create_cubemap(
    mut cubemap_factory: ResMut<CubemapFactory>,
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut to_be_removed: Vec<usize> = Vec::new();

    'blk: for (i, (result_handle, cubemap_handle)) in cubemap_factory.0.iter().enumerate() {
        let handles = cubemap_handle.as_vec();

        for handle in handles.iter() {
            if assets.get_load_state(handle).unwrap() != LoadState::Loaded {
                continue 'blk;
            }
        }

        let textures: Vec<Image> = handles
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

        images.insert(result_handle, image);

        to_be_removed.push(i);
    }

    for i in to_be_removed {
        cubemap_factory.0.remove(i);
    }
}
