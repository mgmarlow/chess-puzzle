use std::ffi::CString;

fn main() {
    let c_str = CString::new("Hello, World!").unwrap();

    unsafe {
        raylib::InitWindow(800, 600, c_str.as_ptr());

        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::EndDrawing();
        }

        raylib::CloseWindow();
    }
}
