// This file was generated by gir (af5277e) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeIter(Boxed<ffi::GtkTreeIter>);

    match fn {
        copy => |ptr| ffi::gtk_tree_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_iter_free(ptr),
    }
}

impl TreeIter {
}
