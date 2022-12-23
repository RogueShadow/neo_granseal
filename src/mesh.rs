use crate::math::Vec2;
use crate::mesh::FillStyle::*;
use crate::shape_pipeline::Vertex;
use crate::util::{
    cubic_to_point, quadratic_to_point, text_to_path, Contour, LineSegment, PathBuilder, PathData,
    Ray,
};
use crate::Color;
use std::f32::consts::{PI, TAU};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FillStyle {
    Solid(Color),
    FadeDown(Color, Color),
    FadeLeft(Color, Color),
    Corners(Color, Color, Color, Color),
    Radial(Color, Color),
}
#[derive(Copy, Clone, Debug)]
pub enum LineStyle {
    Center,
    Left,
    Right,
}
#[derive(Copy, Clone, Debug)]
pub struct MBState {
    pub cursor: Vec2,
    pub filled: bool,
    pub fill_style: FillStyle,
    pub line_style: LineStyle,
    pub thickness: f32,
    pub rotation: f32,
    pub rot_origin: Vec2,
    pub resolution: f32,
    pub path_start: Vec2,
}
impl Default for MBState {
    fn default() -> Self {
        Self {
            cursor: Vec2::new(0, 0),
            filled: true,
            fill_style: Solid(Color::WHITE),
            line_style: LineStyle::Center,
            thickness: 1.0,
            rotation: 0.0,
            rot_origin: Vec2::new(0, 0),
            resolution: 8.0,
            path_start: Vec2::new(0, 0),
        }
    }
}
impl MBState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum MBShapes {
    Line(Vec2, Vec2, Option<MBState>),
    Rect(Vec2, Option<MBState>),
    Oval(Vec2, Option<MBState>),
}
pub trait FillStyleShorthand {
    fn solid(&mut self, c: Color);
    fn fade_down(&mut self, c1: Color, c2: Color);
    fn fade_left(&mut self, c1: Color, c2: Color);
    fn radial(&mut self, c1: Color, c2: Color);
    fn corners(&mut self, c1: Color, c2: Color, c3: Color, c4: Color);
}

