/// Inspired by https://github.com/Bobox214/Kataster/tree/master/src (MIT License)
use core::fmt::Debug;

pub enum GameStatus {
    Idle,
    Entering,
    Exiting,
    Running,
}

/// A resource which should be added to the world with a custom scene enum
pub struct GameState<TScene: Clone + Copy + Debug> {
    pub status: GameStatus,
    pub current: Option<TScene>,
    pub next: Option<TScene>,
}

impl<TScene: Clone + Copy + Debug> GameState<TScene> {
    /// Set the current transition, to be carried out in the next state update
    pub fn set_transition(&mut self, next: TScene) {
        self.next = Some(next);
    }

    /// returns true if the current scene is defined and matches the given scene
    pub fn is_in_scene(&self, scene: &TScene) -> bool {
        if self.current.is_none() {
            return false;
        }

        std::mem::discriminant(&self.current.unwrap()) == std::mem::discriminant(scene)
    }

    /// returns true if the current status matches the given scene
    pub fn is_in_status(&self, status: &GameStatus) -> bool {
        std::mem::discriminant(&self.status) == std::mem::discriminant(status)
    }

    /// Update the state, moving from Idle >> Entering >> Running or
    /// Running >> Exiting >> Entering >> Running one frame at a time
    pub fn update(&mut self) {
        match &self.status {
            GameStatus::Idle => {
                match self.next {
                    Some(next_state) => {
                        println!("[Transition] IDLE to ENTERED::{:?}", next_state);
                        self.status = GameStatus::Entering;
                        self.current = Some(next_state);
                    }
                    None => {
                        println!("[Transition] IDLE to ? ignored as no next state");
                    }
                };
            }
            GameStatus::Entering => {
                println!(
                    "[Transition] ENTERED::{:?} to RUNNING::{:?}",
                    self.current, self.current
                );
                self.status = GameStatus::Running;
            }
            GameStatus::Exiting => match self.next {
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

                    self.status = GameStatus::Entering;
                    self.current = self.next.clone();
                    self.next = None;
                }
                _ => {
                    println!("[Transition] Can't move from EXITING::{:?} to ENTERING::None, no next state defined", self.current.unwrap());
                }
            },
            GameStatus::Running => match self.next {
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
                    self.status = GameStatus::Exiting;
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
            status: GameStatus::Idle,
            current: None,
            next: Some(TestStates::A),
        };

        match gs.status {
            GameStatus::Idle => assert!(true),
            _ => assert!(false),
        };

        gs.update();

        match gs.status {
            GameStatus::Entering => match gs.current {
                Some(TestStates::A) => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }
}
