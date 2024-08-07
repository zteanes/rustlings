// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    for i in 0..num { // i < 3 loop in essence; i set to 0 and loops until num parameter
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
