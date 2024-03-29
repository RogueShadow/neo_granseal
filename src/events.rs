use std::time::Duration;
use winit::event::{MouseScrollDelta, WindowEvent};

#[derive()]
pub enum Event {
    KeyEvent {
        state: KeyState,
        key: Key,
    },
    MousePressed {
        state: KeyState,
        button: MouseButton,
    },
    MouseMoved(f64, f64),
    MouseWheel(f32, f32),
    Draw,
    Update(Duration),
    Load,
    Resized(u32, u32),
    Custom(Box<dyn std::any::Any>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u16),
    Back,
    Forward,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}
#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub struct ModifierState {
    pub(crate) shift: bool,
    pub(crate) alt: bool,
    pub(crate) ctrl: bool,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Key {
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Grave,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equals,
    BackSpace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LBracket,
    RBracket,
    BackSlash,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    SemiColon,
    Apostrophe,
    Enter,
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    ForwardSlash,
    RShift,
    LCtrl,
    LWin,
    LAlt,
    Space,
    RAlt,
    RWin,
    RCtrl,

    PrintScreen,
    ScrollLock,
    Pause,

    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,

    Up,
    Left,
    Down,
    Right,

    NumLock,
    NumDivide,
    NumMultiply,
    NumSubtract,
    Num7,
    Num8,
    Num9,
    NumAdd,
    Num4,
    Num5,
    Num6,
    Num1,
    Num2,
    Num3,
    NumEnter,
    Num0,
    NumDecimal,

    NotImplemented,
}

pub fn map_events(event: &winit::event::WindowEvent) -> Option<Event> {
    match event {
        winit::event::WindowEvent::KeyboardInput {
            event:
                winit::event::KeyEvent {
                    physical_key: keycode,
                    state,
                    ..
                },
            ..
        } => Some(Event::KeyEvent {
            state: match state {
                winit::event::ElementState::Pressed => KeyState::Pressed,
                winit::event::ElementState::Released => KeyState::Released,
            },
            key: map_keys(keycode),
        }),
        winit::event::WindowEvent::MouseInput {
            device_id: _device_id,
            state,
            button,
        } => Some(Event::MousePressed {
            state: match state {
                winit::event::ElementState::Pressed => KeyState::Pressed,
                winit::event::ElementState::Released => KeyState::Released,
            },
            button: map_mouse_buttons(button),
        }),
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(Event::MouseMoved(position.x, position.y))
        }
        winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
            MouseScrollDelta::LineDelta(x, y) => Some(Event::MouseWheel(*x, *y)),
            _ => None,
        },
        winit::event::WindowEvent::Resized(size) => Some(Event::Resized(size.width, size.height)),
        event => {
            match event {
                WindowEvent::RedrawRequested => {}
                _ => println!("Unhandled event {:?}", event),
            };
            None
        }
    }
}

fn map_mouse_buttons(button: &winit::event::MouseButton) -> MouseButton {
    match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Other(b) => MouseButton::Other(*b),
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
    }
}

pub fn map_keys(key: &winit::keyboard::PhysicalKey) -> Key {
    use winit::keyboard::KeyCode::*;
    use winit::keyboard::PhysicalKey::Code as P;
    use Key as G;

    match key {
        P(Escape) => G::Escape,
        P(F1) => G::F1,
        P(F2) => G::F2,
        P(F3) => G::F3,
        P(F4) => G::F4,
        P(F5) => G::F5,
        P(F6) => G::F6,
        P(F7) => G::F7,
        P(F8) => G::F8,
        P(F9) => G::F9,
        P(F10) => G::F10,
        P(F11) => G::F11,
        P(F12) => G::F12,
        P(Backquote) => G::Grave,
        P(Digit1) => G::Key1,
        P(Digit2) => G::Key2,
        P(Digit3) => G::Key3,
        P(Digit4) => G::Key4,
        P(Digit5) => G::Key5,
        P(Digit6) => G::Key6,
        P(Digit7) => G::Key7,
        P(Digit8) => G::Key8,
        P(Digit9) => G::Key9,
        P(Digit0) => G::Key0,
        P(Minus) => G::Minus,
        P(Equal) => G::Equals,
        P(Backspace) => G::BackSpace,
        P(Tab) => G::Tab,
        P(KeyQ) => G::Q,
        P(KeyW) => G::W,
        P(KeyE) => G::E,
        P(KeyR) => G::R,
        P(KeyT) => G::T,
        P(KeyY) => G::Y,
        P(KeyU) => G::U,
        P(KeyI) => G::I,
        P(KeyO) => G::O,
        P(KeyP) => G::P,
        P(BracketLeft) => G::LBracket,
        P(BracketRight) => G::RBracket,
        P(Backslash) => G::BackSlash,
        P(CapsLock) => G::CapsLock,
        P(KeyA) => G::A,
        P(KeyS) => G::S,
        P(KeyD) => G::D,
        P(KeyF) => G::F,
        P(KeyG) => G::G,
        P(KeyH) => G::H,
        P(KeyJ) => G::J,
        P(KeyK) => G::K,
        P(KeyL) => G::L,
        P(Semicolon) => G::SemiColon,
        P(Quote) => G::Apostrophe,
        P(Enter) => G::Enter,
        P(ShiftLeft) => G::LShift,
        P(KeyZ) => G::Z,
        P(KeyX) => G::X,
        P(KeyC) => G::C,
        P(KeyV) => G::V,
        P(KeyB) => G::B,
        P(KeyN) => G::N,
        P(KeyM) => G::M,
        P(Comma) => G::Comma,
        P(Period) => G::Period,
        P(Slash) => G::ForwardSlash,
        P(ShiftRight) => G::RShift,
        P(ControlLeft) => G::LCtrl,
        P(SuperLeft) => G::LWin,
        P(AltLeft) => G::LAlt,
        P(Space) => G::Space,
        P(AltRight) => G::RAlt,
        P(SuperRight) => G::RWin,
        P(ControlRight) => G::RCtrl,
        P(PrintScreen) => G::PrintScreen,
        P(ScrollLock) => G::ScrollLock,
        P(Pause) => G::Pause,
        P(Insert) => G::Insert,
        P(Home) => G::Home,
        P(PageUp) => G::PageUp,
        P(Delete) => G::Delete,
        P(End) => G::End,
        P(PageDown) => G::PageDown,
        P(ArrowUp) => G::Up,
        P(ArrowLeft) => G::Left,
        P(ArrowDown) => G::Down,
        P(ArrowRight) => G::Right,
        P(NumLock) => G::NumLock,
        P(NumpadDivide) => G::NumDivide,
        P(NumpadMultiply) => G::NumMultiply,
        P(NumpadSubtract) => G::NumSubtract,
        P(Numpad7) => G::Num7,
        P(Numpad8) => G::Num8,
        P(Numpad9) => G::Num9,
        P(NumpadAdd) => G::NumAdd,
        P(Numpad4) => G::Num4,
        P(Numpad5) => G::Num5,
        P(Numpad6) => G::Num6,
        P(Numpad1) => G::Num1,
        P(Numpad2) => G::Num2,
        P(Numpad3) => G::Num3,
        P(NumpadEnter) => G::NumEnter,
        P(Numpad0) => G::Num0,
        P(NumpadDecimal) => G::NumDecimal,
        _ => G::NotImplemented,
    }
}
