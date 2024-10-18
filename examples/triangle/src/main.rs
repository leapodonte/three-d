// use log::warn;
use three_d::*;
// use winit::{
//     application::ApplicationHandler,
//     event::WindowEvent,
//     event_loop::{ActiveEventLoop, EventLoop},
//     window::{Window, WindowId},
// };

// struct Application {
//     initialized: bool,
//     window: Option<Window>,
// }

// impl Application {
//     fn new<T>() -> Self {
//         Self {
//             initialized: false,
//             window: None,
//         }
//     }
// }

// impl ApplicationHandler for Application {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         if self.initialized {
//             return;
//         }
//         self.initialized = true;

//         #[cfg(not(target_arch = "wasm32"))]
//         let mut window_builder = winit::window::Window::default_attributes()
//             .with_title("winit window")
//             // .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
//             .with_maximized(false);
//         #[cfg(target_arch = "wasm32")]
//         let mut window_builder = {
//             use wasm_bindgen::JsCast;
//             use winit::platform::web::WindowAttributesExtWebSys;
//             // use winit::platform::web::WindowExtWebSys;
//             let canvas = web_sys::window()
//                 .unwrap()
//                 .document()
//                 .unwrap()
//                 .get_elements_by_tag_name("canvas")
//                 .item(0)
//                 .unwrap()
//                 .dyn_into::<web_sys::HtmlCanvasElement>()
//                 .unwrap();
//             winit::window::Window::default_attributes()
//                 .with_canvas(Some(canvas))
//                 // .with_inner_size(winit::dpi::LogicalSize::new(500, 500))
//                 .with_prevent_default(false)
//         };
//         window_builder = window_builder.with_min_inner_size(winit::dpi::LogicalSize::new(10, 10));
//         // window_builder = window_builder.with_inner_size(winit::dpi::LogicalSize::new(500, 500));
//         let winit_window = event_loop.create_window(window_builder).unwrap();
//         // winit_window.focus_window();
//         // #[cfg(target_arch = "wasm32")]
//         // {
//         //     use winit::platform::web::WindowExtWebSys;
//         //     let canvas = &winit_window.canvas().unwrap();
//         //     web_sys::window()
//         //         .unwrap()
//         //         .document()
//         //         .unwrap()
//         //         .body()
//         //         .unwrap()
//         //         .append_child(canvas)
//         //         .unwrap();
//         // }
//         // let window =
//         //     Window::from_winit_window(winit_window, event_loop, SurfaceSettings::default(), false)
//         //         .unwrap();
//     }

//     fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         let window = &self.window.unwrap();
//         match event {
//             WindowEvent::Resized(size) => {
//                 // window.resize(size);
//             }
//             WindowEvent::Focused(focused) => {
//                 if focused {
//                     warn!("Window={window_id:?} focused");
//                 } else {
//                     warn!("Window={window_id:?} unfocused");
//                 }
//             }
//             WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
//                 warn!("Window={window_id:?} changed scale to {scale_factor}");
//             }
//             WindowEvent::ThemeChanged(theme) => {
//                 warn!("Theme changed to {theme:?}");
//             }
//             WindowEvent::RedrawRequested => {
//                 if let Err(err) = window.draw() {
//                     error!("Error drawing window: {err}");
//                 }
//             }
//             WindowEvent::Occluded(occluded) => {
//                 window.set_occluded(occluded);
//             }
//             WindowEvent::CloseRequested => {
//                 warn!("Closing Window={window_id:?}");
//                 self.windows.remove(&window_id);
//             }
//             WindowEvent::ModifiersChanged(modifiers) => {
//                 window.modifiers = modifiers.state();
//                 warn!("Modifiers changed to {:?}", window.modifiers);
//             }
//             WindowEvent::MouseWheel { delta, .. } => match delta {
//                 MouseScrollDelta::LineDelta(x, y) => {
//                     warn!("Mouse wheel Line Delta: ({x},{y})");
//                 }
//                 MouseScrollDelta::PixelDelta(px) => {
//                     warn!("Mouse wheel Pixel Delta: ({},{})", px.x, px.y);
//                 }
//             },
//             WindowEvent::KeyboardInput {
//                 event,
//                 is_synthetic: false,
//                 ..
//             } => {
//                 let mods = window.modifiers;

//                 // Dispatch actions only on press.
//                 if event.state.is_pressed() {
//                     let action = if let Key::Character(ch) = event.logical_key.as_ref() {
//                         Self::process_key_binding(&ch.to_uppercase(), &mods)
//                     } else {
//                         None
//                     };

