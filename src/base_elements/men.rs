// 八门

use core::fmt;

pub struct Men {
    name: String,
    origin_gong_num: usize,
}

pub enum Men8 {
    Xiu,
    Sheng,
    Shang,
    Du,
    // 景门，日京也，为与惊们区别，顾定义为 JingR
    JingR,
    Si,
    // 惊门，心京也，为与景门区别，顾定义为 JingX
    JingX,
    Kai,
}

impl fmt::Display for Men8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Men8::Xiu => write!(f, "休"),
            Men8::Sheng => write!(f, "生"),
            Men8::Shang => write!(f, "伤"),
            Men8::Du => write!(f, "杜"),
            Men8::JingR => write!(f, "景"),
            Men8::Si => write!(f, "死"),
            Men8::JingX => write!(f, "惊"),
            Men8::Kai => write!(f, "开"),
        }
    }
}

impl Men8 {
    pub fn get_men(&self) -> Men {
        match self {
            Men8::Xiu => Men {
                name: "休".to_string(),
                origin_gong_num: 1,
            },
            Men8::Sheng => Men {
                name: "生".to_string(),
                origin_gong_num: 8,
            },
            Men8::Shang => Men {
                name: "伤".to_string(),
                origin_gong_num: 3,
            },
            Men8::Du => Men {
                name: "杜".to_string(),
                origin_gong_num: 4,
            },
            Men8::JingR => Men {
                name: "景".to_string(),
                origin_gong_num: 9,
            },
            Men8::Si => Men {
                name: "死".to_string(),
                origin_gong_num: 2,
            },
            Men8::JingX => Men {
                name: "惊".to_string(),
                origin_gong_num: 7,
            },
            Men8::Kai => Men {
                name: "开".to_string(),
                origin_gong_num: 6,
            },
        }
    }
}
