extern crate pkg_config;
extern crate gcc;

fn main() {
    match pkg_config::find_library("unqlite") {
        Ok(..) => return,
        Err(e) => {
            gcc::Config::new().file("src/unqlite.c").flag("-O3").compile("libunqlite.a");
        },
    }
}
