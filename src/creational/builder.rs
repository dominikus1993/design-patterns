pub struct Product {
    id: i32,
    name: String,
}

pub trait Builder {
    fn set_id(&mut self, id: i32);
    fn set_name(&mut self, name: String);
    fn build(&mut self) -> Product;
}


pub struct ConcreteBuilder {
    id: Option<i32>,
    name: Option<String>,
}

impl ConcreteBuilder {
    pub fn new() -> ConcreteBuilder {
        ConcreteBuilder {
            id: None,
            name: None,
        }
    }
    
}

impl Builder for ConcreteBuilder {
    fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn build(&mut self) -> Product {
        let id = self.id.get_or_insert_with(|| 0);
        let name = self.name.get_or_insert_with(|| String::from(""));
        Product {
            id: id.clone(),
            name: name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder = ConcreteBuilder::new();

        builder.set_id(1);
        builder.set_name(String::from("Product 1"));

        let product = builder.build();

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Product 1");
    }
}