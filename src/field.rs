use ndarray::Array2 as arr2;
use rayon::prelude::*;
use sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

const EPS: f64 = 0.0000001;
#[derive(Debug)]
pub enum PixelType {
    House,
    Food,
    Obsticale,
    Empty,
}

fn scent_to_color(scent: f64) -> u8 {
    let mut x = scent * 1000.0;
    if x > 10.0 {
        x = 10.0;
    }
    x = x.atan() * 500.0 / 3.141592;
    x as u8
}

pub struct Field {
    width: usize,
    height: usize,
    house: Vec<Rect>,
    food: Vec<Rect>,
    obsticales: Vec<Rect>,
    house_scent_map: Vec<arr2<f64>>,
    food_scent_map: Vec<arr2<f64>>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let my_field = Field {
            width: width,
            height: height,
            house: Vec::new(),
            food: Vec::new(),
            obsticales: Vec::new(),
            house_scent_map: vec![arr2::<f64>::zeros((width, height)); 8],
            food_scent_map: vec![arr2::<f64>::zeros((width, height)); 8],
        };
        my_field
    }

    pub fn scent_decrease(&mut self) {
        self.food_scent_map.par_iter_mut().for_each(|map_| {
            map_.par_map_inplace(|x| {
                *x *= 0.995;
            })
        });
        self.house_scent_map.par_iter_mut().for_each(|map_| {
            map_.par_map_inplace(|x| {
                *x *= 0.995;
            })
        });
    }

    pub fn scent_from_ant(&mut self, ant: &crate::ants::Ant) {
        let field_to_update = if ant.has_food {
            &mut self.food_scent_map
        } else {
            &mut self.house_scent_map
        };
        let scent = 10000.0 / ((ant.distance_passed * ant.distance_passed) as f64);
        if let Some(iter) =
            field_to_update[ant.dir].get_mut([ant.point.x as usize, ant.point.y as usize])
        {
            *iter += scent;
        }
    }

    pub fn goal_near(&self, position: Point, looking_for_house: bool) -> Option<Point> {
        let goal_map = if looking_for_house {
            &self.house
        } else {
            &self.food
        };
        for dx in -1..2 {
            for dy in -1..2 {
                let x = position.x + dx;
                let y = position.y + dy;
                for object in goal_map {
                    if object.contains_point(Point::new(x as i32, y as i32)) {
                        return Some(Point::new(dx, dy));
                    }
                }
            }
        }
        None
    }

    pub fn sniff(&self, position: Point, looking_for_house: bool) -> Option<Vec<f64>> {
        let map_to_sniff = if looking_for_house {
            &self.house_scent_map
        } else {
            &self.food_scent_map
        };
        let x = position.x as usize;
        let y = position.y as usize;
        if let Some(_) = map_to_sniff[0].get([x, y]) {
            let smells = map_to_sniff
                .iter()
                .map(|map_| map_[[x, y]])
                .collect::<Vec<f64>>();
            if smells.iter().sum::<f64>() <= EPS {
                return None;
            }
            Some(smells)
        } else {
            None
        }
    }

    pub fn paint(&self, canvas: &mut Canvas<Window>, scale: i32) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let mut food_heat_map: arr2<f64> = arr2::<f64>::zeros((self.width, self.height));
        let mut house_heat_map: arr2<f64> = arr2::<f64>::zeros((self.width, self.height));
        for i in 0..8 {
            food_heat_map += &self.food_scent_map[i];
            house_heat_map += &self.house_scent_map[i];
        }
        for h in 0..self.height {
            for w in 0..self.width {
                let food_color = scent_to_color(food_heat_map[[w, h]]);
                let house_color = scent_to_color(house_heat_map[[w, h]]);
                let red = if food_color > house_color {
                    food_color
                } else {
                    house_color
                };
                let blue = house_color;
                let green = food_color;
                canvas.set_draw_color(sdl2::pixels::Color::RGB(red, green, blue));
                canvas.fill_rect(Rect::new(
                    w as i32 * scale,
                    h as i32 * scale,
                    scale as u32,
                    scale as u32,
                ));
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        self.house.iter().for_each(|&rect| {
            canvas.fill_rect(Rect::new(
                rect.x * scale,
                rect.y * scale,
                rect.width() * scale as u32,
                rect.height() * scale as u32,
            ));
        });
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        self.food.iter().for_each(|&rect| {
            canvas.fill_rect(Rect::new(
                rect.x * scale,
                rect.y * scale,
                rect.width() * scale as u32,
                rect.height() * scale as u32,
            ));
        });
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        self.obsticales.iter().for_each(|&rect| {
            canvas.fill_rect(Rect::new(
                rect.x * scale,
                rect.y * scale,
                rect.width() * scale as u32,
                rect.height() * scale as u32,
            ));
        });
    }

    pub fn add_food(&mut self, food_: Rect) {
        self.food.push(food_);
    }
    pub fn add_house(&mut self, house_: Rect) {
        self.house.push(house_);
    }
    pub fn add_obsticale(&mut self, obs: Rect) {
        self.obsticales.push(obs);
    }
    pub fn check_position(&self, position: Point) -> PixelType {
        if position.x < 0 || position.x >= self.width as i32 {
            return PixelType::Obsticale;
        }
        if position.y < 0 || position.y >= self.height as i32 {
            return PixelType::Obsticale;
        }
        for obsticale in &self.obsticales {
            if obsticale.contains_point(position) {
                return PixelType::Obsticale;
            }
        }
        for food_ in &self.food {
            if food_.contains_point(position) {
                return PixelType::Food;
            }
        }
        for house_ in &self.house {
            if house_.contains_point(position) {
                return PixelType::House;
            }
        }
        PixelType::Empty
    }
}
