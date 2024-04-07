use std::borrow::BorrowMut;

use crate::sdl::Sdl;
use crate::error::Error;
use crate::game::input::MAX_CONTROLLERS as MAX_CTL;

pub fn init(sdl: &mut Sdl) -> Result<(), Error> {
    let num_joy = sdl.controller.num_joysticks()?;
    if sdl.controller_count != num_joy {
        let max = MAX_CTL as u32;
        let max = if num_joy < max { num_joy } else { max };
        for index in 1..max {
            println!("Found Controller index: {:?}", index);
        }
        sdl.controller_count = num_joy;
    }
    Ok(())
}

pub fn add(sdl: &Sdl, index: u32) -> Result<(), Error> {
    let mut controllers = sdl.controllers.borrow_mut();
    if index < MAX_CTL as u32 && index > 0 {
        let ctrl_list = controllers.borrow_mut();
        let new_ctrl = sdl.controller.open(index as u32)?;
        if let Some(spot) = ctrl_list.get_mut(index as usize) {
            *spot = Some(new_ctrl);
        }
    }
    println!("added controller: {}", index);
    Ok(())
}

pub fn remove(sdl: &Sdl, index: u32) -> Result<(), Error> {
    let mut controllers = sdl.controllers.borrow_mut();
    if index < MAX_CTL as u32 && index > 0 {
        let ctrl_list = controllers.borrow_mut();
        if let Some(spot) = ctrl_list.get_mut(index as usize) {
            *spot = None;
        }
    }
    println!("removed controller: {}", index);
    Ok(())
}
