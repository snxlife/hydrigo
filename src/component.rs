use crate::{cursor::Cursor, stylesheet::StyleSheet};

use dyn_clone::DynClone;

pub trait Drawable: DynClone {
    fn render(&self, outer: &mut Cursor, style: &StyleSheet);
}

dyn_clone::clone_trait_object!(Drawable);

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
