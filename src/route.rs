use dioxus::prelude::*;

use crate::{
    components::Navbar,
    pages::{Home, NotFound},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
