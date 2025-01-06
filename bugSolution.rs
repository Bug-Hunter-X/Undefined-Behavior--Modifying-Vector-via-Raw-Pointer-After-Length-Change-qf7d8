fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let ptr = v.as_mut_ptr();

    unsafe {
        // Only modify elements within the original length
        for i in 0..len {
            *ptr.add(i) = *ptr.add(i) * 2; // Example modification
        }
    }

    println!("{:?}", v);
} 