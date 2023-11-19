use hydrigo_macros::component;

use crate::{
    component::Drawable, cursor::Cursor, component::Component,
    stylesheet::StyleSheet
};

/*impl_attribute! {
    RawTextAttribute {
        content(
            Default::default()
        ) : set_content(String),
    }
}*/

struct RawTextDrawable {
    pub(crate) content: String,
}

impl Drawable for RawTextDrawable {
    fn render(&self, _outer: &mut Cursor, _style: &StyleSheet) {
        print!("{}", self.content);
    }
}

#[component]
pub struct RawText {
    content: String,
}

impl RawText {
    pub fn new(attr: RawTextAttribute) -> Self {
        Self {
            content: attr.content,
        }
    }
}

impl Component for RawText {
    type Attribute = RawTextAttribute;

    fn into_drawable(self) -> Box<dyn Drawable> {
        Box::new(RawTextDrawable {
            content: self.content,
        })
    }
}

impl<T> From<T> for RawText
where
    T: ToString + 'static,
{
    fn from(value: T) -> Self {
        Self {
            content: value.to_string(),
        }
    }
}
