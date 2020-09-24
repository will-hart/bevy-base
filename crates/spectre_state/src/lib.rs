/// Inspired by https://github.com/Bobox214/Kataster/tree/master/src (MIT License)
use core::fmt::Debug;

pub enum GameStateStatus {
    Idle,
    Entered,
    Exiting,
    Running,
}

/// A resource which should be added to the world with a custom scene enum
pub struct GameState<TScene: Clone + Copy + Debug> {
    pub status: GameStateStatus,
    pub current: Option<TScene>,
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
        match &self.status {
            GameStateStatus::Idle => {
                match self.next {
                    Some(next_state) => {
                        println!("[Transition] IDLE to ENTERED::{:?}", next_state);
                        self.status = GameStateStatus::Entered;
                        self.current = Some(next_state);
                    }
                    None => {
                        println!("[Transition] IDLE to ? ignored as no next state");
                    }
                };
            }
            GameStateStatus::Entered => {
                println!(
                    "[Transition] ENTERED::{:?} to RUNNING::{:?}",
                    self.current, self.current
                );
                self.status = GameStateStatus::Running;
            }
            GameStateStatus::Exiting => match self.next {
                Some(_) => {
                    match self.current {
                        None => println!(
                            "[Transition] EXITING::NoState to ENTERED::{:?}",
                            self.next.unwrap()
                        ),
                        Some(_) => println!(
                            "[Transition] EXITING::{:?} to ENTERED::{:?}",
                            self.current.unwrap(),
                            self.next.unwrap()
                        ),
                    }

                    self.status = GameStateStatus::Entered;
                    self.current = self.next.clone();
                    self.next = None;
                }
                _ => {
                    println!("[Transition] Can't move from EXITING::{:?} to ENTERING::None, no next state defined", self.current.unwrap());
                }
            },
            GameStateStatus::Running => match self.next {
                None => {} // no transition queued
                Some(_) => {
                    match self.current {
                        None => println!(
                            "[Transition] RUNNING::NoState, EXITING::{:?}",
                            self.next.unwrap()
                        ),
                        Some(_) => println!(
                            "[Transition] RUNNING::{:?} to EXITING::{:?}, next state is {:?}",
                            self.current.unwrap(),
                            self.current.unwrap(),
                            self.next.unwrap(),
                        ),
                    }
                    self.status = GameStateStatus::Exiting;
                }
            },
        }
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
            status: GameStateStatus::Idle,
            current: None,
            next: Some(TestStates::A),
        };

        match gs.status {
            GameStateStatus::Idle => assert!(true),
            _ => assert!(false),
        };

        gs.update();

        match gs.status {
            GameStateStatus::Entered => match gs.current {
                Some(TestStates::A) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }
}
