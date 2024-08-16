trait Observer<T> {
    fn get_id(&self) -> i32;
    fn on_update(&mut self, value: T);
}

#[derive(Debug, Clone, Copy)]
struct IntObserver {
    id: i32,
    current_value: i32,
}

impl IntObserver {
    fn new(id: i32) -> IntObserver {
        IntObserver{id: id, current_value: 0}
    }
    
}

impl Observer<i32> for IntObserver {
    fn on_update(&mut self, value: i32) {
        self.current_value = value;
        print!("Value: {}", value);
    }
    
    fn get_id(&self) -> i32 {
        return self.id;
    }
}

trait Subject<'a, T> {
    fn subscribe(&mut self, subject: &'a mut dyn Observer<i32>);
    fn unsubscribe(&mut self, id: i32);
    fn notify_observers(&mut self);
}

struct IntSubject<'a> {
    value: i32,
    observers: Vec<&'a mut dyn Observer<i32>>,
}

impl<'a> Subject<'a, i32> for IntSubject<'a> {
    fn subscribe(&mut self, observer: &'a mut dyn Observer<i32>) {
        self.observers.push(observer);
    }

    fn notify_observers(&mut self) {
        for observer in self.observers.iter_mut() {
            observer.on_update(self.value);
        }
    }
    
    fn unsubscribe(&mut self, id: i32) {
        self.observers.retain(|x| x.get_id() != id);
    }
}

impl<'a> IntSubject<'a> {
    fn new(value: i32) -> IntSubject<'a> {
        IntSubject{value: value, observers: vec![]}
    }

    fn increment(&mut self) {
        self.value += 1;
        self.notify_observers();
    }
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_observable() {
        let mut subject = IntSubject::new(0);
        let mut observer = IntObserver::new(1);

        subject.subscribe(&mut observer);

        subject.increment();

        assert_eq!(observer.current_value, 1);
    }

    #[test]
    fn test_multi_observable() {
        let mut subject = IntSubject::new(0);
        let mut observer = IntObserver::new(1);
        let mut observer2 = IntObserver::new(2);

        subject.subscribe(&mut observer);
        subject.subscribe(&mut observer2);

        subject.increment();
        subject.increment();

        assert_eq!(observer.current_value, 2);
        assert_eq!(observer2.current_value, 2);
    }

    #[test]
    fn test_multi_observable_unsubscribe() {
        let mut subject = IntSubject::new(0);
        let mut observer = IntObserver::new(1);
        let mut observer2 = IntObserver::new(2);
        let id = observer.get_id();
        subject.subscribe(&mut observer);
        subject.subscribe(&mut observer2);

        subject.unsubscribe(id);

        subject.increment();
        subject.increment();

        assert_eq!(observer.current_value, 0);
        assert_eq!(observer2.current_value, 2);
    }
    
}