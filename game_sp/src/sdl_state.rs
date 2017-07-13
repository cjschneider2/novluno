
use sdl2;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::image::Sdl2ImageContext;

use error::Error;

pub struct SdlState {
    pub context: Sdl,
    pub video: VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub image: Sdl2ImageContext,
    pub event: EventPump,
}

impl SdlState {
    pub fn new() -> Result<SdlState, Error> {
        // Setup SDL
        let context = sdl2::init()?;
        let video= context.video()?;
        let window = video.window("novluno", 400, 400)
                          .position_centered()
                          .opengl()
                          .build()?;

        let canvas = window.into_canvas().present_vsync().build()?;
        let image = image::init(INIT_PNG | INIT_JPG)?;
        let event = context.event_pump()?;

        let state = SdlState {
            context: context,
            video: video,
            canvas: canvas,
            image: image,
            event: event,
        };

        Ok(state)
    }
}
