use hydrigo_macros::component;

use crate::{component::Drawable, cursor::Cursor, component::Component,
    stylesheet::StyleSheet};

struct GroupDrawable {
    pub(crate) parts: Vec<Box<dyn Drawable>>,
}

impl Drawable for GroupDrawable {
    fn render(&self, outer: &mut Cursor, style: &StyleSheet) {
        for part in &self.parts {
            part.render(outer, style);
        }
    }
}

#[component]
pub struct Group {
    content: Vec<Box<dyn Drawable>>,
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