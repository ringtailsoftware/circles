use macroquad::prelude::*;
use core::f32::consts::PI;

enum Axis {
    XAxis,
    YAxis,
}

fn sq(n:f32) -> f32 {
    n * n
}

// Given a line o,p and area, return the 4 corners of constant area rectangle
fn map_pos(o:Vec2, p:Vec2, area:f32) -> (Vec2,Vec2,Vec2,Vec2) {
    let len_om = (sq(p.x-o.x) + sq(p.y-o.y)).sqrt();
    let len_mm2 = area / len_om;
    let angle_om = (p.y-o.y).atan2(p.x-o.x);
    (
        vec2(o.x + len_mm2 * (angle_om - PI/2.).cos(), o.y + len_mm2 * (angle_om - PI/2.).sin()),
        vec2(p.x + len_mm2 * (angle_om - PI/2.).cos(), p.y + len_mm2 * (angle_om - PI/2.).sin()),
        vec2(o.x + len_mm2 * (angle_om + PI/2.).cos(), o.y + len_mm2 * (angle_om + PI/2.).sin()),
        vec2(p.x + len_mm2 * (angle_om + PI/2.).cos(), p.y + len_mm2 * (angle_om + PI/2.).sin()),
    )
}

// draw the constant area rectangle
fn draw_area_box(o:Vec2, p1:Vec2, thickness:f32, area:f32) {
    let (p2,p3,p4,p5) = map_pos(o, p1, area);

    draw_line(o.x, o.y, p1.x, p1.y, thickness, YELLOW);
    draw_line(p1.x, p1.y, p3.x, p3.y, thickness, YELLOW);
    draw_line(p3.x, p3.y, p2.x, p2.y, thickness, YELLOW);
    draw_line(p2.x, p2.y, o.x, o.y, thickness, YELLOW);
    draw_circle(p2.x, p2.y, thickness * 3., WHITE);

    draw_line(o.x, o.y, p1.x, p1.y, thickness, YELLOW);
    draw_line(p1.x, p1.y, p5.x, p5.y, thickness, YELLOW);
    draw_line(p5.x, p5.y, p4.x, p4.y, thickness, YELLOW);
    draw_line(p4.x, p4.y, o.x, o.y, thickness, YELLOW);
    draw_circle(p4.x, p4.y, thickness * 3., WHITE);
}

// draw an axis line
fn draw_axis(srcr:Rect, thickness:f32, axis:Axis) {
    match axis {
        Axis::XAxis => {
            draw_line(-srcr.w, 0., srcr.w, 0., thickness, WHITE);
        }
        Axis::YAxis => {
            draw_line(0., -srcr.h, 0., srcr.h, thickness, WHITE);
        }
    }
}

// find circle touching 3 points
fn find_circle(pt_a:Vec2, pt_b:Vec2, pt_c:Vec2) -> (Vec2, f32) {
    let x1 = pt_a.x;
    let y1 = pt_a.y;
    let x2 = pt_b.x;
    let y2 = pt_b.y;
    let x3 = pt_c.x;
    let y3 = pt_c.y;
    let x12 = x1 - x2;
    let x13 = x1 - x3;
    let y12 = y1 - y2;
    let y13 = y1 - y3;
    let y31 = y3 - y1;
    let y21 = y2 - y1;
    let x31 = x3 - x1;
    let x21 = x2 - x1;
    let sx13 = sq(x1) - sq(x3);
    let sy13 = sq(y1) - sq(y3);
    let sx21 = sq(x2) - sq(x1);
    let sy21 = sq(y2) - sq(y1);
 
    let f = ((sx13) * (x12)
            + (sy13) * (x12)
            + (sx21) * (x13)
            + (sy21) * (x13))
            / (2. * ((y31) * (x12) - (y21) * (x13)));
    let g = ((sx13) * (y12)
            + (sy13) * (y12)
            + (sx21) * (y13)
            + (sy21) * (y13))
            / (2. * ((x31) * (y12) - (x21) * (y13)));
 
    let c = -sq(x1) - sq(y1) - 2. * g * x1 - 2. * f * y1;
 
    // eqn of circle be x^2 + y^2 + 2*g*x + 2*f*y + c = 0
    // where centre is (h = -g, k = -f) and radius r
    // as r^2 = h^2 + k^2 - c
    let h = -g;
    let k = -f;
    let sqr_of_r = h * h + k * k - c;
 
    // r is the radius
    let r = sqr_of_r.sqrt();
    (vec2(h, k), r)
}

// given circle defined by (c,r), map, find the circles traced by corners of constant area rect
fn map_circle(o:Vec2, area:f32, c:Vec2, r:f32) -> (Vec2, f32, Vec2, f32) {
    // 3 points on circle
    let pt_a = vec2(c.x + r, c.y);
    let pt_b = vec2(c.x - r, c.y);
    let pt_c = vec2(c.x, c.y + r);

    let (pt_a2, _, pt_a3, _) = map_pos(o, pt_a, area);
    let (pt_b2, _, pt_b3, _) = map_pos(o, pt_b, area);
    let (pt_c2, _, pt_c3, _) = map_pos(o, pt_c, area);

    let (c1, r1) = find_circle(pt_a2, pt_b2, pt_c2);
    let (c2, r2) = find_circle(pt_a3, pt_b3, pt_c3);

    (c1, r1, c2, r2)
}

