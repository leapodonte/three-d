use three_d::*;
use three_d::{renderer::*, FrameInputGenerator, SurfaceSettings, WindowedContext};

pub fn main() {
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
            // .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_prevent_default(true)
    };
    window_builder = window_builder.with_min_inner_size(winit::dpi::LogicalSize::new(100, 100));
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
    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    // Add an animation to the triangle.
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Update the animation of the triangle
        model.animate(frame_input.accumulated_time as f32);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, &model, &[]
            );

        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}
