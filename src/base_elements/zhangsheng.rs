use core::fmt;

use once_cell::sync::Lazy;

// 十二长生
pub enum ZhangSheng12 {
    ZhangSheng,
    MuYu,
    GuanDai,
    LinGuan,
    DiWang,
    Shuai,
    Bing,
    Si,
    Mu,
    Jue,
    Tai,
    Yang,
}

impl fmt::Display for ZhangSheng12 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZhangSheng12::ZhangSheng => write!(f, "长生"),
            ZhangSheng12::MuYu => write!(f, "沐浴"),
            ZhangSheng12::GuanDai => write!(f, "冠带"),
            ZhangSheng12::LinGuan => write!(f, "临官"),
            ZhangSheng12::DiWang => write!(f, "帝旺"),
            ZhangSheng12::Shuai => write!(f, "衰"),
            ZhangSheng12::Bing => write!(f, "病"),
            ZhangSheng12::Si => write!(f, "死"),
            ZhangSheng12::Mu => write!(f, "墓"),
            ZhangSheng12::Jue => write!(f, "绝"),
            ZhangSheng12::Tai => write!(f, "胎"),
            ZhangSheng12::Yang => write!(f, "养"),
        }
    }
}

pub static ZHANG_SHENG_LIST: Lazy<Vec<ZhangSheng12>> = Lazy::new(|| {
    let zhangsheng = vec![
        ZhangSheng12::ZhangSheng,
        ZhangSheng12::MuYu,
        ZhangSheng12::GuanDai,
        ZhangSheng12::LinGuan,
        ZhangSheng12::DiWang,
        ZhangSheng12::Shuai,
        ZhangSheng12::Bing,
        ZhangSheng12::Si,
        ZhangSheng12::Mu,
        ZhangSheng12::Jue,
        ZhangSheng12::Tai,
        ZhangSheng12::Yang,
    ];
    zhangsheng
});
