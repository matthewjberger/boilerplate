use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub trait Application {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, _time: f32) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_mouse(&mut self, _button: &MouseButton, _button_state: &ElementState) -> Result<()> {
        Ok(())
    }

    fn on_key(&mut self, _keycode: &VirtualKeyCode, _keystate: &ElementState) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, _event: Event<()>, _window: &Window) -> Result<()> {
        Ok(())
    }

    fn on_resize(&mut self, _width: u32, _height: u32) -> Result<()> {
        Ok(())
    }
}

pub fn run(mut application: impl Application + 'static, title: &str) -> Result<()> {
    env_logger::init();
    log::info!("App started");

    let (width, height) = (800.0, 600.0);

    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height))
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        if let Err(error) = run_loop(&mut application, &mut window, &event, control_flow) {
            log::error!("Application error: {}", error);
        }
    });
}

fn run_loop(
    application: &mut impl Application,
    window: &mut Window,
    event: &Event<()>,
    control_flow: &mut ControlFlow,
) -> Result<()> {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if *window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => {
                if let (Some(VirtualKeyCode::Escape), ElementState::Pressed) =
                    (input.virtual_keycode, input.state)
                {
                    *control_flow = ControlFlow::Exit;
                }

                if let Some(keycode) = input.virtual_keycode.as_ref() {
                    application.on_key(keycode, &input.state)?;
                }
            }
            WindowEvent::MouseInput { button, state, .. } => application.on_mouse(button, state)?,
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
