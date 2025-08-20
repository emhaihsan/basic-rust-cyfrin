use hello_rust::foo;
use hello_rust::my;

fn main() {
    my::print();
    my::a::print();
    let _s = my::a::build(1);
    my::a::print_foo();
}
