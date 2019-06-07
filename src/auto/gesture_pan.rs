// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureDrag;
use GestureSingle;
use Orientation;
use PanDirection;
use PropagationPhase;
use Widget;
use gdk;
use glib::StaticType;
use glib::ToValue;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct GesturePan(Object<gtk_sys::GtkGesturePan, gtk_sys::GtkGesturePanClass, GesturePanClass>) @extends GestureDrag, GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    pub fn new<P: IsA<Widget>>(widget: &P, orientation: Orientation) -> GesturePan {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(gtk_sys::gtk_gesture_pan_new(widget.as_ref().to_glib_none().0, orientation.to_glib())).unsafe_cast()
        }
    }

    pub fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(gtk_sys::gtk_gesture_pan_get_orientation(self.to_glib_none().0))
        }
    }

    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            gtk_sys::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation.to_glib());
        }
    }

    pub fn connect_pan<F: Fn(&GesturePan, PanDirection, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pan_trampoline<F: Fn(&GesturePan, PanDirection, f64) + 'static>(this: *mut gtk_sys::GtkGesturePan, direction: gtk_sys::GtkPanDirection, offset: libc::c_double, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(direction), offset)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pan\0".as_ptr() as *const _,
                Some(transmute(pan_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_orientation_notify<F: Fn(&GesturePan) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<F: Fn(&GesturePan) + 'static>(this: *mut gtk_sys::GtkGesturePan, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute(notify_orientation_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

pub struct GesturePanBuilder {
    orientation: Option<Orientation>,
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GesturePanBuilder {
    pub fn new() -> Self {
        Self {
            orientation: None,
            button: None,
            exclusive: None,
            touch_only: None,
            n_points: None,
            window: None,
            propagation_phase: None,
            widget: None,
        }
    }

    pub fn build(self) -> GesturePan {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new(GesturePan::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window(mut self, window: &gdk::Window) -> Self {
        self.window = Some(window.clone());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget(mut self, widget: &Widget) -> Self {
        self.widget = Some(widget.clone());
        self
    }
}

impl fmt::Display for GesturePan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GesturePan")
    }
}
