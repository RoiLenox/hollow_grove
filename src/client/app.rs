//! Native window/event-loop ownership for the Phase II Hueman proof.

use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use super::input::direction_for_key;
use super::model::{
    ClientWorldState, LOGICAL_HEIGHT, LOGICAL_WIDTH, MovementState, WALK_POSE_DURATION,
};
use super::presentation::{PresentationError, ScenePresentation};
use super::render::{RenderError, SpriteRenderer};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClientOptions {
    pub exit_after_first_frame: bool,
}

pub fn run_client(options: ClientOptions) -> Result<(), ClientError> {
    let event_loop = EventLoop::new().map_err(|error| ClientError::EventLoop(error.to_string()))?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = NativeClientApp::new(options);
    event_loop
        .run_app(&mut app)
        .map_err(|error| ClientError::EventLoop(error.to_string()))?;
    app.failure.map_or(Ok(()), Err)
}

struct RunningClient {
    window: Arc<Window>,
    renderer: SpriteRenderer,
    state: ClientWorldState,
    movement_started: Option<Instant>,
    rendered_once: bool,
}

struct NativeClientApp {
    options: ClientOptions,
    running: Option<RunningClient>,
    failure: Option<ClientError>,
}

impl NativeClientApp {
    const fn new(options: ClientOptions) -> Self {
        Self {
            options,
            running: None,
            failure: None,
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, error: ClientError) {
        self.failure = Some(error);
        event_loop.exit();
    }
}

impl ApplicationHandler for NativeClientApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.running.is_some() || self.failure.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("Hollow Grove — Hueman Overworld Proof")
            .with_visible(false)
            .with_inner_size(LogicalSize::new(1280.0, 720.0))
            .with_min_inner_size(LogicalSize::new(
                f64::from(LOGICAL_WIDTH),
                f64::from(LOGICAL_HEIGHT),
            ));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                self.fail(event_loop, ClientError::WindowCreation(error.to_string()));
                return;
            }
        };
        let renderer = match pollster::block_on(SpriteRenderer::new(
            Arc::clone(&window),
            event_loop.owned_display_handle(),
        )) {
            Ok(renderer) => renderer,
            Err(error) => {
                self.fail(event_loop, ClientError::Renderer(error));
                return;
            }
        };
        window.set_visible(true);
        window.request_redraw();
        self.running = Some(RunningClient {
            window,
            renderer,
            state: ClientWorldState::new(),
            movement_started: None,
            rendered_once: false,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(running) = self.running.as_mut() else {
            return;
        };
        if running.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                running.renderer.resize(size);
                running.window.request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                running.renderer.resize(running.window.inner_size());
                running.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                let PhysicalKey::Code(code) = event.physical_key else {
                    return;
                };
                if code == KeyCode::Escape {
                    event_loop.exit();
                    return;
                }
                if let Some(direction) = direction_for_key(code) {
                    running.state.move_hueman(direction);
                    running.movement_started = (running.state.hueman.movement
                        == MovementState::Walking)
                        .then(Instant::now);
                    running.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let scene = match ScenePresentation::resolve(&running.state) {
                    Ok(scene) => scene,
                    Err(error) => {
                        self.fail(event_loop, ClientError::Presentation(error));
                        return;
                    }
                };
                match running.renderer.render(&scene) {
                    Ok(()) => {
                        if !running.rendered_once {
                            running.rendered_once = true;
                            if self.options.exit_after_first_frame {
                                println!("hollow-grove-client: rendered first frame");
                                event_loop.exit();
                            }
                        }
                    }
                    Err(error) if error.surface_is_recoverable() => {
                        running.renderer.resize(running.window.inner_size());
                        running.window.request_redraw();
                    }
                    Err(error) if error.surface_is_timeout() => {
                        eprintln!("hollow-grove-client: transient {error}");
                        running.window.request_redraw();
                    }
                    Err(error) => self.fail(event_loop, ClientError::Renderer(error)),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(running) = self.running.as_mut() else {
            return;
        };
        let Some(started) = running.movement_started else {
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        };
        let deadline = started + WALK_POSE_DURATION;
        if Instant::now() >= deadline {
            running.state.finish_hueman_step();
            running.movement_started = None;
            running.window.request_redraw();
            event_loop.set_control_flow(ControlFlow::Wait);
        } else {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
        }
    }
}

#[derive(Debug)]
pub enum ClientError {
    EventLoop(String),
    WindowCreation(String),
    Presentation(PresentationError),
    Renderer(RenderError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventLoop(error) => write!(formatter, "event loop failed: {error}"),
            Self::WindowCreation(error) => write!(formatter, "window creation failed: {error}"),
            Self::Presentation(error) => error.fmt(formatter),
            Self::Renderer(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ClientError {}
