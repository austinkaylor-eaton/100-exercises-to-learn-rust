// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

/// An order for a given product
pub struct Order {
    /// The name of the product
    product_name: String,
    /// The quantity of the product available
    quantity: u32,
    /// The unit price of the product
    unit_price: u32
}

impl Order {
    fn product_name_is_empty(product_name: &String) {
        if product_name.is_empty() {
            panic!("Product Name cannot be empty")
        }
    }
    
    fn product_name_is_greater_than_300_chars(product_name: &String) {
        if product_name.len() >= 300 { 
            panic!("Product Name cannot be greater than 300 characters")
        }
    }
    
    fn quantity_is_less_than_zero(quantity: &u32) {
        if quantity <= &0 {
            panic!("Quantity must be greater than 0")
        }
    }
    
    fn unit_price_is_less_than_zero(unit_price: &u32) {
        if unit_price <= &0 {
            panic!("Unit Price must be greater than 0")
        }
    }
    
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order{
        Self::product_name_is_empty(&product_name);
        Self::product_name_is_greater_than_300_chars(&product_name);
        Self::quantity_is_less_than_zero(&quantity);
        Self::unit_price_is_less_than_zero(&unit_price);
        
        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
    
    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::product_name_is_empty(&new_product_name);
        Self::product_name_is_greater_than_300_chars(&new_product_name);
        self.product_name = new_product_name;
    }
    
    pub fn set_quantity(&mut self, new_quantity: u32) {
        Self::quantity_is_less_than_zero(&new_quantity);
        self.quantity = new_quantity;
    }
    
    pub fn set_unit_price(&mut self, new_unit_price: u32) {
        Self::unit_price_is_less_than_zero(&new_unit_price);
        self.unit_price = new_unit_price;
    }
    
    pub fn total(&self) -> u32 {
        let total: u32 = &self.quantity * &self.unit_price;
        total
    }
}
