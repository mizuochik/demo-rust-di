use demo_rust_di::static_cake2::{self, MainDep, Handler};

fn main() {
    let d = static_cake2::new_dep();
    d.handler().handle();
}
