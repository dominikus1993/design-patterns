


struct Product {
    id: i32,
    name: String,
}

pub trait ProductsProxy {
    fn get_product(&self, id: i32) -> Option<&Product>;
}


pub struct ProductsProxyImpl {
    products: Vec<Product>,
}

impl ProductsProxyImpl {
    pub fn new() -> ProductsProxyImpl {
        ProductsProxyImpl {
            products: vec![
                Product{id: 1, name: String::from("Product 1")},
                Product{id: 2, name: String::from("Product 2")},
                Product{id: 3, name: String::from("Product 3")},
            ]
        }
    }
}


impl ProductsProxy for ProductsProxyImpl {
    fn get_product(&self, id: i32) -> Option<&Product> {
        return self.products.iter().find(|p| p.id == id);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let proxy: Box<dyn ProductsProxy> = Box::new(ProductsProxyImpl::new());


        let product = proxy.get_product(1).unwrap();

        assert_eq!(product.name, "Product 1");
    }
}