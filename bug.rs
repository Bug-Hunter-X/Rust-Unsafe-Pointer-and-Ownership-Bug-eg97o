fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("Value at ptr: {}", *ptr);
    // The line below causes an error because v is dropped
    println!("Vector: {:?}", v);
}