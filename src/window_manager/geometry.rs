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
}
