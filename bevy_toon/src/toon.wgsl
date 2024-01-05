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

struct LightOutput {
    intensity: f32,
    rim_color: vec3<f32>,
};

fn direct_light(in: ToonInput) -> LightOutput {
    let n_directional_lights = view_bindings::lights.n_directional_lights;

    var intensity = 0.0;
    var rim_color = vec3(0.0);
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        let light = &view_bindings::lights.directional_lights[i];
        let L = (*light).direction_to_light;
        let N = in.N;

        intensity += saturate(dot(N, L));
        rim_color += rim_light(in, L, intensity > Threshold);
    }

    let smoothness = saturate(toon.smoothness) * 0.5;
    intensity = smoothstep(Threshold - smoothness, Threshold + smoothness, intensity);
    intensity = max(0.2, intensity);

    return LightOutput(intensity, rim_color);
}

fn rim_light(in: ToonInput, L: vec3<f32>, light: bool) -> vec3<f32> {
    if (toon.rim_light_visible == 1u && !light) {
        return vec3(0.0);
    }

    let VoN = dot(in.V, in.N);
    var rim = 1.0 - max(0.0, VoN);
    rim = pow(rim, 5.0);

    let NoL = saturate(dot(in.N, L));
    rim *= pow(NoL, 5.0);

    return toon.rim_color.rgb * rim * toon.rim_color.a;
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    let toon_in = toon_from_vertex_output(in);

    var color = toon.base_color;
    let light = direct_light(toon_in);
    color = vec4(color.rgb + light.rim_color.rgb, color.a);

    return FragmentOutput(color * light.intensity);
}