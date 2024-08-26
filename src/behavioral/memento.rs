trait Memento<'a, T> {
    fn restore(&self) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderStatus {
    New,
    Pending,
    Completed,
    Cancelled,
}

struct Order {
    id : i32,
    status: OrderStatus,
}

struct OrderMemento {
    id : i32,
    status: OrderStatus,
}

impl Memento<'_, Order> for OrderMemento {
    fn restore(&self) -> Order {
        Order{id: self.id, status: self.status}
    }
}


impl Order {
    fn new(id: i32) -> Order {
        Order{id: id, status: OrderStatus::New}
    }

    fn create_memento(&self) -> OrderMemento {
        OrderMemento{id: self.id, status: self.status}
    }

    fn set_pending(&mut self) {
        self.status = OrderStatus::Pending;
    }

    fn set_completed(&mut self) {
        self.status = OrderStatus::Completed;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_discount_strategy() {
        let mut order = Order::new(1);

        let memento = order.create_memento();

        order.set_pending();

        assert_eq!(order.status, OrderStatus::Pending);

        let restored_order = memento.restore();

        assert_eq!(restored_order.status, OrderStatus::New);
    }
}