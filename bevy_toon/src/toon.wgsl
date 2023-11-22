#import bevy_pbr::{
    forward_io::VertexOutput, 
    mesh_view_bindings as view_bindings,
    pbr_functions,
}

struct ToonMaterialUniform {
    base_color: vec4<f32>,
    smoothness: f32,
    rim_light_visible: u32,
    rim_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> toon: ToonMaterialUniform;

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

const Threshold = 0.2;

struct ToonInput {
    N: vec3<f32>,
    V: vec3<f32>,
};

fn toon_from_vertex_output(vertex: VertexOutput) -> ToonInput {
    let V = pbr_functions::calculate_view(vertex.world_position, view_bindings::view.projection[3].w == 1.0);

    return ToonInput(
        vertex.world_normal,
        V,
    );
}

fn direct_light(in: ToonInput) -> f32 {
    let n_directional_lights = view_bindings::lights.n_directional_lights;

    var intensity = 0.0;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        let light = &view_bindings::lights.directional_lights[i];
        let L = (*light).direction_to_light;
        let N = in.N;

        intensity += saturate(dot(N, L));
    }

    let smoothness = saturate(toon.smoothness) * 0.5;
    intensity = smoothstep(Threshold - smoothness, Threshold + smoothness, intensity);
    intensity = max(0.2, intensity);

    return intensity;
}

fn rim_light(in: ToonInput, light: bool) -> vec3<f32> {
    if (toon.rim_light_visible == 1u && !light) {
        return vec3(0.0);
    }

    let VoN = dot(in.V, in.N);
    var rim_alpha = 1.0 - max(0.0, VoN);

    if (rim_alpha < 0.6) {
        rim_alpha = 0.0;
    }

    return toon.rim_color.rgb * rim_alpha * toon.rim_color.a;
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    let toon_in = toon_from_vertex_output(in);

    var color = toon.base_color;
    
    let intensity = direct_light(toon_in);
    
    let rim = rim_light(toon_in, intensity > Threshold);

    color = vec4(color.rgb + rim.rgb, color.a);

    return FragmentOutput(color * intensity);
}