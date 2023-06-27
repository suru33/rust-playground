pub struct MySmartPointer {
    pub value: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer: {}", self.value);
    }
}
