// Beginnings of a library of shaping functions. Or so.
fn border(st: vec2<f32>, t: vec2<f32>) -> f32 {
    var tl = step(t,st);
    var br = step(t,1.0 - st);
    return tl.x * tl.y * br.x * br.y;
}
fn oval(st: vec2<f32>) -> f32 {
    var pct = distance(vec2<f32>(0.5,0.5),st);
    pct = smoothstep(0.0,0.05,1.0 - pct);
    return pct;
}
struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Transform {
    pos: vec2<f32>,
    a: f32,
}
struct Material {
    color: vec4<f32>,
    kind: i32,
}


@group(0) @binding(0)
var<uniform> screen: vec2<f32>;
@group(0) @binding(1)
var<uniform> timer: f32;

@group(1) @binding(0)
var<storage,read> transforms: array<Transform>;

@group(1) @binding(1)
var<storage,read> materials: array<Material>;

@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput, @builtin(instance_index) inst: u32) -> VertexOutput {
    let a = transforms[inst].a;
    let rotation = mat2x2<f32>(cos(a),-sin(a),sin(a),cos(a));
    var pos = ((in.pos.xy) / screen) * 2.0;
    var trans = ((transforms[inst].pos / screen) - 0.5) * 2.0 ;
    var output = (rotation * pos) + trans;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(output,1.0,1.0);
    out.tex = in.tex;
    out.color = materials[inst].color;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

