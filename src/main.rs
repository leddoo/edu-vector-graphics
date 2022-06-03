#[derive(Clone, Copy, Debug)]
struct V2 (f32, f32);

#[derive(Clone, Copy, Debug)]
struct Line (V2, V2);


impl Line {
    fn direction(self) -> V2 {
        V2(self.1.0 - self.0.0, self.1.1 - self.0.1)
    }

    fn intersect(self, other: Line) -> Option<(f32, f32)> {
        /*  compute start points `p` and directions `d`,
            such that the lines are given by: `a_p + t*a_d` and `b_p + u*b_d`,
            for numbers t, u.
        */

        let a_p = self.0;
        let a_d = self.direction();

        let b_p = other.0;
        let b_d = other.direction();

        /*  to find the intersection, solve:
                a_p + t*a_d = b_p + u*b_d

            rearrange:
                t*a_d - u*b_d = b_p - a_p

            matrixify:
                |a_d.x  -b_d.x| * |t| = b_p - a_p
                |a_d.y  -b_d.y|   |u|

            invert the matrix:
                |a_d.x  -b_d.x| ^ -1  = 1/det * |-b_d.y  b_d.x|
                |a_d.y  -b_d.y|                 |-a_d.y  a_d.x|
                where det = (a_d.x*(-b_d.y) - (-b_d.x)*a_d.y)

            thus:
                |t| = 1/det * |-b_d.y  b_d.x| * |(b_p.x - a_p.x)|
                |u|           |-a_d.y  a_d.x|   |(b_p.y - a_p.y)|
        */

        let det = a_d.0*(-b_d.1) - (-b_d.0)*a_d.1;
        if det == 0.0 {
            return None
        }

        let t = 1.0/det * (-b_d.1*(b_p.0 - a_p.0) + b_d.0*(b_p.1 - a_p.1));
        let u = 1.0/det * (-a_d.1*(b_p.0 - a_p.0) + a_d.0*(b_p.1 - a_p.1));
        Some((t, u))
    }

    fn make_thicc(self, width: f32) -> Vec<Line> {
        let direction = self.direction();
        let left = V2(-direction.1, direction.0);

        let length = (left.0*left.0 + left.1*left.1).sqrt();
        let normal = V2(left.0/length, left.1/length);

        let width = width/2.0;

        let p0 = V2(self.0.0 + width*normal.0, self.0.1 + width*normal.1);
        let p1 = V2(self.1.0 + width*normal.0, self.1.1 + width*normal.1);
        let p2 = V2(self.1.0 - width*normal.0, self.1.1 - width*normal.1);
        let p3 = V2(self.0.0 - width*normal.0, self.0.1 - width*normal.1);
        vec![
            Line(p0, p1),
            Line(p1, p2),
            Line(p2, p3),
            Line(p3, p0),
        ]
    }
}


fn compute_winding(path: &Vec<Line>, x: f32, y: f32) -> i32 {
    // shoot a ray towards negative infinity in x.
    let ray = Line(V2(x, y), V2(x - 1.0, y));

    let mut winding = 0;
    for line in path {
        // do the lines intersect?
        if let Some((t, u)) = ray.intersect(*line) {

            // do the ray/segment actually intersect?
            let ray_hit     = t >= 0.0;
            let segment_hit = u >= 0.0 && u <= 1.0;
            if ray_hit && segment_hit {
                // line goes up -> positive winding, else negative.
                let delta = if line.1.1 >= line.0.1 { 1 } else { -1 };
                winding += delta;
            }
        }
    }

    winding
}


enum FillRule {
    NonZero,
    EvenOdd,
}

fn rasterize(path: &Vec<Line>, w: u32, h: u32, fill_rule: FillRule) {
    for y in 0..h {
        for x in 0..w {
            let winding = compute_winding(path, x as f32 + 0.5, y as f32 + 0.5);

            let filled = match fill_rule {
                FillRule::NonZero => winding != 0,
                FillRule::EvenOdd => winding % 2 != 0,
            };
            print!("{}", if filled { "#" } else { "." });
        }
        println!();
    }
    println!()
}


fn fill(path: &Vec<Line>, w: u32, h: u32, fill_rule: FillRule) {
    rasterize(path, w, h, fill_rule);
}

fn stroke(path: &Vec<Line>, w: u32, h: u32, stroke_width: f32) {
    let mut stroke = vec![];
    for line in path {
        stroke.extend(line.make_thicc(stroke_width));
    }
    fill(&stroke, w, h, FillRule::NonZero);
}


fn main() {
    fill(&vec![
        Line(V2( 2.0, 9.0), V2(15.0, 1.0)),
        Line(V2(15.0, 1.0), V2(28.0, 9.0)),
        Line(V2(28.0, 9.0), V2( 2.0, 9.0)),

        Line(V2( 7.0, 8.0), V2(15.0, 6.0)),
        Line(V2(15.0, 6.0), V2(23.0, 8.0)),
        Line(V2(23.0, 8.0), V2(15.0, 3.0)),
        Line(V2(15.0, 3.0), V2( 7.0, 8.0)),
    ], 30, 10, FillRule::NonZero);

    let eight = vec![
        Line(V2( 4.0,  2.0), V2(16.0,  2.0)),
        Line(V2(16.0,  2.0), V2(16.0,  6.0)),
        Line(V2(16.0,  6.0), V2( 6.0,  9.0)),
        Line(V2( 6.0,  9.0), V2( 6.0, 13.0)),
        Line(V2( 6.0, 13.0), V2(14.0, 13.0)),
        Line(V2(14.0, 13.0), V2(14.0,  9.0)),
        Line(V2(14.0,  9.0), V2( 4.0,  6.0)),
        Line(V2( 4.0,  6.0), V2( 4.0,  2.0)),

        Line(V2( 6.0,  6.0), V2(16.0,  9.0)),
        Line(V2(16.0,  9.0), V2(16.0, 14.0)),
        Line(V2(16.0, 14.0), V2( 4.0, 14.0)),
        Line(V2( 4.0, 14.0), V2( 4.0,  9.0)),
        Line(V2( 4.0,  9.0), V2(14.0,  6.0)),
        Line(V2(14.0,  6.0), V2(14.0,  3.0)),
        Line(V2(14.0,  3.0), V2( 6.0,  3.0)),
        Line(V2( 6.0,  3.0), V2( 6.0,  6.0)),
    ];
    fill(&eight, 20, 16, FillRule::EvenOdd);
    fill(&eight, 20, 16, FillRule::NonZero);

    stroke(&vec![
        Line(V2(3.5, 2.0), V2(3.5, 7.0)),
        Line(V2(3.5, 4.5), V2(7.5, 4.5)),
        Line(V2(7.5, 2.0), V2(7.5, 7.0)),

        Line(V2(11.5, 2.0), V2(11.5, 7.0)),
    ], 15, 9, 1.0);
}

