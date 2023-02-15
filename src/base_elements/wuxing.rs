use core::fmt;

// 五行
#[derive(Debug, Clone, Copy)]
pub enum WuXing {
    Mu,
    Huo,
    Tu,
    Jin,
    Shui,
}

impl fmt::Display for WuXing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WuXing::Mu => write!(f, "木"),
            WuXing::Huo => write!(f, "火"),
            WuXing::Tu => write!(f, "土"),
            WuXing::Jin => write!(f, "金"),
            WuXing::Shui => write!(f, "水"),
        }
    }
}

impl WuXing {
    pub fn sheng(&self) -> WuXing {
        match self {
            WuXing::Mu => WuXing::Huo,
            WuXing::Huo => WuXing::Tu,
            WuXing::Tu => WuXing::Jin,
            WuXing::Jin => WuXing::Shui,
            WuXing::Shui => WuXing::Mu,
        }
    }

    pub fn ke(&self) -> WuXing {
        match self {
            WuXing::Mu => WuXing::Tu,
            WuXing::Huo => WuXing::Jin,
            WuXing::Tu => WuXing::Shui,
            WuXing::Jin => WuXing::Mu,
            WuXing::Shui => WuXing::Huo,
        }
    }
}
