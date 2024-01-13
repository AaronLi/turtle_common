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
        type ExecutionError;
        async fn inspect(&mut self, direction: TurtleActionDirection) -> Result<Option<String>, Self::ExecutionError>;
        async fn move_(&mut self, direction: TurtleMoveDirection) -> Result<bool, Self::ExecutionError>;
        async fn turn(&mut self, direction: TurtleTurnDirection) -> Result<bool, Self::ExecutionError>;
        async fn dig(&mut self, direction: TurtleActionDirection, tool: TurtleToolSide) -> Result<bool, Self::ExecutionError>;
        async fn place(&mut self, direction: TurtleActionDirection, text: Option<&str>) -> Result<bool, Self::ExecutionError>;
        async fn suck(&mut self, direction: TurtleActionDirection) -> Result<bool, Self::ExecutionError>;
        async fn drop(&mut self, direction: TurtleActionDirection) -> Result<bool, Self::ExecutionError>;
        async fn get_fuel_level(&mut self) -> Result<u64, Self::ExecutionError>;
        async fn refuel(&mut self, amount: u8) -> Result<bool, Self::ExecutionError>;
        async fn select(&mut self, slot: u8) -> Result<bool, Self::ExecutionError>;
        async fn get_item_count(&mut self) -> Result<u64, Self::ExecutionError>;
        async fn get_item_detail(&mut self, slot: u8) -> Result<Option<String>, Self::ExecutionError>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
