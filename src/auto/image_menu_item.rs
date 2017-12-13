// This file was generated by gir (13f739b) from gir-files (db49619)
// DO NOT EDIT

use AccelGroup;
use Actionable;
use Bin;
use Buildable;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ImageMenuItem(Object<ffi::GtkImageMenuItem, ffi::GtkImageMenuItemClass>): MenuItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_image_menu_item_get_type(),
    }
}

impl ImageMenuItem {
    pub fn new() -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_from_stock<'a, P: Into<Option<&'a AccelGroup>>>(stock_id: &str, accel_group: P) -> ImageMenuItem {
        assert_initialized_main_thread!();
        let accel_group = accel_group.into();
        let accel_group = accel_group.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_from_stock(stock_id.to_glib_none().0, accel_group.0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for ImageMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ImageMenuItemExt {
    fn get_always_show_image(&self) -> bool;

    fn get_image(&self) -> Option<Widget>;

    fn get_use_stock(&self) -> bool;

    fn set_accel_group(&self, accel_group: &AccelGroup);

    fn set_always_show_image(&self, always_show: bool);

    fn set_image<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, image: Q);

    fn set_use_stock(&self, use_stock: bool);

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_always_show_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ImageMenuItem> + IsA<glib::object::Object>> ImageMenuItemExt for O {
    fn get_always_show_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_image_menu_item_get_always_show_image(self.to_glib_none().0))
        }
    }

    fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_image_menu_item_get_image(self.to_glib_none().0))
        }
    }

    fn get_use_stock(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_image_menu_item_get_use_stock(self.to_glib_none().0))
        }
    }

    fn set_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_image_menu_item_set_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_image_menu_item_set_always_show_image(self.to_glib_none().0, always_show.to_glib());
        }
    }

    fn set_image<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, image: Q) {
        let image = image.into();
        let image = image.to_glib_none();
        unsafe {
            ffi::gtk_image_menu_item_set_image(self.to_glib_none().0, image.0);
        }
    }

    fn set_use_stock(&self, use_stock: bool) {
        unsafe {
            ffi::gtk_image_menu_item_set_use_stock(self.to_glib_none().0, use_stock.to_glib());
        }
    }

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accel-group",
                transmute(notify_accel_group_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_always_show_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::always-show-image",
                transmute(notify_always_show_image_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::image",
                transmute(notify_image_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-stock",
                transmute(notify_use_stock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accel_group_trampoline<P>(this: *mut ffi::GtkImageMenuItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ImageMenuItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ImageMenuItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_always_show_image_trampoline<P>(this: *mut ffi::GtkImageMenuItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ImageMenuItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ImageMenuItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_image_trampoline<P>(this: *mut ffi::GtkImageMenuItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ImageMenuItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ImageMenuItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_stock_trampoline<P>(this: *mut ffi::GtkImageMenuItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ImageMenuItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ImageMenuItem::from_glib_borrow(this).downcast_unchecked())
}
