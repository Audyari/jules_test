fn main() {
    let s1 = String::from("halo");

    let s2 = &s1; // Meminjam (borrow) s1 tanpa memindahkannya

    println!("Isi s2 (peminjam) adalah: {}", s2);
    println!("Isi s1 (pemilik) adalah: {}", s1); // Sekarang s1 masih bisa diakses!
}
