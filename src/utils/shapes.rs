use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub fn draw_circunference(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
    let padding = radius / 10;
    for i in 0..padding {
        _draw_circunference(canvas, center_x, center_y, radius - i);
    }

}

 /// draws an unfilled circle requires sdl2
pub fn _draw_circunference(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
    // Mid Point circle algorithm
    // begin at upper perpendicular radius

    let diameter = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;

    let mut tx = 1;
    let mut ty = 1;

    let mut err = tx - diameter;
    let mut point_arr: Vec::<Point> = vec![];
    while x >= y {
        // points for each octanct of the circle
        point_arr.push(Point::new(center_x + x, center_y + y));
        point_arr.push(Point::new(center_x + x, center_y - y));
        point_arr.push(Point::new(center_x - x, center_y + y));
        point_arr.push(Point::new(center_x - x, center_y - y));
        point_arr.push(Point::new(center_x + y, center_y + x));
        point_arr.push(Point::new(center_x + y, center_y - x));
        point_arr.push(Point::new(center_x - y, center_y + x));
        point_arr.push(Point::new(center_x - y, center_y - x));

        if err <= 0 {
            y += 1;
            err += ty;
            ty += 2;
        }
        if err > 0 {
            x -= 1;
            tx += 2;
            err += tx - diameter;
        }
    }
    canvas.draw_points(&point_arr[..]).unwrap();
}

/// draw circle in canvas with current sdl color
pub fn draw_circle(canvas: &mut Canvas<Window>, x0: i32, y0: i32, r: i32) {
    let d: i32 = r * 2;
    for i in 0..d {
        for j in 0..d {
            let dx = r - i;
            let dy = r - j;
            if (dx * dx + dy * dy) <= (r * r) {
                canvas.draw_point(Point::new(x0 + dx, y0 + dy)).expect("failed to draw point in canvas");
            }
        }
    }
}

