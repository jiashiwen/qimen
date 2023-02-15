//八神
pub struct Shen {
    name: String,
    // abbreviation: String,
    ju_gong_num: usize,
}

pub enum Shen8 {
    ZhiFu,
    TengShe,
    TaiYin,
    LiuHe,
    BaiHu,
    XuanWu,
    JiuDi,
    JiuTian,
}
// 、、、、勾陈、朱雀、九地、九天
impl Shen8 {
    pub fn get_shen(&self) -> Shen {
        match self {
            Shen8::ZhiFu => Shen {
                name: "直符".to_string(),
                ju_gong_num: 1,
            },
            Shen8::TengShe => Shen {
                name: "腾蛇".to_string(),
                ju_gong_num: 8,
            },
            Shen8::TaiYin => Shen {
                name: "太阴".to_string(),
                ju_gong_num: 3,
            },
            Shen8::LiuHe => Shen {
                name: "六合".to_string(),
                ju_gong_num: 4,
            },
            Shen8::BaiHu => Shen {
                name: "白虎".to_string(),
                ju_gong_num: 9,
            },
            Shen8::XuanWu => Shen {
                name: "玄武".to_string(),
                ju_gong_num: 2,
            },
            Shen8::JiuDi => Shen {
                name: "九地".to_string(),
                ju_gong_num: 7,
            },
            Shen8::JiuTian => Shen {
                name: "九天".to_string(),
                ju_gong_num: 6,
            },
        }
    }
}

pub enum Shen8_YangDun {
    ZhiFu,
    TengShe,
    TaiYin,
    LiuHe,
    GouChen,
    ZhuQue,
    JiuDi,
    JiuTian,
}

impl Shen8_YangDun {
    pub fn get_shen(&self) -> Shen {
        match self {
            Shen8_YangDun::ZhiFu => Shen {
                name: "直符".to_string(),
                ju_gong_num: 1,
            },
            Shen8_YangDun::TengShe => Shen {
                name: "腾蛇".to_string(),
                ju_gong_num: 8,
            },
            Shen8_YangDun::TaiYin => Shen {
                name: "太阴".to_string(),
                ju_gong_num: 3,
            },
            Shen8_YangDun::LiuHe => Shen {
                name: "六合".to_string(),
                ju_gong_num: 4,
            },
            Shen8_YangDun::GouChen => Shen {
                name: "勾陈".to_string(),
                ju_gong_num: 9,
            },
            Shen8_YangDun::ZhuQue => Shen {
                name: "朱雀".to_string(),
                ju_gong_num: 2,
            },
            Shen8_YangDun::JiuDi => Shen {
                name: "九地".to_string(),
                ju_gong_num: 7,
            },
            Shen8_YangDun::JiuTian => Shen {
                name: "九天".to_string(),
                ju_gong_num: 6,
            },
        }
    }
}
