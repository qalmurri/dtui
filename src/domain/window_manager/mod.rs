pub mod focus;
pub mod geometry;
pub mod manager;
pub mod resize;
pub mod window;
pub mod z_order;

pub const MAX_WINDOWS: usize = 32;

pub use geometry::Geometry;
pub use manager::WindowManager;
pub use window::Window;
