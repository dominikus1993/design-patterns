
trait RoundBlock {
    fn get_radius(&self) -> f64;
}

struct CircleBlock {
    radius: f64,
}

struct SquareBlock {
    side: f64,
}

struct RoundHole {
    radis: f64,
}

impl RoundHole {

    fn new(radius: f64) -> RoundHole {
        RoundHole{radis: radius}
    }

    fn fits(&self, block: Box<dyn RoundBlock>) -> bool {
        print!("Radius: {}", block.get_radius());
        return block.get_radius() <= self.radis; 
    }
}

impl RoundBlock for CircleBlock {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

struct SquareToRoundAdapter {
    square: SquareBlock,
}

impl RoundBlock for SquareToRoundAdapter {
    fn get_radius(&self) -> f64 {
        self.square.side * 2f64.sqrt() / 2f64
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_adapter() {
        let square = SquareBlock{side: 2.0};

        let adapter = SquareToRoundAdapter{square: square};

        let hole = RoundHole::new(5.0);

        let fits = hole.fits(Box::new(adapter));

        assert_eq!(fits, true);
    }

    #[test]
    fn test_adapter_when_no_fits() {
        let square = SquareBlock{side: 2.0};

        let adapter = SquareToRoundAdapter{square: square};

        let hole = RoundHole::new(1.0);

        let fits = hole.fits(Box::new(adapter));

        assert_eq!(fits, false);
    }
}