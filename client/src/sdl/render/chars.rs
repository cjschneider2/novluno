#![allow(unused_imports)]
use sdl::Sdl;
use game::Game;

use resource_manager::list_manager::ListType;
use geometry::rectangle::Rectangle;
use geometry::point::Point;
use core_compat::entity::rmd_type::RmdType;
use core_compat::entity::sprite_type::SpriteType;
use sdl2;
use sdl2::pixels::Color;

pub fn chars(sdl: &mut Sdl, game: &mut Game) {
    let _list = game.list_manager.get_list(ListType::Chr0);

    let (p_x, p_y) = game.state.player.position;
    let color = Color::RGB(100, 100, 100);
    let rect = sdl2::rect::Rect::new(p_x as i32, p_y as i32, 30, 30);
    sdl.canvas.set_draw_color(color);
    sdl.canvas.draw_rect(rect).unwrap();
}
