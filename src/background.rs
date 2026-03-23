#![allow(dead_code)]

use crate::palette::*;
use std::cell::Cell;
use std::rc::Rc;
use three_d::*;
use wasm_bindgen::*;
use wasm_bindgen_futures::spawn_local;

pub fn init_background(theme: &Theme) {
    // Config
    let canvas_id = "Three-d-bg".to_string();

    // Fetch colors from Theme
    let colors = BgColors {
        background: theme.active_palette().surface.muted.to_srgba(255),
        object: theme.active_palette().brand.base.to_srgba(127),
        light: theme.active_palette().neutral.prominent.to_srgba(255),
    };

    // Fetch mouse data
    let mouse = setup_mouse(&canvas_id);

    // Assync 3D Context
    spawn_local(async move {
        // Window and Context
        let window = Window::new(WindowSettings {
            title: canvas_id,
            ..Default::default()
        })
        .unwrap();
        let context = window.gl();

        // Camera
        let h = 1.0;
        let mut camera = setup_camera(&window, 7.5, h);

        // Load 3D Model
        let cpu_mesh = load_mesh("palette.glb").await;

        // Instances and mesh
        let grid_parameters = get_grid_parameters(&window, h, 50);
        let mut instances = init_instances(&grid_parameters);
        let mut mesh = setup_instanced_mesh(&context, cpu_mesh, &instances, colors.object);

        // Light
        let light = AmbientLight::new(&context, 0.5, colors.light);

        // Render Loop
        window.render_loop(move |frame_input| {
            camera.set_viewport(frame_input.viewport);

            // Mouse animation logic updates
            mouse_update_instance(&mouse, &mut instances, &grid_parameters);

            mesh.geometry.set_instances(&instances);

            // Render the frame into window
            render_frame(&camera, &mesh, &light, &colors.background, frame_input)
        });
    });
}

struct BgColors {
    background: Srgba,
    object: Srgba,
    light: Srgba,
}

pub type MouseState = (Rc<Cell<f32>>, Rc<Cell<f32>>);

fn setup_mouse(canvas_id: &str) -> MouseState {
    // Create mouse x and y, which will be returned
    let mouse_x = Rc::new(Cell::new(0.0f32));
    let mouse_y = Rc::new(Cell::new(0.0f32));

    // Create variables shared by closure - Rc allows shared ownership
    let closure_mouse_x = mouse_x.clone();
    let closure_mouse_y = mouse_y.clone();

    // Duplicate canvas_id to be taken ownership by closure
    let canvas_id = canvas_id.to_string();

    // Init a closure
    let closure =
        wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let win = web_sys::window().unwrap();
            let doc = win.document().unwrap();

            let canvas = doc
                .get_element_by_id(&canvas_id)
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();

            let rect = canvas.get_bounding_client_rect();
            let w = rect.width() as f32;
            let h = rect.height() as f32;

            // Normalizes to [-1.0, 1.0]
            closure_mouse_x.set(((event.client_x() as f64 - rect.left()) as f32 / w) * 2.0 - 1.0);
            closure_mouse_y.set(((event.client_y() as f64 - rect.top()) as f32 / h) * 2.0 - 1.0);
        });

    // Set up a listener for mouse movement
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();

    // Keep the closure alive even after the end of scope
    closure.forget();

    // Returns mouse_x and mouse_y
    (mouse_x, mouse_y)
}

fn setup_camera(window: &Window, z: f32, h: f32) -> Camera {
    Camera::new_orthographic(
        window.viewport(),
        vec3(0.0, 0.0, z),   // camera position
        vec3(0.0, 0.0, 0.0), // look at origin
        vec3(0.0, 1.0, 0.0), // up direction
        h,                   // frustum height
        1.0,                 // near plane
        10.0,                // far plane
    )
}

