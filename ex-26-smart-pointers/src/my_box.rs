use std::ops::Deref;

#[derive(Debug)]
pub struct MyBox<T> (T);

impl<T> MyBox<T> {
    /// Create new `MyBox`
    ///
    /// # Arguments
    ///
    /// * `value`: any value
    ///
    /// returns: MyBox<T>
    /// # Examples
    ///
    /// ```
    /// use my_box::MyBox;
    /// let mb = MyBox::new(100);
    /// ```
    pub fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }

    /// Get the value reference in `MyBox`
    ///
    /// returns: &T
    /// # Examples
    ///
    /// ```
    /// use my_box::MyBox;
    /// let mb = MyBox::new(100);
    /// println!("my box: {}", *mb.value());
    /// ```
    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /// This is the implementation for `&` operator
    /// it is same as [`value`](#method.value) function
    /// instead of `my_box.value()`, you can use `&my_box` which
    /// is equal to `*(my_box.deref())`
    ///
    /// returns: &T
    /// # Examples
    ///
    /// ```
    /// use my_box::MyBox;
    /// let mb = MyBox::new(100);
    /// println!("my box: {}", *(&mb));
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
