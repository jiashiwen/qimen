// 节气

use super::yinyang::YinYang;

pub struct JieQi {
    name: String,
    ju_gong_num: usize,
}

pub struct YongJu {
    yinyang: YinYang,
    ju_list: Vec<usize>,
}

//立春、雨水、惊蛰、春分、清明、谷雨、立夏、小满、芒种、夏至、小暑、大暑、立秋、处暑、白露、秋分、寒露、霜降、立冬、小雪、大雪、冬至、小寒、大寒

pub enum JieQi24 {
    LiChun,
    YuShui,
    JingZhe,
    ChunFen,
    QingMing,
    GuYu,
    LiXia,
    XiaoMan,
    MangZhong,
    XiaZhi,
    XiaoShu,
    DaShu,
    LiQiu,
    ChuShu,
    BaiLu,
    QiuFen,
    HanLu,
    ShuangJiang,
    LiDong,
    XiaoXue,
    DaXue,
    DongZhi,
    XiaoHan,
    DaHan,
}

pub fn yongju(jieqi: &JieQi24) -> YongJu {
    match jieqi {
        JieQi24::DongZhi | JieQi24::JingZhe => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![1, 7, 4],
        },
        JieQi24::XiaoHan => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![2, 8, 5],
        },
        JieQi24::LiChun => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![8, 5, 2],
        },
        JieQi24::YuShui => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![9, 6, 3],
        },
        JieQi24::QingMing | JieQi24::LiXia => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![4, 1, 7],
        },
        JieQi24::GuYu | JieQi24::XiaoMan => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![5, 2, 8],
        },
        JieQi24::MangZhong => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![6, 3, 9],
        },
        JieQi24::XiaZhi | JieQi24::BaiLu => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![9, 3, 6],
        },
        JieQi24::XiaoShu => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![8, 2, 5],
        },
        JieQi24::DaShu | JieQi24::QiuFen => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![7, 1, 4],
        },
        JieQi24::LiQiu => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![2, 5, 8],
        },
        JieQi24::ChuShu => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![1, 4, 7],
        },
        JieQi24::HanLu | JieQi24::LiDong => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![6, 9, 3],
        },
        JieQi24::ShuangJiang | JieQi24::XiaoXue => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![5, 8, 2],
        },
        JieQi24::LiDong => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![6, 9, 3],
        },
        JieQi24::DaXue => YongJu {
            yinyang: YinYang::Yin,
            ju_list: vec![4, 7, 1],
        },

        JieQi24::DaHan | JieQi24::ChunFen => YongJu {
            yinyang: YinYang::Yang,
            ju_list: vec![3, 9, 6],
        },
    }
}
