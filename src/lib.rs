use std::{error::Error, fmt, str::FromStr};

use regex::Regex;

pub mod crop;
pub mod info;
pub mod preview;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    t: i64,
    l: i64,
    b: i64,
    r: i64,
}

impl Rect {
    pub fn from_ltbr(t: i64, l: i64, b: i64, r: i64) -> Rect {
        Rect { l, t, r, b }
    }

    pub fn from_xywh(x: i64, y: i64, w: i64, h: i64) -> Rect {
        Rect {
            l: x,
            t: y,
            r: x + w,
            b: y + h,
        }
    }

    pub fn from_args(s: &str) -> Result<Self, String> {
        s.parse().map_err(|_| format!("invalid rect: {}", s))
    }

    pub fn left(&self) -> i64 {
        self.l
    }

    pub fn top(&self) -> i64 {
        self.t
    }

    pub fn right(&self) -> i64 {
        self.r
    }

    pub fn bottom(&self) -> i64 {
        self.b
    }

    pub fn width(&self) -> i64 {
        self.r - self.l
    }

    pub fn height(&self) -> i64 {
        self.b - self.t
    }

    pub fn inset(&self, inner: &Rect) -> Rect {
        let l = (self.l + inner.l).max(0);
        let t = (self.t + inner.t).max(0);
        let r = (self.r - inner.r).max(l);
        let b = (self.b - inner.b).max(t);
        Rect { l, t, r, b }
    }

    pub fn is_empty(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect::from_ltbr(0, 0, 0, 0)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}-{}x{}", self.l, self.t, self.r, self.b)
    }
}
impl FromStr for Rect {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(d) = s.parse::<i64>() {
            return Ok(Rect::from_ltbr(d, d, d, d));
        }

        let p = Regex::new(r"^(\d+)x(\d+)$").unwrap();
        if let Some(caps) = p.captures(s) {
            let h = caps[1].parse::<i64>()?;
            let v = caps[2].parse::<i64>()?;
            return Ok(Rect::from_ltbr(h, v, h, v));
        }

        let p = Regex::new(r"^(\d+)x(\d+)-(\d+)x(\d+)$").unwrap();
        if let Some(caps) = p.captures(s) {
            let l = caps[1].parse::<i64>()?;
            let t = caps[2].parse::<i64>()?;
            let r = caps[3].parse::<i64>()?;
            let b = caps[4].parse::<i64>()?;
            return Ok(Rect::from_ltbr(t, l, b, r));
        }

        Err(format!("invalid rect: {}", s).into())
    }
}
