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

impl Settings {
    pub fn new(width: i32, height: i32, vehicle: i32, gap: i32, safety_distance: f64) -> Settings {
        let half_width = width / 2;
        let half_height = height / 2;
        let vehicle_width = 2 * vehicle;
        let offset_road = gap + vehicle_width;
        let offset_road_s = gap + vehicle;

        Self {
            width,
            height,
            vehicle,
            gap,
            safety_distance,
            offset_road,

            horizontal_road_1: half_height - offset_road,
            vertical_road_1: half_width - offset_road,
            horizontal_road_2: half_height + offset_road,
            vertical_road_2: half_width + offset_road,

            appearance_vehicle_up: Point::new(half_width + (offset_road_s / 2), height),
            appearance_vehicle_down:Point::new(half_width - 3 * offset_road_s / 2, -vehicle), 
            appearance_vehicle_left:Point::new(width,  half_height - 3* offset_road_s / 2),
            appearance_vehicle_right: Point::new(-vehicle, half_height + offset_road_s / 2),

            change_direction_1:Point::new(half_width - 3 * offset_road_s / 2, half_height - 3* offset_road_s / 2),
            change_direction_2:Point::new(half_width + (offset_road_s / 2), half_height + offset_road_s / 2),

            stop_point_first: Point::new(half_width - 3 * offset_road_s / 2, half_height - offset_road - vehicle),
            stop_point_second: Point::new(half_width - offset_road - vehicle, half_height + offset_road_s / 2),
            stop_point_third: Point::new(half_width + offset_road, half_height - 3* offset_road_s / 2),
            stop_point_fourth: Point::new(half_width + (offset_road_s / 2), half_height + offset_road),

            dis_vehicle_first: Point::new(half_width + (offset_road_s / 2), half_height - offset_road - vehicle),
            dis_vehicle_second:Point::new(half_width - offset_road - vehicle, half_height - 3* offset_road_s / 2), 
            dis_vehicle_third:Point::new(half_width + offset_road,  half_height + offset_road_s / 2),
            dis_vehicle_fourth: Point::new(half_width - 3 * offset_road_s / 2, half_height + offset_road),
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::rect::Point;

    #[test]
    fn test_offset_road_calculation() {
        let settings = Settings::new(800, 600, 20, 10, 10.0);
        assert_eq!(settings.offset_road, 10 + 2 * 20); // gap + 2 * vehicle
    }

    #[test]
    fn test_road_coordinates() {
        let settings = Settings::new(800, 600, 20, 10, 10.0);
        let expected_half_width = 800 / 2;
        let expected_half_height = 600 / 2;
        let offset = 10 + 2 * 20;

        assert_eq!(settings.horizontal_road_1, expected_half_height - offset);
        assert_eq!(settings.vertical_road_2, expected_half_width + offset);
    }

    #[test]
    fn test_vehicle_appearance_positions() {
        let settings = Settings::new(800, 600, 20, 10, 10.0);
        let offset_road_s = 10 + 20;

        assert_eq!(settings.appearance_vehicle_up, Point::new(400 + offset_road_s / 2, 600));
        assert_eq!(settings.appearance_vehicle_right, Point::new(-20, 300 + offset_road_s / 2));
    }

    #[test]
    fn test_stop_points_consistency() {
        let settings = Settings::new(800, 600, 20, 10, 10.0);
        // Stop point should be above horizontal_road_1
        assert!(settings.stop_point_first.y < settings.horizontal_road_1);
        // Stop point should be to the left of vertical_road_1
        assert!(settings.stop_point_second.x < settings.vertical_road_1);
    }

}
