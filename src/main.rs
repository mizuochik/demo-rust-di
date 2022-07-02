use demo_rust_di::dynamic;

fn main() {
    let c = dynamic::new_container();
    c.handler.handle();
}
