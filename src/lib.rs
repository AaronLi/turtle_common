pub mod turtle {
    use std::fmt::Display;

    use async_trait::async_trait;

    #[derive(Copy, Clone, Debug)]
    pub enum TurtleActionDirection {
        Up,
        Down,
        Forward
    }

    impl Display for TurtleActionDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                TurtleActionDirection::Up => "up",
                TurtleActionDirection::Down => "down",
                TurtleActionDirection::Forward => "forward",
            })
        }
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
        type ExecutionException;
        async fn inspect(&mut self, direction: TurtleActionDirection) -> Result<Option<String>, Self::ExecutionException>;
        async fn move_(&mut self, direction: TurtleMoveDirection) -> Result<bool, Self::ExecutionException>;
        async fn turn(&mut self, direction: TurtleTurnDirection) -> Result<bool, Self::ExecutionException>;
        async fn dig(&mut self, direction: TurtleActionDirection, tool: TurtleToolSide) -> Result<bool, Self::ExecutionException>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
