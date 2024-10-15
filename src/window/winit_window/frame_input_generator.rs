use super::FrameInput;
use crate::control::*;
use crate::core::*;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::TouchPhase;
use winit::event::WindowEvent;
use winit::keyboard::PhysicalKey;

///
/// Use this to generate [FrameInput] for a new frame with a custom [winit](https://crates.io/crates/winit) window.
/// [FrameInput] is automatically generated if using the default [Window](crate::window::Window).
///
pub struct FrameInputGenerator {
    last_time: Instant,
    first_frame: bool,
    events: Vec<Event>,
    accumulated_time: f64,
    viewport: Viewport,
    window_width: u32,
    window_height: u32,
    device_pixel_ratio: f64,
    cursor_pos: Option<LogicalPoint>,
    finger_id: Option<u64>,
    secondary_cursor_pos: Option<LogicalPoint>,
    secondary_finger_id: Option<u64>,
    modifiers: Modifiers,
    mouse_pressed: Option<MouseButton>,
}

impl FrameInputGenerator {
    ///
    /// Creates a new frame input generator.
    ///
    fn new(size: PhysicalSize<u32>, device_pixel_ratio: f64) -> Self {
        let (window_width, window_height): (u32, u32) =
            size.to_logical::<f32>(device_pixel_ratio).into();
        crate::log!(
            "FrameInputGenerator::new {} {}",
            window_width,
            window_height
        );
        Self {
            events: Vec::new(),
            accumulated_time: 0.0,
            viewport: Viewport::new_at_origo(size.width, size.height),
            window_width,
            window_height,
            device_pixel_ratio,
            first_frame: true,
            last_time: Instant::now(),
            cursor_pos: None,
            finger_id: None,
            secondary_cursor_pos: None,
            secondary_finger_id: None,
            modifiers: Modifiers::default(),
            mouse_pressed: None,
        }
    }

    ///
    /// Creates a new frame input generator from a [winit](https://crates.io/crates/winit) window.
    ///
    pub fn from_winit_window(window: &winit::window::Window) -> Self {
        Self::new(window.inner_size(), window.scale_factor())
    }

    ///
    /// Generates [FrameInput] for a new frame. This should be called each frame and the generated data should only be used for one frame.
    ///
    pub fn generate(&mut self, context: &Context) -> FrameInput {
        let now = Instant::now();
        let duration = now.duration_since(self.last_time);
        let elapsed_time =
            duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
        self.accumulated_time += elapsed_time;
        self.last_time = now;

        crate::log!(
            "FrameInputGenerator::generate {} {} : viewport {}x{}",
            self.window_width,
            self.window_height,
            self.viewport.width,
            self.viewport.height
        );

        let frame_input = FrameInput {
            events: self.events.drain(..).collect(),
            elapsed_time,
            accumulated_time: self.accumulated_time,
            viewport: self.viewport,
            window_width: self.window_width,
            window_height: self.window_height,
            device_pixel_ratio: self.device_pixel_ratio as f32,
            first_frame: self.first_frame,
            context: context.clone(),
        };
        self.first_frame = false;

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(exit_time) = option_env!("THREE_D_EXIT").map(|v| v.parse::<f64>().unwrap()) {
            if exit_time < frame_input.accumulated_time {
                #[cfg(feature = "image")]
                if let Some(path) = option_env!("THREE_D_SCREENSHOT") {
                    let pixels = frame_input.screen().read_color::<[u8; 4]>();
                    let img = image::DynamicImage::ImageRgba8(
                        image::ImageBuffer::from_raw(
                            frame_input.viewport.width,
                            frame_input.viewport.height,
                            pixels.into_iter().flatten().collect::<Vec<_>>(),
                        )
                        .unwrap(),
                    );
                    img.resize(
                        frame_input.window_width,
                        frame_input.window_height,
                        image::imageops::FilterType::Triangle,
                    )
                    .save(path)
                    .unwrap();
                }
                std::process::exit(0);
            }
        }
        frame_input
    }

