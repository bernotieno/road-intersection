use std::rc::Rc;

use rand::Rng;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{lane::Stage, Cross, Settings};

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub position: Point,
    pub color: Color,
    pub route: Route,
    pub stop_point: Point,
    pub destination: Route,
    pub velocity: i32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
    pub stage: Stage,
    pub lane: Cross,
    settings: Rc<Settings>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Route {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Vehicle {
    pub fn new(route: Route, velocity: i32, settings: Rc<Settings>, stop_point: Point, lane: Cross) -> Self {
        let (color, destination) = Self::random(route);
        Self {
            position: Point::new(0, 0),
            color,
            destination,
            route,
            velocity,
            is_changed_direction: false,
            is_stopped: false,
            settings,
            lane,
            stage:Stage::Waiting,
            stop_point
        }
    }

    pub fn has_reached_end(&self) -> bool {
        let border_x = self.position.x < -self.settings.vehicle
            || self.position.x > self.settings.width + self.settings.vehicle;
        let border_y = self.position.y < -self.settings.vehicle
            || self.position.y > self.settings.height + self.settings.vehicle;

        border_x || border_y
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.position.x as f64 - other.position.x as f64;
        let dy = self.position.y as f64 - other.position.y as f64;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    pub fn distance_to(&self, point: Point) -> f64 {
        let dx = self.position.x as f64 - point.x as f64;
        let dy = self.position.y as f64 - point.y as f64;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }

    pub fn resume(&mut self) {
        self.is_stopped = false;
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        if self.stage == Stage::Crossing && self.is_stopped {
            self.is_stopped = false;
        }

        if !self.is_stopped {
            self.move_forward();
        }

        canvas.set_draw_color(self.color);
        let rect = Rect::new(
            self.position.x,
            self.position.y,
            self.settings.vehicle as u32,
            self.settings.vehicle as u32,
        );
        canvas.fill_rect(rect).unwrap();
    }

    pub fn random(route: Route) -> (Color, Route) {
        let mut rng = rand::thread_rng();
        let color = match rng.gen_range(0, 3) {
            0 => Color::GREEN,
            1 => Color::BLUE,
            _ => Color::YELLOW,
        };

        let destination = match color {
            Color::YELLOW => match route {
                Route::Up => Route::Left,
                Route::Down => Route::Right,
                Route::Left => Route::Down,
                _ => Route::Up,
            },
            Color::BLUE => route,
            Color::GREEN => match route {
                Route::Up => Route::Right,
                Route::Down => Route::Left,
                Route::Right => Route::Right,
                _ => Route::Up,
            },
            _ => Route::Up, // Default route for other colors
        };
        (color, destination)
    }

    pub fn s(&mut self) {
        match self.lane {
            Cross::First => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_fourth {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),
                    
                }
            },
            Cross::Second => {
                match self.color {
                    Color::BLUE | Color::GREEN  => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),
                    
                }
            },
            Cross::Third => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_fourth {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),
                    
                }
            },
            Cross::Fourth => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),
                    
                }
            },
        }
    }

    pub fn spawn(&mut self, direction: Route) {
        match direction {
            Route::Up => {
                self.position = self.settings.appearance_vehicle_up;
            }
            Route::Down => {
                self.position = self.settings.appearance_vehicle_down;
            }
            Route::Left => {
                self.position = self.settings.appearance_vehicle_left;
            }
            Route::Right => {
                self.position = self.settings.appearance_vehicle_right;
            }
            _ => (),
        }
    }

    pub fn move_forward(&mut self) {
        if self.is_stopped {
            return;
        };

        // if self.position == self.stop_point {
        //     self.stage = Stage::Crossing;
        // }
        self.s();

        match self.route {
            Route::Up => {
                if !self.is_changed_direction {
                    self.position.y -= self.velocity
                } else {
                    let d = if self.destination == Route::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == self.settings.change_direction_1.y)
                    && (self.destination == Route::Left)
                    || self.destination == Route::Right
                        && (self.position.y == self.settings.change_direction_2.y)
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Down => {
                if !self.is_changed_direction {
                    self.position.y += self.velocity
                } else {
                    let d = if self.destination == Route::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == self.settings.change_direction_2.y)
                    && (self.destination == Route::Right)
                    || (self.position.y == self.settings.change_direction_1.y)
                        && self.destination == Route::Left
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Left => {
                if !self.is_changed_direction {
                    self.position.x -= self.velocity
                } else {
                    let d = if self.destination == Route::Down {
                        1
                    } else {
                        -1
                    };
                    self.position.y += d * self.velocity;
                };

                if self.destination == Route::Down
                    && self.position.x == self.settings.change_direction_1.x
                    || self.destination == Route::Up
                        && self.position.x == self.settings.change_direction_2.x
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Right => {
                if !self.is_changed_direction {
                    self.position.x += self.velocity
                } else {
                    self.position.y -= self.velocity;
                };

                if self.destination == Route::Up
                    && self.position.x == self.settings.change_direction_2.x
                {
                    self.is_changed_direction = true;
                };
            }
            _ => (),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use sdl2::pixels::Color;

    #[cfg(test)]
fn mock_settings() -> Rc<Settings> {
    Rc::new(Settings {
        width: 800,
        height: 600,
        vehicle: 5,
        gap: 10,
        safety_distance: 1.5,
        offset_road: 20,

        horizontal_road_1: 100,
        vertical_road_1: 150,
        horizontal_road_2: 200,
        vertical_road_2: 250,

        appearance_vehicle_up: Point::new(10, 0),
        appearance_vehicle_down: Point::new(10, 100),
        appearance_vehicle_left: Point::new(0, 50),
        appearance_vehicle_right: Point::new(100, 50),

        change_direction_1: Point::new(30, 30),
        change_direction_2: Point::new(60, 60),

        stop_point_first: Point::new(5, 5),
        stop_point_second: Point::new(15, 15),
        stop_point_third: Point::new(25, 25),
        stop_point_fourth: Point::new(35, 35),

        dis_vehicle_first: Point::new(1, 1),
        dis_vehicle_second: Point::new(2, 2),
        dis_vehicle_third: Point::new(3, 3),
        dis_vehicle_fourth: Point::new(4, 4),
    })
}

    

    #[test]
    fn test_new_vehicle_defaults() {
        let settings = mock_settings();
        let v = Vehicle::new(Route::Up, 5, settings.clone(), Point::new(0, 0), Cross::First);
        assert_eq!(v.position, Point::new(0, 0));
        assert_eq!(v.velocity, 5);
        assert_eq!(v.route, Route::Up);
        assert_eq!(v.settings.width, 800); // Check Rc still works
        assert_eq!(v.stage, Stage::Waiting);
        assert!(!v.is_stopped);
    }

    #[test]
    fn test_has_reached_end_false() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings, Point::new(0, 0), Cross::First);
        v.position = Point::new(100, 100);
        assert_eq!(v.has_reached_end(), false);
    }

    #[test]
    fn test_has_reached_end_true() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings, Point::new(0, 0), Cross::First);
        v.position = Point::new(900, 100); // beyond width + vehicle
        assert_eq!(v.has_reached_end(), true);
    }

    #[test]
    fn test_distance() {
        let settings = mock_settings();
        let mut v1 = Vehicle::new(Route::Up, 5, settings.clone(), Point::new(0, 0), Cross::First);
        let mut v2 = Vehicle::new(Route::Up, 5, settings, Point::new(0, 0), Cross::First);
        v1.position = Point::new(3, 4);
        v2.position = Point::new(0, 0);
        assert_eq!(v1.distance(&v2), 5.0);
    }

    #[test]
    fn test_distance_to() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings, Point::new(0, 0), Cross::First);
        v.position = Point::new(6, 8);
        assert_eq!(v.distance_to(Point::new(0, 0)), 10.0);
    }

    #[test]
    fn test_stop_and_resume() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings, Point::new(0, 0), Cross::First);
        assert!(!v.is_stopped);
        v.stop();
        assert!(v.is_stopped);
        v.resume();
        assert!(!v.is_stopped);
    }

    #[test]
    fn test_spawn_sets_position() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings.clone(), Point::new(0, 0), Cross::First);
        v.spawn(Route::Left);
        assert_eq!(v.position, settings.appearance_vehicle_left);
    }

    #[test]
    fn test_move_forward_up_no_turn() {
        let settings = mock_settings();
        let mut v = Vehicle::new(Route::Up, 5, settings.clone(), Point::new(0, 0), Cross::First);
        v.position = Point::new(0, 100);
        v.destination = Route::Up;
        v.move_forward();
        assert_eq!(v.position, Point::new(0, 95));
    }

}