//                     if let Some(action) = action {
//                         self.handle_action(event_loop, window_id, action);
//                     }
//                 }
//             }
//             WindowEvent::MouseInput { button, state, .. } => {
//                 let mods = window.modifiers;
//                 if let Some(action) = state
//                     .is_pressed()
//                     .then(|| Self::process_mouse_binding(button, &mods))
//                     .flatten()
//                 {
//                     self.handle_action(event_loop, window_id, action);
//                 }
//             }
//             WindowEvent::CursorLeft { .. } => {
//                 warn!("Cursor left Window={window_id:?}");
//                 window.cursor_left();
//             }
//             WindowEvent::CursorMoved { position, .. } => {
//                 warn!("Moved cursor to {position:?}");
//                 window.cursor_moved(position);
//             }
//             WindowEvent::ActivationTokenDone { token: _token, .. } => {
//                 #[cfg(any(x11_platform, wayland_platform))]
//                 {
//                     startup_notify::set_activation_token_env(_token);
//                     if let Err(err) = self.create_window(event_loop, None) {
//                         error!("Error creating new window: {err}");
//                     }
//                 }
//             }
//             WindowEvent::Ime(event) => match event {
//                 Ime::Enabled => warn!("IME enabled for Window={window_id:?}"),
//                 Ime::Preedit(text, caret_pos) => {
//                     warn!("Preedit: {}, with caret at {:?}", text, caret_pos);
//                 }
//                 Ime::Commit(text) => {
//                     warn!("Committed: {}", text);
//                 }
//                 Ime::Disabled => warn!("IME disabled for Window={window_id:?}"),
//             },
//             WindowEvent::PinchGesture { delta, .. } => {
//                 window.zoom += delta;
//                 let zoom = window.zoom;
//                 if delta > 0.0 {
//                     warn!("Zoomed in {delta:.5} (now: {zoom:.5})");
//                 } else {
//                     warn!("Zoomed out {delta:.5} (now: {zoom:.5})");
//                 }
//             }
//             WindowEvent::RotationGesture { delta, .. } => {
//                 window.rotated += delta;
//                 let rotated = window.rotated;
//                 if delta > 0.0 {
//                     warn!("Rotated counterclockwise {delta:.5} (now: {rotated:.5})");
//                 } else {
//                     warn!("Rotated clockwise {delta:.5} (now: {rotated:.5})");
//                 }
//             }
//             WindowEvent::PanGesture { delta, phase, .. } => {
//                 window.panned.x += delta.x;
//                 window.panned.y += delta.y;
//                 warn!("Panned ({delta:?})) (now: {:?}), {phase:?}", window.panned);
//             }
//             WindowEvent::DoubleTapGesture { .. } => {
//                 warn!("Smart zoom");
//             }
//             WindowEvent::TouchpadPressure { .. }
//             | WindowEvent::HoveredFileCancelled
//             | WindowEvent::KeyboardInput { .. }
//             | WindowEvent::CursorEntered { .. }
//             | WindowEvent::AxisMotion { .. }
//             | WindowEvent::DroppedFile(_)
//             | WindowEvent::HoveredFile(_)
//             | WindowEvent::Destroyed
//             | WindowEvent::Touch(_)
//             | WindowEvent::Moved(_) => (),
//         }
//     }
// }

pub fn main() {
    // let window = Window::new(WindowSettings {
    //     title: "Triangle!".to_string(),
    //     initial_size: Some((720, 720)),
    //     ..Default::default()
    // })
    // .unwrap();
    // let event_loop = EventLoop::new().unwrap();

    // let mut state = Application::new();
    // event_loop.run_app(&mut state).unwrap();

    // Create a window (a canvas on web)
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    let mut window_builder = winit::window::Window::default_attributes()
        .with_title("winit window")
        // .with_min_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_maximized(false);
    #[cfg(target_arch = "wasm32")]
    let mut window_builder = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;
        // use winit::platform::web::WindowExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_elements_by_tag_name("canvas")
            .item(0)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        winit::window::Window::default_attributes()
            .with_canvas(Some(canvas))
            // .with_inner_size(winit::dpi::LogicalSize::new(500, 500))
            .with_prevent_default(false)
    };
    window_builder = window_builder.with_min_inner_size(winit::dpi::LogicalSize::new(10, 10));
    // window_builder = window_builder.with_inner_size(winit::dpi::LogicalSize::new(500, 500));
    let winit_window = event_loop.create_window(window_builder).unwrap();
    // winit_window.focus_window();
    // #[cfg(target_arch = "wasm32")]
    // {
    //     use winit::platform::web::WindowExtWebSys;
    //     let canvas = &winit_window.canvas().unwrap();
    //     web_sys::window()
    //         .unwrap()
    //         .document()
    //         .unwrap()
    //         .body()
    //         .unwrap()
    //         .append_child(canvas)
    //         .unwrap();
    // }
    let window =
        Window::from_winit_window(winit_window, event_loop, SurfaceSettings::default(), false)
            .unwrap();
    // Get the graphics context from the window
    let context = window.gl();

    // Create a camera
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    // let mut camera = Camera::new_orthographic(
    //     window.viewport(),
    //     vec3(0.0, 0.0, 2.0),
    //     vec3(0.0, 0.0, 0.0),
    //     vec3(0.0, 1.0, 0.0),
    //     degrees(45.0),
    //     0.1,
    //     10.0,
    // );

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Srgba::RED,   // bottom right
        Srgba::GREEN, // bottom left
        Srgba::BLUE,  // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    // Add an animation to the triangle.
    // model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.002)));

    // let mut last_window_width = 0;
    // let mut last_window_height = 0;

    // let mut last_viewer_width = 0;
    // let mut last_viewer_height = 0;

    // let mut last_viewer_x = -50;
    // let mut last_viewer_y = -50;

    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // if frame_input.window_width != last_window_width || frame_input.window_height != last_window_height {
        //     log!("Window(w,h): {} {}", frame_input.window_width, frame_input.window_height);
        //     last_window_width = frame_input.window_width;
        //     last_window_height = frame_input.window_height;
        // }
        // if frame_input.viewport.width != last_viewer_width || frame_input.viewport.height != last_viewer_height {
        //     log!("viewer(w,h): {} {}", frame_input.viewport.width, frame_input.viewport.height);
        //     last_viewer_width = frame_input.viewport.width;
        //     last_viewer_height = frame_input.viewport.height;
        // }

        // if frame_input.viewport.x != last_viewer_x || frame_input.viewport.y != last_viewer_y {
        //     log!("viewer(x,y): {} {}", frame_input.viewport.x, frame_input.viewport.y);
        //     last_viewer_x = frame_input.viewport.x;
        //     last_viewer_y = frame_input.viewport.y;
        // }

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Update the animation of the triangle
        // model.animate(frame_input.accumulated_time as f32);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.2, 0.2, 0.2, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model, &[]
            );

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}
