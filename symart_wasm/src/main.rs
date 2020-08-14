extern crate serde_json;
#[macro_use]
extern crate stdweb;
extern crate symart_base;
extern crate symart_designs;
extern crate symart_wasm;

use stdweb::unstable::TryInto;
use stdweb::web::{document, Element, IEventTarget, INode, INonElementParentNode};
use stdweb::web::event::{ClickEvent, IEvent};
use stdweb::web::html_element::CanvasElement;
use symart_base::{Design, DrawResponse};
use symart_designs::lines::Lines;
use symart_designs::squiggles::Squiggles;
use symart_designs::quasitrap::Quasitrap;
use symart_wasm::{call_when_loaded, make_image_data};

fn draw(res: &DrawResponse, celt: &CanvasElement) {
    let img = &res.im;
    celt.set_width(img.width());
    celt.set_height(img.height());
    let ctx = celt.get_context().unwrap();
    let idata = make_image_data(&ctx, img);
    ctx.put_image_data(idata, 0.0, 0.0).unwrap();
}

fn setup<D: Design>(elt: &Element) {
    let s = D::schema().to_string();
    let editor = js! {
        return new JSONEditor(@{elt}, {
            "schema": JSON.parse(@{&s}),
            "disable_edit_json": true,
            "disable_properties": true
        });
    };
    let draw_button = document().create_element("button").unwrap();
    draw_button.append_child(&document().create_text_node("Draw"));
    elt.append_child(&draw_button);
    elt.append_child(&document().create_element("br").unwrap());
    let canvas: CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();
    elt.append_child(&canvas);
    let listener = move |ev: ClickEvent| {
        ev.prevent_default();
        let json = js! {
            return JSON.stringify(@{&editor}.getValue());
        };
        let req = serde_json::from_str(json.as_str().unwrap()).unwrap();
        let res = D::draw(&req).unwrap();
        draw(&res, &canvas);
    };
    draw_button.add_event_listener(listener);
}

fn add<D: Design>(parent: &Element, menu: &Element) {
    let div = document().create_element("div").unwrap();
    parent.append_child(&div);
    setup::<D>(&div);
}

fn do_setup() {
    let form = document().get_element_by_id("form").unwrap();
    let par = document().create_element("p").unwrap();
    add::<Lines>(&form, &par);
    add::<Squiggles>(&form, &par);
    add::<Quasitrap>(&form, &par);
}

fn main() {
    stdweb::initialize();
    call_when_loaded(do_setup);
    stdweb::event_loop();
}
