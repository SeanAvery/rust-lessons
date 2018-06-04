mod my;

fn function() {
    println!("called functtion");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
