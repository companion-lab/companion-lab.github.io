//! UI components, organized in atomic-design layers:
//! - [`atoms`]: the smallest leaf units (buttons, links)
//! - [`molecules`]: small composites of atoms (button groups, cards, rows)
//! - [`organisms`]: full page sections (navbar, hero, footer)
//!
//! Pages (`src/pages/`) compose organisms.

mod atoms;
mod molecules;
mod organisms;

pub use atoms::*;
pub use molecules::*;
pub use organisms::*;
