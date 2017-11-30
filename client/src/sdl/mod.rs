use std::cell::RefCell;
use std::borrow::BorrowMut;

use sdl2;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;

use core_compat::entity::sprite_type::SpriteType;
use core_compat::entity::rmd_type::RmdType;

use error::Error;
use resource_manager::list_manager::ListType;
use game::Game;
use game::input::Controller;
use game::input::MAX_CONTROLLERS as MAX_CTL;

pub struct Sdl {
    pub context: sdl2::Sdl,
    // video/rendering
    pub video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    // audio
    pub audio: sdl2::AudioSubsystem,
    pub audio_spec: sdl2::audio::AudioSpecDesired,
    // event handlers
    pub event_pump: RefCell<sdl2::EventPump>,
    // controllers
    pub controller: sdl2::GameControllerSubsystem,
    pub controllers: RefCell<[Option<sdl2::controller::GameController>; MAX_CTL]>,
    pub controller_count: u32,
}

impl Sdl {
    pub fn new(width: u32, height: u32) -> Result<Sdl, Error> {
        // -- load SDL2 contexts
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = video.window("Novluno", width, height)
            .position_centered()
            .opengl()
            .build()?;
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()?;
        let texture_creator = canvas.texture_creator();
        let controller = context.game_controller()?;
        let controllers = RefCell::new([None, None, None, None]);
        let event_pump = RefCell::new(context.event_pump()?);
        let audio = context.audio()?;
        let audio_spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),
            samples: Some(4),
        };

        // -- Create SDL state object
        let sdl = Sdl {
            context,
            video,
            canvas,
            texture_creator,
            event_pump,
            audio,
            audio_spec,
            controller,
            controllers,
            controller_count: 0,
        };
        Ok(sdl)
    }

    pub fn init_game_controllers(&mut self) -> Result<(), Error> {
        let num_joy = self.controller.num_joysticks()?;
        if self.controller_count != num_joy {
            let max = MAX_CTL as u32;
            let max = if num_joy < max { num_joy } else { max };
            for index in 1..max {
                println!("Found Controller index: {:?}", index);
            }
            self.controller_count = num_joy;
        }
        Ok(())
    }

    pub fn add_game_controller(&self, index: u32) -> Result<(), Error> {
        let mut controllers = self.controllers.borrow_mut();
        if index < MAX_CTL as u32 && index > 0 {
            let ctrl_list = controllers.borrow_mut();
            let new_ctrl = self.controller.open(index as u32)?;
            if let Some(spot) = ctrl_list.get_mut(index as usize) {
                *spot = Some(new_ctrl);
            }
        }
        println!("added controller: {}", index);
        Ok(())
    }

    pub fn remove_game_controller(&self, index: i32) -> Result<(), Error> {
        let mut controllers = self.controllers.borrow_mut();
        if index < MAX_CTL as i32 && index > 0 {
            let ctrl_list = controllers.borrow_mut();
            if let Some(spot) = ctrl_list.get_mut(index as usize) {
                *spot = None;
            }
        }
        println!("removed controller: {}", index);
        Ok(())
    }

    pub fn handle_events(
        &mut self,
        game: &mut Game
    ) {
        let mut event_pump = self.event_pump.borrow_mut();
        let mut last_event: Option<Event> = None;
        while let Some(new_event) = event_pump.poll_event() {
            if last_event.is_none() || new_event != last_event.unwrap() {
                match new_event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                    => {
                        game.input.should_quit = true;
                    }
                    Event::KeyDown { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = true;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    }
                    Event::KeyUp { keycode: Some(key), repeat, .. }
                    => {
                        let is_down = false;
                        if !repeat {
                            process_keycode(key, is_down, game.get_mut_keyboard());
                        }
                    }
                    Event::Window { win_event: w_event, .. } => {
                        match w_event {
                            WindowEvent::Enter => (),
                            WindowEvent::Leave => (),
                            WindowEvent::SizeChanged(x, y) => {
                                game.input.should_resize = Some((x, y));
                                println!("Window size change: ({},{})", x, y);
                            }
                            _ => (),
                        }
                    }
                    Event::MouseMotion { .. } => (),
                    Event::ControllerDeviceAdded { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        self.add_game_controller(index).unwrap();
                    }
                    Event::ControllerDeviceRemoved { which: index, .. } => {
                        println!("{:?}: {:?}", new_event, index);
                        self.remove_game_controller(index).unwrap();
                    }
                    Event::JoyDeviceAdded { .. } => (),
                    _ => {
                        println!("{:?}", new_event);
                    }
                }
            }
            last_event = Some(new_event);
        } // end while new SDL event
    }


    pub fn render(&mut self, game: &mut Game, _dt: f32) {
        // start frame
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 100, 255));

        // draw background color
        self.canvas.clear();

        // render game map
        self.render_map(game);

        // finish frame
        self.canvas.present();
    }

    pub fn render_map(&mut self, game: &mut Game) {
        let tle_list = game.list_manager.get_list(ListType::Tile).unwrap();
        let obj_list = game.list_manager.get_list(ListType::Object).unwrap();
        let map = game.map_manager.get_map(game.state.map).unwrap();
        let tile_stride = map.size_x() as i32;
        let tile_height = 24;
        let tile_width = 48;
        let mut tile_x = 0;
        let mut tile_y = 0;
        let mut img_count = 0;

        for map_tile in map.tiles().iter() {
            let x_offset = tile_x * tile_width;
            let y_offset = tile_y * tile_height;
            // draw tile
            let tle_entry = map_tile.tle_rmd_entry;
            if tle_entry.file() != 0 {
                let file = tle_entry.file() as usize;
                let index = tle_entry.index() as usize;
                let rmd = game.data_manager.get_data(RmdType::Tile, file).unwrap();
                let entry = rmd.get_entry(index).unwrap();
                for img in entry.images() {
                    for id in img.get_image_id_list().iter() {
                        let item = tle_list.get_item(*id as usize).unwrap();
                        let sprite = game.sprite_manager.get_sprite_entry(&item.entry, SpriteType::Tile, self).unwrap();
                        let src_rect = Rect::new(
                            img.source_x(),
                            img.source_y(),
                            img.source_width() as u32,
                            img.source_height() as u32);
                        let dst_rect = Rect::new(
                            x_offset, y_offset,
                            tile_width as u32, tile_height as u32);
                        let _ = self.canvas.copy(&sprite.texture, src_rect, dst_rect);

                        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 100, 100));
                        let _ = self.canvas.draw_rect(dst_rect);
                    }
                }
            }
            // draw tile objects
            if img_count < 20 {

            let obj_entry = map_tile.obj_rmd_entry;
            if obj_entry.file() != 0 {
                let file = obj_entry.file() as usize;
                let index = obj_entry.index() as usize;
                let rmd = game.data_manager.get_data(RmdType::Object, file).unwrap();
                let entry = rmd.get_entry(index).unwrap();
                for img in entry.images() {
                    for id in img.get_image_id_list().iter() {
                        let item = obj_list.get_item(*id as usize).unwrap();
                        let sprite = game.sprite_manager.get_sprite_entry(&item.entry, SpriteType::Object, self).unwrap();
                        let src_rect = Rect::new(
                            img.source_x(),
                            img.source_y(),
                            img.source_width() as u32,
                            img.source_height() as u32);
                        let dst_rect = Rect::new(
                            x_offset + img.dest_x(),
                            y_offset + img.dest_y(),
                            img.source_width() as u32,
                            img.source_height() as u32);
                        let _ = self.canvas.copy(&sprite.texture, src_rect, dst_rect);

                        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(10, 255, 100));
                        let _ = self.canvas.draw_rect(dst_rect);
                        img_count += 1;
                    }
                }
            }

            }
            // update tile positions
            tile_x += 1;
            if tile_x >= tile_stride {
                tile_x = 0;
                tile_y += 1;
            }
        }
    }
}

fn process_keycode(
    key: sdl2::keyboard::Keycode,
    is_down: bool,
    input: &mut Controller
) {
    match key {
        Keycode::W => input.move_up.key_press(is_down),
        Keycode::A => input.move_left.key_press(is_down),
        Keycode::S => input.move_down.key_press(is_down),
        Keycode::D => input.move_right.key_press(is_down),
        Keycode::Q => input.left_shoulder.key_press(is_down),
        Keycode::E => input.right_shoulder.key_press(is_down),
        Keycode::Up => input.action_up.key_press(is_down),
        Keycode::Down => input.action_down.key_press(is_down),
        Keycode::Right => input.action_right.key_press(is_down),
        Keycode::Left => input.action_left.key_press(is_down),
        Keycode::F => (),
        Keycode::Space => (),
        _ => (),
    }
}
