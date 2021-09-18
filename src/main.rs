use vulkano::{Version, device::physical::PhysicalDevice, instance::{ApplicationInfo, Instance, debug::{DebugCallback, MessageSeverity, MessageType}}};
use winit::{dpi::LogicalSize, event::{KeyboardInput, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_TITLE: &'static str = "Beyond demo";

const ENGINE_NAME: &'static str = "Beyond";

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64))
        .build(&event_loop);

    let app_info = ApplicationInfo {
        application_name: Some(WINDOW_TITLE.into()),
        application_version: Some(Version { major: 0, minor: 1, patch: 0 }),
        engine_name: Some(ENGINE_NAME.into()),
        engine_version: Some(Version { major: 0, minor: 1, patch: 0 }),
    };

    let mut required_extensions = vulkano_win::required_extensions();

    required_extensions.ext_debug_utils = true;

    let instance = Instance::new(Some(&app_info), Version::V1_2, &required_extensions, ["VK_LAYER_KHRONOS_validation"]).expect("Cannot create Vulkan instance");

    let debug_callback = DebugCallback::new(&instance,
MessageSeverity { error: true, warning: true, information: true, verbose: true },
        MessageType { general: true, validation: true, performance: true },
    |msg| println!("DEBUG: {}", msg.description)).expect("Cannot create debug callback");

    PhysicalDevice::enumerate(&instance).for_each(|device: PhysicalDevice| println!("{:?}", device));

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    },
                    winit::event::WindowEvent::KeyboardInput { input, .. } => {
                        match input {
                            KeyboardInput {virtual_keycode, state, ..} => {
                                match (virtual_keycode, state) {
                                    (Some(VirtualKeyCode::Escape), winit::event::ElementState::Pressed) => {
                                    },
                                    _ => {},
                                }
                            },
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    })
}
