#[macro_use]
extern crate stdweb;

use stdweb::web::{
    document,
    //HtmlElement,
    IParentNode,
    //Element,
    CanvasRenderingContext2d,
    window,
    IEventTarget,
    IWindowOrWorker,
};
use stdweb::web::html_element::{
    CanvasElement,
    ImageElement
};
use stdweb::web::event::{
    MouseMoveEvent,
    MouseDownEvent,
    ResizeEvent,
    LoadEndEvent,
    ProgressEvent,
};

use stdweb::unstable::TryInto;
use stdweb::traits::{
    IHtmlElement,
    IMouseEvent,
};

use stdweb::web::error::DrawImageError;

// Shamelessly stolen from webplatform's TodoMVC example.
// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }


struct image {
    x: i32,
    y: i32,
    path: str,
}

fn main() {
    stdweb::initialize();
    let canvas: CanvasElement = document().query_selector(
        "#twenty"
    ).unwrap().unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    {
        let canvas = canvas.clone();
        window().add_event_listener(move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        });
    }
    let image = ImageElement::new();
    image.set_src("assets/1.png");
    image.add_event_listener(move |_: ProgressEvent| {
        console!(log, "IMAGE LOADED");

    });
    canvas.add_event_listener(move |event: MouseDownEvent| {
        console!(log, event.client_x());
        console!(log, event.client_y());
    });

    {
        let context = context.clone();
        window().set_timeout(move || {
            match context.draw_image(image, 10.0, 10.0) {
                Ok(_) => {
                    console!(log, "SHIT IS NOT BUSTED");
                },
                Err(_) => {
                    console!(log, "SHIT IS BUSTED");
                }
            };
        }, 1000);
    }

    canvas.add_event_listener(move |event: MouseMoveEvent| {
        context.fill_rect(
            f64::from(event.client_x() - 5), 
            f64::from(event.client_y() - 5), 
            10.0, 
            10.0
        );
    });
}
