use piston_window::*;
use std::ptr;

pub struct Sprites {
    pub alien_a1: *const G2dTexture,
    pub alien_a2: *const G2dTexture,
    pub alien_b1: *const G2dTexture,
    pub alien_b2: *const G2dTexture,
    pub alien_c1: *const G2dTexture,
    pub alien_c2: *const G2dTexture,
    pub canon: *const G2dTexture,
    pub bullet: *const G2dTexture,
}

impl Sprites {
    pub fn new() -> Sprites {
        Sprites {
            alien_a1: ptr::null(),
            alien_a2: ptr::null(),
            alien_b1: ptr::null(),
            alien_b2: ptr::null(),
            alien_c1: ptr::null(),
            alien_c2: ptr::null(),
            canon: ptr::null(),
            bullet: ptr::null(),
        }
    }
}
