// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct StreamableContent(Interface<ffi::AtkStreamableContent>);

    match fn {
        get_type => || ffi::atk_streamable_content_get_type(),
    }
}

pub const NONE_STREAMABLE_CONTENT: Option<&StreamableContent> = None;

pub trait StreamableContentExt: 'static {
    fn get_mime_type(&self, i: i32) -> Option<glib::GString>;

    fn get_n_mime_types(&self) -> i32;

    //fn get_stream(&self, mime_type: &str) -> /*Ignored*/Option<glib::IOChannel>;

    fn get_uri(&self, mime_type: &str) -> Option<glib::GString>;
}

impl<O: IsA<StreamableContent>> StreamableContentExt for O {
    fn get_mime_type(&self, i: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_streamable_content_get_mime_type(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn get_n_mime_types(&self) -> i32 {
        unsafe { ffi::atk_streamable_content_get_n_mime_types(self.as_ref().to_glib_none().0) }
    }

    //fn get_stream(&self, mime_type: &str) -> /*Ignored*/Option<glib::IOChannel> {
    //    unsafe { TODO: call ffi:atk_streamable_content_get_stream() }
    //}

    fn get_uri(&self, mime_type: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_streamable_content_get_uri(
                self.as_ref().to_glib_none().0,
                mime_type.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for StreamableContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StreamableContent")
    }
}