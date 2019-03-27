use std::mem;

use amethyst::core::{math::Matrix4, GlobalTransform};
use amethyst::renderer::*;
use gfx_core::state::{Blend, ColorMask};
use glsl_layout::*;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub(crate) struct ViewArgs {
    proj: mat4,
    view: mat4,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub(crate) struct VertexArgs {
    proj: mat4,
    view: mat4,
    model: mat4,
    rgba: vec4,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Uniform)]
pub(crate) struct TextureOffsetPod {
    pub u_offset: vec2,
    pub v_offset: vec2,
}

pub(crate) fn add_texture(effect: &mut Effect, texture: &Texture) {
    effect.data.textures.push(texture.view().clone());
    effect.data.samplers.push(texture.sampler().clone());
}

pub(crate) fn setup_textures(builder: &mut EffectBuilder<'_>) {
    #[cfg(feature = "profiler")]
    profile_scope!("render_setuptextures");

    builder.with_texture("albedo");

    #[cfg(feature = "profiler")]
    profile_scope!("render_setuptextureoffsets");

    builder.with_raw_constant_buffer(
        "AlbedoOffset",
        mem::size_of::<<TextureOffsetPod as Uniform>::Std140>(),
        1,
    );
}

pub fn set_view_args(
    effect: &mut Effect,
    encoder: &mut Encoder,
    camera: Option<(&Camera, &GlobalTransform)>,
) {
    #[cfg(feature = "profiler")]
    profile_scope!("render_setviewargs");

    #[allow(clippy::option_map_unwrap_or_else)]
    let view_args = camera
        .as_ref()
        .map(|&(ref cam, ref transform)| {
            let proj: [[f32; 4]; 4] = cam.proj.into();
            let view: [[f32; 4]; 4] = transform
                .0
                .try_inverse()
                .expect("Unable to get inverse of camera transform")
                .into();
            ViewArgs {
                proj: proj.into(),
                view: view.into(),
            }
        })
        .unwrap_or_else(|| {
            let identity: [[f32; 4]; 4] = Matrix4::identity().into();
            ViewArgs {
                proj: identity.into(),
                view: identity.into(),
            }
        });
    effect.update_constant_buffer("ViewArgs", &view_args.std140(), encoder);
}

pub fn default_transparency() -> Option<(ColorMask, Blend, Option<DepthMode>)> {
    Some((ColorMask::all(), ALPHA, Some(DepthMode::LessEqualWrite)))
}
