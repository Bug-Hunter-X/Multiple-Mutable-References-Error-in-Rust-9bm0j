fn main() {
    let mut x = 5;
    { //Use a scope to limit the lifetime of a mutable borrow
        let y = &mut x;
        *y = 10;  
    }
    let z = &mut x; 
    *z = 15;
}