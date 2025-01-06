# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector through a raw pointer after its length has changed. The code attempts to directly modify the vector's elements using `as_mut_ptr()`, leading to unpredictable consequences.

The solution shows how to avoid this issue by ensuring that the vector's length is not changed after obtaining the raw pointer or by using safer methods of vector manipulation.