use crate::{
    component::Drawable, cursor::Cursor, impl_attribute, component::Component,
    stylesheet::StyleSheet, value::Value,
};

impl_attribute! {
    TextAttribute {
        content(
            Value::from_const("".to_owned())
        ) : set_content(Value<String>),
    }
}

#[derive(Clone)]
struct TextDrawable {
    pub(crate) content: String,
}

impl Drawable for TextDrawable {
    fn render(&self, outer: &mut Cursor, style: &StyleSheet) {
        print!("{}", self.content);
    }
}

pub struct Text {
    content: Value<String>,
}

impl Text {
    pub fn new(attr: TextAttribute) -> Self {
        Self {
            content: attr.content,
        }
    }
}

impl Component for Text {
    type Attribute = TextAttribute;

    fn into_drawable(self) -> Box<dyn Drawable> {
        Box::new(TextDrawable {
            content: self.content.value(),
        })
    }
}

impl<T> From<T> for Text
where
    T: ToString + 'static,
{
    fn from(value: T) -> Self {
        Self {
            content: Value::from_const(value.to_string()),
        }
    }
}
