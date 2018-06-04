pub fn function() {
    println!("caleld my::nested::function()");
}

#[allow(dead_code)]
pub fn public_function() {
    println!("called my::inacessible::public_function()");
}
