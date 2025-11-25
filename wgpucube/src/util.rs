#[cfg(target_arch = "wasm32")]
mod dom_element_ids {
    pub(crate) const ERROR_OVERLAY: &str = "error-overlay";
    pub(crate) const ERROR_MESSAGE: &str = "error-message";
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn show_error_overlay(error: anyhow::Error) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let overlay = document
        .get_element_by_id(dom_element_ids::ERROR_OVERLAY)
        .expect("error-overlay element not found");
    let message_element = document
        .get_element_by_id(dom_element_ids::ERROR_MESSAGE)
        .expect("error-message element not found");

    message_element.set_text_content(Some(&format!("{:?}", error)));
    overlay.class_list().remove_1("hidden").unwrap();
}
