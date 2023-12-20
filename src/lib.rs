pub mod turtle {
    use std::fmt::Display;

    use async_trait::async_trait;

    #[derive(Copy, Clone, Debug)]
    pub enum TurtleActionDirection {
        Up,
        Down,
        Forward
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TurtleMoveDirection {
        Up,
        Down,
        Forward,
        Back
    }
    
    #[derive(Copy, Clone, Debug)]
    pub enum TurtleTurnDirection {
        Left,
        Right
    }

    #[derive(Copy, Clone, Debug)]
    pub enum TurtleToolSide {
        Left,
        Right
    }

    impl Display for TurtleToolSide {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                TurtleToolSide::Left => "left",
                TurtleToolSide::Right => "right",
            })
        }
    }

    #[async_trait]
    pub trait Turtle {
        async fn inspect(&mut self, direction: TurtleActionDirection) -> Option<String>;
        async fn move_(&mut self, direction: TurtleMoveDirection) -> bool;
        async fn turn(&mut self, direction: TurtleTurnDirection) -> bool;
        async fn dig(&mut self, direction: TurtleActionDirection, tool: TurtleToolSide) -> bool;
        async fn place(&mut self, direction: TurtleActionDirection, text: Option<&str>) -> bool;
        async fn suck(&mut self, direction: TurtleActionDirection) -> bool;
        async fn drop(&mut self, direction: TurtleActionDirection) -> bool;
        async fn get_fuel_level(&mut self) -> u64;
        async fn refuel(&mut self, amount: u8) -> bool;
        async fn select(&mut self, slot: u8) -> bool;
        async fn get_item_count(&mut self) -> u64;
        async fn get_item_detail(&mut self, slot: u8) -> Option<String>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
