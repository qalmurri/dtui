#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
