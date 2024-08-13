
trait Component {
    fn operation(&self) -> String;
}

struct ConcreteComponent;

impl Component for ConcreteComponent {
    fn operation(&self) -> String {
        "ConcreteComponent".to_string()
    }
}

struct Decorator {
    component: Box<dyn Component>,
}

impl Component for Decorator {
    fn operation(&self) -> String {
        format!("Decorator({})", self.component.operation())
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let decorator = Decorator{component: Box::new(ConcreteComponent{})};


        let product = decorator.operation();

        assert_eq!(product, "Decorator(ConcreteComponent)");
    }
}