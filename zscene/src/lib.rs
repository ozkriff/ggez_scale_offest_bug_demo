use std::{cell::RefCell, fmt, rc::Rc, time::Duration};

use gwg::{Context, GameResult};

// TODO: z-order? (https://github.com/ozkriff/zemeroth/issues/319)

pub use crate::{
    action::{Action, Boxed},
    sprite::{Facing, Sprite},
};

pub mod action;

mod sprite;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GwgError(gwg::GameError),
    NoDimensions,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GwgError(ref e) => write!(f, "gwg Error: {}", e),
            Error::NoDimensions => write!(f, "The drawable has no dimensions"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::GwgError(ref e) => Some(e),
            Error::NoDimensions => None,
        }
    }
}

impl From<gwg::GameError> for Error {
    fn from(e: gwg::GameError) -> Self {
        Error::GwgError(e)
    }
}

#[derive(Debug)]
struct LayerData {
    sprites: Vec<Sprite>,
}

#[derive(Debug, Clone)]
pub struct Layer {
    data: Rc<RefCell<LayerData>>,
}

impl Layer {
    pub fn new() -> Self {
        let data = LayerData {
            sprites: Vec::new(),
        };
        Self {
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn add(&mut self, sprite: &Sprite) {
        self.data.borrow_mut().sprites.push(sprite.clone());
    }

    pub fn remove(&mut self, sprite: &Sprite) {
        let mut data = self.data.borrow_mut();
        data.sprites.retain(|another| !sprite.is_same(another))
    }

    pub fn has_sprite(&self, sprite: &Sprite) -> bool {
        let data = self.data.borrow();
        data.sprites.iter().any(|other| other.is_same(sprite))
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Scene {
    layers: Vec<Layer>,
    interpreter: ActionInterpreter,
}

impl Scene {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers,
            interpreter: ActionInterpreter::new(),
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        for layer in &self.layers {
            for sprite in &layer.data.borrow().sprites {
                sprite.draw(context)?;
            }
        }
        Ok(())
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.interpreter.add(action);
    }

    pub fn tick(&mut self, dtime: Duration) {
        self.interpreter.tick(dtime);
    }
}

#[derive(Debug)]
struct ActionInterpreter {
    actions: Vec<Box<dyn Action>>,
}

impl ActionInterpreter {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, mut action: Box<dyn Action>) {
        action.begin();
        self.actions.push(action);
    }

    pub fn tick(&mut self, dtime: Duration) {
        let mut forked_actions = Vec::new();
        for action in &mut self.actions {
            action.update(dtime);
            while let Some(forked_action) = action.try_fork() {
                forked_actions.push(forked_action);
            }
            if action.is_finished() {
                action.end();
            }
        }
        for action in forked_actions {
            self.add(action);
        }
        self.actions.retain(|action| !action.is_finished());
    }
}
