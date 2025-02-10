fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 10; // Modify the value
    }
    println!("Value at ptr: {}", *ptr); 

    // Now, properly manage the vector's ownership
    let v_safe = unsafe { Vec::from_raw_parts(ptr, 3, 3) };
    println!("Vector: {:?}", v_safe); // Access the vector safely
} 