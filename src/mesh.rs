use std::f32::consts::{PI, TAU};
use num_traits::AsPrimitive;
use rusttype::OutlineBuilder;
use crate::{Color, Point};
use crate::shape_pipeline::{Vertex};

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum FillStyle {
    Solid(Color),
    FadeDown(Color,Color),
    FadeLeft(Color,Color),
    Corners(Color,Color,Color,Color),
    Radial(Color,Color)
}
#[derive(Copy,Clone,Debug)]
pub enum LineStyle {
    Center,
    Left,
    Right,
}
#[derive(Copy, Clone,Debug)]
pub struct MBState {
    pub cursor: Point,
    pub filled: bool,
    pub fill_style: FillStyle,
    pub line_style: LineStyle,
    pub thickness: f32,
    pub rotation: f32,
    pub rot_origin: Point,
    pub resolution: f32,
    pub path_start: Point,
}
impl MBState {
    pub fn new() -> Self {
        MBState {
            cursor: Point::new(0,0),
            filled: true,
            fill_style: FillStyle::Solid(Color::WHITE),
            line_style: LineStyle::Center,
            thickness: 1.0,
            rotation: 0.0,
            rot_origin: Point::new(0,0),
            resolution: 8.0,
            path_start: Point::new(0,0)
        }
    }
}
#[derive(Debug,Clone,Copy)]
pub enum MBShapes {
    Line(Point,Point,Option<MBState>),
    Rect(Point,Option<MBState>),
    Oval(Point,Option<MBState>),
}

