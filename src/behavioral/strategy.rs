struct Product {
    price: f64,
    discount: f64,
}

trait PriceCalculator {
    fn calculate(&self, product: Product) -> f64;
}

struct DiscountCalculator;

impl PriceCalculator for DiscountCalculator {
    fn calculate(&self, product: Product) -> f64 {
        product.price * (1.0 - product.discount)
    }
}


struct TaxCalculator;

impl PriceCalculator for TaxCalculator {
    fn calculate(&self, product: Product) -> f64 {
        product.price * 1.23
    }
}

struct PriceCalculatorContext {
    calculator: Box<dyn PriceCalculator>,
}

impl PriceCalculatorContext {
    fn new(calculator: Box<dyn PriceCalculator>) -> PriceCalculatorContext {
        PriceCalculatorContext{calculator: calculator}
    }

    fn set_strategy(&mut self, calculator: Box<dyn PriceCalculator>) {
        self.calculator = calculator;
    }
}

impl PriceCalculator for PriceCalculatorContext {
    fn calculate(&self, product: Product) -> f64 {
        self.calculator.calculate(product)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_discount_strategy() {
        let subject = PriceCalculatorContext::new(Box::new(DiscountCalculator{}));

        let product = Product{price: 100.0, discount: 0.1};

        let result = subject.calculate(product);

        assert_eq!(result, 90.0);
    }


    #[test]
    fn test_tax_strategy() {
        let mut subject = PriceCalculatorContext::new(Box::new(DiscountCalculator{}));

        let product = Product{price: 100.0, discount: 0.1};

        subject.set_strategy(Box::new(TaxCalculator{}));

        let result = subject.calculate(product);

        assert_eq!(result, 123.0);
    }
}