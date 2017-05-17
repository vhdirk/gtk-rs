// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use ColorChooser;
use Container;
use Widget;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorButton(Object<ffi::GtkColorButton>): Button, Bin, Container, Widget, Actionable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_rgba(rgba: &gdk::RGBA) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new_with_rgba(rgba.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ColorButtonExt {
    fn get_title(&self) -> Option<String>;

    //fn set_color(&self, color: /*Ignored*/&gdk::Color);

    fn set_title(&self, title: &str);

    fn get_property_alpha(&self) -> u32;

    fn set_property_alpha(&self, alpha: u32);

    #[cfg(feature = "v3_20")]
    fn get_property_show_editor(&self) -> bool;

    #[cfg(feature = "v3_20")]
    fn set_property_show_editor(&self, show_editor: bool);

    fn connect_color_set<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ColorButton> + IsA<glib::object::Object>> ColorButtonExt for O {
    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_color_button_get_title(self.to_glib_none().0))
        }
    }

    //fn set_color(&self, color: /*Ignored*/&gdk::Color) {
    //    unsafe { TODO: call ffi::gtk_color_button_set_color() }
    //}

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn get_property_alpha(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "alpha".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_alpha(&self, alpha: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "alpha".to_glib_none().0, Value::from(&alpha).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_20")]
    fn get_property_show_editor(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-editor".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[cfg(feature = "v3_20")]
    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-editor".to_glib_none().0, Value::from(&show_editor).to_glib_none().0);
        }
    }

    fn connect_color_set<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "color-set",
                transmute(color_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn color_set_trampoline<P>(this: *mut ffi::GtkColorButton, f: glib_ffi::gpointer)
where P: IsA<ColorButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ColorButton::from_glib_none(this).downcast_unchecked())
}
