use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapDirection {
    Left,
    Down,
    Up,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Geometry {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Geometry {
    pub fn snap(direction: SnapDirection, area_width: u16, area_height: u16) -> Self {
        let left_width = area_width / 2;
        let right_width = area_width - left_width;
        let top_height = area_height / 2;
        let bottom_height = area_height - top_height;

        match direction {
            SnapDirection::Left => Self {
                x: 0,
                y: 0,
                width: left_width,
                height: area_height,
            },
            SnapDirection::Right => Self {
                x: left_width,
                y: 0,
                width: right_width,
                height: area_height,
            },
            SnapDirection::Up => Self {
                x: 0,
                y: 0,
                width: area_width,
                height: top_height,
            },
            SnapDirection::Down => Self {
                x: 0,
                y: top_height,
                width: area_width,
                height: bottom_height,
            },
        }
    }

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

    /// Resize satu langkah sambil mempertahankan batas desktop.
    ///
    /// Jika pembesaran melewati tepi kanan atau bawah, window digeser
    /// terlebih dahulu agar ukuran baru tetap dapat dipertahankan.
    pub fn resize(self, dw: i16, dh: i16, area_width: u16, area_height: u16) -> Self {
        if area_width == 0 || area_height == 0 {
            return Self {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
        }

        let current = self.fit(area_width, area_height);
        let min_width = 20.min(area_width);
        let min_height = 5.min(area_height);

        let width = (i32::from(current.width) + i32::from(dw))
            .clamp(i32::from(min_width), i32::from(area_width)) as u16;

        let height = (i32::from(current.height) + i32::from(dh))
            .clamp(i32::from(min_height), i32::from(area_height)) as u16;

        // Untuk resize horizontal, geser posisi ke kiri jika ukuran baru
        // tidak muat di sisi kanan.
        let max_x = area_width.saturating_sub(width);
        let x = current.x.min(max_x);

        // Untuk resize vertikal, geser posisi ke atas jika ukuran baru
        // tidak muat di sisi bawah.
        let max_y = area_height.saturating_sub(height);
        let y = current.y.min(max_y);

        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/window_manager/geometry.rs"]
mod tests;
