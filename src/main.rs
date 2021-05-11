use rand::thread_rng;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
mod ants;
mod field;

const HEIGHT: u32 = 900;
const WIDTH: u32 = 1600;

fn hello(field: &mut crate::field::Field) -> Point {
    field.add_obsticale(Rect::new(50, 80, 10, 300));
    field.add_obsticale(Rect::new(150, 80, 10, 300));
    field.add_obsticale(Rect::new(60, 230, 90, 10));

    field.add_obsticale(Rect::new(200, 80, 10, 300));
    field.add_obsticale(Rect::new(210, 80, 90, 10));
    field.add_obsticale(Rect::new(210, 230, 90, 10));
    field.add_obsticale(Rect::new(210, 370, 90, 10));

    field.add_obsticale(Rect::new(350, 80, 10, 300));
    field.add_obsticale(Rect::new(360, 370, 90, 10));

    field.add_obsticale(Rect::new(500, 80, 10, 300));
    field.add_obsticale(Rect::new(510, 370, 90, 10));

    field.add_obsticale(Rect::new(650, 80, 10, 300));
    field.add_obsticale(Rect::new(750, 80, 10, 300));
    field.add_obsticale(Rect::new(660, 80, 90, 10));
    field.add_obsticale(Rect::new(660, 370, 90, 10));

    field.add_house(Rect::new(100, 250, 10, 10));
    field.add_food(Rect::new(100, 200, 10, 10));
    field.add_food(Rect::new(520, 350, 10, 10));
    Point::new(105, 255)
}

fn flappy_bird(field: &mut crate::field::Field) -> Point {
    let mut x: u32 = 0;
    let n = 5;
    let dx = 800 / n;
    for i in 0..n {
        x = 50 + rand::random::<u32>() % 300;
        field.add_obsticale(Rect::new(i * dx + 70, 0, 10, x - 50));
        field.add_obsticale(Rect::new(i * dx + 70, x as i32, 10, 450 - x));
    }
    field.add_house(Rect::new(30, 220, 10, 10));
    field.add_food(Rect::new(770, 220, 10, 10));
    Point::new(35, 225)
}

fn main() {
    let sdl_context = sdl2::init().expect("Sdl2 failed to initialize");
    let video_subsystem = sdl_context.video().expect("Video subsystem failed to load");
    let window = video_subsystem
        .window("Ants", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create a window");
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Faild to build canvas");
    canvas.clear();
    let scale = 2;
    let mut my_field = crate::field::Field::new(
        (WIDTH / scale as u32) as usize,
        (HEIGHT / scale as u32) as usize,
    );
    //let start = hello(&mut my_field);
    let start = flappy_bird(&mut my_field);
    let mut my_population = crate::ants::Population::new();
    for _ in 0..10000 {
        my_population.add_ant(start.x, start.y);
    }
    my_field.paint(&mut canvas, scale);
    canvas.present();
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get the pool of events");
    'outer: loop {
        my_population.ants_move(&mut my_field);
        my_field.scent_decrease();
        my_population.add_scent_to_field(&mut my_field);
        my_population.paint(&mut canvas, scale);
        my_population.reborn_of_old_ants(10000, start, &mut canvas, scale);
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'outer;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'outer;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    my_field.paint(&mut canvas, scale);
                }
                _ => {}
            }
        }
    }
}
