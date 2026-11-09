// `pub(crate)`: the SSG route list (src/server/mod.rs) enumerates the doc
// slugs from docs::DOC_ARTICLES when prerendering.
pub(crate) mod docs;
mod home;
mod legal;
mod not_found;
mod plugins;

pub use docs::*;
pub use home::*;
pub use legal::*;
pub use not_found::*;
pub use plugins::*;
