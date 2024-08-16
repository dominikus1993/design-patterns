trait Command {
    fn execute(&mut self);
}

#[derive(Clone, Copy)]
struct Light {
    is_on: bool,
}

impl Light {
    fn new() -> Light {
        Light{is_on: false}
    }

    fn on(&mut self) {
        self.is_on = true;
        print!("Light is on");
    }

    fn off(&mut self) {
        self.is_on = false;
        print!("Light is off");
    }
}

struct LightOnCommand<'a> {
    light: &'a mut Light,
}

impl<'a> LightOnCommand<'a> {
    fn new(light:&mut Light) -> LightOnCommand {
        LightOnCommand{light: light}
    }
}

impl<'a> Command for LightOnCommand<'a> {
    fn execute(&mut self) {
        self.light.on();
    }
}

struct LightOffCommand<'a> {
    light:&'a mut Light,
}

impl<'a> LightOffCommand<'a> {
    fn new(light:&'a mut Light) -> LightOffCommand<'a> {
        LightOffCommand{light: light}
    }
}

impl<'a> Command for LightOffCommand<'a> {
    fn execute(&mut self) {
        self.light.off();
    }
}

struct CommandInvoker<'a> {
    commands: Vec<&'a mut dyn Command>,
}

impl<'a> CommandInvoker<'a> {

    fn new() -> CommandInvoker<'a> {
        CommandInvoker{commands: vec![]}
    }

    fn add_command(&mut self, command: &'a mut dyn Command) {
        self.commands.push(command);
    }

    fn execute_command(&mut self) {
        let cmd = self.commands.pop();
        match cmd {
            Some(c) => c.execute(),
            None => print!("No command to execute"),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_light_off() {
        let mut subject = Light::new();
        let mut light_off = LightOffCommand::new(&mut subject);

        let mut invoker = CommandInvoker::new();

        invoker.add_command(&mut light_off);

        invoker.execute_command();

        let check = subject.clone();
        assert_eq!(check.is_on, false);
    }

    #[test]
    fn test_light_on() {
        let mut subject = Light::new();
        let mut light_on: LightOnCommand = LightOnCommand::new(&mut subject);

        let mut invoker = CommandInvoker::new();

        invoker.add_command(&mut light_on);

        invoker.execute_command();

        let check = subject.clone();
        assert_eq!(check.is_on, true);
    }
}