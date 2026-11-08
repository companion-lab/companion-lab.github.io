use dioxus::prelude::*;

use crate::{
    components::Navbar,
    pages::{Docs, DocsArticle, Home, Legal, NotFound, Plugins},
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

    #[route("/plugins")]
    Plugins {},

    #[route("/legal")]
    Legal {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
