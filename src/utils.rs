#![allow(dead_code)]
use wasm_bindgen::UnwrapThrowExt;

/// Convenience function to avoid repeating expect logic.
pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("Can't find the global Window")
}

/// Convenience function to access the head element.
pub fn head() -> web_sys::HtmlHeadElement {
    document()
        .head()
        .expect_throw("Can't find the head element")
}

/// Convenience function to access the web_sys DOM document.
pub fn document() -> web_sys::Document {
    window().document().expect_throw("Can't find document")
}

/// Convenience function to access `document.body`.
pub fn body() -> web_sys::HtmlElement {
    document().body().expect_throw("Can't find document body")
}

/// Convenience function to access `document.documentElement`.
pub fn document_element() -> web_sys::Element {
    document()
        .document_element()
        .expect_throw("Can't find document element")
}

/// Convenience function to access the web_sys history.
pub fn history() -> web_sys::History {
    window().history().expect_throw("Can't find history")
}

pub fn local_storage_get(key: &str) -> Option<String> {
    window().local_storage().ok()??.get(key).ok()?
}

pub fn local_storage_set(key: &str, val: &str) -> Option<()> {
    window().local_storage().ok()??.set(key, val).ok()
}

pub fn media_query(key: &str) -> Option<web_sys::MediaQueryList> {
    window()
        .match_media(key)
        .expect_throw("Can't find media query")
}
