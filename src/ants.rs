use lazy_static;
use rand::prelude::*;
use rand::thread_rng;
use rayon::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

lazy_static::lazy_static! {
    static ref DIRECTIONS: Vec<Point> = vec![Point::new(1,1),
                                             Point::new(1,0),
                                             Point::new(1,-1),
                                             Point::new(0,-1),
                                             Point::new(-1,-1),
                                             Point::new(-1,0),
                                             Point::new(-1,1),
                                             Point::new(0,1)
                                       ];
    static ref RAND_DISTR: Vec<usize> = vec![10000,200,4,2,1];
}

fn coin_flip() -> i32 {
    let size: usize = RAND_DISTR.iter().sum();
    let mut coin: usize = rand::random::<usize>() % size;
    let mut index = 0;
    while coin > 0 {
        if coin <= RAND_DISTR[index] {
            break;
        }
        coin -= RAND_DISTR[index];
        index += 1;
    }
    index as i32
}

fn random_choice(prob: &Vec<f64>) -> usize {
    let total: f64 = prob.iter().sum();
    let mut rng = rand::thread_rng();
    let mut coin: f64 = rng.gen::<f64>() * total;
    let mut index = 0;
    while coin > 0.0 {
        if coin < prob[index] {
            return index;
        }
        coin -= prob[index];
        index += 1;
    }
    index
}

pub struct Ant {
    pub point: Point,
    pub dir: usize,
    pub has_food: bool,
    pub distance_passed: f64,
    pub is_random: bool,
    pub left_house: bool,
}

impl Ant {
    fn random_move(&self) -> usize {
        let sign = if rand::random::<bool>() { 1 } else { -1 };
        let new_dir = (self.dir + (sign * coin_flip() + 16) as usize) % 8;
        new_dir
    }

    fn react_to_pixel(&mut self, pixel_type: crate::field::PixelType, dir: usize) -> bool {
        match pixel_type {
            crate::field::PixelType::Empty => {
                self.move_in_dir(dir);
                if !self.left_house {
                    self.left_house = true;
                }
                true
            }
            crate::field::PixelType::Food => {
                self.distance_passed = 1.0;
                if !self.has_food {
                    self.has_food = true;
                    true
                } else {
                    false
                }
            }
            crate::field::PixelType::House => {
                self.distance_passed = 1.0;
                if self.has_food {
                    self.has_food = false;
                    true
                } else if !self.left_house {
                    self.move_in_dir(dir);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn make_random_move(&mut self, field: &crate::field::Field) {
        self.random_move();
        let mut tries_count = 0;
        for _ in 0..10 {
            let dir = self.random_move();
            let pixel_type = field.check_position(self.point + DIRECTIONS[dir]);
            if self.react_to_pixel(pixel_type, dir) {
                break;
            }
            tries_count += 1;
        }
        if tries_count >= 10 {
            let mut rng = thread_rng();
            let mut indexes = [0, 1, 2, 3, 4, 5, 6, 7];
            indexes.shuffle(&mut rng);
            for i in 0..8 {
                let dir = (self.dir + indexes[i]) % 8;
                let pixel_type = field.check_position(self.point + DIRECTIONS[dir]);
                if self.react_to_pixel(pixel_type, dir) {
                    break;
                }
            }
        }
    }

    pub fn make_move(&mut self, field: &crate::field::Field) {
        let mut dir = 8;
        if let Some(point) = field.goal_near(self.point, self.has_food) {
            for i in 0..8 {
                if DIRECTIONS[i].x == point.x && DIRECTIONS[i].y == point.y {
                    dir = i;
                    break;
                }
            }
        }
        if dir == 8 {
            if !self.is_random {
                if let Some(smells) = field.sniff(self.point, self.has_food) {
                    dir = (random_choice(&smells) + 4) % 8;
                }
            }
        }
        if dir < 8 {
            let pixel_type = field.check_position(self.point + DIRECTIONS[dir]);
            if !self.react_to_pixel(pixel_type, dir) {
                self.make_random_move(field);
            }
        } else {
            self.make_random_move(field);
        }
        self.distance_passed += 1.0;
    }

    pub fn move_in_dir(&mut self, dir: usize) {
        self.dir = dir % 8;
        self.point += DIRECTIONS[self.dir];
    }

    pub fn add_scent(&self, field: &mut crate::field::Field) {
        field.scent_from_ant(self);
    }
}

pub struct Population {
    pub ants: Vec<Ant>,
}

impl Population {
    pub fn new() -> Self {
        Population { ants: Vec::new() }
    }
    pub fn add_ant(&mut self, house_width: i32, house_height: i32) {
        let is_random = (rand::random::<usize>() % 1000) < 100;
        let ant = Ant {
            point: Point::new(house_width, house_height),
            dir: rand::random::<usize>() % 8,
            has_food: false,
            distance_passed: 1.0,
            is_random: is_random,
            left_house: false,
        };
        self.ants.push(ant);
    }

    pub fn ants_move(&mut self, field: &crate::field::Field) {
        self.ants
            .par_iter_mut()
            .for_each(|ant| ant.make_move(field));
    }

    pub fn paint(&self, canvas: &mut Canvas<Window>, scale: i32) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        for ant in &self.ants {
            if ant.left_house {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.fill_rect(Rect::new(
                    (ant.point.x - DIRECTIONS[ant.dir].x) * scale,
                    (ant.point.y - DIRECTIONS[ant.dir].y) * scale,
                    scale as u32,
                    scale as u32,
                ));
                let color = if ant.has_food {
                    sdl2::pixels::Color::RGB(0, 255, 0)
                } else {
                    sdl2::pixels::Color::RGB(255, 0, 0)
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(
                    ant.point.x * scale,
                    ant.point.y * scale,
                    scale as u32,
                    scale as u32,
                ));
            }
        }
    }

    pub fn add_scent_to_field(&mut self, field: &mut crate::field::Field) {
        self.ants.iter().for_each(|ant| ant.add_scent(field));
    }

    pub fn reborn_of_old_ants(
        &mut self,
        ttl: usize,
        newborn_position: Point,
        canvas: &mut Canvas<Window>,
        scale: i32,
    ) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.ants
            .iter_mut()
            .filter(|ant| ant.distance_passed > ttl as f64)
            .for_each(|ant| {
                if (rand::random::<usize>() % 1000) < 2 {
                    canvas.fill_rect(Rect::new(
                        ant.point.x * scale,
                        ant.point.y * scale,
                        scale as u32,
                        scale as u32,
                    ));
                    ant.point = newborn_position;
                    ant.distance_passed = 1.0;
                    ant.has_food = false;
                    ant.is_random = (rand::random::<usize>() % 1000) < 100;
                    ant.left_house = false;
                }
            });
    }
}
