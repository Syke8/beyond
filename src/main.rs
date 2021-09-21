use vulkano::{Version, device::{Device, DeviceExtensions, Features, physical::PhysicalDevice}, instance::{ApplicationInfo, Instance, debug::{DebugCallback, MessageSeverity, MessageType}}};
use vulkano_win::VkSurfaceBuild;
use winit::{dpi::LogicalSize, event::{KeyboardInput, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

const APPLICATION_NAME: &'static str = "Beyond demo";
const APPLICATION_VERSION: Version = Version { major: 0, minor: 1, patch: 0 };

const ENGINE_NAME: &'static str = "Beyond";
const ENGINE_VERSION: Version = Version { major: 0, minor: 1, patch: 0 };

// TODO: Starting to be ugly fat code, let's refactor that properly from the  bottom before continuing learning
fn main() {
    let event_loop = EventLoop::new();

    let app_info = ApplicationInfo {
        application_name: Some(APPLICATION_NAME.into()),
        application_version: Some(APPLICATION_VERSION),
        engine_name: Some(ENGINE_NAME.into()),
        engine_version: Some(ENGINE_VERSION),
    };

    let mut required_extensions = vulkano_win::required_extensions();

    required_extensions.ext_debug_utils = true;

    let instance = Instance::new(Some(&app_info), Version::V1_2, &required_extensions, ["VK_LAYER_KHRONOS_validation"]).expect("Cannot create Vulkan instance");

    let debug_callback = DebugCallback::new(&instance,
MessageSeverity { error: true, warning: true, information: true, verbose: true },
        MessageType { general: true, validation: true, performance: true },
    |msg| println!("DEBUG: {}", msg.description)).expect("Cannot create debug callback");

    let surface = WindowBuilder::new()
        .with_title(APPLICATION_NAME)
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64))
        .build_vk_surface(&event_loop, instance.to_owned())
        .expect("Cannot create Vulkan surface");

    let mut graphics_queue_index = 0;

    let gpu = PhysicalDevice::enumerate(&instance).find(|device| {
        for (idx, queue) in device.queue_families().enumerate() {
            if queue.supports_graphics() && surface.is_supported(queue).expect("Surface doesn't support graphics queue") {
                graphics_queue_index = idx;
                return true
            }
        }

        false
    }).expect("No available GPU");

    println!("SELECTED : {} - {:?} - {} v{}", gpu.properties().device_name, gpu.properties().device_type, gpu.properties().driver_name.as_ref().unwrap(), gpu.properties().driver_info.as_ref().unwrap());

    let queue_family = gpu.queue_families().nth(graphics_queue_index).unwrap();

    let (device, mut queues) = Device::new(gpu, &Features::none(), &DeviceExtensions::none(), [(queue_family, 1.0), (queue_family, 1.0)]).expect("Cannot create Device");
    let graphics_queue = queues.next().unwrap();
    let present_queue = queues.next().unwrap();

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
