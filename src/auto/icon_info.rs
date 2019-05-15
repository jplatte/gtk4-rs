// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use IconTheme;
use StyleContext;
use gdk;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconInfo(Object<gtk_sys::GtkIconInfo, gtk_sys::GtkIconInfoClass, IconInfoClass>);

    match fn {
        get_type => || gtk_sys::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    pub fn new_for_pixbuf<P: IsA<IconTheme>>(icon_theme: &P, pixbuf: &gdk_pixbuf::Pixbuf) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_info_new_for_pixbuf(icon_theme.as_ref().to_glib_none().0, pixbuf.to_glib_none().0))
        }
    }

    pub fn get_base_scale(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_info_get_base_scale(self.to_glib_none().0)
        }
    }

    pub fn get_base_size(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_info_get_base_size(self.to_glib_none().0)
        }
    }

    pub fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(gtk_sys::gtk_icon_info_get_filename(self.to_glib_none().0))
        }
    }

    pub fn is_symbolic(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_info_is_symbolic(self.to_glib_none().0))
        }
    }

    pub fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_icon(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn load_icon_async<P: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(&self, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gtk_sys:gtk_icon_info_load_icon_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn load_icon_async_future(&self) -> Box_<future::Future<Output = Result<gdk_pixbuf::Pixbuf, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.load_icon_async(
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn load_symbolic(&self, fg: /*Ignored*/&gdk::RGBA, success_color: /*Ignored*/Option<&gdk::RGBA>, warning_color: /*Ignored*/Option<&gdk::RGBA>, error_color: /*Ignored*/Option<&gdk::RGBA>) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
    //    unsafe { TODO: call gtk_sys:gtk_icon_info_load_symbolic() }
    //}

    //pub fn load_symbolic_async<P: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, fg: /*Ignored*/&gdk::RGBA, success_color: /*Ignored*/Option<&gdk::RGBA>, warning_color: /*Ignored*/Option<&gdk::RGBA>, error_color: /*Ignored*/Option<&gdk::RGBA>, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gtk_sys:gtk_icon_info_load_symbolic_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn load_symbolic_async_future(&self, fg: /*Ignored*/&gdk::RGBA, success_color: /*Ignored*/Option<&gdk::RGBA>, warning_color: /*Ignored*/Option<&gdk::RGBA>, error_color: /*Ignored*/Option<&gdk::RGBA>) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let fg = fg.clone();
        //let success_color = success_color.map(ToOwned::to_owned);
        //let warning_color = warning_color.map(ToOwned::to_owned);
        //let error_color = error_color.map(ToOwned::to_owned);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.load_symbolic_async(
        //        &fg,
        //        success_color.as_ref().map(::std::borrow::Borrow::borrow),
        //        warning_color.as_ref().map(::std::borrow::Borrow::borrow),
        //        error_color.as_ref().map(::std::borrow::Borrow::borrow),
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    pub fn load_symbolic_for_context<P: IsA<StyleContext>>(&self, context: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_for_context(self.to_glib_none().0, context.as_ref().to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn load_symbolic_for_context_async<P: IsA<StyleContext>, Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, context: &P, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: Q) {
    //    unsafe { TODO: call gtk_sys:gtk_icon_info_load_symbolic_for_context_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(&self, context: &P) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let context = context.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.load_symbolic_for_context_async(
        //        &context,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    pub fn load_texture(&self) -> Option<gdk::Texture> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_info_load_texture(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconInfo")
    }
}
