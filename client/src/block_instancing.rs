use crate::material::block_atlas_material::BlockAtlasMaterial;
use bevy::core::{Pod, Zeroable};
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::system::lifetimeless::*;
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    extract_materials, MaterialProperties, MeshPipeline, MeshPipelineKey, MeshUniform,
    PreparedMaterial, RenderMaterials, SetMaterialBindGroup, SetMeshBindGroup,
    SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::mesh::{GpuBufferInfo, MeshVertexBufferLayout};
use bevy::render::render_asset::{PrepareAssetSet, RenderAssets};
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult, RenderPhase,
    SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::{
    AsBindGroupError, BindGroupLayout, Buffer, BufferInitDescriptor, BufferUsages, PipelineCache,
    RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError,
    SpecializedMeshPipelines, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderSet};
use std::marker::PhantomData;

#[derive(Clone, Component, Deref, ExtractComponent)]
pub struct BlockQuadInstanceMaterialData(pub Vec<BlockQuadInstanceData>);

pub struct BlockQuadMaterialPlugin;

impl Plugin for BlockQuadMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<BlockQuadInstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom<BlockAtlasMaterial>>()
            .init_resource::<BlockQuadPipeline<BlockAtlasMaterial>>()
            .init_resource::<ExtractedMaterials<BlockAtlasMaterial>>()
            .init_resource::<RenderMaterials<BlockAtlasMaterial>>()
            .init_resource::<SpecializedMeshPipelines<BlockQuadPipeline<BlockAtlasMaterial>>>()
            // .init_resource::<MaterialPipeline<BlockAtlasMaterial>>()
            .add_system(extract_materials::<BlockAtlasMaterial>.in_schedule(ExtractSchedule))
            .add_system(
                prepare_materials::<BlockAtlasMaterial>
                    .in_set(RenderSet::Prepare)
                    .after(PrepareAssetSet::PreAssetPrepare),
            )
            .add_system(prepare_instance_buffers.in_set(RenderSet::Prepare))
            .add_system(queue_custom::<BlockAtlasMaterial>.in_set(RenderSet::Queue));
    }
}

#[derive(Resource)]
pub struct ExtractedMaterials<M: Material> {
    extracted: Vec<(Handle<M>, M)>,
    removed: Vec<Handle<M>>,
}

impl<M: Material> Default for ExtractedMaterials<M> {
    fn default() -> Self {
        Self {
            extracted: Default::default(),
            removed: Default::default(),
        }
    }
}

pub struct PrepareNextFrameMaterials<M: Material> {
    assets: Vec<(Handle<M>, M)>,
}

impl<M: Material> Default for PrepareNextFrameMaterials<M> {
    fn default() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

pub fn prepare_materials<M: Material>(
    mut prepare_next_frame: Local<PrepareNextFrameMaterials<M>>,
    mut extracted_assets: ResMut<ExtractedMaterials<M>>,
    mut render_materials: ResMut<RenderMaterials<M>>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
    pipeline: Res<BlockQuadPipeline<M>>,
) {
    let queued_assets = std::mem::take(&mut prepare_next_frame.assets);
    for (handle, material) in queued_assets.into_iter() {
        match prepare_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }

    for removed in std::mem::take(&mut extracted_assets.removed) {
        render_materials.remove(&removed);
    }

    for (handle, material) in std::mem::take(&mut extracted_assets.extracted) {
        match prepare_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }
}

fn prepare_material<M: Material>(
    material: &M,
    render_device: &RenderDevice,
    images: &RenderAssets<Image>,
    fallback_image: &FallbackImage,
    pipeline: &BlockQuadPipeline<M>,
) -> Result<PreparedMaterial<M>, AsBindGroupError> {
    let prepared = material.as_bind_group(
        &pipeline.material_layout,
        render_device,
        images,
        fallback_image,
    )?;
    Ok(PreparedMaterial {
        bindings: prepared.bindings,
        bind_group: prepared.bind_group,
        key: prepared.data,
        properties: MaterialProperties {
            alpha_mode: material.alpha_mode(),
            depth_bias: material.depth_bias(),
        },
    })
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct BlockQuadInstanceData {
    pub position: Vec3,
    pub block_side: u32,
    pub atlas_section_from: Vec2,
    pub atlas_section_to: Vec2,
}

#[allow(clippy::too_many_arguments)]
fn queue_custom<M: Material>(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<BlockQuadPipeline<M>>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<BlockQuadPipeline<M>>>,
    pipeline_cache: ResMut<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    material_meshes: Query<
        (Entity, &MeshUniform, &Handle<Mesh>),
        With<BlockQuadInstanceMaterialData>,
    >,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom<M>>();

    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut transparent_phase) in &mut views {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();

        for (entity, mesh_uniform, mesh_handle) in &material_meshes {
            let Some(mesh) = meshes.get(mesh_handle) else {
                continue;
            };

            let key = view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();

            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: rangefinder.distance(&mesh_uniform.transform),
            });
        }
    }
}

#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &BlockQuadInstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(instance_data.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instance_data.len(),
        });
    }
}

#[derive(Resource)]
pub struct BlockQuadPipeline<M: Material> {
    marker: PhantomData<M>,
    material_layout: BindGroupLayout,
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

impl<M: Material> FromWorld for BlockQuadPipeline<M> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let render_device = world.resource::<RenderDevice>();
        let mesh_pipeline = world.resource::<MeshPipeline>().clone();

        let shader = asset_server.load("shaders/block-instanced.wgsl");

        Self {
            marker: PhantomData,
            material_layout: M::bind_group_layout(render_device),
            mesh_pipeline,
            shader,
        }
    }
}

impl<M: Material> SpecializedMeshPipeline for BlockQuadPipeline<M> {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<BlockQuadInstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Uint32,
                    offset: VertexFormat::Float32x3.size(),
                    shader_location: 4,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 5,
                },
            ],
        });

        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();

        // descriptor.layout.insert(2, self.material_layout.clone());
        descriptor.layout = vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.material_layout.clone(),
        ];

        Ok(descriptor)
    }
}

type DrawCustom<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetMaterialBindGroup<M, 2>,
    DrawMeshInstanced,
);

pub struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = SRes<RenderAssets<Mesh>>;
    type ViewWorldQuery = ();
    type ItemWorldQuery = (Read<Handle<Mesh>>, Read<InstanceBuffer>);

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        (mesh_handle, instance_buffer): (&'w Handle<Mesh>, &'w InstanceBuffer),
        meshes: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_handle) else {
            return RenderCommandResult::Failure;
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw(0..*vertex_count, 0..instance_buffer.length as u32);
            }
        }

        RenderCommandResult::Success
    }
}
