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
    pub explosion: Texture,
    pub spaceship: Texture,
    pub bullet: Texture,
    pub block_full: Texture,
    pub block_full_bl: Texture,
    pub block_full_br: Texture,
    pub block_full_tl: Texture,
    pub block_full_tr: Texture,
    pub block_half: Texture,
    pub block_half_bl: Texture,
    pub block_half_br: Texture,
    pub block_half_tl: Texture,
    pub block_half_tr: Texture,
    pub block_weak: Texture,
    pub block_weak_bl: Texture,
    pub block_weak_br: Texture,
    pub block_weak_tl: Texture,
    pub block_weak_tr: Texture,
}

impl Assets {
    pub fn new() -> Assets {
        let fonts = Path::new("./assets/fonts");
        let images = Path::new("./assets/images");
        let settings = TextureSettings::new();

        let font =
            GlyphCache::new(
                fonts.join("space_invaders.ttf"),
                (),
                settings.clone()
            ).unwrap();

        let canon =
            Texture::from_path(images.join("canon.png"), &settings)
                .unwrap();

        let alpha_up =
            Texture::from_path(images.join("alpha_up.png"), &settings)
                .unwrap();

        let alpha_down =
            Texture::from_path(images.join("alpha_down.png"), &settings)
                .unwrap();

        let beta_up =
            Texture::from_path(images.join("beta_up.png"), &settings)
                .unwrap();

        let beta_down =
            Texture::from_path(images.join("beta_down.png"), &settings)
                .unwrap();

        let gamma_up =
            Texture::from_path(images.join("gamma_up.png"), &settings)
                .unwrap();

        let gamma_down =
            Texture::from_path(images.join("gamma_down.png"), &settings)
                .unwrap();

        let explosion =
            Texture::from_path(images.join("explosion.png"), &settings)
                .unwrap();

        let spaceship =
            Texture::from_path(images.join("spaceship.png"), &settings)
                .unwrap();

        let bullet =
            Texture::from_path(images.join("bullet.png"), &settings)
                .unwrap();

        let bunker = images.join("bunker");

        let block_full =
            Texture::from_path(bunker.join("full.png"), &settings)
                .unwrap();

        let block_full_bl =
            Texture::from_path(bunker.join("full_bl.png"), &settings)
                .unwrap();

        let block_full_br =
            Texture::from_path(bunker.join("full_br.png"), &settings)
                .unwrap();

        let block_full_tl =
            Texture::from_path(bunker.join("full_tl.png"), &settings)
                .unwrap();

        let block_full_tr =
            Texture::from_path(bunker.join("full_tr.png"), &settings)
                .unwrap();

        let block_half =
            Texture::from_path(bunker.join("half.png"), &settings)
                .unwrap();

        let block_half_bl =
            Texture::from_path(bunker.join("half_bl.png"), &settings)
                .unwrap();

        let block_half_br =
            Texture::from_path(bunker.join("half_br.png"), &settings)
                .unwrap();

        let block_half_tl =
            Texture::from_path(bunker.join("half_tl.png"), &settings)
                .unwrap();

        let block_half_tr =
            Texture::from_path(bunker.join("half_tr.png"), &settings)
                .unwrap();

        let block_weak =
            Texture::from_path(bunker.join("weak.png"), &settings)
                .unwrap();

        let block_weak_bl =
            Texture::from_path(bunker.join("weak_bl.png"), &settings)
                .unwrap();

        let block_weak_br =
            Texture::from_path(bunker.join("weak_br.png"), &settings)
                .unwrap();

        let block_weak_tl =
            Texture::from_path(bunker.join("weak_tl.png"), &settings)
                .unwrap();

        let block_weak_tr =
            Texture::from_path(bunker.join("weak_tr.png"), &settings)
                .unwrap();

        Assets {
            font,
            canon,
            alpha_up,
            alpha_down,
            beta_up,
            beta_down,
            gamma_up,
            gamma_down,
            explosion,
            spaceship,
            bullet,
            block_full,
            block_full_bl,
            block_full_br,
            block_full_tl,
            block_full_tr,
            block_half,
            block_half_bl,
            block_half_br,
            block_half_tl,
            block_half_tr,
            block_weak,
            block_weak_bl,
            block_weak_br,
            block_weak_tl,
            block_weak_tr,
        }
    }
}
