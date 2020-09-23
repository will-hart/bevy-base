#[derive(Debug)]
pub enum GameStateStatus<T> {
    Idle,
    Entered(T),
    Exiting(T),
    Running(T),
}

/// A resource which should be added to the world with a custom scene enum
#[derive(Debug)]
pub struct GameState<TScene: Clone + Copy> {
    pub current: GameStateStatus<TScene>,
    pub next: Option<TScene>,
}

impl<TScene: Clone + Copy> GameState<TScene> {
    /// Set the current transition, to be carried out in the next state update
    pub fn set_transition(&mut self, next: TScene) {
        self.next = Some(next);
    }

    /// Update the state, moving from Idle >> Entering >> Running or
    /// Running >> Exiting >> Entering >> Running one frame at a time
    pub fn update(&mut self) {
        if self.next.is_none() {
            return;
        }

        match &self.current {
            GameStateStatus::Idle => {
                self.current = GameStateStatus::Entered(self.next.unwrap());
            }
            GameStateStatus::Entered(state) => {
                self.next = None;
                self.current = GameStateStatus::Running(*state);
            }
            GameStateStatus::Exiting(_) => {
                self.current = GameStateStatus::Entered(self.next.unwrap());
            }
            GameStateStatus::Running(state) => {
                self.current = GameStateStatus::Exiting(*state);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub enum TestStates {
        A,
        B,
    }

    #[test]
    fn transitions_on_update() {
        let mut gs = GameState::<TestStates> {
            current: GameStateStatus::Idle,
            next: Some(TestStates::A),
        };

        match gs.current {
            GameStateStatus::Idle => assert!(true),
            _ => assert!(false),
        };

        gs.update();

        match gs.current {
            GameStateStatus::Entered(entered_state) => match entered_state {
                TestStates::A => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }
}
