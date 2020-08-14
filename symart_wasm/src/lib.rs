extern crate image;
#[macro_use]
extern crate stdweb;

use std::ops::Deref;

use image::{GenericImageView, Pixel};
use stdweb::unstable::TryInto;
use stdweb::web::{CanvasRenderingContext2d, ImageData, IEventTarget, TypedArray};
use stdweb::web::event::ResourceLoadEvent;

pub fn make_image_data<I>(ctx: &CanvasRenderingContext2d, img: &I) -> ImageData
where
    I: GenericImageView,
    I::Pixel: Pixel<Subpixel=u8>
{
    let sz = 4 * (img.height() as usize) * (img.width() as usize);
    let mut buf = Vec::with_capacity(sz);
    buf.resize(sz, 0);
    for (x, y, pix) in img.pixels() {
        let pos = (img.width() as usize) * (y as usize) + (x as usize);
        let rgba = pix.to_rgba();
        buf[(4*pos)..(4*(pos+1))].copy_from_slice(&rgba.0[..]);
    }
    let data = ctx.create_image_data(img.width() as f64, img.height() as f64).unwrap();
    let arr: TypedArray<u8> = buf.deref().into();
    js! { @{&data}.data.set(@{arr}); }
    data
}

pub fn call_when_loaded<F>(mut f: F) where
    F: FnMut() -> () + 'static
{
    if js! { return document.readyState == "complete" }.try_into().unwrap() {
        f()
    } else {
        stdweb::web::window().add_event_listener(move |_: ResourceLoadEvent| f());
    }
}
