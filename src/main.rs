use demo_rust_di::{dynamic, static_param, static_cake::{self, Handler}};

fn main() {
    dynamic::new_container().handler().handle();
    static_param::new_container().handler().handle();
    static_cake::new_container().handler().handle();
}
