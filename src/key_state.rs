use glutin::event::ElementState;

pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    pub fn is_pressed(&self) -> bool {
        match self {
            Self::Pressed => true,
            _ => false,
        }
    }

    fn is_released(&self) -> bool {
        match self {
            Self::Released => true,
            _ => false,
        }
    }
}

impl From<ElementState> for KeyState {
    fn from(e: ElementState) -> Self {
        match e {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

pub struct MovementState {
    pub forward: KeyState,
    pub backward: KeyState,
    pub right: KeyState,
    pub left: KeyState,
}

impl MovementState {
    pub fn new() -> Self {
        Self {
            forward: KeyState::Released,
            backward: KeyState::Released,
            right: KeyState::Released,
            left: KeyState::Released,
        }
    }
}
