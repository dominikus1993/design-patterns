
pub trait OrderComposite {
    fn add(&mut self, product: Box<dyn OrderComposite>);
    fn get_total(&self) -> f64;
}

struct Product {
    id: i32,
    price: f64,
}

impl OrderComposite for Product {
    fn add(&mut self, _product: Box<dyn OrderComposite>) {
    }

    fn get_total(&self) -> f64 {
        self.price
    }
}

struct BoxProduct {
    products: Vec<Box<dyn OrderComposite>>,
}

impl BoxProduct {
    fn new() -> BoxProduct {
        BoxProduct{products: vec![]}
    }
    
}

impl OrderComposite for BoxProduct {
    fn add(&mut self, product: Box<dyn OrderComposite>) {
        self.products.push(product);
    }

    fn get_total(&self) -> f64 {
        self.products.iter().map(|p| p.get_total()).sum()
    }
}

struct OrderCalculator {
    order: Vec<Box<dyn OrderComposite>>,
}

impl OrderComposite for OrderCalculator {
    fn add(&mut self, product: Box<dyn OrderComposite>) {
        self.order.push(product);
    }

    fn get_total(&self) -> f64 {
        self.order.iter().map(|p| p.get_total()).sum()
    }
}

impl OrderCalculator {
    fn new() -> OrderCalculator {
        OrderCalculator{order: vec![]}
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let product = Product{id: 1, price: 10.0};
        let mut box_product = BoxProduct::new();
        box_product.add(Box::new(Product{id: 2, price: 20.0}));

        let mut calculator = OrderCalculator::new();
        calculator.add(Box::new(product));
        calculator.add(Box::new(box_product));
        let product = calculator.get_total();

        assert_eq!(product, 30.0);
    }
}