// 九星

use core::fmt;

pub struct Xing {
    name: String,
    abbreviation: String,
    ju_gong_num: usize,
    ji_gong_num: usize,
}

pub enum Xing9 {
    Peng,
    Ren,
    Chong,
    Fu,
    Ying,
    Rui,
    Zhu,
    Xin,
    Qin,
}

impl fmt::Display for Xing9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Xing9::Peng => write!(f, "蓬"),
            Xing9::Ren => write!(f, "任"),
            Xing9::Chong => write!(f, "冲"),
            Xing9::Fu => write!(f, "辅"),
            Xing9::Ying => write!(f, "英"),
            Xing9::Rui => write!(f, "芮"),
            Xing9::Zhu => write!(f, "柱"),
            Xing9::Xin => write!(f, "心"),
            Xing9::Qin => write!(f, "禽"),
        }
    }
}

// Todo 标注天禽寄宫
impl Xing9 {
    pub fn get_shen(&self) -> Xing {
        match self {
            Xing9::Peng => Xing {
                name: "天蓬".to_string(),
                abbreviation: "蓬".to_string(),
                ju_gong_num: 1,
                ji_gong_num: 1,
            },
            Xing9::Ren => Xing {
                name: "天任".to_string(),
                abbreviation: "任".to_string(),
                ju_gong_num: 8,
                ji_gong_num: 8,
            },
            Xing9::Chong => Xing {
                name: "天冲".to_string(),
                abbreviation: "冲".to_string(),
                ju_gong_num: 3,
                ji_gong_num: 3,
            },
            Xing9::Fu => Xing {
                name: "天辅".to_string(),
                abbreviation: "辅".to_string(),
                ju_gong_num: 4,
                ji_gong_num: 4,
            },
            Xing9::Ying => Xing {
                name: "天英".to_string(),
                abbreviation: "英".to_string(),
                ju_gong_num: 9,
                ji_gong_num: 9,
            },
            Xing9::Rui => Xing {
                name: "天芮".to_string(),
                abbreviation: "芮".to_string(),
                ju_gong_num: 2,
                ji_gong_num: 2,
            },
            Xing9::Zhu => Xing {
                name: "天柱".to_string(),
                abbreviation: "柱".to_string(),
                ju_gong_num: 7,
                ji_gong_num: 7,
            },
            Xing9::Xin => Xing {
                name: "天心".to_string(),
                abbreviation: "心".to_string(),
                ju_gong_num: 6,
                ji_gong_num: 6,
            },
            Xing9::Qin => Xing {
                name: "天禽".to_string(),
                abbreviation: "禽".to_string(),
                ju_gong_num: 5,
                ji_gong_num: 2,
            },
        }
    }
}
