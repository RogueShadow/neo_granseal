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
    x: f32,
    y: f32,
    rotation: f32,
}
struct Material {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    k: i32,
}
@group(0) @binding(0)
var<uniform> screen: vec2<f32>;
@group(0) @binding(1)
var<uniform> timer: f32;
@group(1) @binding(0)
var<uniform> transform: Transform;
@group(2) @binding(0)
var<uniform> material: Material;

@vertex
fn vs_main(@builtin(vertex_index) index: u32, in: VertexInput) -> VertexOutput {
    var pos = ((in.pos.xy) / screen) * 2.0;
    var trans = ((vec2<f32>(transform.x,transform.y)  / screen) - 0.5) * 2.0 ;
    var output = pos + trans;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(output,1.0,1.0);
    out.tex = in.tex;
    out.color = vec4<f32>(material.r,material.g,material.b,material.a);
    //out.color = vec4<f32>(1.0,0.0,0.0,1.0);
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

