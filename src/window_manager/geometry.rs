use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Geometry {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Geometry {
    pub fn cascade(index: usize, area_width: u16, area_height: u16) -> Self {
        let width = 38.min(area_width);
        let height = 10.min(area_height);

        let max_x = area_width.saturating_sub(width);
        let max_y = area_height.saturating_sub(height);

        let max_step = (max_x.min(max_y) / 2) as usize;
        let step = if max_step == 0 {
            0
        } else {
            index % (max_step + 1)
        };

        let offset = (step as u16).saturating_mul(2);

        Self {
            x: offset.min(max_x),
            y: offset.min(max_y),
            width,
            height,
        }
    }

    pub fn fit(self, area_width: u16, area_height: u16) -> Self {
        let width = self.width.min(area_width);
        let height = self.height.min(area_height);

        Self {
            x: self.x.min(area_width.saturating_sub(width)),
            y: self.y.min(area_height.saturating_sub(height)),
            width,
            height,
        }
    }

    pub fn resize(self, dw: i16, dh: i16, area_width: u16, area_height: u16) -> Self {
        let geometry = self.fit(area_width, area_height);

        let available_width = area_width.saturating_sub(geometry.x);
        let available_height = area_height.saturating_sub(geometry.y);

        let min_width = 20.min(available_width);
        let min_height = 5.min(available_height);

        let width = (i32::from(geometry.width) + i32::from(dw))
            .clamp(i32::from(min_width), i32::from(available_width)) as u16;

        let height = (i32::from(geometry.height) + i32::from(dh))
            .clamp(i32::from(min_height), i32::from(available_height)) as u16;

        Self {
            x: geometry.x,
            y: geometry.y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Geometry;

    #[test]
    fn cascade_keeps_window_inside_terminal() {
        let geometry = Geometry::cascade(3, 100, 30);

        assert!(geometry.x + geometry.width <= 100);
        assert!(geometry.y + geometry.height <= 30);
    }

    #[test]
    fn cascade_adapts_to_small_terminal() {
        let geometry = Geometry::cascade(0, 15, 4);

        assert_eq!(geometry.width, 15);
        assert_eq!(geometry.height, 4);
        assert_eq!(geometry.x, 0);
        assert_eq!(geometry.y, 0);
    }

    #[test]
    fn fit_clamps_position_and_size() {
        let geometry = Geometry {
            x: 90,
            y: 40,
            width: 50,
            height: 20,
        }
        .fit(100, 30);

        assert_eq!(
            geometry,
            Geometry {
                x: 50,
                y: 10,
                width: 50,
                height: 20,
            }
        );
    }

    #[test]
    fn resize_respects_minimum_size() {
        let geometry = Geometry {
            x: 0,
            y: 0,
            width: 30,
            height: 10,
        }
        .resize(-100, -100, 100, 30);

        assert_eq!(geometry.width, 20);
        assert_eq!(geometry.height, 5);
    }

    #[test]
    fn resize_does_not_exceed_available_area() {
        let geometry = Geometry {
            x: 10,
            y: 5,
            width: 30,
            height: 10,
        }
        .resize(100, 100, 60, 25);

        assert_eq!(geometry.width, 50);
        assert_eq!(geometry.height, 20);
        assert!(geometry.x + geometry.width <= 60);
        assert!(geometry.y + geometry.height <= 25);
    }

    #[test]
    fn resize_handles_zero_sized_terminal() {
        let geometry = Geometry {
            x: 10,
            y: 5,
            width: 30,
            height: 10,
        }
        .resize(1, 1, 0, 0);

        assert_eq!(
            geometry,
            Geometry {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            }
        );
    }
}
