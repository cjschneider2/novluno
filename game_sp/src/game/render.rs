
use ::error::Error;
use super::buffer;

pub fn clear_with_blue (
    buffer: &mut buffer::Image,
) -> Result<(), Error> {
    let r = 16u8;
    let g = 64u8;
    let b = 240u8;
    assert!( p_size <= m_size);
    for p_y in 0..buffer.height {
        let row = 4 * p_y * buffer.width;
        for p_x in 0..buffer.width {
            let loc = (row + (4 * p_x)) as usize;
            unsafe {
                *buffer.memory.get_unchecked_mut(loc + 0) = b;
                *buffer.memory.get_unchecked_mut(loc + 1) = g;
                *buffer.memory.get_unchecked_mut(loc + 2) = r;
                *buffer.memory.get_unchecked_mut(loc + 3) = 0xFF;
            }
        }
    }
    Ok(())
}

pub fn weird_gradient (
    buffer: &mut buffer::Image,
    x_offset: u32,
    y_offset: u32
) -> Result<(), Error> {
    let p_size = (buffer.height * buffer.width * buffer.bytes_per_pixel) as usize;
    let m_size = buffer.memory.len();
    assert!( p_size <= m_size);
    for p_y in 0..buffer.height {
        let row = 4 * p_y * buffer.width;
        for p_x in 0..buffer.width {
            let red   = 0x00;
            let blue  = ((p_x + x_offset) % 0xFF) as u8;
            let green = ((p_y + y_offset) % 0xFF) as u8;
            let alpha = 0xFF;
            let loc = (row + (4 * p_x)) as usize;
            unsafe {
                *buffer.memory.get_unchecked_mut(loc + 0) = blue;
                *buffer.memory.get_unchecked_mut(loc + 1) = green;
                *buffer.memory.get_unchecked_mut(loc + 2) = red;
                *buffer.memory.get_unchecked_mut(loc + 3) = alpha;
            }
        }
    }
    Ok(())
}

pub fn player (
    buffer: &mut buffer::Image,
    player_x: u32,
    player_y: u32
) -> Result<(), Error> {
    let top = player_y;
    let bottom = player_y + 10;
    // TODO(CJS): We could probably assert! and use get_unchecked eventaully...
    // let end = buffer.memory.len();
    for x in player_x..player_x+10 {
        let x_offset = x * buffer.bytes_per_pixel;
        let mut y_offset = top * buffer.pitch;
        let mut pixel;
        for _ in top..bottom {
            pixel = (x_offset + y_offset) as usize;
            // HACK(CJS): until I get byteorder to write here efficently i'll
            // just hack in the for bytes manually in LE order...
            // let color = 0xFFFFFFFF; // white
            if let Some(p) = buffer.memory.get_mut(pixel + 0) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 1) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 2) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 3) { *p = 0xFF; }
            y_offset += buffer.pitch;
        }
    }
    Ok(())
}
