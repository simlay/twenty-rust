#[macro_use]
extern crate stdweb;

mod game;

use stdweb::web::{
    document,
    //HtmlElement,
    IParentNode,
    //Element,
    CanvasRenderingContext2d,
    window,
    IEventTarget,
};

use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::html_element::{
    CanvasElement,
};
use stdweb::web::event::{
    MouseMoveEvent,
    MouseDownEvent,
    MouseUpEvent,
};

use stdweb::unstable::TryInto;
//use stdweb::traits::{
//    //IHtmlElement,
//    //IMouseEvent,
//};

use game::{
    TILE_WIDTH,
    TILE_HEIGHT,
    VIRTICAL_TILES,
    HORIZONTAL_TILES,
    Game,
};




// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector(
        "#twenty"
    ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width((TILE_WIDTH*HORIZONTAL_TILES) as u32);
    canvas.set_height((TILE_HEIGHT*VIRTICAL_TILES) as u32);

    let rc = Rc::new(RefCell::new(Game::new(context)));

    enclose!((rc) {
        rc.borrow_mut().init();
        window().request_animation_frame(move |time| {
            rc.borrow_mut().animate(time, rc.clone());
        });
    });
    enclose!((rc) {
        canvas.add_event_listener(move |event: MouseDownEvent| {
            rc.borrow_mut().mouse_down(event);
        });
    });
    enclose!((rc) {
        canvas.add_event_listener(move |event: MouseMoveEvent| {
            rc.borrow_mut().mouse_move(event);
        });
    });
    enclose!((rc) {
        canvas.add_event_listener(move |event: MouseUpEvent| {
            rc.borrow_mut().mouse_up(event);
        });
    });

    /*
    canvas.add_event_listener(move |event: MouseMoveEvent| {
        context.fill_rect(
            f64::from(event.client_x() - 5),
            f64::from(event.client_y() - 5),
            10.0,
            10.0
        );
    });
    */
}
