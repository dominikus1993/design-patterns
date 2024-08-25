
trait Mediator<'a, T> {
    fn notify(&self, sender: &mut T, event: &str);
}

trait AirplaneComponent {
    fn request_landing(&mut self);
    fn request_takeoff(&mut self);
    fn notify_ait_traffic_control(&mut self, event: &str);
}

struct CommercialAirplane<'a> {
    mediator: &'a dyn Mediator<'a, CommercialAirplane<'a>>,
    messages: Vec<String>,
}

impl<'a> AirplaneComponent for CommercialAirplane<'a> {
    
    fn request_landing(&mut self) {
        self.mediator.notify(self, "landing");
    }

    fn request_takeoff(&mut self) {
        self.mediator.notify(self, "takeoff");
    }

    fn notify_ait_traffic_control(&mut self, event: &str) {
        println!("Commercial airplane received event: {}", event);
        self.messages.push(event.to_string());
    }
}

struct AirTrafficControl;

impl<'a> Mediator<'a, CommercialAirplane<'a>> for AirTrafficControl {
    fn notify(&self, sender: &mut CommercialAirplane, event: &str) {
        sender.notify_ait_traffic_control(event);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_premium_user_calculator() {
        let mediator = AirTrafficControl{};
        let mut airplane = CommercialAirplane{mediator: &mediator, messages: vec![]};

        airplane.request_landing();

        assert_eq!(airplane.messages, vec!["landing"]);
    }
}