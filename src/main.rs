use demo_rust_di::{static_param::{self, Handler}};

fn main() {
    static_param::new_container().handler().handle();
}
