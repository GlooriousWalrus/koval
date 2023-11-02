use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ClassVariant {
    #[default]
    Unstyled,
    Str(&'static str),
    Vec(&'static [&'static str]),
}

impl Variant for ClassVariant {
    fn as_vec(&self) -> Vec<&str> {
        match self {
            ClassVariant::Unstyled => Vec::<&str>::new(), // "".split(" ").collect::<Vec<&str>>(),
            ClassVariant::Str(x) => x.split(" ").collect(),
            ClassVariant::Vec(x) => x.to_vec(),
        }
    }
    fn to_string(&self) -> String {
        self.as_vec()
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Display for ClassVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&Variant::to_string(self))
    }
}

pub trait Variant {
    fn to_string(&self) -> String;
    fn as_vec(&self) -> Vec<&str>;
}
