// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use gdk_pixbuf;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Picture(Object<gtk_sys::GtkPicture, gtk_sys::GtkPictureClass, PictureClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_picture_get_type(),
    }
}

impl Picture {
    pub fn new() -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_picture_new()).unsafe_cast()
        }
    }

    //pub fn new_for_file(file: /*Ignored*/Option<&gio::File>) -> Picture {
    //    unsafe { TODO: call gtk_sys:gtk_picture_new_for_file() }
    //}

    pub fn new_for_filename<P: AsRef<std::path::Path>>(filename: P) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_picture_new_for_filename(filename.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    //pub fn new_for_paintable(paintable: /*Ignored*/Option<&gdk::Paintable>) -> Picture {
    //    unsafe { TODO: call gtk_sys:gtk_picture_new_for_paintable() }
    //}

    pub fn new_for_pixbuf(pixbuf: Option<&gdk_pixbuf::Pixbuf>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_picture_new_for_pixbuf(pixbuf.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_for_resource(resource_path: Option<&str>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_picture_new_for_resource(resource_path.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for Picture {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PICTURE: Option<&Picture> = None;

pub trait PictureExt: 'static {
    fn get_alternative_text(&self) -> Option<GString>;

    fn get_can_shrink(&self) -> bool;

    //fn get_file(&self) -> /*Ignored*/Option<gio::File>;

    fn get_keep_aspect_ratio(&self) -> bool;

    //fn get_paintable(&self) -> /*Ignored*/Option<gdk::Paintable>;

    fn set_alternative_text(&self, alternative_text: Option<&str>);

    fn set_can_shrink(&self, can_shrink: bool);

    //fn set_file(&self, file: /*Ignored*/Option<&gio::File>);

    fn set_filename(&self, filename: Option<&str>);

    fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool);

    //fn set_paintable(&self, paintable: /*Ignored*/Option<&gdk::Paintable>);

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn set_resource(&self, resource_path: Option<&str>);

    fn connect_property_alternative_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Picture>> PictureExt for O {
    fn get_alternative_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_picture_get_alternative_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_can_shrink(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_picture_get_can_shrink(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_file(&self) -> /*Ignored*/Option<gio::File> {
    //    unsafe { TODO: call gtk_sys:gtk_picture_get_file() }
    //}

    fn get_keep_aspect_ratio(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_picture_get_keep_aspect_ratio(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_paintable(&self) -> /*Ignored*/Option<gdk::Paintable> {
    //    unsafe { TODO: call gtk_sys:gtk_picture_get_paintable() }
    //}

    fn set_alternative_text(&self, alternative_text: Option<&str>) {
        unsafe {
            gtk_sys::gtk_picture_set_alternative_text(self.as_ref().to_glib_none().0, alternative_text.to_glib_none().0);
        }
    }

    fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            gtk_sys::gtk_picture_set_can_shrink(self.as_ref().to_glib_none().0, can_shrink.to_glib());
        }
    }

    //fn set_file(&self, file: /*Ignored*/Option<&gio::File>) {
    //    unsafe { TODO: call gtk_sys:gtk_picture_set_file() }
    //}

    fn set_filename(&self, filename: Option<&str>) {
        unsafe {
            gtk_sys::gtk_picture_set_filename(self.as_ref().to_glib_none().0, filename.to_glib_none().0);
        }
    }

    fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        unsafe {
            gtk_sys::gtk_picture_set_keep_aspect_ratio(self.as_ref().to_glib_none().0, keep_aspect_ratio.to_glib());
        }
    }

    //fn set_paintable(&self, paintable: /*Ignored*/Option<&gdk::Paintable>) {
    //    unsafe { TODO: call gtk_sys:gtk_picture_set_paintable() }
    //}

    fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gtk_sys::gtk_picture_set_pixbuf(self.as_ref().to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            gtk_sys::gtk_picture_set_resource(self.as_ref().to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    fn connect_property_alternative_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alternative-text\0".as_ptr() as *const _,
                Some(transmute(notify_alternative_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::can-shrink\0".as_ptr() as *const _,
                Some(transmute(notify_can_shrink_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::file\0".as_ptr() as *const _,
                Some(transmute(notify_file_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::keep-aspect-ratio\0".as_ptr() as *const _,
                Some(transmute(notify_keep_aspect_ratio_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::paintable\0".as_ptr() as *const _,
                Some(transmute(notify_paintable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_alternative_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPicture, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Picture> {
    let f: &F = &*(f as *const F);
    f(&Picture::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_can_shrink_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPicture, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Picture> {
    let f: &F = &*(f as *const F);
    f(&Picture::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPicture, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Picture> {
    let f: &F = &*(f as *const F);
    f(&Picture::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_keep_aspect_ratio_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPicture, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Picture> {
    let f: &F = &*(f as *const F);
    f(&Picture::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_paintable_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkPicture, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Picture> {
    let f: &F = &*(f as *const F);
    f(&Picture::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Picture")
    }
}
