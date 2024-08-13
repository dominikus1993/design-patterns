
trait Operation {
    fn make(&self, a: i32, b: i32) -> i32;
}

trait FactoryMethod {
    fn create_operation(&self) -> Box<dyn Operation>;

    fn calculate(&self, a: i32, b: i32) -> i32 {
        let operation = self.create_operation();
        operation.make(a, b)
    }
}

struct AddOperation;

impl Operation for AddOperation {
    fn make(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct AddFactory;

impl FactoryMethod for AddFactory {
    fn create_operation(&self) -> Box<dyn Operation> {
        Box::new(AddOperation)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let operation = AddFactory{};

        let result = operation.calculate(1, 2);

        assert_eq!(result, 3);
    }
}