pub struct MeshBuilder {
    state: MBState,
    pub meshes: Vec<Mesh>,
    states: Vec<MBState>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            state: MBState::new(),
            meshes: vec![],
            states: vec![],
        }
    }
    pub fn set_cursor_p(&mut self, pos: Point) {
        self.set_cursor(pos.x, pos.y);
    }
    pub fn set_cursor(&mut self, x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) {
        self.state.cursor.x = x.as_();
        self.state.cursor.y = y.as_();
    }
    pub fn move_cursor(&mut self, x: impl AsPrimitive<f32>, y: impl AsPrimitive<f32>) {
        self.state.cursor.x += x.as_();
        self.state.cursor.y += y.as_();
    }
    pub fn set_style(&mut self, style: FillStyle) {
        self.state.fill_style = style;
    }
    pub fn set_rotation(&mut self, rotation: f32, origin: Point) {
        self.state.rot_origin = origin;
        self.state.rotation = rotation;
    }
    pub fn set_filled(&mut self, filled: bool) {
        self.state.filled = filled;
    }
    pub fn set_thickness(&mut self, thickness: f32) {
        self.state.thickness = thickness;
    }
    pub fn set_line_style(&mut self, style: LineStyle) {self.state.line_style = style; }
    pub fn set_resolution(&mut self, res: f32) {self.state.resolution = res;}
    pub fn push(&mut self) {
        self.states.push(self.state);
    }
    pub fn pop(&mut self) {
        self.state = self.states.pop().unwrap_or_else(||{self.state});
    }

    pub fn shape(&mut self, shape: MBShapes) {
        match shape {
            MBShapes::Line(begin,end, state) => {self.push();self.state = state.unwrap_or(self.state);self.line(begin,end);self.pop();}
            MBShapes::Rect(size,state) => {self.push();self.state = state.unwrap_or(self.state);self.rect(size);self.pop();}
            MBShapes::Oval(size,state) => {self.push();self.state = state.unwrap_or(self.state);self.oval(size);self.pop();}
        }
    }

    pub fn rect(&mut self, size: Point) {
        let mut m = if self.state.filled {
            rect_filled(self.state.cursor, self.state.cursor + size, self.state.fill_style)
        } else {
            rect_outlined(self.state.cursor, self.state.cursor + size, self.state.thickness, self.state.fill_style)
        };

        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }

        self.meshes.push(m);
    }
    pub fn oval(&mut self, radius: Point) {
        let mut m = if self.state.filled {
            oval_filled(self.state.cursor - radius / 2.0, radius, 0.0, TAU, self.state.resolution, self.state.fill_style)
        } else {
            oval_outlined(self.state.cursor - radius / 2.0, radius, 0.0, TAU, self.state.resolution, self.state.thickness, self.state.fill_style)
        };
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn arc(&mut  self, radius: Point, arc_begin: f32, arc_end: f32) {
        let mut m = if self.state.filled {
            oval_filled(self.state.cursor - radius / 2.0, radius, arc_begin, arc_end, self.state.resolution, self.state.fill_style)
        } else {
            oval_outlined(self.state.cursor - radius / 2.0, radius, arc_begin,arc_end,self.state.resolution,self.state.thickness, self.state.fill_style)
        };
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn line(&mut self, begin: Point, end: Point) {
        let mut m = line(self.state.cursor + begin,self.state.cursor + end,self.state.thickness,self.state.line_style,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn move_to(&mut self, pos: Point) {
        self.state.path_start = pos;
        self.state.cursor = pos;
    }
    pub fn close_path(&mut self) {
        self.line_to(self.state.path_start);
    }
    pub fn line_to(&mut self, end: Point) {
        let mut m = line(self.state.cursor, end, self.state.thickness,self.state.line_style,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.set_cursor_p(end);
        self.meshes.push(m);
    }
    pub fn quadratic(&mut self, begin:  Point, control: Point, end: Point) {
        let mut m = quadratic_curve(begin,end,control,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn quad_to(&mut self, control: Point, end: Point) {
        let mut m = quadratic_curve(self.state.cursor,end,control,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.set_cursor_p(end);
        self.meshes.push(m);
    }
    pub fn cubic(&mut self, begin: Point, control1: Point, control2: Point, end: Point) {
        let mut m = cubic_curve(begin,control1,control2,end,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn cubic_to(&mut self, control1: Point,control2: Point, end: Point) {
        let mut m = cubic_curve(self.state.cursor,control1,control2,end,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.set_cursor_p(end);
        self.meshes.push(m);
    }
    pub fn triangle_raw(&mut self, p1: Point, p2: Point, p3: Point) {
        let (c1,c2,c3,c4) = style_colors(self.state.fill_style);
        let mut m = raw_triangle_filled(p1,p2,p3,c1,c2,c3);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn quad_raw(&mut self, p1: Point, p2: Point, p3: Point, p4: Point) {
        let mut m = raw_quad_filled(p1,p2,p3,p4,self.state.fill_style);
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
        self.meshes.push(m);
    }
    pub fn build(self) -> Mesh {
        combine(self.meshes)
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}


impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
        }
    }
    pub fn add(mut self, other: &Mesh) -> Mesh {
        let start = self.vertices.len() as u32;
        self.vertices.extend(&other.vertices);
        let indices: Vec<u32> = other.indices.iter().map(|i| i + start).collect();
        self.indices.extend(indices);
        self
    }
    pub fn min_x(&self) -> f32 {
        self.vertices.iter().fold(f32::MAX,|acc,v| if acc < v.x {acc} else {v.x})
    }
    pub fn min_y(&self) -> f32 {
        self.vertices.iter().fold(f32::MAX,|acc,v| if acc < v.y {acc} else {v.y})
    }
    pub fn max_x(&self) -> f32 {
        self.vertices.iter().fold(f32::MIN, |acc, v| if acc > v.x {acc} else {v.x})
    }
    pub fn max_y(&self) -> f32 {
        self.vertices.iter().fold(f32::MIN, |acc, v| if acc > v.y {acc} else {v.y})
    }
    pub fn mid_x(&self) -> f32 {
        let dx = self.max_x() - self.min_x();
        self.max_x() - dx / 2.0
    }
    pub fn mid_y(&self) -> f32 {
        let dy = self.max_y() - self.min_y();
        self.max_y() - dy / 2.0
    }
    pub fn translate(&mut self, pos: Point) {
        for mut v in self.vertices.iter_mut() {
            v.x += pos.x;
            v.y += pos.y;
        }
    }
    pub fn scale(&mut self, scale: f32) {
        self.vertices.iter_mut().for_each(|v| {
            v.x *= scale;
            v.y *= scale;
        })
    }
    pub fn rotate(&mut self, rotation: f32) {
        let rot = cgmath::Matrix2::new(rotation.cos(),-rotation.sin(),rotation.sin(),rotation.cos());
        self.vertices.iter_mut().for_each(|v|{
            let p = rot * cgmath::vec2(v.x,v.y);
            v.x = p.x;
            v.y = p.y;
        })
    }
    pub fn style(&mut self, style: FillStyle) -> &Self {
        let minx = self.min_x();
        let maxx = self.max_x();
        let miny = self.min_y();
        let maxy = self.max_y();
        let dx = maxx - minx;
        let dy = maxy - miny;
        let midx = maxx - dx / 2.0;
        let midy = maxy - dy / 2.0;
        let texu = |x: f32| {
            (x - minx) / dx
        };
        let texv = |y: f32| {
            (y - miny) / dy
        };
        match style {
            FillStyle::Solid(c1) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.set_color(c1);
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                });
            }
            FillStyle::FadeDown(c1,c2) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    if v.y < midy {
                        v.set_color(c1)
                    } else {v.set_color(c2)}
                })
            }
            FillStyle::FadeLeft(c1,c2) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    if v.x < midx {
                        v.set_color(c1)
                    } else {v.set_color(c2)}
                })
            }
            FillStyle::Corners(c1, c2, c3, c4) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                    match (v.x < midx,v.y < midy) {
                        (true, true) => {v.set_color(c1)}
                        (false,true) => {v.set_color(c2)}
                        (false,false) => {v.set_color(c3)}
                        (true,false) => {v.set_color(c4)}
                    }
                })
            }
            FillStyle::Radial(c1,c2) => {
                let mx = self.mid_x();
                let my = self.mid_y();
                self.vertices.iter_mut().for_each(|v| {
                    let dx = (v.x - mx).abs();
                    let dy = (v.y - my).abs();
                    let d = dx*dx + dy*dy;
                    if d > 1.0 {
                        v.set_color(c1);
                    }else{v.set_color(c2)}
                })
            }
        };
        self
    }
}

