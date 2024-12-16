use i_slint_backend_winit::winit::dpi::PhysicalPosition;
use i_slint_backend_winit::winit::monitor::MonitorHandle;
use i_slint_backend_winit::winit::window::Window;
use i_slint_backend_winit::WinitWindowAccessor;

pub fn center_window(window: &slint::Window) {

    if window.has_winit_window() {

        window.with_winit_window(|window: &Window| {

            match window.current_monitor() {
                Some(monitor) => set_centered(window, &monitor),
                None => (),
            };

            None as Option<()>
        });
    }
}

fn set_centered(window: &Window, monitor: &MonitorHandle) {

    let window_size = window.outer_size();

    let monitor_size = monitor.size();
    let monitor_position = monitor.position();

    let mut monitor_window_position = PhysicalPosition { x: 0, y: 0 };

    monitor_window_position.x =
        (monitor_position.x as f32 + (monitor_size.width as f32 * 0.5) - (window_size.width as f32 * 0.5)) as i32;

    monitor_window_position.y = monitor_position.y;

    window.set_outer_position(monitor_window_position);
}

pub fn get_window(window: &slint::Window, pixels: i32) {
    if window.has_winit_window() {

        window.with_winit_window(|window: &Window| {

            match window.current_monitor() {
                Some(monitor) => animate_window(window, &monitor, pixels),
                None => (),
            };

            None as Option<()>
        });
    }
}

fn animate_window(window: &Window, monitor: &MonitorHandle, pixels: i32) {
    let monitor_position = monitor.position();
    let mut monitor_window_position = PhysicalPosition {
        x: monitor_position.x,
        y: monitor_position.y,
    };

    let window_size = window.outer_size();

    let monitor_size = monitor.size();

    monitor_window_position.x =
    (monitor_position.x as f32 + (monitor_size.width as f32 * 0.5) - (window_size.width as f32 * 0.5)) as i32;
    monitor_window_position.y = monitor_window_position.y + (pixels * 30);
    window.set_outer_position(monitor_window_position);
}