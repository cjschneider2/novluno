use crate::core_compat::entity::rmd_type::RmdType;
use crate::core_compat::entity::sprite_type::SpriteType;
use crate::game::Game;
use crate::geometry::point::Point;
use crate::geometry::rectangle::Rectangle;
use crate::resource_manager::list_manager::ListType;
use crate::sdl2::rect::Rect;
use crate::sdl::Sdl;
use crate::sdl::render::map::next_tile;
use sdl::render;


struct DrawObject {
    texture: *mut sdl2::sys::SDL_Texture,
    src: Rect,
    dst: Rect,
    z: i32,
    highlight: bool,
}

pub fn objects(sdl: &mut Sdl, game: &mut Game) {
    let obj_list = game.list_manager.get_list(ListType::Object).unwrap();
    let map = game.map_manager.get_map(game.state.map).unwrap();
    let tile_stride = map.size_x() as i32;
    let tile_height = 24i32;
    let tile_width = 48i32;
    let mut tile_x = 0i32;
    let mut tile_y = 0i32;

    let _view_bounds = Rectangle::new_from_points(
        (-100 - game.state.map_off.0, -100 - game.state.map_off.1),
        (100 + game.window.0, 100 + game.window.1),
    );

    let mut draw_list = Vec::<DrawObject>::new();

    for map_tile in map.tiles().iter() {
        let (map_x, map_y) = game.state.map_off;
        let tile_offset = Point::new(
            tile_x * tile_width,
            tile_y * tile_height,
        );
        let mouse_offset = Point::new(
            game.input.mouse_x - map_x,
            game.input.mouse_y - map_y,
        );

        // skip tiles which are out out of view
        let tile_rect = Rectangle::new_from_points((tile_offset.x, tile_offset.y), (tile_width, tile_height));

        // debug: active rectangle
        let is_active = tile_rect.contains_point(&mouse_offset);

        // draw tile objects
        let obj_entry = map_tile.obj_rmd_entry;
        if obj_entry.file() != 0 {
            let file = obj_entry.file() as usize;
            let index = obj_entry.index() as usize;
            if let Ok(rmd) = game.data_manager.get_data(RmdType::Object, file) {
                if let Some(entry) = rmd.get_entry(index) {
                    for img in entry.images() {
                        for id in img.image_id.iter() {
                            // get the sprite
                            let _id: usize = *id as usize;
                            let item = obj_list.get_item(_id).unwrap();
                            let sprite = game.sprite_manager.get_sprite_entry(&item.entry, SpriteType::Object, sdl).unwrap();

                            // calculate the sprite's image offsets
                            let img_rect = Rect::new(0, 0, sprite.sprite.x_dim as u32, sprite.sprite.y_dim as u32);
                            let img_x_1_off = img.source_x1 - sprite.sprite.x_off;
                            let img_y_1_off = img.source_y1 - sprite.sprite.y_off;
                            let _src_pts = [(img_x_1_off, img_y_1_off).into(), (img.source_x2 - sprite.sprite.x_off, img.source_y2 - sprite.sprite.y_off).into()];
                            let mut _x_diff = 0;
                            let mut _y_diff = 0;
                            let mut src_rect = Rect::from_enclose_points(&_src_pts, None).unwrap();
                            if let Some(rect) = src_rect.intersection(img_rect) {
                                if img_x_1_off < 0 { _x_diff = -img_x_1_off; }
                                if img_y_1_off < 0 { _y_diff = -img_y_1_off; }
                                src_rect = rect;
                            }

                            // actually move the destination rectangle into position
                            let mut dst_rect = Rect::new(_x_diff, _y_diff, src_rect.width(), src_rect.height());
                            dst_rect.offset(game.state.map_off.0, game.state.map_off.1);
                            dst_rect.offset(tile_offset.x, tile_offset.y);
                            dst_rect.offset(img.dest_x, img.dest_y);

                            draw_list.push(DrawObject {
                                texture: sprite.texture.raw(),
                                src: src_rect,
                                dst: dst_rect,
                                z: tile_x + tile_y - img.render_z,
                                highlight: is_active,
                            });
                        }
                    }
                }
            }
        } // end if obj_entry != 0

        // update tile positions
        next_tile(&mut tile_x, &mut tile_y, tile_stride);
    }

    // sort by z-index
    draw_list.sort_unstable_by(|obj1, obj2| {
        obj1.z.partial_cmp(&obj2.z).unwrap()
    });

    // render
    struct DebugList {
        dst: Rect,
        z: i32,
    }
    let mut debug_list = Vec::<DebugList>::new();
    draw_list.iter().for_each(|obj| {
        let texture = unsafe {
            sdl.texture_creator.raw_create_texture(obj.texture)
        };
        let _ = sdl.canvas.copy(&texture, obj.src, obj.dst);
        if obj.highlight {
            debug_list.push(DebugList {
                dst: obj.dst,
                z: obj.z
            })
        }
    });

    // debug renders
    debug_list.iter().for_each(|obj| {
        // let (x, y) = game.state.map_off;
        sdl.canvas.set_draw_color(sdl2::pixels::Color::RGB(10, 10, 255));
        let _ = sdl.canvas.draw_rect(obj.dst);
        let z_idx = format!("z:{}", obj.z);
        render::text::line(sdl, &z_idx, obj.dst.left(), obj.dst.top());
    });
}
