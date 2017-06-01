extern crate gcc;

fn main() {
    gcc::Config::new().file("src/routes.c").include("/usr/include/libnl3").compile("libroutes.a");
    println!("cargo:rustc-flags=-l nl-3 -l nl-route-3");
}
