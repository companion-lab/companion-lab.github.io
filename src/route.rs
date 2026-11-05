use dioxus::prelude::*;

use crate::{
    components::Navbar,
    pages::{Docs, DocsArticle, Home, NotFound},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/docs")]
    Docs {},

    #[route("/docs/:slug")]
    DocsArticle { slug: String },

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
