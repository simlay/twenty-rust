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
use std::cmp::{
    min,
    max,
};
use stdweb::web::Date;

pub const TILE_WIDTH  : i32 = 80;
pub const TILE_HEIGHT : i32 = 80;

pub const GRID_WIDTH  : i32 = 7;
pub const GRID_HEIGHT : i32 = 8;



#[derive(Clone, Debug)]
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
    pub fn inside(&self, x: i32, y: i32) -> bool {
        x > self.x - TILE_WIDTH/2 &&
        x < self.x + TILE_WIDTH/2 &&
        y > self.y - TILE_HEIGHT/2 &&
        y < self.y + TILE_HEIGHT/2
    }
}

fn get_image(value: i8) -> ImageElement {
    let image = ImageElement::new();
    image.set_src(&format!("assets/{}.png", value));
    image
}

struct TimeBar {
    percent: f32,
}

static LOAD_TIMEOUT: f64 = 5000.0;

pub struct Game {
    images: HashMap<i8, ImageElement>,
    context: CanvasRenderingContext2d,
    drag_tile: Option<Tile>,
    time_old: f64,
    count_down: f64,
    tiles: Vec<Tile>,
    //air_tiles: Vec<Tile>,
}

impl Game {
    pub fn new(context: CanvasRenderingContext2d) -> Game {
        let mut images = HashMap::new();
        for i in 1..=20 {
            images.insert(i, get_image(i));
        }

        Game {
            images: images,
            context: context,
            drag_tile: None,
            time_old: 0.0,
            count_down: LOAD_TIMEOUT,
            tiles: Vec::new(),
            //air_tiles: Vec::new(),
        }
    }

    /*
    pub fn images_loaded(&self) -> bool {
        self.images.values().filter(|image| image.complete()).count() > 0
    }
    */

    pub fn init(&mut self) {
        self.new_row();
    }

    fn new_row(&mut self) {

        let date = Date::now();

        for y in 0..=0 {
            for x in 0..=GRID_WIDTH {
                let random = (date*date)*(y as f64 + 1.0)*(x as f64 + 1.0) % 5.0;

                self.tiles.push(Tile::new(
                        x*TILE_WIDTH - TILE_WIDTH/2,
                        (GRID_HEIGHT - y)* TILE_HEIGHT - TILE_HEIGHT/2,
                        0 as i8 + 1));
            }
        }
    }
    pub fn draw(&self) {
        self.context.fill_rect(
            0.0,
            0.0,
            (TILE_WIDTH * GRID_WIDTH) as f64,
            (TILE_HEIGHT * GRID_HEIGHT) as f64,
        );
        match self.drag_tile {
            Some(ref tile) => {
                self.context.draw_image(
                    self.images.get(&tile.value).unwrap().clone(),
                    (tile.x - TILE_WIDTH /2) as f64,
                    (tile.y - TILE_HEIGHT/2) as f64,
                    ).unwrap();
            },
            None => {},
        }
        for tile in self.tiles.iter() {
            self.context.draw_image(
                self.images.get(&tile.value).unwrap().clone(),
                (tile.x - TILE_WIDTH/2) as f64,
                (tile.y - TILE_HEIGHT/2) as f64,
                ).unwrap();
        }
    }

    fn get_tile_coordinates(&self, x: i32, y: i32) -> (i32, i32) {
        (
            min(max(x / TILE_WIDTH , 0), GRID_WIDTH  - 1),
            min(max(y / TILE_HEIGHT, 0), GRID_HEIGHT - 1)
        )
    }


    fn collision(&self, center_x: i32, center_y: i32, value: i8) -> bool {
        let padding = 5;
        let corners = [
            (center_x - TILE_WIDTH/2 + padding, center_y - TILE_HEIGHT/2),
            (center_x + TILE_WIDTH/2 - padding, center_y + TILE_HEIGHT/2),
            (center_x - TILE_WIDTH/2 + padding, center_y + TILE_HEIGHT/2),
            (center_x + TILE_WIDTH/2 - padding, center_y - TILE_HEIGHT/2),
        ];

        let max_x = TILE_WIDTH * GRID_WIDTH;
        let max_y = TILE_HEIGHT * GRID_HEIGHT;

        let mut collided = false;
        for (x, y) in corners.iter() {
            if  *x < 0 || *y < 0 || *x > max_x || *y > max_y {
                collided = true;
                break;
            }
            for tile in self.tiles.iter() {
                if tile.inside(*x, *y) && tile.value != value {
                    collided = true;
                    break;
                }
            }
        }
        collided
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.iter().filter(|tile| {
            tile.inside(x, y)
        }).next()
    }