async fn load_mesh(path: &str) -> CpuMesh {
    // three_d_asset requires a full absolute URL
    let href = web_sys::window().unwrap().location().origin().unwrap();

    // Ensure trailing slash before appending
    let base = if href.ends_with('/') {
        href.clone()
    } else {
        format!("{}/", href)
    };
    let url = format!("{}public/{}", base, path);

    // Load the raw asset bytes
    let mut loaded = three_d_asset::io::load_async(&[url.as_str()])
        .await
        .unwrap();

    // Desserialize the asset into a CpuModel
    let cpu_model: CpuModel = loaded.deserialize(path).unwrap();

    // Extract the triangle mesh geometry
    match &cpu_model.geometries.first().unwrap().geometry {
        three_d_asset::Geometry::Triangles(mesh) => mesh.clone(),
        _ => panic!("Not triangles"),
    }
}

pub struct GridParameters {
    pub rows: usize,
    pub cols: usize,
    pub half_w: f32,
    pub half_h: f32,
    pub spacing: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}
fn get_grid_parameters(window: &Window, fh: f32, n: usize) -> GridParameters {
    let w = window.viewport().width as f32;
    let h = window.viewport().height as f32;
    let aspect = w / h;

    let half_h = fh * 3.75;
    let half_w = half_h * aspect;
    let spacing = half_w * 2.0 / n as f32;

    let cols = ((half_w * 2.0 / spacing).ceil() as usize + 2).max(1);
    let rows = ((half_h * 2.0 / spacing).ceil() as usize + 2).max(1);

    GridParameters {
        rows,
        cols,
        spacing,
        half_w,
        half_h,
        offset_x: (cols as f32) * spacing / 2.0,
        offset_y: (rows as f32) * spacing / 2.0,
    }
}

fn init_instances(grid_parameters: &GridParameters) -> Instances {
    let mut instances = Instances::default();
    instances.transformations = (0..grid_parameters.rows)
        .flat_map(|row| {
            (0..grid_parameters.cols).map(move |col| {
                Mat4::from_translation(vec3(
                    col as f32 * grid_parameters.spacing - grid_parameters.offset_x,
                    row as f32 * grid_parameters.spacing - grid_parameters.offset_y,
                    0.0,
                )) * Mat4::from_angle_x(degrees(90.0))
            })
        })
        .collect();
    instances
}

fn setup_instanced_mesh(
    context: &Context,
    cpu_mesh: CpuMesh,
    instances: &Instances,
    color: Srgba,
) -> Gm<InstancedMesh, ColorMaterial> {
    Gm::new(
        InstancedMesh::new(&context, &instances, &cpu_mesh),
        ColorMaterial {
            color,
            ..Default::default()
        },
    )
}

fn mouse_update_instance(
    mouse: &MouseState,
    instances: &mut Instances,
    grid_parameters: &GridParameters,
) {
    let mouse_world_x = mouse.0.get() * grid_parameters.half_w;
    let mouse_world_y = -mouse.1.get() * grid_parameters.half_h;

    instances.transformations = (0..grid_parameters.rows)
        .flat_map(|row| {
            (0..grid_parameters.cols).map(move |col| {
                let x = col as f32 * grid_parameters.spacing - grid_parameters.offset_x;
                let y = row as f32 * grid_parameters.spacing - grid_parameters.offset_y;
                let dx = mouse_world_x - x;
                let dy = mouse_world_y - y;
                let dist = (dx * dx + dy * dy).sqrt();
                let influence = (1.0 / (1.0 + dist * 0.5)).clamp(0.0, 1.0);
                let angle_y = dx * influence * 0.5;
                let angle_x = dy * influence * 0.5;

                Mat4::from_translation(vec3(x, y, 0.0))
                    * Mat4::from_angle_y(radians(angle_y))
                    * Mat4::from_angle_x(radians(angle_x))
                    * Mat4::from_angle_x(degrees(90.0))
            })
        })
        .collect();
}

fn render_frame(
    camera: &Camera,
    mesh: &Gm<InstancedMesh, ColorMaterial>,
    light: &AmbientLight,
    color: &Srgba,
    frame_input: FrameInput,
) -> FrameOutput {
    frame_input
        .screen()
        .clear(ClearState::color_and_depth(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
            1.0,
        ))
        .render(&camera, [&mesh], &[&light]);
    FrameOutput::default()
}
