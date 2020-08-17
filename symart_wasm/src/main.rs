extern crate serde_json;
#[macro_use]
extern crate stdweb;
extern crate symart_base;
extern crate symart_designs;
extern crate symart_wasm;

use stdweb::unstable::TryInto;
use stdweb::web::{document, Element, IElement, IEventTarget, INode, INonElementParentNode};
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

/*
fn add<D: Design>(parent: &Element) {
    let div = document().create_element("div").unwrap();
    div.set_attribute("class","tab").unwrap();
    let radio = document().create_element("input").unwrap();
    radio.set_attribute("type","radio").unwrap();
    radio.set_attribute("class","input").unwrap();
    radio.set_attribute("checked","true").unwrap();
    let id = str::replace(D::name()," ","-");
    radio.set_attribute("id",&id).unwrap();
    radio.set_attribute("name","designs").unwrap();
    let lbl = document().create_element("label").unwrap();
    lbl.set_attribute("for",&id).unwrap();
    lbl.set_attribute("class","label");
    let lbl_txt = document().create_text_node(D::name());
    lbl.append_child(&lbl_txt);
    parent.append_child(&div);
    div.append_child(&radio);
    div.append_child(&lbl);
    let content = document().create_element("div").unwrap();
    content.set_attribute("class","panel");
    div.append_child(&content);
    setup::<D>(&content);
}
*/

fn add<D: Design>(parent: &Element, menu: &Element)
{
    let id = str::replace(D::name()," ","-");
    let div = document().create_element("div").unwrap();
    div.set_attribute("id",&id).unwrap();
    setup::<D>(&div);
    parent.append_child(&div);
    let li = document().create_element("li").unwrap();
    let a = document().create_element("a").unwrap();
    a.set_attribute("href",&format!("#{}",&id)).unwrap();
    let txt = document().create_text_node(D::name());
    a.append_child(&txt);
    li.append_child(&a);
    menu.append_child(&li);
}

fn do_setup() {
    let form = document().get_element_by_id("form").unwrap();
    let menu = document().create_element("ul").unwrap();
    form.append_child(&menu);
    add::<Lines>(&form, &menu);
    add::<Squiggles>(&form, &menu);
    add::<Quasitrap>(&form, &menu);
    js! { $("#form").tabs(); }
}

fn main() {
    stdweb::initialize();
    call_when_loaded(do_setup);
    stdweb::event_loop();
}
