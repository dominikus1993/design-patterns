
trait Element {
    fn accept(&self, visitor: Box<dyn Visitor>);
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
    fn accept(&self, visitor: Box<dyn Visitor>) {
        visitor.visit_product(self);
    }
}

trait Visitor {
    fn visit_product(&self, product: &Product);
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
    
    fn visit_product(&self, product: &Product) {
        self.json.push_str(&format!("{{\"id\": {}, \"name\": \"{}\", \"price\": {}}}", product.id, product.name, product.price));
    }
}