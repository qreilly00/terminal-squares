use winit::{
    event_loop::{
        ControlFlow,
        EventLoop,
    },
};

fn main() {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = application::Application::new();

    event_loop.run_app(&mut app).unwrap();

    /*let a = 2;
    let b = 1;

    let test = (a, b);

    println!("{}", (test.0 + test.1).to_string());*/

    // Consider creating a custom hashmap. Make each func class return its closues along with its name as a string?
}

pub mod square {
    use pixels::{
        Pixels,
    };

    pub struct Square {
        x: f32,
        y: f32,
        z: f32,

        width: f32,
        height: f32,
        bredth: f32,

        angle: f32,
    }

    impl Square {
        pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, bredth: f32) -> Self {
            Self {
                x,
                y,
                z,

                width,
                height,
                bredth,

                angle: 0.0,
            }
        }

        pub fn draw(&mut self, pixels: &mut Pixels, frame_width: i32) {

            let mut points: Vec<Vec<f32>> = vec![
                vec![
                    self.x,
                    self.y,
                    self.z,
                ],
                vec![
                    self.x + self.width,
                    self.y,
                    self.z,
                ],
                vec![
                    self.x,
                    self.y + self.height,
                    self.z,
                ],
                vec![
                    self.x + self.width,
                    self.y + self.height,
                    self.z,
                ],
                // Back part
                vec![
                    self.x,
                    self.y,
                    self.z + self.bredth,
                ],
                vec![
                    self.x + self.width,
                    self.y,
                    self.z + self.bredth,
                ],
                vec![
                    self.x,
                    self.y + self.height,
                    self.z + self.bredth,
                ],
                vec![
                    self.x + self.width,
                    self.y + self.height,
                    self.z + self.bredth,
                ],
            ];

            let item_offset = (self.x + (self.width / 2.0), (self.y + (self.height / 2.0)), self.z + (self.bredth / 2.0));


            let xy_rotation: Vec<Vec<f32>> = vec![
                vec![self.angle.cos(),  -self.angle.sin(),  0.0],
                vec![self.angle.sin(),  self.angle.cos(),   0.0],
                vec![0.0,               0.0,                1.0],
            ];

            let xz_rotation: Vec<Vec<f32>> = vec![
                vec![self.angle.cos(),      0.0,    self.angle.sin()],
                vec![0.0,                   1.0,                 0.0],
                vec![-self.angle.sin(),     0.0,    self.angle.cos()],

            ];

            let rotate = |rot: &Vec<Vec<f32>>, points: &Vec<Vec<f32>>| -> Vec<Vec<f32>> {

                let rotated_points_1: Vec<Vec<f32>> = vec![
                    vec![
                        (((rot[0][0] * (points[0][0] - item_offset.0)) + (rot[0][1] * (points[0][1] - item_offset.1)) + (rot[0][2] * (points[0][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[0][0] - item_offset.0)) + (rot[1][1] * (points[0][1] - item_offset.1)) + (rot[1][2] * (points[0][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[0][0] - item_offset.0)) + (rot[2][1] * (points[0][1] - item_offset.1)) + (rot[2][2] * (points[0][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[1][0] + self.width - item_offset.0)) + (rot[0][1] * (points[1][1] - item_offset.1)) + (rot[0][2] * (points[1][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[1][0] + self.width - item_offset.0)) + (rot[1][1] * (points[1][1] - item_offset.1)) + (rot[1][2] * (points[1][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[1][0] + self.width - item_offset.0)) + (rot[2][1] * (points[1][1] - item_offset.1)) + (rot[2][2] * (points[1][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[2][0] - item_offset.0)) + (rot[0][1] * (points[2][1] + self.height - item_offset.1)) + (rot[0][2] * (points[2][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[2][0] - item_offset.0)) + (rot[1][1] * (points[2][1] + self.height - item_offset.1)) + (rot[1][2] * (points[2][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[2][0] - item_offset.0)) + (rot[2][1] * (points[2][1] + self.height - item_offset.1)) + (rot[2][2] * (points[2][2] - item_offset.2)))+ self.z),
                    ],
                    // Back part
                    vec![
                        (((rot[0][0] * (points[3][0] + self.width - item_offset.0)) + (rot[0][1] * (points[3][1] + self.height - item_offset.1)) + (rot[0][2] * (points[3][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[3][0] + self.width - item_offset.0)) + (rot[1][1] * (points[3][1] + self.height - item_offset.1)) + (rot[1][2] * (points[3][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[3][0] + self.width - item_offset.0)) + (rot[2][1] * (points[3][1] + self.height - item_offset.1)) + (rot[2][2] * (points[3][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[4][0] - item_offset.0)) + (rot[0][1] * (points[4][1] - item_offset.1)) + (rot[0][2] * (points[4][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[4][0] - item_offset.0)) + (rot[1][1] * (points[4][1] - item_offset.1)) + (rot[1][2] * (points[4][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[4][0] - item_offset.0)) + (rot[2][1] * (points[4][1] - item_offset.1)) + (rot[2][2] * (points[4][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[5][0] + self.width - item_offset.0)) + (rot[0][1] * (points[5][1] - item_offset.1)) + (rot[0][2] * (points[5][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[5][0] + self.width - item_offset.0)) + (rot[1][1] * (points[5][1] - item_offset.1)) + (rot[1][2] * (points[5][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[5][0] + self.width - item_offset.0)) + (rot[2][1] * (points[5][1] - item_offset.1)) + (rot[2][2] * (points[5][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[6][0] - item_offset.0)) + (rot[0][1] * (points[6][1] + self.height - item_offset.1)) + (rot[0][2] * (points[6][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[6][0] - item_offset.0)) + (rot[1][1] * (points[6][1] + self.height - item_offset.1)) + (rot[1][2] * (points[6][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[6][0] - item_offset.0)) + (rot[2][1] * (points[6][1] + self.height - item_offset.1)) + (rot[2][2] * (points[6][2] - item_offset.2)))+ self.z),
                    ],

                    vec![
                        (((rot[0][0] * (points[7][0] + self.width - item_offset.0)) + (rot[0][1] * (points[7][1] + self.height - item_offset.1)) + (rot[0][2] * (points[7][2] - item_offset.2))) + self.x),
                        (((rot[1][0] * (points[7][0] + self.width - item_offset.0)) + (rot[1][1] * (points[7][1] + self.height - item_offset.1)) + (rot[1][2] * (points[7][2] - item_offset.2))) + self.y),
                        (((rot[2][0] * (points[7][0] + self.width - item_offset.0)) + (rot[2][1] * (points[7][1] + self.height - item_offset.1)) + (rot[2][2] * (points[7][2] - item_offset.2)))+ self.z),
                    ],
                ];

                rotated_points_1
            };

            points = rotate(&xy_rotation, &points);
            points = rotate(&xz_rotation, &points);

            for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                let x = (i % frame_width as usize) as i32;
                let y = (i / frame_width as usize) as i32;

                if x == points[0][0] as i32 && y == points[0][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[1][0] as i32 && y == points[1][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[2][0] as i32 && y == points[2][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[3][0] as i32 && y == points[3][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[4][0] as i32 && y == points[4][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[5][0] as i32 && y == points[5][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[6][0] as i32 && y == points[6][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
                else if x == points[7][0] as i32 && y == points[7][1] as i32 {
                    pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }

            }

            if self.angle - 0.5 < 0.0 {
                self.angle = 6.283;
            } else {
                self.angle -= 0.5;
            }
        }

        pub fn get_x(self) -> f32 { self.x }
        pub fn get_y(self) -> f32 { self.y }
        pub fn get_z(self) -> f32 { self.z }

        pub fn get_width(self) -> f32 { self.width }
        pub fn get_height(self) -> f32 { self.height }
        pub fn get_bredth(self) -> f32 { self.bredth }

        pub fn set_x(mut self, x: f32) { self.x = x; }
        pub fn set_y(mut self, y: f32) { self.y = y; }
        pub fn set_z(mut self, z: f32) { self.z = z; }

        pub fn set_width(mut self, width: f32) { self.width = width; }
        pub fn set_height(mut self, height: f32) { self.height = height; }
        pub fn set_bredth(mut self, bredth: f32) { self.bredth = bredth; }
    }
}

pub mod application {
    use super::window_manager::WindowManager;
    use super::square::Square;

    use pixels::{
        Pixels,
        SurfaceTexture,
        wgpu::Color,
    };

    use winit::{
        application::ApplicationHandler,
        dpi::LogicalSize,
        window::{
            Window,
            WindowId,
        },
        event::{
            WindowEvent,
        },
        event_loop::{
            ActiveEventLoop,
        },
    };

    pub struct Application {
        window_width: f32,
        window_height: f32,

        windows: Vec<WindowManager>,
        squares: Vec<Square>,
    }

    impl Application {
        pub fn new() -> Self {
            let mut tmp: Vec<Square> = Vec::new();
            tmp.push(Square::new(250.0, 250.0, 250.0, 32.0, 32.0, 32.0));

            Self {
                window_width:   1000.0,
                window_height:  1000.0,

                windows: Vec::new(),

                squares: tmp,
            }
        }

        fn create_window(&mut self, event_loop: &ActiveEventLoop) {
            let window = {
                let size = LogicalSize::new(self.window_width as f64, self.window_height as f64);

                let window_attributes = Window::default_attributes()
                    .with_title("Hello Pixels")
                    .with_inner_size(size)
                    .with_min_inner_size(size);

                event_loop.create_window(window_attributes).unwrap()
            };

            let pixels = {
                let window_size = window.inner_size();

                let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

                Pixels::new(self.window_width as u32, self.window_height as u32, surface_texture).unwrap()
            };

            // Try and remember to ask someone what to do when the window closes and resume is called for a new one. Doesn't seem to be any check closed function.
            self.windows.push(WindowManager::new(window, pixels));
        }
    }

    impl ApplicationHandler for Application {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            // This will need to be revisited after I find the answer to the closing windows thing.
            if self.windows.len() == 0 {
                self.create_window(&event_loop);
            }
        }

        fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
            // /self.windows[0].request_redraw();
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                },
                WindowEvent::RedrawRequested => {
                    if self.windows.len() > 0 { // Revisit this.
                        /*let win_len = self.windows.len() - 1 as usize;
                        self.windows[win_len].clear();*/
                        //println!("Test");

                        self.windows[0].clear();

                        self.squares[0].draw(self.windows[0].get_pixels(), self.window_width as i32);

                        self.windows[0].render();
                    }
                },
                _ => (),
            }
        }

        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            self.windows[0].get_window().request_redraw();
        }
    }
}

pub mod window_manager {
    use pixels::{
        Pixels,
    };

    use winit::{
        window::{
            Window,
            WindowId,
        },
    };

    pub struct WindowManager {
        window: Window,
        pixels: Pixels,
    }

    impl WindowManager {
        pub fn new(window: Window, pixels: Pixels) -> Self {
            Self{
                window,
                pixels,
            }
        }

        pub fn clear(&mut self) {
            for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                pixel.copy_from_slice(&[0000, 0000, 0000, 0000]);
            }

        }

        pub fn render(&mut self) {
            self.pixels.render();
        }

        pub fn get_pixels(&mut self) -> &mut Pixels { &mut self.pixels }

        pub fn get_window(&self) -> &Window { &self.window } // Another area that needs revisited.

        //pub fn get_window_id(&self) -> WindowId { self.window.id() }
    }
}
