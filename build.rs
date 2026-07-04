fn main() {
    println!("cargo:rerun-if-changed=assets/app.html");
    println!("cargo:rerun-if-changed=assets/index.html");
}
