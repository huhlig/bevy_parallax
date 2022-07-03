// Import Mesh View Bind Group from bevy_sprite
// https://github.com/bevyengine/bevy/blob/main/crates/bevy_sprite/src/mesh2d/mesh2d_view_bind_group.wgsl
#import bevy_sprite::mesh2d_view_bind_group
// struct View {
//   view_proj: mat4x4<f32>;
//   view: mat4x4<f32>;
//   inverse_view: mat4x4<f32>;
//   projection: mat4x4<f32>;
//   world_position: vec3<f32>;
//   near: f32;
//   far: f32;
//   width: f32;
//   height: f32;
// };

struct Vertex {
    [[builtin(vertex_index)]] vertexIndex: u32;
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct ParallaxMaterial {
    x_speed: f32;
    y_speed: f32;
    x_scale: f32;
    y_scale: f32;
};

[[group(0), binding(0)]] var<uniform> view: View;
[[group(1), binding(0)]] var texture: texture_2d<f32>;
[[group(1), binding(1)]] var texture_sampler: sampler;
[[group(1), binding(2)]] var<uniform> material: ParallaxMaterial;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> [[builtin(position)]] vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>( 1.0,  1.0), // 0
        vec2<f32>( 1.0, -1.0), // 1
        vec2<f32>(-1.0, -1.0), // 2
        vec2<f32>(-1.0,  1.0), // 3
        vec2<f32>(-1.0, -1.0), // 4
        vec2<f32>( 1.0,  1.0)  // 5
    );
    let world_position = view.view_proj * vec4<f32>(vertex.position, 1.0);
    return vec4<f32>(pos[vertex.vertexIndex], world_position.z, world_position.w);
}

[[stage(fragment)]]
fn fragment([[builtin(position)]] clip_position: vec4<f32>) -> [[location(0)]] vec4<f32> {
    let uv = vec2<f32>(
        ((clip_position.x / view.width) + (view.world_position.x * material.x_speed)) * material.x_scale,
        ((clip_position.y / view.height) - (view.world_position.y * material.y_speed)) * material.y_scale 
    );
    let color = textureSample(texture, texture_sampler, uv);
    return color;
}