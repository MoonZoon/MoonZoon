pub trait Styleable: Sized {
    fn style(self) -> Self {
        self
    } 
}
