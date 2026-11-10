use dioxus::prelude::*;

use crate::components::{
    scroll_to_entry_hash, scroll_to_id, AmbientBackground, Cta, Features, Footer, Hero, Specs,
    SECTION_SCROLL_TARGET,
};

/// Home page
#[component]
pub fn Home() -> Element {
    // Scroll to a section requested from another page (navbar/footer links)
    // or re-requested while already here. The tracked read subscribes the
    // effect to the signal; `take` clears it in the same step so the effect
    // settles after one extra run instead of looping.
    use_effect(move || {
        let target = SECTION_SCROLL_TARGET();
        if target.is_some() {
            SECTION_SCROLL_TARGET.with_mut(|target| target.take());
            scroll_to_id(target.unwrap());
        }
    });

    // Honor `#section` URLs on first load, reliably — see scroll_to_entry_hash
    // for why a single "read the hash and scroll" pass loses races.
    use_effect(scroll_to_entry_hash);

    rsx! {
        AmbientBackground {}
        Hero {}
        Features {}
        Specs {}
        Cta {}
        Footer {}
    }
}
