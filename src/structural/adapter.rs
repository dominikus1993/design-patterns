use crate::structural::composite::*;

trait PromotionCalculator {
    fn calculate_promotion(&self) -> f64;
}

trait PriceCalculator {
    fn calculate(&self) -> f64;
}

struct OrderCalculator {
    products: Vec<Box<dyn OrderComposite>>,
}

impl OrderCalculator {
    fn new() -> OrderCalculator {
        OrderCalculator{products: vec![]}
    }

    fn add(&mut self, product: Box<dyn OrderComposite>) {
        self.products.push(product);
    }
}

impl PriceCalculator for OrderCalculator {
    fn calculate(&self) -> f64 {
        self.products.iter().map(|p| p.get_total()).sum()
    }
}

struct OrderCalculatorWithPromotion {
    price_calculator: OrderCalculator,
}

impl PromotionCalculator for OrderCalculatorWithPromotion {
    fn calculate_promotion(&self) -> f64 {
        let promotion = 0.2;
        self.price_calculator.calculate() * promotion
    }
}

impl OrderCalculatorWithPromotion {
    fn new(price_calculator: OrderCalculator) -> OrderCalculatorWithPromotion {
        OrderCalculatorWithPromotion{price_calculator: price_calculator}
    }

    fn add(&mut self, product: Box<dyn OrderComposite>) {
        self.price_calculator.add(product);
    }

    fn calculate(&self) -> f64 {
        let total = self.price_calculator.calculate();
        let total_with_promotion = self.calculate_promotion();
        total - total_with_promotion
    }
}