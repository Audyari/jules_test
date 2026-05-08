fn main() {
    let mut x = 5;
    println!("Nilai x adalah: {}", x);
    x = 6; // ERROR: x bersifat immutable
    println!("Nilai x sekarang adalah: {}", x);
}
