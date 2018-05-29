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

pub const TILE_WIDTH : usize = 80;
pub const TILE_HEIGHT : usize = 80;

pub const GRID_WIDTH : usize = 7;
pub const GRID_HEIGHT : usize = 8;



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
    //tiles: Vec<Tile>,
    images: HashMap<i8, ImageElement>,
    context: CanvasRenderingContext2d,
    grid: [[i8; GRID_HEIGHT as usize]; GRID_WIDTH as usize],
    drag_tile: Option<Tile>,
    time_old: f64,
    air_tiles: Vec<Tile>,
}

struct TimeBar {
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
            grid: [[0; GRID_HEIGHT]; GRID_WIDTH],
            drag_tile: None,
            time_old: 0.0,
            air_tiles: Vec::new(),
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
        for x in 0..GRID_WIDTH {
            self.grid[x][GRID_HEIGHT - 2] = self.grid[x][GRID_HEIGHT - 1];
        }

        let date = Date::now();
        for x in 0..GRID_WIDTH {
            let random = (date*date)*(x as f64 + 1.0) % 5.0;
            self.grid[x][GRID_HEIGHT - 1] = random as i8 + 1;
        }
    }
    pub fn draw(&self) {
        /*
        self.context.clear_rect(
            0.0,
            0.0,
            (TILE_WIDTH as f64)*(GRID_WIDTH as f64),
            (TILE_HEIGHT as f64)*(GRID_HEIGHT as f64),
        );
        */
        self.context.fill_rect(
            0.0,
            0.0,
            (TILE_WIDTH as f64)*(GRID_WIDTH as f64),
            (TILE_HEIGHT as f64)*(GRID_HEIGHT as f64),
        );
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                //console!(log, "GRID: ", x as i8, y as i8, self.grid[x][y]);
                match self.grid[x][y] {
                    0 => {
                    },
                    value => {
                        self.context.draw_image(
                            self.images.get(&value).unwrap().clone(),
                            (x as i32 *TILE_WIDTH as i32) as f64,
                            (y as i32 *TILE_HEIGHT as i32) as f64,
                            ).unwrap();
                    }
                }
            }
        };
        {
            match self.drag_tile {
                Some(ref tile) => {
                    self.context.draw_image(
                        self.images.get(&tile.value).unwrap().clone(),
                        (tile.x - (TILE_WIDTH as i32)/2) as f64,
                        (tile.y - (TILE_HEIGHT as i32)/2) as f64,
                        ).unwrap();
                },
                None => {},
            }
        }
        for air_tile in self.air_tiles.iter() {
            self.context.draw_image(
                self.images.get(&air_tile.value).unwrap().clone(),
                (air_tile.x - (TILE_WIDTH as i32)/2) as f64,
                (air_tile.y - (TILE_HEIGHT as i32)/2) as f64,
                ).unwrap();
        }
    }

    fn take_tile(&self, x: i32, y: i32) -> Option<Tile> {
        let (x_tile, y_tile) = self.get_tile_coordinates(x, y);
        match self.grid[x_tile][y_tile] {
            0 => {
                None
                /*
                 * TODO: Make it so one can pick up the tile again.
                match self.air_tiles.iter().filter(|tile| {
                    x > 1
                }).next().take() {
                    None => {
                        None,
                    }
                    Some(tile) => {
                        Some(tile)
                    }
                }
                */
            },
            value => {
                Some(Tile::new(x, y, value))
            }
        }
    }
    fn get_tile_coordinates(&self, x: i32, y: i32) -> (usize, usize) {
        (
            min(max(x / TILE_WIDTH as i32, 0), GRID_WIDTH as i32 - 1) as usize,
            min(max(y / TILE_HEIGHT as i32, 0), GRID_HEIGHT as i32 - 1) as usize
        )
    }

    pub fn mouse_down(&mut self, event: MouseDownEvent) {
        self.drag_tile = self.take_tile(event.client_x(), event.client_y());
        match self.drag_tile {
            Some(ref tile) => {
                let (x_tile, y_tile) = self.get_tile_coordinates(tile.x, tile.y);
                self.grid[x_tile][y_tile] = 0;
            },
            None => {
            },
        }
    }

    fn collision(&self, center_x: i32, center_y: i32) -> bool {
        let tile_height = TILE_HEIGHT as i32 - 50;
        let tile_width = TILE_WIDTH as i32 - 50;
        let corners = [
            (center_x - tile_width/2, center_y - tile_height/2),
            (center_x + tile_width/2, center_y + tile_height/2),
            (center_x - tile_width/2, center_y + tile_height/2),
            (center_x + tile_width/2, center_y - tile_height/2),
        ];

        let mut collided = false;
        let max_x : i32 = TILE_WIDTH as i32 * GRID_WIDTH as i32;
        let max_y : i32 = TILE_HEIGHT as i32 * GRID_HEIGHT as i32;

        for (x, y) in corners.iter() {
            let (x_tile, y_tile) = self.get_tile_coordinates(*x, *y);
            if (self.grid[x_tile][y_tile] != 0 ||
                x < &0 ||
                y < &0 ||
                x > &max_x ||
                y > &max_y
            ) {
                collided = true
            }
        }
        collided
    }

    pub fn mouse_move(&mut self, event: MouseMoveEvent) {
        self.drag_tile = match self.drag_tile.take() {
            Some(mut tile) => {
                if !self.collision(event.client_x(), event.client_y()) {
                    tile.x = event.client_x();
                    tile.y = event.client_y();
                }
                Some(tile)
            },
            None => {
                None
            }
        }
    }

    pub fn mouse_up(&mut self, _event: MouseUpEvent) {
        match self.drag_tile.take() {
            Some(mut tile) => {
                let (x_tile, y_tile) = self.get_tile_coordinates(tile.x, tile.y);

                if self.grid[x_tile][y_tile] == tile.value {
                    self.grid[x_tile][y_tile] += 1;
                } else {
                    match self.grid[x_tile][y_tile] {
                        0 => {
                            //self.grid[x_tile][y_tile] = tile.value;

                            // This is for air tiles
                            let (tile_x, _) = self.get_tile_coordinates(tile.x, tile.y);
                            tile.x = (1 + tile_x as i32)*(TILE_WIDTH as i32) - (TILE_WIDTH as i32)/2;
                            self.air_tiles.push(tile);
                        },
                        _ => {
                        }
                    }
                }
            },
            None => {
            },
        }
        self.drag_tile = None;
    }

    fn drop_tiles(&mut self) {

        let delta_y = 5;

        for tile in self.air_tiles.iter() {
            match self.collision(
                tile.x + (TILE_WIDTH as i32)/2,
                tile.y + (TILE_HEIGHT as i32)/2,
            ) {
                true => {
                    let (x_tile, y_tile) = self.get_tile_coordinates(tile.x, tile.y);
                    console!(log, tile.x, tile.y, tile.value, x_tile as i32, y_tile as i32);
                    if self.grid[x_tile][y_tile] == tile.value {
                        self.grid[x_tile][y_tile] += 1;
                    } else if self.grid[x_tile][y_tile] == 0 {
                        self.grid[x_tile][y_tile] = tile.value;
                    }
                },
                false => {}

            }
        }

        self.air_tiles = self.air_tiles.iter().filter(|tile| {
            !self.collision(
             tile.x - (TILE_WIDTH as i32)/2,
             tile.y - (TILE_HEIGHT as i32)/2,
            ) && tile.y < TILE_HEIGHT as i32 * GRID_HEIGHT as i32
        }).map(|tile| {
            Tile::new(tile.x, tile.y + delta_y, tile.value)
        }).collect();
    }

    pub fn animate(&mut self, time: f64, rc: Rc<RefCell<Self>>) {
        let _dt = (time - self.time_old) as f32;
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
