
interface OrderItem {
    productId: string;
    quantity: number;
}

type OrderState = "PENDING" | "SHIPPED" | "DELIVERED" | "CANCELED";

interface Order {
    orderId: string;
    orderDate: Date;
    orderItems: OrderItem[];
    state: OrderState;
}


interface OrderAbstractFactory {
    createOrder(orderId: string, orderDate: Date, orderItems: OrderItem[]): Order;
}