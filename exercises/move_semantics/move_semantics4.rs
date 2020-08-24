// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we instead create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

fn main() {
   
    // Removed, no longer needed. let vec0 = Vec::new();

   // `fill_vec()` doesn't taken any parameters in the new version. let mut vec1 = fill_vec(vec0);

   let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer take `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![22, 44, 66];

    // No longer needed, as we can create the vector inline using the vec macro.
    // vec.push(22);
    // vec.push(44);
    // vec.push(66);

    vec
}
