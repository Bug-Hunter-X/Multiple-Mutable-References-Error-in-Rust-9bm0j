# Multiple Mutable References in Rust

This example showcases a common error in Rust when working with mutable references.  The code attempts to create multiple mutable references to the same variable (`x`), which is not allowed in Rust due to its strict borrowing rules.

The error occurs when trying to create the second mutable reference (`z`).  Rust's borrow checker prevents this to avoid data races and ensure memory safety.

The solution demonstrates a possible way to resolve this, either by using a single mutable reference or modifying the code's logic to not require multiple simultaneous mutable borrows.