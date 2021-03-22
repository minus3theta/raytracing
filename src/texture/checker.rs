use crate::{Color, Point3};

use super::{Texture, TextureEnum, TexturePtr};

#[derive(Clone)]
pub struct Checker {
    even: TexturePtr,
    odd: TexturePtr,
}

impl Checker {
    pub fn new(even: impl Into<TexturePtr>, odd: impl Into<TexturePtr>) -> Self {
        Self {
            even: even.into(),
            odd: odd.into(),
        }
    }
}

impl Into<TextureEnum> for Checker {
    fn into(self) -> TextureEnum {
        TextureEnum::Checker(self)
    }
}

impl Into<TexturePtr> for Checker {
    fn into(self) -> TexturePtr {
        Into::<TextureEnum>::into(self).into()
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