fn style_colors(style: FillStyle) -> (Color, Color, Color, Color) {
    match style {
        FillStyle::Solid(color) => {(color,color,color,color)}
        FillStyle::FadeDown(color1, color2) => {(color2,color1,color1,color2)}
        FillStyle::FadeLeft(color1, color2) => {(color1,color1,color2,color2)}
        FillStyle::Corners(c1, c2, c3, c4) => {(c1,c2,c3,c4)}
        FillStyle::Radial(c1,c2) => {(c1,c1,c2,c2)}
    }
}
pub fn rect_filled(top_left: Point, bottom_right: Point, style: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(style);
    let mut mesh = Mesh::new();
    mesh.vertices = vec![
        Vertex::point(top_left).rgba(c2),
        Vertex::new(bottom_right.x,top_left.y).rgba(c3),
        Vertex::point(bottom_right).rgba(c4),
        Vertex::new(top_left.x,bottom_right.y).rgba(c1)
    ];
    mesh.indices = vec![2,1,0, 3,2,0];
    mesh
}
pub fn rect_outlined(top_left: Point, bottom_right: Point, thickness: f32, style: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(style);
    let mut mesh = Mesh::new();
    match style {
        FillStyle::Radial(c1, c3) => {
            mesh.vertices = vec![
                Vertex::point(top_left).rgba(c1),
                Vertex::new(bottom_right.x, top_left.y).rgba(c1),
                Vertex::point(bottom_right).rgba(c1),
                Vertex::new(top_left.x, bottom_right.y).rgba(c1),
                Vertex::point(top_left + thickness).rgba(c3),
                Vertex::new(bottom_right.x - thickness, top_left.y + thickness).rgba(c3),
                Vertex::point(bottom_right - thickness).rgba(c3),
                Vertex::new(top_left.x + thickness, bottom_right.y - thickness).rgba(c3),
            ];
        }
        _ => {
            mesh.vertices = vec![
                Vertex::point(top_left).rgba(c2),
                Vertex::new(bottom_right.x, top_left.y).rgba(c3),
                Vertex::point(bottom_right).rgba(c4),
                Vertex::new(top_left.x, bottom_right.y).rgba(c1),
                Vertex::point(top_left + thickness).rgba(c2),
                Vertex::new(bottom_right.x - thickness, top_left.y + thickness).rgba(c3),
                Vertex::point(bottom_right - thickness).rgba(c4),
                Vertex::new(top_left.x + thickness, bottom_right.y - thickness).rgba(c1),
            ];
        }
    }
    mesh.indices = vec![
        4,1,0,   4,5,1,
        5,2,1,   6,2,5,
        6,3,2,   7,3,6,
        3,7,4,   0,3,4,
    ];
    mesh
}
pub fn oval_filled(center: Point, radius: Point, arc_begin: f32, arc_end: f32, resolution: f32, style: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(style);
    let mut mesh = Mesh::new();
    mesh.vertices = vec![Vertex::point(center).rgba(c1)];
    let start_angle = arc_begin;
    let end_angle = arc_end;
    let arc_length = (end_angle - start_angle).abs() * radius.len();
    let vertex_count = arc_length / resolution;
    let angle_step = (end_angle - start_angle).abs() / vertex_count;
    let mut a = start_angle;
    (0..=(vertex_count as u32 + 1)).for_each(|i| {
        if i <= vertex_count.floor() as u32 {
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * a.cos(), center.y + radius.y * a.sin()).rgba(c3)
            );
        }else{
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * end_angle.cos(),center.y + radius.y * end_angle.sin()).rgba(c3)
            );
            mesh.indices.push(0);
            mesh.indices.push(i+1);
            mesh.indices.push(i);
        }
        if i > 0 {
            mesh.indices.push(0);
            mesh.indices.push(i);
            mesh.indices.push(i - 1);
        }
        a += angle_step;
    });
    mesh
}
pub fn oval_outlined(center: Point, radius: Point, arc_begin: f32, arc_end: f32, resolution: f32, thickness: f32 , style: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(style);
    let mut mesh = Mesh::new();
    let start_angle = arc_begin;
    let end_angle = arc_end;
    let arc_length = (end_angle - start_angle).abs() * radius.len();
    let vertex_count = arc_length / resolution;
    let angle_step = (end_angle - start_angle).abs() / vertex_count;
    let mut a = start_angle;
    (0..=(vertex_count as u32 + 1)).for_each(|i| {
        if i <= vertex_count.floor() as u32 {
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * a.cos(), center.y + radius.y * a.sin()).rgba(c1)
            );
            mesh.vertices.push(
                Vertex::new(center.x + (radius.x - thickness) * a.cos(), center.y + (radius.y - thickness) * a.sin()).rgba(c3)
            );
        }else{
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * end_angle.cos(),center.y + radius.y * end_angle.sin()).rgba(c1)
            );
            mesh.vertices.push(
                Vertex::new(center.x + (radius.x - thickness) * end_angle.cos(), center.y + (radius.y - thickness) * end_angle.sin()).rgba(c3)
            );


        }
        if i > 0 {
            let v0 = i*2 - 2;
            let v1 = i*2 - 1;
            let v2 = i*2;
            let v3 = i*2 + 1;
            mesh.indices.push(v0);
            mesh.indices.push(v1);
            mesh.indices.push(v2);

            mesh.indices.push(v2);
            mesh.indices.push(v1);
            mesh.indices.push(v3);

        }
        a += angle_step;
    });
    mesh
}

