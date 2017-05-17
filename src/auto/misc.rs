// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    pub struct Misc(Object<ffi::GtkMisc>): Widget;

    match fn {
        get_type => || ffi::gtk_misc_get_type(),
    }
}

pub trait MiscExt {
    fn get_alignment(&self) -> (f32, f32);

    fn get_padding(&self) -> (i32, i32);

    fn set_alignment(&self, xalign: f32, yalign: f32);

    fn set_padding(&self, xpad: i32, ypad: i32);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_xpad(&self) -> i32;

    fn set_property_xpad(&self, xpad: i32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn get_property_ypad(&self) -> i32;

    fn set_property_ypad(&self, ypad: i32);
}

impl<O: IsA<Misc> + IsA<glib::object::Object>> MiscExt for O {
    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_misc_get_alignment(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_misc_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_misc_set_alignment(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_misc_set_padding(self.to_glib_none().0, xpad, ypad);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_xpad(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xpad".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xpad(&self, xpad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xpad".to_glib_none().0, Value::from(&xpad).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn get_property_ypad(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ypad".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_ypad(&self, ypad: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ypad".to_glib_none().0, Value::from(&ypad).to_glib_none().0);
        }
    }
}
