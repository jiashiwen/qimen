use core::fmt;

use chrono::DateTime;
use chrono::Datelike;
use once_cell::sync::Lazy;

use super::{wuxing::WuXing, yinyang::YinYang};

pub struct Gan {
    pub name: String,
    pub yinyang: YinYang,
    pub wuxing: WuXing,
}

// 天干
#[derive(Debug, Clone, Copy)]
pub enum TianGan {
    Jia,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui,
}

impl fmt::Display for TianGan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TianGan::Jia => write!(f, "甲"),
            TianGan::Yi => write!(f, "乙"),
            TianGan::Bing => write!(f, "丙"),
            TianGan::Ding => write!(f, "丁"),
            TianGan::Wu => write!(f, "戊"),
            TianGan::Ji => write!(f, "己"),
            TianGan::Geng => write!(f, "庚"),
            TianGan::Xin => write!(f, "辛"),
            TianGan::Ren => write!(f, "壬"),
            TianGan::Gui => write!(f, "癸"),
        }
    }
}

impl TianGan {
    pub fn get_gan(&self) -> Gan {
        match self {
            TianGan::Jia => Gan {
                name: "甲".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Mu,
            },
            TianGan::Yi => Gan {
                name: "乙".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Mu,
            },
            TianGan::Bing => Gan {
                name: "丙".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Huo,
            },
            TianGan::Ding => Gan {
                name: "丁".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Huo,
            },
            TianGan::Wu => Gan {
                name: "戊".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Tu,
            },
            TianGan::Ji => Gan {
                name: "己".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Tu,
            },
            TianGan::Geng => Gan {
                name: "庚".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Jin,
            },
            TianGan::Xin => Gan {
                name: "辛".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Jin,
            },
            TianGan::Ren => Gan {
                name: "壬".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Shui,
            },
            TianGan::Gui => Gan {
                name: "癸".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Shui,
            },
        }
    }
}

pub struct Zhi {
    pub name: String,
    pub yinyang: YinYang,
    pub wuxing: WuXing,
    pub canggan: Vec<TianGan>,
}

// 地支
#[derive(Debug, Clone, Copy)]
pub enum DiZhi {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

impl fmt::Display for DiZhi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiZhi::Zi => write!(f, "子"),
            DiZhi::Chou => write!(f, "丑"),
            DiZhi::Yin => write!(f, "寅"),
            DiZhi::Mao => write!(f, "卯"),
            DiZhi::Chen => write!(f, "辰"),
            DiZhi::Si => write!(f, "巳"),
            DiZhi::Wu => write!(f, "午"),
            DiZhi::Wei => write!(f, "未"),
            DiZhi::Shen => write!(f, "申"),
            DiZhi::You => write!(f, "酉"),
            DiZhi::Xu => write!(f, "戌"),
            DiZhi::Hai => write!(f, "亥"),
        }
    }
}

impl DiZhi {
    // Todo
    pub fn get_zhi(&self) -> Zhi {
        match self {
            DiZhi::Zi => Zhi {
                name: "子".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Shui,
                canggan: vec![TianGan::Gui],
            },
            DiZhi::Chou => Zhi {
                name: "丑".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Tu,
                canggan: vec![TianGan::Ji, TianGan::Gui, TianGan::Xin],
            },
            DiZhi::Yin => Zhi {
                name: "寅".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Mu,
                canggan: vec![TianGan::Jia, TianGan::Bing, TianGan::Wu],
            },
            DiZhi::Mao => Zhi {
                name: "卯".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Mu,
                canggan: vec![TianGan::Yi],
            },
            DiZhi::Chen => Zhi {
                name: "辰".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Tu,
                canggan: vec![TianGan::Wu, TianGan::Yi, TianGan::Gui],
            },
            DiZhi::Si => Zhi {
                name: "巳".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Huo,
                canggan: vec![TianGan::Bing, TianGan::Wu, TianGan::Geng],
            },
            DiZhi::Wu => Zhi {
                name: "午".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Huo,
                canggan: vec![TianGan::Ding, TianGan::Ji],
            },
            DiZhi::Wei => Zhi {
                name: "未".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Tu,
                canggan: vec![TianGan::Ji, TianGan::Ding, TianGan::Yi],
            },
            DiZhi::Shen => Zhi {
                name: "申".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Jin,
                canggan: vec![TianGan::Geng, TianGan::Ren, TianGan::Wu],
            },
            DiZhi::You => Zhi {
                name: "酉".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Jin,
                canggan: vec![TianGan::Xin],
            },
            DiZhi::Xu => Zhi {
                name: "戌".to_string(),
                yinyang: YinYang::Yang,
                wuxing: WuXing::Tu,
                canggan: vec![TianGan::Wu, TianGan::Xin, TianGan::Ding],
            },
            DiZhi::Hai => Zhi {
                name: "亥".to_string(),
                yinyang: YinYang::Yin,
                wuxing: WuXing::Shui,
                canggan: vec![TianGan::Ren, TianGan::Jia],
            },
        }
    }
}

pub static TIAN_GAN_LIST: Lazy<Vec<TianGan>> = Lazy::new(|| {
    let list = vec![
        TianGan::Jia,
        TianGan::Yi,
        TianGan::Bing,
        TianGan::Ding,
        TianGan::Wu,
        TianGan::Ji,
        TianGan::Geng,
        TianGan::Xin,
        TianGan::Ren,
        TianGan::Gui,
    ];
    list
});

pub static DI_ZHI_LIST: Lazy<Vec<DiZhi>> = Lazy::new(|| {
    let list = vec![
        DiZhi::Zi,
        DiZhi::Chou,
        DiZhi::Yin,
        DiZhi::Mao,
        DiZhi::Chen,
        DiZhi::Si,
        DiZhi::Wu,
        DiZhi::Wei,
        DiZhi::Shen,
        DiZhi::You,
        DiZhi::Xu,
        DiZhi::Hai,
    ];
    list
});
// 四柱，干支厉年月日时
// 阳历转干支厉：https://wannianli.tianqi.com/news/273260.html + 五鼠遁元
pub struct SiZhu {
    pub year: (TianGan, DiZhi),
    pub month: (TianGan, DiZhi),
    pub day: (TianGan, DiZhi),
    pub hour: (TianGan, DiZhi),
}

pub static JIAZI_TABLE: Lazy<Vec<Vec<&str>>> = Lazy::new(|| {
    let mut v = Vec::new();
    v.push(vec![
        "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉",
    ]);
    v.push(vec![
        "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未",
    ]);
    v.push(vec![
        "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳",
    ]);
    v.push(vec![
        "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
    ]);
    v.push(vec![
        "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑",
    ]);
    v.push(vec![
        "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
    ]);
    v
});

// 公历转干支厉
pub fn data_to_ganzhi() {
    let date_str = "2020-04-12 22:10:57 +02:00";
    let datetime = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z").unwrap();
    let year = datetime.year();
}

#[cfg(test)]
mod test {
    use super::JIAZI_TABLE;
    //cargo test consts::test::test_jiazi_table --  --nocapture
    #[test]
    fn test_jiazi_table() {
        for i in JIAZI_TABLE.iter() {
            println!("{:?}", i);
        }
    }
}