    pub fn mouse_down(&mut self, event: MouseDownEvent) {
        if self.drag_tile.is_none() {
            self.drag_tile = match self.get_tile(event.client_x(), event.client_y()) {
                Some(tile) => {
                    Some(Tile::new(tile.x, tile.y, tile.value))
                },
                None => {
                    None
                },
            };
            self.tiles = self.tiles.iter().filter(|tile| {
                !tile.inside(event.client_x(), event.client_y())
            }).map(|tile| {
                Tile::new(tile.x, tile.y, tile.value)
            }).collect();
        }
    }

    pub fn mouse_move(&mut self, event: MouseMoveEvent) {
        self.drag_tile = match self.drag_tile.take() {
            Some(mut tile) => {
                let mouse_x = event.client_x();
                let mouse_y = event.client_y();

                let mut new_x = tile.x;
                let mut new_y = tile.y;

                if tile.x < mouse_x {
                    for x in tile.x..=mouse_x {
                        if !self.collision(x, tile.y, tile.value) {
                            new_x = x;
                        } else {
                            break;
                        }
                    }
                } else {
                    for x in (mouse_x..=tile.x).rev() {
                        if !self.collision(x, tile.y, tile.value) {
                            new_x = x;
                        } else {
                            break;
                        }
                    }
                }
                tile.x = new_x;

                if tile.y < mouse_y {
                    for y in tile.y..=mouse_y {
                        if !self.collision(tile.x, y, tile.value) {
                            new_y = y;
                        } else {
                            break;
                        }
                    }
                } else {
                    for y in (mouse_y..=tile.y).rev() {
                        if !self.collision(tile.x, y, tile.value) {
                            new_y = y;
                        } else {
                            break;
                        }
                    }
                }


                tile.y = new_y;

                /*
                if !self.collision(mouse_x, mouse_y, tile.value) {
                    tile.x = mouse_x;
                    tile.y = mouse_y;
                } else if !self.collision(mouse_x, tile.y, tile.value) {
                    tile.x = mouse_x;
                } else if !self.collision(tile.y, event.client_y(), tile.value) {
                    tile.y = event.client_y()
                }
                */
                Some(tile)
            },
            None => {
                None
            }
        };
    }

    fn join_tiles(&mut self, new_tile: Tile) {
        self.tiles = self.tiles.iter().map(|tile| {
            match tile.inside(new_tile.x, new_tile.y) && new_tile.value == tile.value {
                true => {
                    Tile::new(tile.x, tile.y, tile.value + 1)
                },
                false => {
                    Tile::new(tile.x, tile.y, tile.value)
                }
            }
        }).collect();


        if self.tiles.iter().filter(|tile| {
            tile.inside(new_tile.x, new_tile.y)
        }).count() == 0 {
            self.tiles.push(new_tile);
        };
    }



    pub fn mouse_up(&mut self, _event: MouseUpEvent) {

        match self.drag_tile.take() {
            Some(mut drag_tile) => {
                let (tile_x, _) = self.get_tile_coordinates(drag_tile.x, drag_tile.y);
                drag_tile.x = tile_x*TILE_WIDTH  + TILE_WIDTH/2;
                self.join_tiles(drag_tile);
            },
            None => {
            },
        }
        self.drag_tile = None;
    }

    fn drop_tiles(&mut self) {

        let delta_y = 5;

        self.tiles = self.tiles.iter().map(|tile| {
            if !self.collision(
             tile.x,
             tile.y + delta_y,
             tile.value,
            ) {
                Tile::new(tile.x, tile.y + delta_y, tile.value)
            } else {
                Tile::new(tile.x, tile.y, tile.value)
            }
        }).collect();
    }

    pub fn animate(&mut self, time: f64, rc: Rc<RefCell<Self>>) {
        /*
        let dt = time - self.time_old;
        self.time_old = time;
        self.count_down -= dt;
        if self.count_down < 0.0 {
            self.count_down = LOAD_TIMEOUT;
            self.new_row();
        }
        */
        self.drop_tiles();


        self.draw();
        {
            let rc = rc.clone();
            window().request_animation_frame(move |time| {
                rc.borrow_mut().animate(time, rc.clone());
            });
        }
    }
}
