fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a : [u64; 100] = [0;100]; // u 64 if size 100 with the 0'
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
