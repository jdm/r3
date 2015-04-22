#![cfg_attr(test, feature(test))]
#![feature(rustc_private, slice_patterns)]

extern crate arena;
extern crate graphics;
extern crate gfx as gfx_core;
extern crate gfx_device_gl as gfx_device;
extern crate gfx_graphics;
extern crate piston;
extern crate glutin_window;

use gfx_core::traits::*;
use gfx_graphics::{Gfx2d, GlyphCache};

pub mod gfx {
    pub use graphics::math::Matrix2d as Mat2;
    pub use graphics::types::*;

    use gfx_graphics as g2d;
    use gfx_device as dev;

    pub type GlyphCache = g2d::GlyphCache<dev::Resources>;
    pub type BackEnd<'a> = g2d::GraphicsBackEnd<'a, dev::Resources,
                                                    dev::CommandBuffer,
                                                    dev::Output>;
}

use piston::event::*;
use piston::input::{Button, MouseButton};
use piston::window::{WindowSettings, Size, OpenGLWindow};
use glutin_window::{GlutinWindow, OpenGL};

#[macro_use]
pub mod ui;
use ui::Px;
use ui::layout::Hit;
use ui::draw::DrawCx;
use ui::text::FontFaces;

pub mod demo;
use demo::Demo;

fn main() {
    let mut window = GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new(
            "r3 UI demo".to_string(),
            Size { width: 800, height: 600 }
        ).exit_on_esc(true)
    );

    let (mut device, mut factory) = gfx_device::create(|s| window.get_proc_address(s));
    let mut renderer = factory.create_renderer();

    let mut g2d = Gfx2d::new(&mut device, &mut factory);

    let mut fonts = FontFaces {
        regular: GlyphCache::new("assets/NotoSans/NotoSans-Regular.ttf".as_ref(), &mut factory).unwrap(),
        mono: GlyphCache::new("assets/Hasklig/Hasklig-Regular.otf".as_ref(), &mut factory).unwrap()
    };

    let a = Demo::new([0.0, 1.0, 1.0], "a");
    let b = Demo::new([1.0, 0.0, 1.0], "b");
    let c = Demo::new([1.0, 1.0, 0.0], "c");
    let d = Demo::new([1.0, 0.0, 0.0], "d");
    let e = Demo::new([0.0, 1.0, 0.0], "e");
    let f = Demo::new([0.0, 0.0, 1.0], "f");
    let root = flow![up: a, flow![right: b, c, d], flow![left: e, f]];

    let (mut x, mut y) = (0.0, 0.0);
    for e in window.events() {
        if let Some(args) = e.render_args() {
            let viewport = args.viewport();
            let sz = viewport.draw_size;
            let frame = factory.make_fake_output(sz[0] as u16, sz[1] as u16);
            g2d.draw(&mut renderer, &frame, viewport, |c, g| {
                ui::layout::compute(&root, &mut fonts, sz[0] as Px, sz[1] as Px);
                graphics::clear(graphics::color::WHITE, g);
                DrawCx {
                    gfx: g,
                    fonts: &mut fonts,
                    transform: c.transform
                }.draw(&root);
            });

            device.submit(renderer.as_buffer());
            renderer.reset();
        }

        if let Some(_) = e.after_render_args() {
            device.after_frame();
            fonts.regular.update(&mut factory);
            fonts.mono.update(&mut factory);
            factory.cleanup();
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            root.hit(ui::event::MouseDown::new(x, y));
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            root.hit(ui::event::MouseUp::new(x, y));
        }

        if let Some([nx, ny]) = e.mouse_cursor_args() {
            x = nx as Px;
            y = ny as Px;
        }
    }
}

#[cfg(test)]
extern crate test;

#[bench]
fn layout(bench: &mut test::Bencher) {
    // TODO use a headless renderer.
    let mut window = GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new(
            "benchmark".to_string(),
            Size { width: 800, height: 600 }
        ).exit_on_esc(true)
    );

    let (_, mut factory) = gfx_device::create(|s| window.get_proc_address(s));

    let mut fonts = FontFaces {
        regular: GlyphCache::new("assets/NotoSans/NotoSans-Regular.ttf".as_ref(), &mut factory).unwrap(),
        mono: GlyphCache::new("assets/Hasklig/Hasklig-Regular.otf".as_ref(), &mut factory).unwrap()
    };

    let a = Demo::new([1.0, 0.0, 0.0], "a");
    let b = Demo::new([0.0, 1.0, 0.0], "b");
    let c = Demo::new([0.0, 0.0, 1.0], "c");
    let root = flow![down: a, flow![right: b, c]];

    // HACK kickstart the glyph caches.
    ui::layout::compute(&root, &mut fonts, 800.0, 600.0);
    fonts.regular.update(&mut factory);
    fonts.mono.update(&mut factory);
    factory.cleanup();

    bench.iter(|| {
        ui::layout::compute(&root, &mut fonts, 800.0, 600.0);
        &root
    });
}
