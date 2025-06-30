use image::ImageReader;
use minifb::{Key, MouseMode, Window, WindowOptions};
use muliphain::input::mouse;
use std::io::Cursor;
use std::time::{Duration, Instant};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const IMG_BYTES: &[u8] = include_bytes!("../ba7823c2-95c2-44f4-96c0-e40734fa3cb2.jpg");

fn decode_image() -> Vec<u32> {
    let img = ImageReader::new(Cursor::new(IMG_BYTES))
        .with_guessed_format()
        .expect("Guess format failed")
        .decode()
        .expect("Decode failed")
        .resize_exact(
            WIDTH as u32,
            HEIGHT as u32,
            image::imageops::FilterType::Triangle,
        )
        .to_rgba8();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for (i, pixel) in img.pixels().enumerate() {
        let [r, g, b, a] = pixel.0;
        buffer[i] = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32);
    }
    buffer
}

fn main() {
    let mut muliphain = muliphain::initialize(WIDTH, HEIGHT, vec![]);

    let background_buffer = muliphain.add_window(0, 0, WIDTH as i32, HEIGHT as i32);
    let background = decode_image();
    let window_buffer =
        unsafe { std::slice::from_raw_parts_mut(background_buffer, WIDTH * HEIGHT) };
    window_buffer.copy_from_slice(&background);

    let windows = [
        (100, 100, 400, 300, 0xfff6e299),
        (200, 200, 400, 300, 0xff408deb),
        (300, 300, 400, 300, 0xff1d325a),
    ];

    for &(x, y, w, h, color) in &windows {
        let buffer_ptr = muliphain.add_window(x, y, w, h);
        let buffer = unsafe { std::slice::from_raw_parts_mut(buffer_ptr, (w * h) as usize) };
        for pixel in buffer.iter_mut() {
            *pixel = color;
        }
    }

    let mut window = Window::new(
        "Muliphain - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(60);

    let mut last_fps_check = Instant::now();
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (mouse_x, mouse_y) = window
            .get_mouse_pos(MouseMode::Discard)
            .unwrap_or((-100.0, -100.0));
        let mx = mouse_x.round() as i32;
        let my = mouse_y.round() as i32;

        let mouse = mouse::Mouse {
            x: mx,
            y: my,
            button: if window.get_mouse_down(minifb::MouseButton::Left) {
                1
            } else {
                0
            },
        };

        muliphain.handle_mouse(mouse.clone());
        muliphain.render(mouse.clone());
        window
            .update_with_buffer(
                unsafe { std::slice::from_raw_parts(muliphain.buffer, WIDTH * HEIGHT) },
                WIDTH,
                HEIGHT,
            )
            .unwrap();

        frame_count += 1;
        let now = Instant::now();
        if now.duration_since(last_fps_check) >= Duration::from_secs(1) {
            window.set_title(&format!("Muliphain - FPS: {}", frame_count));
            frame_count = 0;
            last_fps_check = now;
        }
    }
}
