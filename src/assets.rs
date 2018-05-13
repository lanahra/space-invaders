use opengl_graphics::*;
use std::path::Path;

pub struct Assets {
    pub font: GlyphCache<'static>,
    pub canon: Texture,
    pub alpha_up: Texture,
    pub alpha_down: Texture,
    pub beta_up: Texture,
    pub beta_down: Texture,
    pub gamma_up: Texture,
    pub gamma_down: Texture,
    pub dead: Texture,
    pub spaceship: Texture,
    pub bullet: Texture,
    pub barrier_full: Texture,
    pub barrier_full_bl: Texture,
    pub barrier_full_br: Texture,
    pub barrier_full_tl: Texture,
    pub barrier_full_tr: Texture,
    pub barrier_ok: Texture,
    pub barrier_ok_bl: Texture,
    pub barrier_ok_br: Texture,
    pub barrier_ok_tl: Texture,
    pub barrier_ok_tr: Texture,
    pub barrier_weak: Texture,
    pub barrier_weak_bl: Texture,
    pub barrier_weak_br: Texture,
    pub barrier_weak_tl: Texture,
    pub barrier_weak_tr: Texture,
}

impl Assets {
    pub fn new() -> Assets {
        let fonts = Path::new("./assets/fonts");
        let images = Path::new("./assets/images");
        let settings = TextureSettings::new();

        let font =
            match GlyphCache::new(fonts.join("ca.ttf"), (), settings.clone()) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let canon =
            match Texture::from_path(images.join("canon.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let alpha_up =
            match Texture::from_path(images.join("alpha_up.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let alpha_down =
            match Texture::from_path(images.join("alpha_down.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let beta_up =
            match Texture::from_path(images.join("beta_up.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let beta_down =
            match Texture::from_path(images.join("beta_down.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let gamma_up =
            match Texture::from_path(images.join("gamma_up.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let gamma_down =
            match Texture::from_path(images.join("gamma_down.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let dead =
            match Texture::from_path(images.join("dead.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let spaceship =
            match Texture::from_path(images.join("spaceship.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let bullet =
            match Texture::from_path(images.join("bullet.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier = images.join("barrier");

        let barrier_full =
            match Texture::from_path(barrier.join("full.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_full_bl =
            match Texture::from_path(barrier.join("full_bl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_full_br =
            match Texture::from_path(barrier.join("full_br.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_full_tl =
            match Texture::from_path(barrier.join("full_tl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_full_tr =
            match Texture::from_path(barrier.join("full_tr.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_ok =
            match Texture::from_path(barrier.join("ok.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_ok_bl =
            match Texture::from_path(barrier.join("ok_bl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_ok_br =
            match Texture::from_path(barrier.join("ok_br.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_ok_tl =
            match Texture::from_path(barrier.join("ok_tl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_ok_tr =
            match Texture::from_path(barrier.join("ok_tr.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_weak =
            match Texture::from_path(barrier.join("weak.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_weak_bl =
            match Texture::from_path(barrier.join("weak_bl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_weak_br =
            match Texture::from_path(barrier.join("weak_br.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_weak_tl =
            match Texture::from_path(barrier.join("weak_tl.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        let barrier_weak_tr =
            match Texture::from_path(barrier.join("weak_tr.png"), &settings) {
                Ok(t) => t,
                Err(e) => panic!(e.to_string())
            };

        Assets {
            font,
            canon,
            alpha_up,
            alpha_down,
            beta_up,
            beta_down,
            gamma_up,
            gamma_down,
            dead,
            spaceship,
            bullet,
            barrier_full,
            barrier_full_bl,
            barrier_full_br,
            barrier_full_tl,
            barrier_full_tr,
            barrier_ok,
            barrier_ok_bl,
            barrier_ok_br,
            barrier_ok_tl,
            barrier_ok_tr,
            barrier_weak,
            barrier_weak_bl,
            barrier_weak_br,
            barrier_weak_tl,
            barrier_weak_tr,
        }
    }
}
