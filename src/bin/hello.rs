use anyhow::Result;
use support::{run, Application};

#[derive(Default)]
struct DemoApp;

impl DemoApp {
    fn say_hi(&mut self) {
        println!("Howdy");
    }
}

impl Application for DemoApp {
    fn initialize(&mut self, _window: &winit::window::Window) -> Result<()> {
        println!("Just initialized!");
        self.say_hi();
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run(app, "Hello!")
}
