use core::fmt;

// 阴阳
#[derive(Debug, Clone, Copy)]
pub enum YinYang {
    Yin,
    Yang,
}

impl fmt::Display for YinYang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YinYang::Yin => write!(f, "阴"),
            YinYang::Yang => write!(f, "阳"),
        }
    }
}
