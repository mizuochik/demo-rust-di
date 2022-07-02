use demo_rust_di::static_cake2::{self, MainDI, Handler};

fn main() {
    let d = static_cake2::new_di();
    d.handler().handle();
}
