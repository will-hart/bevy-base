/// Inspired by https://github.com/Bobox214/Kataster/tree/master/src (MIT License)
use core::fmt::Debug;

pub enum GameStateStatus<T> {
    Idle,
    Entered(T),
    Exiting(T),
    Running(T),
}

/// A resource which should be added to the world with a custom scene enum
pub struct GameState<TScene: Clone + Copy + Debug> {
    pub current: GameStateStatus<TScene>,
    pub next: Option<TScene>,
}

impl<TScene: Clone + Copy + Debug> GameState<TScene> {
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
                match self.next {
                    Some(next_state) => {
                        println!("Transition IDLE to ENTERED::{:?}", next_state);
                        self.current = GameStateStatus::Entered(next_state);
                    }
                    None => {
                        // Shouldn't this be impossible given check above?
                        println!("Transition from IDLE ignored as no next_state");
                    }
                };
            }
            GameStateStatus::Entered(state) => {
                println!("Transition ENTERED to RUNNING::{:?}", *state);
                self.next = None;
                self.current = GameStateStatus::Running(*state);
            }
            GameStateStatus::Exiting(current_state) => {
                println!(
                    "Transition EXITING::{:?} to ENTERED::{:?}",
                    *current_state,
                    self.next.unwrap()
                );
                self.current = GameStateStatus::Entered(self.next.unwrap());
            }
            GameStateStatus::Running(state) => {
                println!("Transition RUNNING to EXITING::{:?}", *state);
                self.current = GameStateStatus::Exiting(*state);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    #[allow(dead_code)]
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
