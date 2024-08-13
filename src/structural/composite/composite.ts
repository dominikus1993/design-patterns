export interface Price {
    getPrice(): number;
    addPrice(price: Price): void
}

export class Product implements Price {
    private readonly price: number;
    private readonly id: string;
    private readonly name: string;

    constructor(id: string, name: string, price: number) {
        this.id = id;
        this.name = name;
        this.price = price;
    }
    addPrice(price: Price): void {
    }

    getPrice(): number {
        return this.price;
    }
}

export class BoxOfProducts implements Price {
    private readonly products: Price[] = [];

    addPrice(price: Price): void {
        this.products.push(price);
    }

    getPrice(): number {
        return this.products.reduce((acc, product) => acc + product.getPrice(), 0);
    }
}

export class PriceCalculator {
    private readonly prices: Price[] = [];

    addPrice(price: Price): void {
        this.prices.push(price);
    }

    calculatePrice(): number {
        return this.prices.reduce((acc, product) => acc + product.getPrice(), 0);
    }
}

export function calculatePrice(products: Price[]): number {
    return products.reduce((acc, product) => acc + product.getPrice(), 0);
}