pub fn line(begin: Point, end: Point, thickness: f32, style: LineStyle, fill: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(fill);
    let mut m = Mesh::new();
    let width = thickness / 2.0;
    let swap = if begin.y < end.y {-1.0} else {1.0};
    let dx = (begin.x - end.x) * 2.0;
    let dy = (begin.y - end.y) * 2.0;
    let a = (dx/dy).atan();
    let half_pi = PI / 2.0;
    let sa = a - half_pi;
    let sa2 = a + half_pi;

    let bump2 = Point::new(width * sa.sin(),width * sa.cos());
    let bump = Point::new(width * sa2.sin(),width * sa2.cos());

    match style {
        LineStyle::Center => {
            m.vertices.push(Vertex::new(begin.x + bump2.x, begin.y + bump2.y).rgba(c1));
            m.vertices.push(Vertex::new(begin.x + bump.x, begin.y + bump.y).rgba(c2));
            m.vertices.push(Vertex::new(end.x + bump.x, end.y + bump.y).rgba(c3));
            m.vertices.push(Vertex::new(end.x + bump2.x, end.y + bump2.y).rgba(c4));
        }
        LineStyle::Left => {
            m.vertices.push(Vertex::new(begin.x + bump2.x * 2.0 * swap, begin.y + bump2.y * 2.0 * swap).rgba(c1));
            m.vertices.push(Vertex::new(begin.x, begin.y).rgba(c2));
            m.vertices.push(Vertex::new(end.x, end.y).rgba(c3));
            m.vertices.push(Vertex::new(end.x + bump2.x * 2.0 * swap, end.y + bump2.y * 2.0 * swap).rgba(c4));
        }
        LineStyle::Right => {
            m.vertices.push(Vertex::new(begin.x, begin.y).rgba(c1));
            m.vertices.push(Vertex::new(begin.x + bump.x * 2.0 * swap, begin.y + bump.y * 2.0 * swap).rgba(c2));
            m.vertices.push(Vertex::new(end.x + bump.x * 2.0 * swap, end.y + bump.y * 2.0 * swap).rgba(c3));
            m.vertices.push(Vertex::new(end.x, end.y).rgba(c4));
        }
    };
    if swap < 0.0 {
        m.indices = vec![
            2, 1, 0, 3, 2, 0,
        ];
    } else {
        m.indices = vec![
            0, 1, 2, 0, 2, 3,
        ];
    }
    m
}
pub fn raw_triangle_filled(p1: Point, p2: Point, p3: Point, c1: Color, c2: Color, c3: Color) -> Mesh {
    let mut m = Mesh::new();
    m.vertices.push(Vertex::new(p1.x, p1.y).rgba(c1));
    m.vertices.push(Vertex::new(p2.x,p2.y).rgba(c2));
    m.vertices.push(Vertex::new(p3.x,p3.y).rgba(c3));
    m.indices = vec![2,1,0];
    m
}
pub fn raw_quad_filled(p1: Point, p2: Point, p3: Point,p4: Point, style: FillStyle) -> Mesh {
    let (c1,c2,c3,c4) = style_colors(style);
    let mut m = Mesh::new();
    m.vertices.push(Vertex::new(p1.x,p1.y).rgba(c1));
    m.vertices.push(Vertex::new(p2.x,p2.y).rgba(c2));
    m.vertices.push(Vertex::new(p3.x,p3.y).rgba(c3));
    m.vertices.push(Vertex::new(p4.x,p4.y).rgba(c4));
    m.indices = vec![2,1,0, 3,2,0];
    m
}
pub fn quadratic_curve(begin: Point, end: Point, control: Point, style: FillStyle) -> Mesh {
    let mut m = MeshBuilder::new();
    m.set_style(style);
    let mut points: Vec<Point> = vec![];
    (0..20).for_each(|i|{
        let t = i as f32 / 20.0;
        let p = control + (1.0 - t).powf(2.0) * (begin - control) + t.powf(2.0) * (end - control);
        points.push(p);
    });
    (0..points.len() - 1).for_each(|i|{
        m.line(points[i],points[i+1]);
    });
    m.build()
}
pub fn cubic_curve(begin: Point, control1: Point,control2: Point, end: Point,style: FillStyle) -> Mesh {
    let mut m = MeshBuilder::new();
    m.set_style(style);
    let mut points: Vec<Point> = vec![];
    let distance = (end - begin).len();
    let count = (distance / 2.0) as i32;
    (0..=count).for_each(|i|{
        let t = (i as f32 / count as f32);
        let part1 = (1.0 - t).powf(3.0) * begin;
        let part2 = 3.0 * (1.0 - t).powf(2.0) * t * control1;
        let part3 = 3.0 * (1.0 - t) * t.powf(2.0) * control2;
        let part4 = t.powf(3.0) * end;
        let p = part1 + part2 + part3 + part4;
        points.push(p);
    });
    (0..points.len() - 1).for_each(|i|{
        m.line(points[i],points[i+1]);
    });
    m.build()
}
pub fn glyphs(font: &rusttype::Font, text: &str, scale: f32) -> Mesh {
    let mut g: Vec<Mesh> = vec![];
    let mut advance = 0.0;
    for c in text.chars(){
        let sg = font.glyph(c).scaled(rusttype::Scale::uniform(scale));
        let mut m = MeshBuilder::new();
        sg.build_outline(&mut m);
        let mut mesh = m.build();
        mesh.translate(Point::new(advance,0));
        advance += sg.h_metrics().advance_width;// + sg.h_metrics().left_side_bearing;
        g.push(mesh);
    }
    combine(g)
}
pub fn  glyph(glyph: rusttype::Glyph, scale: f32) -> Mesh {
    let g = glyph.scaled(rusttype::Scale::uniform(scale));
    let mut m = MeshBuilder::new();
    g.build_outline(&mut m);
    m.build()
}

pub fn combine(mut meshes: Vec<Mesh>) -> Mesh {
    meshes.iter_mut().fold(Mesh::new(),|acc, m|acc.add(m))
}

impl rusttype::OutlineBuilder for MeshBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.move_to(Point::new(x,y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let end = Point::new(x,y);
        self.line_to(end);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.quad_to(Point::new(x1,y1),Point::new(x,y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let cp1 = Point::new(x1,y1);
        let cp2 =  Point::new(x2,y2);
        let end = Point::new(x,y);
        self.cubic_to(cp1,cp2,end);
    }

    fn close(&mut self) {
        self.close_path();
    }
}