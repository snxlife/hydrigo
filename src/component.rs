use crate::{cursor::Cursor, stylesheet::StyleSheet};

pub trait Drawable {
    fn render(&self, outer: &mut Cursor, style: &StyleSheet);
}

pub trait Component {
    type Attribute;
    
    fn into_drawable(self) -> Box<dyn Drawable>;
}

impl<T> From<T> for Box<dyn Drawable>
where
    T: Component,
{
    fn from(value: T) -> Self {
        value.into_drawable()
    }
}
