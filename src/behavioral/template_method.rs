
type Price = i32;
type Discount = i32;

struct Product {
    price: Price,
    discount: Discount,
    quantity: i32,
}

impl Product {
    fn new(price: Price, discount: Discount, quantity: i32) -> Product {
        Product{price: price, discount: discount, quantity: quantity}
    }
}


trait PriceCalculator {
    fn calculate(&self, product: &Product) -> f64 {
        self.calculate__price(product) - self.calculate__discount(product)
    }

    fn calculate__price(&self, product: &Product) -> f64 {
        product.price as f64 * product.quantity as f64
    }

    fn calculate__discount(&self, product: &Product) -> f64;
}

struct PremiumUserPriceCalculator;

impl PriceCalculator for PremiumUserPriceCalculator {

    fn calculate__discount(&self, product: &Product) -> f64 {
        product.discount as f64
    }
}

struct RegularUserPriceCalculator;

impl PriceCalculator for RegularUserPriceCalculator {
    fn calculate__discount(&self, product: &Product) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_premium_user_calculator() {
        let product = Product::new(100, 10, 2);

        let calculator = PremiumUserPriceCalculator{};

        let result = calculator.calculate(&product);

        assert_eq!(result, 190.0);
    }

    #[test]
    fn test_regular_user_calculator() {
        let product = Product::new(100, 10, 2);

        let calculator = RegularUserPriceCalculator{};

        let result = calculator.calculate(&product);

        assert_eq!(result, 200.0);
    }
}