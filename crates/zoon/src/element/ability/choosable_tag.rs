// ------ ChoosableTag ------

pub trait ChoosableTag {
    fn with_tag(tag: Tag) -> Self;
}

// ------ Tag ------

#[derive(Copy, Clone)]
pub enum Tag<'a> {
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Main,
    Nav,
    Section,
    Custom(&'a str),
}

impl<'a> Tag<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Address => "address",
            Self::Article => "article",
            Self::Aside => "aside",
            Self::Footer => "footer",
            Self::Header => "header",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Main => "main",
            Self::Nav => "nav",
            Self::Section => "section",
            Self::Custom(tag) => tag,
        }
    }
}
