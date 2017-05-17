// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use Adjustment;
use Container;
use Scrollable;
use Widget;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;

glib_wrapper! {
    pub struct Layout(Object<ffi::GtkLayout>): Container, Widget, Scrollable;

    match fn {
        get_type => || ffi::gtk_layout_get_type(),
    }
}

impl Layout {
    pub fn new<'a, 'b, P: Into<Option<&'a Adjustment>>, Q: Into<Option<&'b Adjustment>>>(hadjustment: P, vadjustment: Q) -> Layout {
        assert_initialized_main_thread!();
        let hadjustment = hadjustment.into();
        let hadjustment = hadjustment.to_glib_none();
        let vadjustment = vadjustment.into();
        let vadjustment = vadjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_layout_new(hadjustment.0, vadjustment.0)).downcast_unchecked()
        }
    }
}

pub trait LayoutExt {
    fn get_bin_window(&self) -> Option<gdk::Window>;

    fn get_size(&self) -> (u32, u32);

    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    fn set_size(&self, width: u32, height: u32);

    fn get_property_height(&self) -> u32;

    fn set_property_height(&self, height: u32);

    fn get_property_width(&self) -> u32;

    fn set_property_width(&self, width: u32);

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);
}

impl<O: IsA<Layout> + IsA<Container> + IsA<glib::object::Object>> LayoutExt for O {
    fn get_bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_bin_window(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_layout_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(self.to_glib_none().0, child_widget.to_glib_none().0, x, y);
        }
    }

    fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(self.to_glib_none().0, width, height);
        }
    }

    fn get_property_height(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "height".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_height(&self, height: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "height".to_glib_none().0, Value::from(&height).to_glib_none().0);
        }
    }

    fn get_property_width(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "width".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_width(&self, width: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "width".to_glib_none().0, Value::from(&width).to_glib_none().0);
        }
    }

    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, Value::from(&x).to_glib_none().0);
        }
    }

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, Value::from(&y).to_glib_none().0);
        }
    }
}