    ///
    /// Handle the [WindowEvent] generated by a [winit](https://crates.io/crates/winit) event loop.
    ///
    pub fn handle_winit_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.viewport = Viewport::new_at_origo(physical_size.width, physical_size.height);
                let logical_size = physical_size.to_logical(self.device_pixel_ratio);
                self.window_width = logical_size.width;
                self.window_height = logical_size.height;
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                // new_inner_size,
                ..
            } => {
                self.device_pixel_ratio = *scale_factor;
                // self.viewport = Viewport::new_at_origo(new_inner_size.width, new_inner_size.height);
                // let logical_size = new_inner_size.to_logical(self.device_pixel_ratio);
                // self.window_width = logical_size.width;
                // self.window_height = logical_size.height;
            }
            WindowEvent::Occluded(false) => {
                self.first_frame = true;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    use winit::keyboard::KeyCode;
                    let state = event.state == winit::event::ElementState::Pressed;
                    if let Some(kind) = translate_virtual_key_code(keycode) {
                        self.events.push(if state {
                            crate::Event::KeyPress {
                                kind,
                                modifiers: self.modifiers,
                                handled: false,
                            }
                        } else {
                            crate::Event::KeyRelease {
                                kind,
                                modifiers: self.modifiers,
                                handled: false,
                            }
                        });
                    } else if keycode == KeyCode::ControlLeft || keycode == KeyCode::ControlRight {
                        self.modifiers.ctrl = state;
                        if !cfg!(target_os = "macos") {
                            self.modifiers.command = state;
                        }
                        self.events.push(crate::Event::ModifiersChange {
                            modifiers: self.modifiers,
                        });
                    } else if keycode == KeyCode::AltLeft || keycode == KeyCode::AltRight {
                        self.modifiers.alt = state;
                        self.events.push(crate::Event::ModifiersChange {
                            modifiers: self.modifiers,
                        });
                    } else if keycode == KeyCode::ShiftLeft || keycode == KeyCode::ShiftRight {
                        self.modifiers.shift = state;
                        self.events.push(crate::Event::ModifiersChange {
                            modifiers: self.modifiers,
                        });
                    } else if (keycode == KeyCode::SuperLeft || keycode == KeyCode::SuperRight)
                        && cfg!(target_os = "macos")
                    {
                        self.modifiers.command = state;
                        self.events.push(crate::Event::ModifiersChange {
                            modifiers: self.modifiers,
                        });
                    }
                }
                if let Some(text) = &event.text {
                    let mut s = String::new();
                    for ch in text.chars() {
                        if is_printable_char(ch) && !self.modifiers.ctrl && !self.modifiers.command
                        {
                            s.push(ch);
                        }
                    }
                    if !s.is_empty() {
                        self.events.push(crate::Event::Text(s));
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(position) = self.cursor_pos {
                    match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            let line_height = 24.0; // TODO
                            self.events.push(crate::Event::MouseWheel {
                                delta: (*x * line_height, *y * line_height),
                                position: position.into(),
                                modifiers: self.modifiers,
                                handled: false,
                            });
                        }
                        winit::event::MouseScrollDelta::PixelDelta(delta) => {
                            let d = delta.to_logical(self.device_pixel_ratio);
                            self.events.push(crate::Event::MouseWheel {
                                delta: (d.x, d.y),
                                position: position.into(),
                                modifiers: self.modifiers,
                                handled: false,
                            });
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(position) = self.cursor_pos {
                    let button = match button {
                        winit::event::MouseButton::Left => Some(crate::MouseButton::Left),
                        winit::event::MouseButton::Middle => Some(crate::MouseButton::Middle),
                        winit::event::MouseButton::Right => Some(crate::MouseButton::Right),
                        _ => None,
                    };
                    if let Some(b) = button {
                        self.events
                            .push(if *state == winit::event::ElementState::Pressed {
                                self.mouse_pressed = Some(b);
                                crate::Event::MousePress {
                                    button: b,
                                    position: position.into(),
                                    modifiers: self.modifiers,
                                    handled: false,
                                }
                            } else {
                                self.mouse_pressed = None;
                                crate::Event::MouseRelease {
                                    button: b,
                                    position: position.into(),
                                    modifiers: self.modifiers,
                                    handled: false,
                                }
                            });
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let p = position.to_logical(self.device_pixel_ratio);
                let delta = if let Some(last_pos) = self.cursor_pos {
                    (p.x - last_pos.x, p.y - last_pos.y)
                } else {
                    (0.0, 0.0)
                };
                let position = LogicalPoint {
                    x: p.x,
                    y: p.y,
                    device_pixel_ratio: self.device_pixel_ratio as f32,
                    height: self.viewport.height as f32,
                };
                self.events.push(crate::Event::MouseMotion {
                    button: self.mouse_pressed,
                    delta,
                    position: position.into(),
                    modifiers: self.modifiers,
                    handled: false,
                });
                self.cursor_pos = Some(position);
            }
            WindowEvent::CursorEntered { .. } => {
                self.events.push(crate::Event::MouseEnter);
            }
            WindowEvent::CursorLeft { .. } => {
                self.mouse_pressed = None;
                self.events.push(crate::Event::MouseLeave);
            }
            WindowEvent::Touch(touch) => {
                let position = touch.location.to_logical::<f32>(self.device_pixel_ratio);
                let position = LogicalPoint {
                    x: position.x,
                    y: position.y,
                    device_pixel_ratio: self.device_pixel_ratio as f32,
                    height: self.viewport.height as f32,
                };
                match touch.phase {
                    TouchPhase::Started => {
                        if self.finger_id.is_none() {
                            self.events.push(crate::Event::MousePress {
                                button: MouseButton::Left,
                                position: position.into(),
                                modifiers: self.modifiers,
                                handled: false,
                            });
                            self.cursor_pos = Some(position);
                            self.finger_id = Some(touch.id);
                        } else if self.secondary_finger_id.is_none() {
                            self.secondary_cursor_pos = Some(position);
                            self.secondary_finger_id = Some(touch.id);
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        if self.finger_id.map(|id| id == touch.id).unwrap_or(false) {
                            self.events.push(crate::Event::MouseRelease {
                                button: MouseButton::Left,
                                position: position.into(),
                                modifiers: self.modifiers,
                                handled: false,
                            });
                            self.cursor_pos = None;
                            self.finger_id = None;
                        } else if self
                            .secondary_finger_id
                            .map(|id| id == touch.id)
                            .unwrap_or(false)
                        {
                            self.secondary_cursor_pos = None;
                            self.secondary_finger_id = None;
                        }
                    }
                    TouchPhase::Moved => {
                        if self.finger_id.map(|id| id == touch.id).unwrap_or(false) {
                            let last_pos = self.cursor_pos.unwrap();
                            if let Some(p) = self.secondary_cursor_pos {
                                self.events.push(crate::Event::MouseWheel {
                                    position: position.into(),
                                    modifiers: self.modifiers,
                                    handled: false,
                                    delta: (
                                        (position.x - p.x).abs() - (last_pos.x - p.x).abs(),
                                        (position.y - p.y).abs() - (last_pos.y - p.y).abs(),
                                    ),
                                });
                            } else {
                                self.events.push(crate::Event::MouseMotion {
                                    button: Some(MouseButton::Left),
                                    position: position.into(),
                                    modifiers: self.modifiers,
                                    handled: false,
                                    delta: (position.x - last_pos.x, position.y - last_pos.y),
                                });
                            }
                            self.cursor_pos = Some(position);
                        } else if self
                            .secondary_finger_id
                            .map(|id| id == touch.id)
                            .unwrap_or(false)
                        {
                            let last_pos = self.secondary_cursor_pos.unwrap();
                            if let Some(p) = self.cursor_pos {
                                self.events.push(crate::Event::MouseWheel {
                                    position: p.into(),
                                    modifiers: self.modifiers,
                                    handled: false,
                                    delta: (
                                        (position.x - p.x).abs() - (last_pos.x - p.x).abs(),
                                        (position.y - p.y).abs() - (last_pos.y - p.y).abs(),
                                    ),
                                });
                            }
                            self.secondary_cursor_pos = Some(position);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}

fn translate_virtual_key_code(key: winit::keyboard::KeyCode) -> Option<crate::Key> {
    use winit::keyboard::KeyCode;

    Some(match key {
        KeyCode::ArrowDown => Key::ArrowDown,
        KeyCode::ArrowLeft => Key::ArrowLeft,
        KeyCode::ArrowRight => Key::ArrowRight,
        KeyCode::ArrowUp => Key::ArrowUp,

        KeyCode::Escape => Key::Escape,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Space => Key::Space,

        KeyCode::Insert => Key::Insert,
        KeyCode::Delete => Key::Delete,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,

        KeyCode::Digit0 | KeyCode::Numpad0 => Key::Num0,
        KeyCode::Digit1 | KeyCode::Numpad1 => Key::Num1,
        KeyCode::Digit2 | KeyCode::Numpad2 => Key::Num2,
        KeyCode::Digit3 | KeyCode::Numpad3 => Key::Num3,
        KeyCode::Digit4 | KeyCode::Numpad4 => Key::Num4,
        KeyCode::Digit5 | KeyCode::Numpad5 => Key::Num5,
        KeyCode::Digit6 | KeyCode::Numpad6 => Key::Num6,
        KeyCode::Digit7 | KeyCode::Numpad7 => Key::Num7,
        KeyCode::Digit8 | KeyCode::Numpad8 => Key::Num8,
        KeyCode::Digit9 | KeyCode::Numpad9 => Key::Num9,

        KeyCode::KeyA => Key::A,
        KeyCode::KeyB => Key::B,
        KeyCode::KeyC => Key::C,
        KeyCode::KeyD => Key::D,
        KeyCode::KeyE => Key::E,
        KeyCode::KeyF => Key::F,
        KeyCode::KeyG => Key::G,
        KeyCode::KeyH => Key::H,
        KeyCode::KeyI => Key::I,
        KeyCode::KeyJ => Key::J,
        KeyCode::KeyK => Key::K,
        KeyCode::KeyL => Key::L,
        KeyCode::KeyM => Key::M,
        KeyCode::KeyN => Key::N,
        KeyCode::KeyO => Key::O,
        KeyCode::KeyP => Key::P,
        KeyCode::KeyQ => Key::Q,
        KeyCode::KeyR => Key::R,
        KeyCode::KeyS => Key::S,
        KeyCode::KeyT => Key::T,
        KeyCode::KeyU => Key::U,
        KeyCode::KeyV => Key::V,
        KeyCode::KeyW => Key::W,
        KeyCode::KeyX => Key::X,
        KeyCode::KeyY => Key::Y,
        KeyCode::KeyZ => Key::Z,

        _ => {
            return None;
        }
    })
}

///
/// A pixel coordinate in logical pixels, where `x` is on the horizontal axis with zero being at the left edge
/// and `y` is on the vertical axis with zero being at top edge.
///
#[derive(Debug, Copy, Clone, PartialEq)]
struct LogicalPoint {
    /// The horizontal pixel distance from the left edge.
    x: f32,
    /// The vertical pixel distance from the top edge.
    y: f32,
    device_pixel_ratio: f32,
    height: f32,
}

impl From<LogicalPoint> for PhysicalPoint {
    fn from(value: LogicalPoint) -> Self {
        Self {
            x: value.x * value.device_pixel_ratio,
            y: value.height - value.y * value.device_pixel_ratio,
        }
    }
}
