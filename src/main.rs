#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
mod winit_helper;
use winit_helper::center_window;
use winit_helper::get_window;

slint::include_modules!();
use slint::{Timer, TimerMode};


fn main() -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(async_main())
}

fn change_window_display(ui: &AppWindow, b_hide: bool) -> Result<(), Box<dyn std::error::Error>> {
    let direction = if b_hide { -1 } else { 1 };
    let ui_handle = ui.as_weak();
    for pixels in 1..10 {
        std::thread::sleep(std::time::Duration::from_millis(16));
        get_window(ui.window(), pixels * direction);
    }
    std::thread::sleep(std::time::Duration::from_secs(1));

    get_window(ui.window(), 0);
    if b_hide {
        ui.set_onTop(false);
        ui_handle.unwrap().window().set_minimized(true);
        ui_handle.unwrap().window().set_maximized(false);
    } else {
        ui.set_onTop(true);
        ui_handle.unwrap().window().set_minimized(false);
        ui_handle.unwrap().window().set_maximized(true);
    }
    Ok(())
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.show()?;

    center_window(ui.window());
    let b_break_done = Arc::new(Mutex::new(false));
    let b_take_break = Arc::new(Mutex::new(true));

     ui.on_request_reset_time_left({
        let ui_handle = ui_handle.clone();
        let b_break_done = b_break_done.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                let mut b_break_done = b_break_done.lock().unwrap();
                ui.set_timeLeft(60);
                println!("Reset timer requested");
                ui.set_titleText("Time till next break:".into());
                let bars = '▓'.to_string().repeat(60);
                ui.set_bars(bars.into());
                *b_break_done = false;
                ui.set_showTimeLeft(true.into());
                let _ = change_window_display(&ui, true);

            }
        }
    }); 

    ui.on_request_exit_program(|| {
        println!("Exit program requested");
        std::process::exit(0); 
    });

    ui.on_request_take_break({
        let b_take_break = b_take_break.clone();
        let ui_handle = ui_handle.clone();

        move || {
            if let Some(ui) = ui_handle.upgrade() {
                let mut b_take_break = b_take_break.lock().unwrap();
                *b_take_break = !*b_take_break;
                ui.set_bBreak(*b_take_break);
                let _ = change_window_display(&ui, true);
                println!("Taking a break{}", &b_take_break);
            }
        }
    });

     ui.on_request_remember_me_later({
        let ui_handle = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                if ui.get_timeLeft() < 5 {
                    ui_handle.window().unwrap().set_minimized();
                } else {
                    ui.set_timeLeft(ui.get_timeLeft() + 10);
                    let length = ui.get_timeLeft();
                    println!("lenght{} ", length);
    
                    let mut bars = '▓'.to_string().repeat(length.try_into().unwrap());
                    bars = bars.chars().take(60).collect();
                    ui.set_bars(bars.into());
                    ui.set_onTop(false);
                    let _ = change_window_display(&ui, true);
                }
            }
        }
    }); 

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1000 * 60), move || {
        let ui_handle = ui_handle.clone();
        let b_break_done = b_break_done.clone();
        let b_take_break = b_take_break.clone();
        (move || {
            if let Some(ui) = ui_handle.upgrade() {
                let mut b_break_done = b_break_done.lock().unwrap();
                let b_take_break = b_take_break.lock().unwrap();
                println!("time left{} ", ui.get_timeLeft());

                if ui.get_timeLeft() == 0 && !*b_break_done {
                    ui.set_timeLeft(3); 
                    ui.set_titleText("Break left:".into());

                    let bars = '▓'.to_string().repeat(10); 
                    ui.set_bars(bars.into());
                    *b_break_done = true;
                }

                if ui.get_timeLeft() - 1 == 1 {
                    ui.set_onTop(true);
                    let ui_handle = ui_handle.clone();
                    let _ = change_window_display(&ui, false);

                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.set_onTop(false);
                        }
                    });
                    let _ = change_window_display(&ui, true);
                }
                if *b_break_done && !*b_take_break {
                    let time_left = ui.get_timeLeft() - 1;
                    
                    if time_left == 0 {
                        ui.set_titleText("Time to get back to work!".into());
                        ui.set_showTimeLeft(false.into());
                    } else {
                        ui.set_timeLeft(time_left);
                        if ui.get_timeLeft() < 60 {
                            let bars_length = (time_left as usize).min(100);

                            let bars = '▓'.to_string().repeat(bars_length);
                            ui.set_bars(bars.into());
                        }
                    }
                    } else {
                    if !*b_take_break {
                        let time_left = ui.get_timeLeft() - 1;
                        ui.set_timeLeft(time_left);
                        if ui.get_timeLeft() < 60 {
                            let bars_length = (time_left as usize).min(100);

                            let bars = '▓'.to_string().repeat(bars_length);
                            ui.set_bars(bars.into());   
                        } 
                    }
                }
            }
        })()
    });

    ui.run()?;
    Ok(())
}
