use log;
use specs::prelude::*;
use ultraviolet::Vec3;

use winit::{ 
    dpi::LogicalSize,
    event::{ Event, ModifiersState, WindowEvent },
    event_loop::{ ControlFlow },
};

use harmony::WinitState;
use harmony::scene::Scene;
use harmony::scene::components::{CameraData, Mesh};

struct WindowSize {
    width: u32,
    height: u32,
}

const WINDOW_SIZE: WindowSize = WindowSize {
    width: 1024,
    height: 768,
};

struct AppState {
}

impl AppState {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl harmony::AppState for AppState {
    fn load(&mut self, app: &mut harmony::Application) {
        let mut scene = Scene::new(None, None);
        scene.world.create_entity().with(Mesh {
            mesh_name: "cube.gltf".into(),
        }).build();

        let actual_window_size = app.get_window_actual_size();

        // We can't render anything without a camera. Add one here.
        // Thankfully we have a method to help.
        let mut camera_data = CameraData::new_perspective(45.0, actual_window_size.width / actual_window_size.height, 0.0, 10.0);
        camera_data.update_view(
            Vec3::new(1.5, -5.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        harmony::scene::entities::camera::create(&mut scene.world, camera_data);

        // You can access the scene here once we store it.
        app.current_scene = Some(scene);
    }
    fn update(&mut self, _app: &mut harmony::Application) {
    }
    fn draw_gui(&mut self, _app: &mut harmony::Application) -> Option<&dyn harmony::gui::Scene> {
        None
    }
    fn draw(&mut self, _app: &mut harmony::Application) { }
}


fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("harmony", log::LevelFilter::Info)
        .init();

    let mut modifiers = ModifiersState::default();

    let (wb, event_loop) = WinitState::create("Harmony - Hello Cube", LogicalSize::new(WINDOW_SIZE.width, WINDOW_SIZE.height));

    let mut application = harmony::Application::new(wb, &event_loop);
    
    let mut app_state = AppState::new();

    application.load(&mut app_state);

    event_loop.run(move |event, _, control_flow| {
        application.run(&mut app_state, &event, control_flow);
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        modifiers = new_modifiers;
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = winit::event_loop::ControlFlow::Exit,
                    _ => {}
                }
            },
            _ => (),
        };
    });
}