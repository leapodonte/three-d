use three_d::*;

pub fn main() {
    // let window = Window::new(WindowSettings {
    //     title: "Triangle!".to_string(),
    //     initial_size: Some((720, 720)),
    //     ..Default::default()
    // })
    // .unwrap();

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
        winit::window::Window::default_attributes()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("canvas")
                    .item(0)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ))
            // .with_inner_size(winit::dpi::LogicalSize::new(500, 500))
            .with_prevent_default(false)
    };
    window_builder = window_builder.with_min_inner_size(winit::dpi::LogicalSize::new(100, 100));
    #[allow(deprecated)]
    let winit_window = event_loop.create_window(window_builder).unwrap();
    winit_window.focus_window();
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
