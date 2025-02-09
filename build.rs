fn main() {
    println!("cargo:rustc-link-search=native=C:/sqlite");
    println!("cargo:rustc-link-lib=static=sqlite3");
}
