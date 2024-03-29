//! This module holds every struct designed to contain various `ViewElement`s. Since every container is itself a [`ViewElement`](super::view::ViewElement), containers can be combined by nesting inside of each other.

mod visibility_toggle;
pub use visibility_toggle::VisibilityToggle;

mod pixel_container;
pub use pixel_container::PixelContainer;

mod shader;
pub use shader::CanShade;

mod collision_container;
pub use collision_container::CollisionContainer;
