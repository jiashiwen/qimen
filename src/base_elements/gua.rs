use serde::{Deserialize, Serialize};

// 卦定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gua8 {
    name: String,
    image: String,
    num_xiantian: usize,
    num_houtian: usize,
}

impl Gua8 {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_image(&self) -> String {
        self.image.clone()
    }

    pub fn get_xiantian_number(&self) -> usize {
        self.num_xiantian
    }

    pub fn get_houtian_number(&self) -> usize {
        self.num_houtian
    }
}

pub enum BaGua {
    Qian,
    Kan,
    Gen,
    Zhen,
    Xun,
    Li,
    Kun,
    Dui,
}

impl BaGua {
    pub fn get_gua(&self) -> Gua8 {
        match self {
            BaGua::Qian => Gua8 {
                name: "乾".to_string(),
                image: "☰".to_string(),
                num_xiantian: 1,
                num_houtian: 6,
            },
            BaGua::Kan => Gua8 {
                name: "坎".to_string(),
                image: "☵".to_string(),
                num_xiantian: 6,
                num_houtian: 1,
            },
            BaGua::Gen => Gua8 {
                name: "艮".to_string(),
                image: "☶".to_string(),
                num_xiantian: 7,
                num_houtian: 8,
            },
            BaGua::Zhen => Gua8 {
                name: "震".to_string(),
                image: "☳".to_string(),
                num_xiantian: 4,
                num_houtian: 3,
            },
            BaGua::Xun => Gua8 {
                name: "巽".to_string(),
                image: "☴".to_string(),
                num_xiantian: 5,
                num_houtian: 4,
            },
            BaGua::Li => Gua8 {
                name: "离".to_string(),
                image: "☲".to_string(),
                num_xiantian: 3,
                num_houtian: 9,
            },
            BaGua::Kun => Gua8 {
                name: "坤".to_string(),
                image: "☷".to_string(),
                num_xiantian: 8,
                num_houtian: 2,
            },
            BaGua::Dui => Gua8 {
                name: "兑".to_string(),
                image: "☱".to_string(),
                num_xiantian: 2,
                num_houtian: 7,
            },
        }
    }
}

#[cfg(test)]
mod test {

    use crate::gua::BaGua;

    //cargo test consts::test::test_gua --  --nocapture
    #[test]
    fn test_gua() {
        let qian = BaGua::Qian;
        let kun = BaGua::Kun;
        let qian_gua = qian.get_gua();
        let kun_gua = kun.get_gua();
        println!("{:?}", qian_gua);
        println!("{:?}", kun_gua);
    }
}
