use sdl::Sdl;
use game::Game;
use resource_manager::list_manager::ListType;
use geometry::rectangle::Rectangle;
use geometry::point::Point;
use core_compat::entity::rmd_type::RmdType;
use sdl2;
use sdl2::rect::Rect;
use sdl::render::map::next_tile;
use core_compat::entity::sprite_type::SpriteType;

pub fn tiles(sdl: &mut Sdl, game: &mut Game) {
    let tle_list = game.list_manager.get_list(ListType::Tile).unwrap();
    let map = game.map_manager.get_map(game.state.map).unwrap();
    let tile_stride = map.size_x() as i32;
    let tile_height = 24i32;
    let tile_width = 48i32;
    let mut tile_x = 0i32;
    let mut tile_y = 0i32;

    let view_bounds = Rectangle::new_from_points(
        (-100 - game.state.map_off.0, -100 - game.state.map_off.1),
        (100 + game.window.0, 100 + game.window.1),
    );

    for map_tile in map.tiles().iter() {
        let tile_offset = Point::new(tile_x * tile_width, tile_y * tile_height);
        let mouse_offset = Point::new(game.input.mouse_x, game.input.mouse_y);

        // skip tiles which out out of view
        if view_bounds.contains_point(&tile_offset) == false {
            next_tile(&mut tile_x, &mut tile_y, tile_stride);
            continue;
        }

        // draw map tile
        let tle_entry = map_tile.tle_rmd_entry;
        if tle_entry.file() != 0 {
            let file = tle_entry.file() as usize;
            let index = tle_entry.index() as usize;
            if let Ok(rmd) = game.data_manager.get_data(RmdType::Tile, file) {
                if let Some(entry) = rmd.get_entry(index) {
                    for img in entry.images() {
                        for id in img.image_id.iter() {
                            let item = tle_list.get_item(*id as usize).unwrap();
                            let sprite = game.sprite_manager.get_sprite_entry(&item.entry, SpriteType::Tile, sdl).unwrap();
                            let _w = (img.source_x2 - img.source_x1) as u32;
                            let _h = (img.source_y2 - img.source_y1) as u32;
                            let src_rect = Rect::new(img.source_x1, img.source_y1, _w, _h);
                            let mut dst_rect = Rect::new(0, 0, tile_width as u32, tile_height as u32);
                            dst_rect.offset(tile_offset.x, tile_offset.y);
                            dst_rect.offset(game.state.map_off.0, game.state.map_off.1);

                            // render
                            let _ = sdl.canvas.copy(&sprite.texture, src_rect, dst_rect);

                            // debug render
                            {
                                let _rect = Rectangle::new_from_points((dst_rect.x(), dst_rect.y()), (dst_rect.width() as i32, dst_rect.height() as i32));
                                if _rect.contains_point(&mouse_offset) {
                                    match map_tile.collision {
                                        0 => sdl.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 10, 10)),
                                        _ => sdl.canvas.set_draw_color(sdl2::pixels::Color::RGB(10, 255, 10)),
                                    }
                                    let _ = sdl.canvas.draw_rect(dst_rect);
                                }
                            }
                        }
                    }
                }
            }
        }

        // update tile positions
        next_tile(&mut tile_x, &mut tile_y, tile_stride);
    }
}

