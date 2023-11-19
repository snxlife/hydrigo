use dyn_clone::DynClone;

use std::rc::Rc;

#[derive(Clone)]
pub enum Value<T>
where
    T: 'static + DynClone,
{
    Const(T),
    Expr(Rc<dyn Fn() -> T>),
}

impl<T> Value<T>
where
    T: 'static + DynClone,
{
    pub fn value(&self) -> T {
        match self {
            Value::Const(v) => *dyn_clone::clone_box(v),
            Value::Expr(v) => v(),
        }
    }
    pub fn from_expr<F>(f: F) -> Self
    where
        F: 'static + Fn() -> T,
    {
        Self::Expr(Rc::new(f))
    }
    pub fn from_const(v: T) -> Self {
        Self::Const(v)
    }
}
