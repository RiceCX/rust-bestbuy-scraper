use kuchiki::traits::*;


pub fn parse_html(html: &str) -> bool {
    let doc = kuchiki::parse_html().one(html);
    let cart_button_node = doc.select_first(".add-to-cart-button").unwrap();
    //assert!(cart_button_node, "This isn't a bestbuy website! The button doesn't even fucking exist.");
    let attributes = cart_button_node.attributes.borrow();

    return !attributes.contains("disabled");
}