// draw line and mapped version
fn draw_line_pair(o:Vec2, area:f32, p1:Vec2, p2:Vec2, thickness:f32) {
    draw_line(p1.x, p1.y, p2.x, p2.y, thickness, WHITE);
    
    // 3 points on the line
    let pt_a = p1;
    let pt_b = p2;
    let pt_c = vec2((p1.x + p2.x)/2., (p1.y + p2.y)/2.);

    // map
    let pt_a2 = map_pos(o, pt_a, area).0;
    let pt_b2 = map_pos(o, pt_b, area).0;
    let pt_c2 = map_pos(o, pt_c, area).0;

    let (c, r) = find_circle(pt_a2, pt_b2, pt_c2);
    draw_circle(c.x, c.y, r, BLUE);

}

// draw circle and mapped circle
fn draw_circle_pair(o:Vec2, area:f32, c:Vec2, r:f32) {
    let (c2, r2, c3, r3) = map_circle(o, area, vec2(c.x, c.y), r);
    draw_circle(c2.x, c2.y, r2, GREEN);
    draw_circle(c3.x, c3.y, r3, GREEN);

    // then map is again - which produces the mirror image
    let (c4, r4, _, _) = map_circle(o, area, vec2(c2.x, c2.y), r2);
    draw_circle(c4.x, c4.y, r4, RED);
    draw_circle(c.x, c.y, r, RED);
}

// draw a set of circles
fn draw_pattern(o:Vec2, area:f32, thickness:f32, anim:bool) {
    let r = 0.25;
    let mut xoff = 0.;

    draw_line_pair(o, area, vec2(o.x - 10., -r), vec2(o.x + 10., -r), thickness);
    draw_line_pair(o, area, vec2(o.x - 10., r), vec2(o.x + 10., r), thickness);

    // animate by adding up to 2*r to x
    if anim {
        xoff = ((((get_time() * 10000.0) % 100000.0) / 100000.0) as f32) * r * 2.;
    }

    for y in (2 .. 20).step_by(2) {
        for x in (-20 .. 21).step_by(2) {
            draw_circle_pair(o, area, vec2(-r*(x as f32) + xoff, -r*(y as f32)), r);
        }
    }
}

// draw everything
fn render(srcr:Rect, dstr:Rect, rt:RenderTarget, anim:bool) {
    let area = 0.1;

    set_default_camera();

    let mousepos = mouse_position();

    // show dstr
    draw_rectangle(dstr.x, dstr.y, dstr.w, dstr.h, BLACK);

    let mut camera = Camera2D::from_display_rect(Rect {x: srcr.x, y: srcr.y, w: srcr.w, h: srcr.h});
    camera.render_target = Some(rt);
    set_camera(&camera);
    clear_background(BLACK);

    // axes
    let thickness = srcr.h / dstr.h;
    draw_axis(srcr, thickness, Axis::XAxis);
    draw_axis(srcr, thickness, Axis::YAxis);

    draw_pattern(vec2(0., 0.), area, thickness, anim);

    if dstr.contains(vec2(mousepos.0, mousepos.1)) {
        let mouse_rel_dst = vec2(mousepos.0 - dstr.x, dstr.h - (mousepos.1 - dstr.y));
        let m2 = vec2((mouse_rel_dst.x / dstr.w) * srcr.w + srcr.x,
                      (mouse_rel_dst.y / dstr.h) * srcr.h + srcr.y);
        draw_circle(m2.x, m2.y, thickness * 3., WHITE);
        draw_area_box(vec2(0., 0.), m2, srcr.h / dstr.h, area);
    }

    set_default_camera();

    draw_texture(rt.texture, dstr.x, dstr.y, WHITE);
}

#[macroquad::main(window_conf)]

async fn main() {
    // destination rect in screen space
    let border = 10.;
    let dstr1 = Rect {x: border, y: border, w: 800.-border*2., h: 800.-border*2.};
    let rt1 = render_target(dstr1.w as u32, dstr1.h as u32);

    let mut centre = vec2(0., 0.);
    let mut dimen = vec2(2., 2.);   // -1 to +1
    let mut anim = true;

    loop {
        // set srcr in world space
        let srcr = Rect {x: centre.x - dimen.x/2., y: centre.y - dimen.y/2., w: dimen.x, h: dimen.y};

        clear_background(BLACK);
        draw_rectangle(dstr1.x-1., dstr1.y-1., dstr1.w+2., dstr1.h+2., WHITE);
        render(srcr, dstr1, rt1, anim);

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            return;
        }

        let thickness = srcr.h / dstr1.h;

        if is_key_down(KeyCode::Equal) {
            dimen.x *= 0.95;
            dimen.y *= 0.95;
        }
        if is_key_down(KeyCode::Minus) {
            dimen.x *= 1.05;
            dimen.y *= 1.05;
        }

        if is_key_pressed(KeyCode::Space) {
            anim = !anim;
        }

        if is_key_down(KeyCode::Left) {
            centre.x -= thickness;
        }
        if is_key_down(KeyCode::Right) {
            centre.x += thickness;
        }
        if is_key_down(KeyCode::Up) {
            centre.y += thickness;
        }
        if is_key_down(KeyCode::Down) {
            centre.y -= thickness;
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Circles".to_string(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

