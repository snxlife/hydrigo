use crate::{component::Drawable, cursor::Cursor, component::Component,
    stylesheet::StyleSheet, value::Value, impl_attribute};

impl_attribute! {
    GroupAttribute {
        content(
            Value::from_const(Vec::new())
        ) : set_content(Value<Vec<Box<dyn Drawable>>>),
    }
}

#[derive(Clone)]
struct GroupDrawable {
    pub(crate) parts: Value<Vec<Box<dyn Drawable>>>,
}

impl Drawable for GroupDrawable {
    fn render(&self, outer: &mut Cursor, style: &StyleSheet) {
        for part in self.parts.value() {
            part.render(outer, style);
        }
    }
}

pub struct Group {
    content: Value<Vec<Box<dyn Drawable>>>,
}

impl Group {
    pub fn new(attr: GroupAttribute) -> Group {
        Self {
            content: attr.content,
        }
    }
}

impl Component for Group {
    type Attribute = GroupAttribute;

    fn into_drawable(self) -> Box<dyn Drawable> {
        Box::new(GroupDrawable {
            parts: self.content,
        })
    }
}