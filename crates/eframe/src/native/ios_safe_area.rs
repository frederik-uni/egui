use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UIEdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

#[allow(unsafe_code)]
/// Gets the ios safe area
/// A safe area defines the area within a view that isn’t covered by a navigation bar, tab bar, toolbar, or other views a window might provide. Safe areas are essential for avoiding a device’s interactive and display features, like Dynamic Island on iPhone or the camera housing on some Mac models. For developer guidance, see Positioning content relative to the safe area.
pub fn get_ios_safe_area_insets() -> Option<UIEdgeInsets> {
    unsafe {
        let shared_application: *mut Object = msg_send![class!(UIApplication), sharedApplication];
        let windows: *mut Object = msg_send![shared_application, windows];
        if windows.is_null() {
            return None;
        }
        let first_object: *mut Object = msg_send![windows, firstObject];
        let safe_area_insets: UIEdgeInsets = msg_send![first_object, safeAreaInsets];
        Some(safe_area_insets)
    }
}
