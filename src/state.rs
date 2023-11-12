use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

pub type Updater<T> = &'static dyn Fn(&T, &T);

/// The collection of [`State`] update handlers
pub mod updater {
    /// This updater won't do anything when a [`State`] is updated
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let state = State::new(1, &updater::ignore);
    /// 
    /// state.set(2); // nothing will happen
    /// assert_eq!(*state.inner(), 2);
    /// ```
    pub fn ignore<T>(_: &T, _: &T) {}

    /// This updater will print the update to terminal when a [`State`] is updated
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let mut state = State::new(1, &updater::print);
    /// 
    /// state.set(2); // will print `[StateUpdated] 1 -> 2`
    /// assert_eq!(*state.inner(), 2);
    /// ```
    pub fn print<T>(old: &T, new: &T)
    where
        T: std::fmt::Debug + 'static,
    {
        use colorful::{Colorful, Color};

        println!(
            "[{}] {} -> {}",
            "StateUpdated".color(Color::LightYellow),
            format!("{:?}", old).color(Color::LightBlue),
            format!("{:?}", new).color(Color::LightBlue)
        );
    }
}


/// The [`State`] represents an internal mutable value that is monitored. It will invoke an [`Updater`] when the internal value changes.
/// 
/// # Examples
/// ```
/// # use hydrigo::state::State;
/// fn count<T: std::fmt::Display>(old: &T, new: &T) {
///     println!("Hey, guys! I just counted from {} to {}!", old, new);
/// }
/// 
/// let state = State::new(1, &count); // don't need to be mutable
/// 
/// state.set(2); // the count function will be invoked
/// assert_eq!(*state.inner(), 2);
/// ```
#[derive(Clone)]
pub struct State<T: 'static> {
    inner: Rc<RefCell<T>>,
    updater: Updater<T>,
}

impl<T> std::fmt::Debug for State<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner())
    }
}

impl<T> State<T> {
    /// Create a [`State`] with an inner value and an updater
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let state = State::new(1, &updater::ignore);
    /// 
    /// assert_eq!(*state.inner(), 1);
    /// ``` 
    pub fn new(inner: T, updater: Updater<T>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
            updater,
        }
    }

    /// Change the updater of a [`State`]
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let mut state = State::new(1, &updater::ignore);
    /// 
    /// state.subscribe(&updater::print);
    /// ``` 
    pub fn subscribe(&mut self, updater: Updater<T>) {
        self.updater = updater;
    }

    /// Borrow the inner value
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let state = State::new(1, &updater::ignore);
    /// 
    /// assert_eq!(*state.inner(), 1);
    /// ``` 
    pub fn inner(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    /// Mutable borrow the inner value.
    /// Modifying a mutable reference won't invoke the updater.
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// let state = State::new(1, &updater::print);
    /// 
    /// *state.inner_mut() = 2; // nothing will be printed to the screen
    /// assert_eq!(*state.inner(), 2);
    /// ```
    pub fn inner_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }

    /// Update the inner value and invoke the updater.
    /// 
    /// # Examples
    /// ```
    /// # use hydrigo::state::{State, updater};
    /// # fn some_func<T>(_: &T, _: &T) {}
    /// let state = State::new(1, &some_func);
    /// 
    /// state.set(2); // it will invoke the function `some_func`
    /// assert_eq!(*state.inner(), 2);
    /// ```
    pub fn set(&self, new: T) {
        (self.updater)(&self.inner(), &new);
        *self.inner_mut() = new;
    }
}