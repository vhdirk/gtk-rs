// This file was generated by gir (25bba39) from gir-files (71d73f0)
// DO NOT EDIT

use TextTag;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TextTagTable(Object<ffi::GtkTextTagTable>);

    match fn {
        get_type => || ffi::gtk_text_tag_table_get_type(),
    }
}

impl TextTagTable {
    pub fn new() -> TextTagTable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_tag_table_new())
        }
    }

    pub fn add(&self, tag: &TextTag) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_tag_table_add(self.to_glib_none().0, tag.to_glib_none().0))
        }
    }

    //pub fn foreach(&self, func: /*Unknown conversion*//*Unimplemented*/TextTagTableForeach, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_text_tag_table_foreach() }
    //}

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_text_tag_table_get_size(self.to_glib_none().0)
        }
    }

    pub fn lookup(&self, name: &str) -> Option<TextTag> {
        unsafe {
            from_glib_none(ffi::gtk_text_tag_table_lookup(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn remove(&self, tag: &TextTag) {
        unsafe {
            ffi::gtk_text_tag_table_remove(self.to_glib_none().0, tag.to_glib_none().0);
        }
    }

    pub fn connect_tag_added<F: Fn(&TextTagTable, &TextTag) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextTagTable, &TextTag) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-added",
                transmute(tag_added_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_tag_changed<F: Fn(&TextTagTable, &TextTag, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextTagTable, &TextTag, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-changed",
                transmute(tag_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_tag_removed<F: Fn(&TextTagTable, &TextTag) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextTagTable, &TextTag) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tag-removed",
                transmute(tag_removed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn tag_added_trampoline(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextTagTable, &TextTag) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(tag))
}

unsafe extern "C" fn tag_changed_trampoline(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, size_changed: glib_ffi::gboolean, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextTagTable, &TextTag, bool) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(tag), from_glib(size_changed))
}

unsafe extern "C" fn tag_removed_trampoline(this: *mut ffi::GtkTextTagTable, tag: *mut ffi::GtkTextTag, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextTagTable, &TextTag) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(tag))
}
