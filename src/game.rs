use stdweb::web::{
    //document,
    //HtmlElement,
    //IParentNode,
    //Element,
    CanvasRenderingContext2d,
    window,
    //IEventTarget,
    //IWindowOrWorker,
};

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use stdweb::web::html_element::{
    //CanvasElement,
    ImageElement
};
use stdweb::web::event::{
    MouseMoveEvent,
    MouseDownEvent,
    MouseUpEvent,
};

//use stdweb::unstable::TryInto;
use stdweb::traits::{
    IMouseEvent,
    //IHtmlElement,
};
pub const TILE_WIDTH : i32 = 80;
pub const TILE_HEIGHT : i32 = 80;
pub const VIRTICAL_TILES : i32 = 8;
pub const HORIZONTAL_TILES : i32 = 7;


#[derive(Clone)]
struct Tile {
    x: i32,
    y: i32,
    value: i8,
}


impl Tile {
    pub fn new(x: i32, y: i32, value: i8) -> Tile {
        Tile {
            x: x,
            y: y,
            value: value,
        }
    }
}

fn get_image(value: i8) -> ImageElement {
    let image = ImageElement::new();
    image.set_src(&format!("assets/{}.png", value));
    image
}

pub struct Game {
    tiles: Vec<Tile>,
    images: HashMap<i8, ImageElement>,
    context: CanvasRenderingContext2d,
}

impl Game {
    pub fn new(context: CanvasRenderingContext2d) -> Game {
        let mut images = HashMap::new();
        for i in 1..=20 {
            images.insert(i, get_image(i));
        }

        Game {
            tiles: Vec::new(),
            images: images,
            context: context,
        }
    }

    pub fn images_loaded(&self) -> bool {
        self.images.values().filter(|image| image.complete()).count() > 0
    }

    pub fn init(&mut self) {
        for i in {1..=HORIZONTAL_TILES} {
            let x = (i - 1)*TILE_WIDTH;
            let y = (TILE_HEIGHT*VIRTICAL_TILES) - TILE_HEIGHT;
            console!(log, "tile ", i, x, y);
            self.tiles.push(
                Tile::new(
                    x,
                    y,
                    i as i8,
                )
            );
        }
    }
    pub fn draw(&self) {
        for tile in self.tiles.iter() {
            self.context.draw_image(
                self.images.get(&tile.value).unwrap().clone(),
                tile.x as f64,
                tile.y as f64
            ).unwrap();
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.iter().filter(|tile| {
            (
                x > tile.x &&
                x < tile.x + TILE_WIDTH &&
                y > tile.y &&
                y < tile.y + TILE_HEIGHT
            )
        }).next()
    }

    pub fn mouse_down(&mut self, event: MouseDownEvent) {
        match self.get_tile(event.client_x(), event.client_y()) {
            Some(tile) => {
                Some(tile)
            },
            None => {
                None
            }
        };
    }

    pub fn mouse_move(&mut self, event: MouseMoveEvent) {
    }

    pub fn mouse_up(&mut self, _event: MouseUpEvent) {
    }

    pub fn animate(&self, _time: f64, rc: Rc<RefCell<Self>>) {
        self.draw();
        {
            let rc = rc.clone();
            window().request_animation_frame(move |time| {
                rc.borrow_mut().animate(time, rc.clone());
            });
        }
    }
}
