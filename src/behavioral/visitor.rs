
trait Element {
    fn accept(&self, visitor: Box<&mut dyn Visitor>);
}

struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl Product {
    fn new(id: u32, name: String, price: f64) -> Self {
        Product { id, name, price }
    }
}

impl Element for Product {
    fn accept(&self, visitor: Box<&mut dyn Visitor>) {
        visitor.visit_product(self);
    }
}

trait Visitor {
    fn visit_product(&mut self, product: &Product);
}

struct JsonVisitor {
    json: String,
}

impl JsonVisitor {
    fn new() -> Self {
        JsonVisitor { json: String::new() }
    }
}

impl Visitor for JsonVisitor {
    
    fn visit_product(&mut self, product: &Product) {
        self.json.push_str(&format!("{{\"id\": {}, \"name\": \"{}\", \"price\": {}}}", product.id, product.name, product.price));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_discount_strategy() {
        let mut visitor = JsonVisitor::new();

        let product = Product{id: 1, name: String::from("Product 1"), price: 100.0};

        product.accept(Box::new(&mut visitor));

        assert_eq!(visitor.json, "{\"id\": 1, \"name\": \"Product 1\", \"price\": 100}");
    }
}