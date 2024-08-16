trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

struct RemoteControll {
    device: Box<dyn Device>,
}

impl RemoteControll {
    fn new(device: Box<dyn Device>) -> RemoteControll {
        RemoteControll{device: device}
    }

    fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
        } else {
            self.device.enable();
        }
    }

    fn is_enabled(&self) -> bool {
        self.device.is_enabled()
    }
}


struct Tv {
    enabled: bool,
}

impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bridge() {
        let tv: Tv = Tv{enabled: false};

        let mut remote = RemoteControll::new(Box::new(tv));

        remote.toggle_power();
        assert_eq!(remote.is_enabled(), true);
    }

    #[test]
    fn test_bridge_double_toggle_power() {
        let tv = Box::new(Tv{enabled: false});

        let mut remote = RemoteControll::new(tv);

        remote.toggle_power();
        remote.toggle_power();
        
        assert_eq!(remote.is_enabled(), false);
    }
}