use egui::{vec2, Sense, Widget};
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UIEdgeInsets {
    top: f64,
    left: f64,
    bottom: f64,
    right: f64,
}

#[allow(unsafe_code)]
/// Gets the ios safe area
/// A safe area defines the area within a view that isn’t covered by a navigation bar, tab bar, toolbar, or other views a window might provide. Safe areas are essential for avoiding a device’s interactive and display features, like Dynamic Island on iPhone or the camera housing on some Mac models. For developer guidance, see Positioning content relative to the safe area.
pub fn get_ios_safe_area_insets() -> UIEdgeInsets {
    unsafe {
        let shared_application: *mut Object = msg_send![class!(UIApplication), sharedApplication];
        let windows: *mut Object = msg_send![shared_application, windows];
        let first_object: *mut Object = msg_send![windows, firstObject];
        let safe_area_insets: UIEdgeInsets = msg_send![first_object, safeAreaInsets];
        safe_area_insets
    }
}

/// Adds spacing at the top on ios devices
#[derive(Default)]
pub struct IosSafeArea;

impl Widget for IosSafeArea {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (id, rect) = ui.allocate_space(vec2(
            ui.available_width(),
            get_ios_safe_area_insets().top as f32,
        ));
        ui.interact(rect, id, Sense::hover())
    }
}
