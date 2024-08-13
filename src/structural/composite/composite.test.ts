import { describe, expect, test, it } from '@jest/globals';
import { BoxOfProducts, calculatePrice, PriceCalculator, Product } from './composite';

describe('composite pattern module', () => {

    test('calculate price of products and box of products', () => {
        const priceCalculator = new PriceCalculator();
        priceCalculator.addPrice(new Product('1', 'product1', 1));
        priceCalculator.addPrice(new Product('2', 'product2', 2));
        const boxOfProducts = new BoxOfProducts();
        boxOfProducts.addPrice(new Product('3', 'product3', 3));
        boxOfProducts.addPrice(new Product('4', 'product4', 4));
        priceCalculator.addPrice(boxOfProducts);
        const subject = priceCalculator.calculatePrice();
        expect(subject).toBe(10);
    });
});