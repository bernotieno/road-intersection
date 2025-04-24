use std::rc::Rc;
use std::time::Duration;

use crate::cars::{Route, Vehicle};
use crate::settings::Settings;
use crate::traffic::TrafficLight;
use sdl2::{rect::Point, render::Canvas, video::Window};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cross {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Crossing,
    Waiting,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
    pub traffic_light: TrafficLight,
    pub cross: Cross,
    pub stage: Stage,
    pub stop_point: Point,
    pub last_light_change: Instant,
    pub change_interval: Duration,
    pub settings: Rc<Settings>,
}

impl Lane {
    pub fn new(cross: Cross, settings: Rc<Settings>) -> Lane {
        Lane {
            vehicles: Vec::new(),
            traffic_light: TrafficLight::new(cross),
            cross,
            stage: Stage::Waiting,
            stop_point: match cross {
                Cross::First => settings.stop_point_first,
                Cross::Second => settings.stop_point_second,
                Cross::Third => settings.stop_point_third,
                Cross::Fourth => settings.stop_point_fourth,
            },
            last_light_change: Instant::now(),
            change_interval: Duration::from_secs(15),
            settings,
        }
    }

    pub fn draw_light(mut self, canvas: &mut Canvas<Window>) {
        self.traffic_light.draw(
            canvas,
            self.settings.width,
            self.settings.height,
            self.settings.vehicle,
        );
    }

    pub fn closest_vehicle_distance(&self) -> Option<f64> {
        self.vehicles
            .iter()
            .map(|vehicle| vehicle.distance_to(self.stop_point))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn stop_vehicules(&mut self) {
        let stop_point = match self.cross {
            Cross::First => self.settings.stop_point_first,
            Cross::Second => self.settings.stop_point_second,
            Cross::Third => self.settings.stop_point_third,
            Cross::Fourth => self.settings.stop_point_fourth,
        };

        let mut vehicles = self.vehicles.iter_mut().collect::<Vec<&mut Vehicle>>();
        for i in 0..vehicles.len() {
            let can_move = if let Some(next_vehicle) = vehicles.iter().nth((i as i32 - 1) as usize)
            {
                vehicles[i].distance(next_vehicle) > self.settings.safety_distance
            } else {
                true
            };

            if (vehicles[i].position == stop_point && self.stage == Stage::Waiting) || !can_move {
                vehicles[i].is_stopped = true;
            }

            if self.stage == Stage::Crossing && vehicles[i].is_stopped {
                vehicles[i].is_stopped = false;
            }
        }
    }

    fn cross(&mut self) {
        if self.stage == Stage::Waiting {
            return;
        }

        let a = |v: &&Vehicle| -> bool {
            match self.cross {
                Cross::First => v.position.y > self.stop_point.y,
                Cross::Second => v.position.x > self.stop_point.x,
                Cross::Third => v.position.x < self.stop_point.x,
                Cross::Fourth => v.position.y < self.stop_point.y,
            }
        };
 
        // vehicle that already cross the stop point and enter in the itersections.
        let vehicle_crossed = self
            .vehicles
            .iter()
            .filter(|v| v.stage == Stage::Waiting && a(v))
            .collect::<Vec<&Vehicle>>();
        let vehicles = self
            .vehicles
            .iter()
            .filter(|v| v.stage == Stage::Waiting && !a(v))
            .collect::<Vec<&Vehicle>>();

        if !vehicle_crossed.is_empty() {
            return;
        }

        if let Some(vehicle) = vehicles.first() {
            if vehicle.distance_to(self.stop_point) > 2.0 * self.settings.safety_distance {
                self.traffic_light.change_traffic_light();
                self.stage = Stage::Waiting;
            } else {
                return;
            }
        } else {
            self.traffic_light.change_traffic_light();
            self.stage = Stage::Waiting;
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        <Lane as Clone>::clone(&self).draw_light(canvas);
        self.stop_vehicules();
        self.cross();
        for i in (0..self.vehicles.len()).rev() {
            self.vehicles[i].update(canvas);

            // Remove vehicles that have reached the end of the lane
            if self.vehicles[i].has_reached_end() {
                self.vehicles.remove(i);
            }
        }
    }

    pub fn add_vehicle(&mut self, route: Route) {
        let mut vehicle =
            Vehicle::new(route, 1, self.settings.clone(), self.stop_point, self.cross);
        vehicle.spawn(route);

        if let Some(last) = self.vehicles.clone().last() {
            if self.settings.safety_distance < vehicle.distance(last) {
                self.vehicles.push(vehicle);
            }
        } else {
            self.vehicles.push(vehicle);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::time::Duration;
    use sdl2::rect::Point;

    fn mock_settings() -> Rc<Settings> {
        Rc::new(Settings {
            width: 800,
            height: 600,
            vehicle: 20,
            gap: 5,
            safety_distance: 10.0,
            offset_road: 20,
            horizontal_road_1: 100,
            vertical_road_1: 100,
            horizontal_road_2: 300,
            vertical_road_2: 300,
            appearance_vehicle_up: Point::new(0, 0),
            appearance_vehicle_down: Point::new(0, 0),
            appearance_vehicle_left: Point::new(0, 0),
            appearance_vehicle_right: Point::new(0, 0),
            change_direction_1: Point::new(0, 0),
            change_direction_2: Point::new(0, 0),
            stop_point_first: Point::new(100, 100),
            stop_point_second: Point::new(200, 200),
            stop_point_third: Point::new(300, 300),
            stop_point_fourth: Point::new(400, 400),
            dis_vehicle_first: Point::new(0, 0),
            dis_vehicle_second: Point::new(0, 0),
            dis_vehicle_third: Point::new(0, 0),
            dis_vehicle_fourth: Point::new(0, 0),
        })
    }

    #[test]
    fn test_lane_new_sets_correct_values() {
        let settings = mock_settings();
        let lane = Lane::new(Cross::First, settings.clone());

        assert_eq!(lane.cross, Cross::First);
        assert_eq!(lane.stage, Stage::Waiting);
        assert_eq!(lane.stop_point, settings.stop_point_first);
        assert_eq!(lane.vehicles.len(), 0);
        assert_eq!(lane.change_interval, Duration::from_secs(15));
    }

    #[test]
    fn test_add_vehicle_adds_to_empty_lane() {
        let settings = mock_settings();
        let mut lane = Lane::new(Cross::First, settings.clone());

        lane.add_vehicle(Route::Up); // Valid variant

        assert_eq!(lane.vehicles.len(), 1);
    }

    #[test]
    fn test_closest_vehicle_distance_none_when_empty() {
        let settings = mock_settings();
        let lane = Lane::new(Cross::First, settings);

        assert_eq!(lane.closest_vehicle_distance(), None);
    }

    #[test]
    fn test_stop_vehicules_stops_vehicle_on_stop_point() {
        let settings = mock_settings();
        let mut lane = Lane::new(Cross::First, settings.clone());

        let mut vehicle = Vehicle::new(
            Route::Up,
            1,
            settings.clone(),
            settings.stop_point_first,
            Cross::First,
        );

        vehicle.position = settings.stop_point_first;
        vehicle.is_stopped = false;

        lane.vehicles.push(vehicle);
        lane.stop_vehicules();

        assert!(lane.vehicles[0].is_stopped);
    }
}
