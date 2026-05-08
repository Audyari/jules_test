fn main() {
    let mut s = String::from("halo");

    let r1 = &mut s;
    let r2 = &mut s; // ERROR: Tidak boleh ada dua mutable borrow dalam satu waktu

    println!("{}, {}", r1, r2);
}
