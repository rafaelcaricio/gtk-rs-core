// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Context;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct FontMap(Object<ffi::PangoFontMap>);

    match fn {
        get_type => || ffi::pango_font_map_get_type(),
    }
}

pub trait FontMapExt {
    #[cfg(feature = "v1_34")]
    fn changed(&self);

    fn create_context(&self) -> Option<Context>;

    #[cfg(feature = "v1_32_4")]
    fn get_serial(&self) -> u32;

    fn get_shape_engine_type(&self) -> Option<String>;

    //fn list_families(&self, families: /*Unimplemented*/Vec<FontFamily>) -> i32;

    //fn load_font(&self, context: &Context, desc: &FontDescription) -> /*Ignored*/Option<Font>;

    //fn load_fontset(&self, context: &Context, desc: &FontDescription, language: /*Ignored*/&mut Language) -> /*Ignored*/Option<Fontset>;
}

impl<O: IsA<FontMap>> FontMapExt for O {
    #[cfg(feature = "v1_34")]
    fn changed(&self) {
        unsafe {
            ffi::pango_font_map_changed(self.to_glib_none().0);
        }
    }

    fn create_context(&self) -> Option<Context> {
        unsafe {
            from_glib_full(ffi::pango_font_map_create_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v1_32_4")]
    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_font_map_get_serial(self.to_glib_none().0)
        }
    }

    fn get_shape_engine_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_map_get_shape_engine_type(self.to_glib_none().0))
        }
    }

    //fn list_families(&self, families: /*Unimplemented*/Vec<FontFamily>) -> i32 {
    //    unsafe { TODO: call ffi::pango_font_map_list_families() }
    //}

    //fn load_font(&self, context: &Context, desc: &FontDescription) -> /*Ignored*/Option<Font> {
    //    unsafe { TODO: call ffi::pango_font_map_load_font() }
    //}

    //fn load_fontset(&self, context: &Context, desc: &FontDescription, language: /*Ignored*/&mut Language) -> /*Ignored*/Option<Fontset> {
    //    unsafe { TODO: call ffi::pango_font_map_load_fontset() }
    //}
}
