use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub vehicle: i32,
    pub gap: i32,
    pub safety_distance: f64,
    pub offset_road: i32,

    pub horizontal_road_1: i32,
    pub vertical_road_1: i32,
    pub horizontal_road_2: i32,
    pub vertical_road_2: i32,

    pub appearance_vehicle_up: Point,
    pub appearance_vehicle_down: Point,
    pub appearance_vehicle_left: Point,
    pub appearance_vehicle_right: Point,

    pub change_direction_1: Point,
    pub change_direction_2: Point,

    pub stop_point_first: Point,
    pub stop_point_second: Point,
    pub stop_point_third: Point,
    pub stop_point_fourth: Point,

    pub dis_vehicle_first: Point,
    pub dis_vehicle_second: Point,
    pub dis_vehicle_third: Point,
    pub dis_vehicle_fourth: Point,
}