pub struct MeshBuilder {
    pub(crate) state: MBState,
    pub meshes: Vec<Mesh>,
    pub(crate) states: Vec<MBState>,
}
impl FillStyleShorthand for MeshBuilder {
    fn solid(&mut self, c: Color) {
        self.set_style(Solid(c))
    }
    fn fade_down(&mut self, c1: Color, c2: Color) {
        self.set_style(FadeDown(c1, c2))
    }
    fn fade_left(&mut self, c1: Color, c2: Color) {
        self.set_style(FadeLeft(c1, c2))
    }
    fn radial(&mut self, c1: Color, c2: Color) {
        self.set_style(Radial(c1, c2))
    }
    fn corners(&mut self, c1: Color, c2: Color, c3: Color, c4: Color) {
        self.set_style(Corners(c1, c2, c3, c4))
    }
}
impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            state: MBState::new(),
            meshes: vec![],
            states: vec![],
        }
    }
    pub fn set_cursor(&mut self, cursor: Vec2) {
        self.state.cursor = cursor;
    }
    pub fn move_cursor(&mut self, pos: Vec2) {
        self.state.cursor += pos;
    }
    pub fn set_style(&mut self, style: FillStyle) {
        self.state.fill_style = style;
    }
    pub fn set_rotation(&mut self, rotation: f32, origin: Vec2) {
        self.state.rot_origin = origin;
        self.state.rotation = rotation;
    }
    pub fn set_filled(&mut self, filled: bool) {
        self.state.filled = filled;
    }
    pub fn set_thickness(&mut self, thickness: f32) {
        self.state.thickness = thickness;
    }
    pub fn set_line_style(&mut self, style: LineStyle) {
        self.state.line_style = style;
    }
    pub fn set_resolution(&mut self, res: f32) {
        self.state.resolution = res;
    }
    pub fn push(&mut self) {
        self.states.push(self.state);
    }
    pub fn pop(&mut self) {
        if !self.states.is_empty() {self.state = self.states.pop().unwrap()}
    }

    pub fn shape(&mut self, shape: MBShapes) {
        match shape {
            MBShapes::Line(begin, end, state) => {
                self.push();
                self.state = state.unwrap_or(self.state);
                self.line(begin, end);
                self.pop();
            }
            MBShapes::Rect(size, state) => {
                self.push();
                self.state = state.unwrap_or(self.state);
                self.rect(size);
                self.pop();
            }
            MBShapes::Oval(size, state) => {
                self.push();
                self.state = state.unwrap_or(self.state);
                self.oval(size);
                self.pop();
            }
        }
    }
    pub fn stroke_path(&mut self, path: &PathData) {
        for seg in path.segments.iter() {
            for con in seg.contours.iter() {
                match con {
                    Contour::MoveTo(begin) => {
                        self.move_to(*begin);
                    }
                    Contour::LineTo(end) => self.line_to(*end),
                    Contour::QuadTo(cp, end) => self.quad_to(*cp, *end),
                    Contour::CubicTo(cp1, cp2, end) => self.cubic_to(*cp1, *cp2, *end),
                    Contour::ClosePath(bool) => {
                        self.close_path(*bool);
                    }
                }
            }
        }
    }
    pub fn draw_polygon(&mut self, polygon: &Polygon) {
        for (start, end) in polygon.edges.iter() {
            self.line(polygon.points[*start], polygon.points[*end]);
        }
    }
    pub fn draw_text(&mut self, font: &rusttype::Font, text: &str, scale: f32) {
        if self.state.filled {
            let v_metrics = font.v_metrics(rusttype::Scale::uniform(scale));
            let glyphs: Vec<_> = font
                .layout(
                    text,
                    rusttype::Scale::uniform(scale),
                    rusttype::point(0.0, 0.0 + v_metrics.ascent),
                )
                .collect();
            self.push();
            let offset = self.state.cursor;
            for g in glyphs {
                if let Some(bb) = g.pixel_bounding_box() {
                    g.draw(|x, y, v| {
                        if v > 0.001 {
                            self.set_cursor(Vec2::new(
                                offset.x + bb.min.x as f32 + x as f32,
                                offset.y + bb.min.y as f32 + y as f32,
                            ));
                            self.rect(Vec2::new(1, 1));
                        }
                    });
                }
            }
            self.pop();
        } else {
            let mut pb = PathBuilder::new();
            pb.set_offset(self.state.cursor);
            text_to_path(&mut pb, font, text, scale);
            self.stroke_path(&pb.build());
        }
    }
    pub fn rect(&mut self, size: Vec2) {
        let mut m = if self.state.filled {
            rect_filled(
                self.state.cursor,
                self.state.cursor + size,
                self.state.fill_style,
            )
        } else {
            rect_outlined(
                self.state.cursor,
                self.state.cursor + size,
                self.state.thickness,
                self.state.fill_style,
            )
        };

        self.do_rotation(&mut m);

        self.meshes.push(m);
    }
    pub fn do_rotation(&self, m: &mut Mesh) {
        if self.state.rotation != 0.0 {
            let offset = -self.state.cursor - self.state.rot_origin;
            m.translate(offset);
            m.rotate(self.state.rotation);
            m.translate(-offset);
        }
    }
    pub fn rounded_rect(&mut self, size: Vec2, radius: f32) {
        let mut m = if self.state.filled {
            rounded_rect_filled(
                self.state.cursor,
                self.state.cursor + size,
                radius,
                self.state.fill_style
            )
        } else {
            rounded_rect_outlined(
                self.state.cursor,
                self.state.cursor + size,
                radius,
                self.state.thickness,
                self.state.fill_style
            )
        };

        self.do_rotation(&mut m);

        self.meshes.push(m);
    }
    pub fn oval(&mut self, radius: Vec2) {
        let mut m = if self.state.filled {
            oval_filled(
                self.state.cursor + radius,
                radius,
                0.0,
                TAU,
                self.state.resolution,
                self.state.fill_style,
            )
        } else {
            oval_outlined(
                self.state.cursor + radius,
                radius,
                0.0,
                TAU,
                self.state.resolution,
                self.state.thickness,
                self.state.fill_style,
            )
        };
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn arc(&mut self, radius: Vec2, arc_begin: f32, arc_end: f32) {
        let mut m = if self.state.filled {
            oval_filled(
                self.state.cursor + radius,
                radius,
                arc_begin,
                arc_end,
                self.state.resolution,
                self.state.fill_style,
            )
        } else {
            oval_outlined(
                self.state.cursor + radius,
                radius,
                arc_begin,
                arc_end,
                self.state.resolution,
                self.state.thickness,
                self.state.fill_style,
            )
        };
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn line(&mut self, begin: Vec2, end: Vec2) {
        let mut m = line(
            self.state.cursor + begin,
            self.state.cursor + end,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
        );
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn move_to(&mut self, pos: Vec2) {
        self.state.path_start = pos;
        self.state.cursor = pos;
    }
    pub fn close_path(&mut self, closed: bool) {
        if closed {
            self.line_to(self.state.path_start);
        }
    }
    pub fn line_to(&mut self, end: Vec2) {
        let mut m = line(
            self.state.cursor,
            end,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
        );
        self.do_rotation(&mut m);
        self.set_cursor(end);
        self.meshes.push(m);
    }
    pub fn quadratic(&mut self, begin: Vec2, control: Vec2, end: Vec2) {
        let mut m = quadratic_curve(
            begin,
            end,
            control,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
            self.state.resolution,
        );
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn quad_to(&mut self, control: Vec2, end: Vec2) {
        let mut m = quadratic_curve(
            self.state.cursor,
            end,
            control,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
            self.state.resolution,
        );
        self.do_rotation(&mut m);
        self.set_cursor(end);
        self.meshes.push(m);
    }
    pub fn cubic(&mut self, begin: Vec2, control1: Vec2, control2: Vec2, end: Vec2) {
        let mut m = cubic_curve(
            begin,
            control1,
            control2,
            end,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
            self.state.resolution,
        );
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn cubic_to(&mut self, control1: Vec2, control2: Vec2, end: Vec2) {
        let mut m = cubic_curve(
            self.state.cursor,
            control1,
            control2,
            end,
            self.state.thickness,
            self.state.line_style,
            self.state.fill_style,
            self.state.resolution,
        );
        self.do_rotation(&mut m);
        self.set_cursor(end);
        self.meshes.push(m);
    }
    pub fn triangle_raw(&mut self, p1: Vec2, p2: Vec2, p3: Vec2) {
        let (c1, c2, c3, _) = style_colors(self.state.fill_style);
        let mut m = raw_triangle_filled(p1, p2, p3, c1, c2, c3);
        self.do_rotation(&mut m);
        self.meshes.push(m);
    }
    pub fn quad_raw(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) {
        let mut m = raw_quad_filled(p1, p2, p3, p4, self.state.fill_style);
        self.do_rotation(&mut m);
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
        self.vertices
            .iter()
            .fold(f32::MAX, |acc, v| if acc < v.x { acc } else { v.x })
    }
    pub fn min_y(&self) -> f32 {
        self.vertices
            .iter()
            .fold(f32::MAX, |acc, v| if acc < v.y { acc } else { v.y })
    }
    pub fn max_x(&self) -> f32 {
        self.vertices
            .iter()
            .fold(f32::MIN, |acc, v| if acc > v.x { acc } else { v.x })
    }
    pub fn max_y(&self) -> f32 {
        self.vertices
            .iter()
            .fold(f32::MIN, |acc, v| if acc > v.y { acc } else { v.y })
    }
    pub fn mid_x(&self) -> f32 {
        let dx = self.max_x() - self.min_x();
        self.max_x() - dx / 2.0
    }
    pub fn mid_y(&self) -> f32 {
        let dy = self.max_y() - self.min_y();
        self.max_y() - dy / 2.0
    }
    pub fn translate(&mut self, pos: Vec2) {
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
        let rot = cgmath::Matrix2::new(
            rotation.cos(),
            -rotation.sin(),
            rotation.sin(),
            rotation.cos(),
        );
        self.vertices.iter_mut().for_each(|v| {
            let p = rot * cgmath::vec2(v.x, v.y);
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
        let texu = |x: f32| (x - minx) / dx;
        let texv = |y: f32| (y - miny) / dy;
        match style {
            Solid(c1) => {
                self.vertices.iter_mut().for_each(|v| {
                    v.set_color(c1);
                    v.u = texu(v.x);
                    v.v = texv(v.y);
                });
            }
            FadeDown(c1, c2) => self.vertices.iter_mut().for_each(|v| {
                v.u = texu(v.x);
                v.v = texv(v.y);
                if v.y < midy {
                    v.set_color(c1)
                } else {
                    v.set_color(c2)
                }
            }),
            FadeLeft(c1, c2) => self.vertices.iter_mut().for_each(|v| {
                v.u = texu(v.x);
                v.v = texv(v.y);
                if v.x < midx {
                    v.set_color(c1)
                } else {
                    v.set_color(c2)
                }
            }),
            Corners(c1, c2, c3, c4) => self.vertices.iter_mut().for_each(|v| {
                v.u = texu(v.x);
                v.v = texv(v.y);
                match (v.x < midx, v.y < midy) {
                    (true, true) => v.set_color(c1),
                    (false, true) => v.set_color(c2),
                    (false, false) => v.set_color(c3),
                    (true, false) => v.set_color(c4),
                }
            }),
            Radial(c1, c2) => {
                let mx = self.mid_x();
                let my = self.mid_y();
                self.vertices.iter_mut().for_each(|v| {
                    let dx = (v.x - mx).abs();
                    let dy = (v.y - my).abs();
                    let d = dx * dx + dy * dy;
                    if d > 1.0 {
                        v.set_color(c1);
                    } else {
                        v.set_color(c2)
                    }
                })
            }
        };
        self
    }
}
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub edges: Vec<(usize, usize)>,
}
impl Polygon {
    pub fn new() -> Self {
        Self {
            points: vec![],
            edges: vec![],
        }
    }
    pub fn get_vertex_and_neighbors(&self, index: usize) -> Vec<Vec2> {
        let mut result: Vec<Vec2> = vec![];
        let edges = self
            .edges
            .iter()
            .filter(|(start, end)| *start == index || *end == index)
            .collect::<Vec<&(usize, usize)>>();
        match &edges.len() {
            0 => {
                println!("No edge for point")
            }
            1 => {
                let (s, e) = edges[0];
                if *s == index {
                    result.push(self.points[*s]);
                    result.push(self.points[*e]);
                } else {
                    result.push(self.points[*e]);
                    result.push(self.points[*s]);
                }
            }
            2 => {
                let (s1, e1) = edges[0];
                let (s2, e2) = edges[1];
                if s1 == e1 {
                    println!("Very odd!")
                }
                if s2 == e2 {
                    println!("Very odd!")
                }
                match (*s1 == index, *s2 == index) {
                    (true, true) => {
                        result.push(self.points[*e1]);
                        result.push(self.points[*e2]);
                    }
                    (true, false) => {
                        result.push(self.points[*e1]);
                        result.push(self.points[*s2]);
                    }
                    (false, true) => {
                        result.push(self.points[*s1]);
                        result.push(self.points[*e2]);
                    }
                    (false, false) => {
                        result.push(self.points[*s1]);
                        result.push(self.points[*s2]);
                    }
                }
            }
            n => {
                println!("{} edges, very odd", n)
            }
        }
        result
    }
}
pub fn path_to_polygon(path: &PathData, resolution: f32) -> Polygon {
    let mut last = Vec2::ZERO;
    let mut points = vec![];
    let mut edges: Vec<(usize, usize)> = vec![];
    for segment in path.segments.iter() {
        for contour in segment.contours.iter() {
            match contour {
                Contour::MoveTo(start) => {
                    points.push(*start);
                    last = *start;
                }
                Contour::LineTo(end) => {
                    points.push(*end);
                    edges.push((points.len() - 2, points.len() - 1));
                    last = *end;
                }
                Contour::QuadTo(cp, end) => {
                    let d = *end - last;
                    let count = (d.magnitude() / resolution).ceil() as i32;
                    let mut lastp = last;
                    (0..=count).for_each(|i| {
                        let t = i as f32 / count as f32;
                        let p = quadratic_to_point(t, last, *cp, *end);
                        points.push(p);
                        edges.push((points.len() - 2, points.len() - 1));
                        lastp = p;
                    });
                    last = lastp;
                }
                Contour::CubicTo(cp1, cp2, end) => {
                    let mut lastp = last;
                    let start = *points.last().unwrap();
                    let d = *end - start;
                    let count = (d.magnitude() / resolution).ceil() as i32;
                    (0..=count).for_each(|i| {
                        let t = i as f32 / count as f32;
                        let p = cubic_to_point(t, start, *cp1, *cp2, *end);
                        points.push(p);
                        edges.push((points.len() - 2, points.len() - 1));
                        lastp = p;
                    });
                    last = lastp;
                }
                Contour::ClosePath(close) => {
                    if *close {
                        edges.push((points.len() - 1, 0));
                    }
                }
            }
        }
    }
    Polygon { points, edges }
}

fn style_colors(style: FillStyle) -> (Color, Color, Color, Color) {
    match style {
        Solid(color) => (color, color, color, color),
        FadeDown(color1, color2) => (color2, color1, color1, color2),
        FadeLeft(color1, color2) => (color1, color1, color2, color2),
        Corners(c1, c2, c3, c4) => (c1, c2, c3, c4),
        Radial(c1, c2) => (c1, c1, c2, c2),
    }
}
pub fn rounded_rect_filled(top_left: Vec2, bottom_right: Vec2,radius: f32, style: FillStyle) -> Mesh {
    let (c1,_,_,_) = style_colors(style);
    let inner_tl = top_left + radius;
    let inner_br = bottom_right - radius;
    let inner_rect = rect_filled(inner_tl,inner_br,Solid(c1));
    let top = rect_filled(Vec2::new(inner_tl.x,top_left.y),Vec2::new(inner_br.x,inner_tl.y),style);
    let new_style = match style {
        Solid(_) => style,
        FadeDown(c1, c2) => FadeDown(c2,c1),
        FadeLeft(c1,c2) => FadeLeft(c2,c1),
        Corners(c1,c2,c3,c4) => Corners(c4,c3,c2,c1),
        Radial(c1,c2) => Radial(c2,c1),
    };
    let bottom = rect_filled(Vec2::new(inner_tl.x,inner_br.y),Vec2::new(inner_br.x,bottom_right.y),new_style);
    let new_style = match style {
        Solid(_) => style,
        FadeDown(c1, c2) => FadeLeft(c1,c2),
        FadeLeft(c1,c2) => FadeDown(c1,c2),
        Corners(c1,c2,c3,c4) => Corners(c2,c1,c4,c3),
        Radial(c1,c2) => Radial(c2,c1),
    };
    let left = rect_filled(Vec2::new(top_left.x,inner_tl.y),Vec2::new(inner_tl.x,inner_br.y),new_style);
    let new_style = match new_style {
        Solid(_) => style,
        FadeDown(c1, c2) => FadeDown(c2,c1),
        FadeLeft(c1,c2) => FadeLeft(c2,c1),
        Corners(c1,c2,c3,c4) => Corners(c4,c3,c2,c1),
        Radial(c1,c2) => Radial(c2,c1),
    };
    let right = rect_filled(Vec2::new(inner_br.x,inner_tl.y),Vec2::new(bottom_right.x,inner_br.y),new_style);

    let arcstart = PI;
    let arclen = PI / 2.0;
    let tl = oval_filled(inner_tl,Vec2::new(radius,radius),arcstart,arcstart + arclen,1.0,style);
    let arcstart = PI + arclen;
    let tr = oval_filled(Vec2::new(inner_br.x,inner_tl.y),Vec2::new(radius,radius),arcstart,arcstart + arclen,1.0,style);
    let arcstart = TAU;
    let br = oval_filled(inner_br,Vec2::new(radius,radius),arcstart,arcstart + arclen,1.0,style);
    let arcstart = TAU + arclen;
    let bl = oval_filled(Vec2::new(inner_tl.x,inner_br.y),Vec2::new(radius,radius),arcstart,arcstart + arclen,1.0,style);

    combine(vec![inner_rect,top,bottom,left,right,tl,tr,br,bl])
}
pub fn rounded_rect_outlined(top_left: Vec2, bottom_right: Vec2,radius: f32,thickness: f32, style: FillStyle) -> Mesh {
    let (c1, c2, c3, c4) = style_colors(style);
    let inner_tl = top_left + radius;
    let inner_br = bottom_right - radius;
    let inner_rect = rect_filled(inner_tl,inner_br,style);
    let top = rect_filled(Vec2::new(inner_tl.x,top_left.y),Vec2::new(inner_br.x,inner_tl.y),Solid(Color::GREEN));
    let bottom = rect_filled(Vec2::new(inner_tl.x,inner_br.y),Vec2::new(inner_br.x,bottom_right.y),Solid(Color::RED));
    let left = rect_filled(Vec2::new(top_left.x,inner_tl.y),Vec2::new(inner_tl.x,inner_br.y),Solid(Color::BLUE));
    let right = rect_filled(Vec2::new(inner_br.x,inner_tl.y),Vec2::new(bottom_right.x,inner_br.y),Solid(Color::YELLOW));

    combine(vec![inner_rect,top,bottom,left,right])
}
pub fn rect_filled(top_left: Vec2, bottom_right: Vec2, style: FillStyle) -> Mesh {
    let (c1, c2, c3, c4) = style_colors(style);
    let mut mesh = Mesh::new();
    mesh.vertices = vec![
        Vertex::point(top_left).rgba(c2),
        Vertex::new(bottom_right.x, top_left.y).rgba(c3),
        Vertex::point(bottom_right).rgba(c4),
        Vertex::new(top_left.x, bottom_right.y).rgba(c1),
    ];
    mesh.indices = vec![2, 1, 0, 3, 2, 0];
    mesh
}
pub fn rect_outlined(top_left: Vec2, bottom_right: Vec2, thickness: f32, style: FillStyle) -> Mesh {
    let (c1, c2, c3, c4) = style_colors(style);
    let mut mesh = Mesh::new();
    match style {
        Radial(c1, c3) => {
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
        4, 1, 0, 4, 5, 1, 5, 2, 1, 6, 2, 5, 6, 3, 2, 7, 3, 6, 3, 7, 4, 0, 3, 4,
    ];
    mesh
}
pub fn oval_filled(
    center: Vec2,
    radius: Vec2,
    arc_begin: f32,
    arc_end: f32,
    resolution: f32,
    style: FillStyle,
) -> Mesh {
    let (c1, _, c3, _) = style_colors(style);
    let mut mesh = Mesh::new();
    mesh.vertices = vec![Vertex::point(center).rgba(c1)];
    let start_angle = arc_begin;
    let end_angle = arc_end;
    let arc_length = (end_angle - start_angle).abs() * radius.magnitude();
    let vertex_count = arc_length / resolution;
    mesh.vertices.reserve((vertex_count + 2.0) as usize);
    mesh.indices.reserve((vertex_count * 3.0) as usize);
    let angle_step = (end_angle - start_angle).abs() / vertex_count;
    let mut a = start_angle;
    (0..=(vertex_count as u32 + 1)).for_each(|i| {
        if i <= vertex_count.floor() as u32 {
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * a.cos(), center.y + radius.y * a.sin()).rgba(c3),
            );
        } else {
            mesh.vertices.push(
                Vertex::new(
                    center.x + radius.x * end_angle.cos(),
                    center.y + radius.y * end_angle.sin(),
                )
                .rgba(c3),
            );
            mesh.indices.push(0);
            mesh.indices.push(i + 1);
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
pub fn oval_outlined(
    center: Vec2,
    radius: Vec2,
    arc_begin: f32,
    arc_end: f32,
    resolution: f32,
    thickness: f32,
    style: FillStyle,
) -> Mesh {
    let (c1, _, c3, _) = style_colors(style);
    let mut mesh = Mesh::new();
    let start_angle = arc_begin;
    let end_angle = arc_end;
    let arc_length = (end_angle - start_angle).abs() * radius.magnitude();
    let vertex_count = arc_length / resolution;
    let angle_step = (end_angle - start_angle).abs() / vertex_count;
    mesh.vertices.reserve((vertex_count * 2.0) as usize);
    mesh.indices.reserve((vertex_count * 6.0) as usize);
    let mut a = start_angle;
    (0..=(vertex_count as u32 + 1)).for_each(|i| {
        if i <= vertex_count.floor() as u32 {
            mesh.vertices.push(
                Vertex::new(center.x + radius.x * a.cos(), center.y + radius.y * a.sin()).rgba(c1),
            );
            mesh.vertices.push(
                Vertex::new(
                    center.x + (radius.x - thickness) * a.cos(),
                    center.y + (radius.y - thickness) * a.sin(),
                )
                .rgba(c3),
            );
        } else {
            mesh.vertices.push(
                Vertex::new(
                    center.x + radius.x * end_angle.cos(),
                    center.y + radius.y * end_angle.sin(),
                )
                .rgba(c1),
            );
            mesh.vertices.push(
                Vertex::new(
                    center.x + (radius.x - thickness) * end_angle.cos(),
                    center.y + (radius.y - thickness) * end_angle.sin(),
                )
                .rgba(c3),
            );
        }
        if i > 0 {
            let v0 = i * 2 - 2;
            let v1 = i * 2 - 1;
            let v2 = i * 2;
            let v3 = i * 2 + 1;
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

pub fn line(begin: Vec2, end: Vec2, thickness: f32, style: LineStyle, fill: FillStyle) -> Mesh {
    let (c1, c2, c3, c4) = style_colors(fill);
    let mut m = Mesh::new();
    let width = thickness / 2.0;
    let swap = if begin.y < end.y { -1.0 } else { 1.0 };
    let dx = (begin.x - end.x) * 2.0;
    let dy = (begin.y - end.y) * 2.0;
    let a = (dx / dy).atan();
    let half_pi = PI / 2.0;
    let sa = a - half_pi;
    let sa2 = a + half_pi;

    let bump2 = Vec2::new(width * sa.sin(), width * sa.cos());
    let bump = Vec2::new(width * sa2.sin(), width * sa2.cos());

    match style {
        LineStyle::Center => {
            m.vertices
                .push(Vertex::new(begin.x + bump2.x, begin.y + bump2.y).rgba(c1));
            m.vertices
                .push(Vertex::new(begin.x + bump.x, begin.y + bump.y).rgba(c2));
            m.vertices
                .push(Vertex::new(end.x + bump.x, end.y + bump.y).rgba(c3));
            m.vertices
                .push(Vertex::new(end.x + bump2.x, end.y + bump2.y).rgba(c4));
        }
        LineStyle::Left => {
            m.vertices.push(
                Vertex::new(
                    begin.x + bump2.x * 2.0 * swap,
                    begin.y + bump2.y * 2.0 * swap,
                )
                .rgba(c1),
            );
            m.vertices.push(Vertex::new(begin.x, begin.y).rgba(c2));
            m.vertices.push(Vertex::new(end.x, end.y).rgba(c3));
            m.vertices.push(
                Vertex::new(end.x + bump2.x * 2.0 * swap, end.y + bump2.y * 2.0 * swap).rgba(c4),
            );
        }
        LineStyle::Right => {
            m.vertices.push(Vertex::new(begin.x, begin.y).rgba(c1));
            m.vertices.push(
                Vertex::new(begin.x + bump.x * 2.0 * swap, begin.y + bump.y * 2.0 * swap).rgba(c2),
            );
            m.vertices.push(
                Vertex::new(end.x + bump.x * 2.0 * swap, end.y + bump.y * 2.0 * swap).rgba(c3),
            );
            m.vertices.push(Vertex::new(end.x, end.y).rgba(c4));
        }
    };
    if swap < 0.0 {
        m.indices = vec![2, 1, 0, 3, 2, 0];
    } else {
        m.indices = vec![0, 1, 2, 0, 2, 3];
    }
    m
}
pub fn raw_triangle_filled(p1: Vec2, p2: Vec2, p3: Vec2, c1: Color, c2: Color, c3: Color) -> Mesh {
    let mut m = Mesh::new();
    m.vertices.push(Vertex::new(p1.x, p1.y).rgba(c1));
    m.vertices.push(Vertex::new(p2.x, p2.y).rgba(c2));
    m.vertices.push(Vertex::new(p3.x, p3.y).rgba(c3));
    m.indices = vec![2, 1, 0];
    m
}
pub fn raw_quad_filled(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2, style: FillStyle) -> Mesh {
    let (c1, c2, c3, c4) = style_colors(style);
    let mut m = Mesh::new();
    m.vertices.push(Vertex::new(p1.x, p1.y).rgba(c1));
    m.vertices.push(Vertex::new(p2.x, p2.y).rgba(c2));
    m.vertices.push(Vertex::new(p3.x, p3.y).rgba(c3));
    m.vertices.push(Vertex::new(p4.x, p4.y).rgba(c4));
    m.indices = vec![2, 1, 0, 3, 2, 0];
    m
}
pub fn quadratic_curve(
    begin: Vec2,
    end: Vec2,
    control: Vec2,
    thickness: f32,
    line_style: LineStyle,
    style: FillStyle,
    resolution: f32,
) -> Mesh {
    let mut meshes: Vec<Mesh> = vec![];
    let mut points: Vec<Vec2> = vec![];
    let distance = (end - begin).magnitude();
    let count = (distance / resolution).ceil() as i32;
    (0..=count).for_each(|i| {
        let t = i as f32 / count as f32;
        points.push(quadratic_to_point(t, begin, control, end));
    });
    (0..points.len() - 1).for_each(|i| {
        meshes.push(line(points[i], points[i + 1], thickness, line_style, style));
    });
    combine(meshes)
}
pub fn cubic_curve(
    begin: Vec2,
    control1: Vec2,
    control2: Vec2,
    end: Vec2,
    thickness: f32,
    line_style: LineStyle,
    style: FillStyle,
    resolution: f32,
) -> Mesh {
    let mut meshes: Vec<Mesh> = vec![];
    let mut points: Vec<Vec2> = vec![];
    let distance = (end - begin).magnitude();
    let count = (distance / resolution).ceil() as i32;
    (0..=count).for_each(|i| {
        let t = i as f32 / count as f32;
        points.push(cubic_to_point(t, begin, control1, control2, end));
    });
    (0..points.len() - 1).for_each(|i| {
        meshes.push(line(points[i], points[i + 1], thickness, line_style, style));
    });
    combine(meshes)
}
pub fn fill_path_fan(center: &Vec2, path: &PathData) -> Mesh {
    let mut mb = Mesh::new();
    mb.vertices.push(Vertex::new(center.x, center.y));
    mb.vertices.extend::<Vec<Vertex>>(
        path_to_polygon(path, 1.0)
            .points
            .iter()
            .map(|p| Vertex::new(p.x, p.y))
            .collect(),
    );
    (1..mb.vertices.len()).for_each(|i| {
        let i = i as u32;
        mb.indices.push(0);
        mb.indices.push(i + 1);
        mb.indices.push(i)
    });
    let i = mb.vertices.len() as u32;
    mb.indices.push(0);
    mb.indices.push(1);
    mb.indices.push(i - 1);
    mb
}

pub fn polygon_trapezoid_map(polygon: &Polygon) -> Mesh {
    let mut mb = MeshBuilder::new();
    let mut line_segments: Vec<LineSegment> = vec![];
    for (start, end) in polygon.edges.iter() {
        line_segments.push(LineSegment::new(
            polygon.points[*start],
            polygon.points[*end],
        ));
        mb.push();
        mb.set_style(Solid(Color::RED));
        mb.set_cursor(polygon.points[*start] - Vec2::new(2, 2));
        mb.rect(Vec2::new(4, 4));
        mb.pop()
    }
    let mut rays: Vec<Ray> = vec![];
    for (i, p) in polygon.points.iter().enumerate() {
        let neighbors = polygon.get_vertex_and_neighbors(i);
        if neighbors.len() == 2 {
            match (neighbors[0].x >= p.x, neighbors[1].x >= p.x) {
                (true, true) => {}
                (false, false) => {
                    rays.push(Ray::new(*p, *p + Vec2::new(0, -1)));
                    rays.push(Ray::new(*p, *p + Vec2::new(0, 1)));
                }
                (true, false) => {
                    rays.push(Ray::new(*p, *p + Vec2::new(0, -1)));
                }
                (false, true) => {
                    rays.push(Ray::new(*p, *p + Vec2::new(0, 1)));
                }
            }
        }
    }
    mb.set_style(Solid(Color::GREEN));
    for r in rays.iter() {
        if let Some(hit) = r.cast(&line_segments) {
            mb.line(r.origin, hit.hit);
        }
    }
    mb.build()
}

pub fn combine(mut meshes: Vec<Mesh>) -> Mesh {
    meshes.iter_mut().fold(Mesh::new(), |acc, m| acc.add(m))
}
