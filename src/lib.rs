#![feature(alloc, box_syntax, catch_panic, core, plugin, rustc_private, slice_patterns)]
#![plugin(regex_macros)]
extern crate regex;
extern crate arena;
extern crate unicode_width;

extern crate image;
#[macro_use(uniform, implement_vertex)]
extern crate glium;
extern crate piston;
extern crate glutin;
extern crate glutin_window;

#[cfg(windows)]
#[macro_use(shared_library)]
extern crate shared_library;

pub mod glyph;
pub mod window;

pub mod cfg {
    pub use ui::color::BreezeDark as ColorScheme;
}

#[macro_use]
pub mod ui;

pub mod ide {
    pub mod rustc;
    pub mod highlight;
}
