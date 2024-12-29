use fltk::{
    app,
    button::Button,
    enums::{Event, Color},
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    rc::Rc,
    cell::RefCell,
    sync::{Arc, Mutex},
};

use ctrlc;

mod settings;
mod batch_template;
mod commands;

use settings::{load_settings, save_settings};
use batch_template::FREE_REGION_BAT;
use commands::spawn_single_command;

fn main() {
    let settings_path = PathBuf::from("settings.txt");
    let init_s = load_settings(&settings_path);
    let shared_settings = Arc::new(Mutex::new(init_s));

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 400, 400, "Single Send example");

    let mut frame = Frame::new(20, 10, 560, 30, "RC-free-region V1.01");
    frame.set_label_size(14);

    let rc_path_input = Rc::new(RefCell::new(Input::new(100, 50, 250, 30, "RC Path:")));
    let prefix_input  = Rc::new(RefCell::new(Input::new(100, 90, 250, 30, "Prefix:")));
    let margin_input  = Rc::new(RefCell::new(Input::new(100, 130, 250, 30, "Margin(0-9):")));
    let number_input  = Rc::new(RefCell::new(Input::new(100, 170, 250, 30, "Number:")));

    let send_button = Rc::new(RefCell::new(Button::new(100, 210, 80, 30, "Send")));

    let y_plus  = Rc::new(RefCell::new(Button::new(160, 260, 60, 30, "y+")));
    let x_minus = Rc::new(RefCell::new(Button::new(100, 300, 60, 30, "x-")));
    let x_plus  = Rc::new(RefCell::new(Button::new(220, 300, 60, 30, "x+")));
    let y_minus = Rc::new(RefCell::new(Button::new(160, 340, 60, 30, "y-")));

    let z_plus  = Rc::new(RefCell::new(Button::new(290, 285, 60, 30, "z+")));
    let z_minus = Rc::new(RefCell::new(Button::new(290, 325, 60, 30, "z-")));

    let out_button = Rc::new(RefCell::new(Button::new(230, 210, 120, 30, "out")));

    {
        x_minus.borrow_mut().set_color(Color::Cyan);
        x_plus.borrow_mut().set_color(Color::Cyan);
        y_plus.borrow_mut().set_color(Color::Green);
        y_minus.borrow_mut().set_color(Color::Green);
        z_plus.borrow_mut().set_color(Color::Red);
        z_minus.borrow_mut().set_color(Color::Red);
    }

    y_plus.borrow_mut().deactivate();
    y_minus.borrow_mut().deactivate();
    x_minus.borrow_mut().deactivate();
    x_plus.borrow_mut().deactivate();
    z_plus.borrow_mut().deactivate();
    z_minus.borrow_mut().deactivate();
    out_button.borrow_mut().deactivate();

    wind.end();
    wind.show();

    let bat_path = std::env::temp_dir().join("FreeRegion_singleSend.bat");
    {
        let mut f = File::create(&bat_path).expect("Failed to create batch");
        f.write_all(FREE_REGION_BAT.as_bytes()).expect("Write error");
    }

    let exe_path = std::env::current_exe().expect("Exe?");
    let exe_dir_buf = exe_path.parent().unwrap().to_path_buf();
    if !exe_dir_buf.join("reconRegion").exists() {
        let _ = create_dir_all(exe_dir_buf.join("reconRegion"));
    }
    let exe_dir_arc = Arc::new(exe_dir_buf);

    {
        let st = shared_settings.lock().unwrap();
        rc_path_input.borrow_mut().set_value(&st.rc_path);
        prefix_input.borrow_mut().set_value(&st.prefix);
        margin_input.borrow_mut().set_value(&st.margin.to_string());
        number_input.borrow_mut().set_value(&st.number.to_string());
    }

    ctrlc::set_handler(move || {
        println!("[INFO] Ctrl+C => exit");
        std::process::exit(0);
    }).unwrap();

    {
        let rc_path_input_c = rc_path_input.clone();
        let prefix_input_c  = prefix_input.clone();
        let margin_input_c  = margin_input.clone();
        let number_input_c  = number_input.clone();

        let shared_c = Arc::clone(&shared_settings);
        let settings_path_c = settings_path.clone();

        let y_plus_c  = y_plus.clone();
        let y_minus_c = y_minus.clone();
        let x_minus_c = x_minus.clone();
        let x_plus_c  = x_plus.clone();
        let z_plus_c  = z_plus.clone();
        let z_minus_c = z_minus.clone();
        let out_c     = out_button.clone();

        send_button.borrow_mut().set_callback(move |_| {
            let rc_path_val = rc_path_input_c.borrow().value();
            let prefix_val  = prefix_input_c.borrow().value();
            let margin_val  = margin_input_c.borrow().value().parse::<u8>().unwrap_or(3);
            let number_val  = number_input_c.borrow().value().parse::<u32>().unwrap_or(1);

            {
                let mut st = shared_c.lock().unwrap();
                st.rc_path = rc_path_val;
                st.prefix  = prefix_val;
                st.margin  = margin_val;
                st.number  = number_val;
                let _ = save_settings(&settings_path_c, &st);
            }

            y_plus_c.borrow_mut().activate();
            y_minus_c.borrow_mut().activate();
            x_minus_c.borrow_mut().activate();
            x_plus_c.borrow_mut().activate();
            z_plus_c.borrow_mut().activate();
            z_minus_c.borrow_mut().activate();
            out_c.borrow_mut().activate();
        });
    }

    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        y_plus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("y+", &st, &exe_dir_local, &bat_local);
        });
    }
    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        y_minus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("y-", &st, &exe_dir_local, &bat_local);
        });
    }
    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        x_minus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("x-", &st, &exe_dir_local, &bat_local);
        });
    }
    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        x_plus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("x+", &st, &exe_dir_local, &bat_local);
        });
    }
    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        z_plus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("z+", &st, &exe_dir_local, &bat_local);
        });
    }
    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        z_minus.borrow_mut().set_callback(move |_| {
            let st = s_arc.lock().unwrap().clone();
            spawn_single_command("z-", &st, &exe_dir_local, &bat_local);
        });
    }

    {
        let s_arc = Arc::clone(&shared_settings);
        let exe_dir_local = Arc::clone(&exe_dir_arc);
        let bat_local = bat_path.clone();
        let settings_path_out = settings_path.clone();

        out_button.borrow_mut().set_callback(move |_| {
            let st_now = {
                let st = s_arc.lock().unwrap().clone();
                st
            };
            spawn_single_command("out", &st_now, &exe_dir_local, &bat_local);

            {
                let exe_dir = exe_dir_local.clone();
                if let Ok(entries) = std::fs::read_dir(&*exe_dir) {
                    for entry in entries {
                        if let Ok(ent) = entry {
                            let path = ent.path();
                            if path.extension().and_then(|ext| ext.to_str()) == Some("rcbox") {
                                println!("Deleting local rcbox => {:?}", path.display());
                                let _ = std::fs::remove_file(path);
                            }
                        }
                    }
                }
            }

            {
                let mut st = s_arc.lock().unwrap();
                st.number += 1;
                let _ = save_settings(&settings_path_out, &st);
            }
        });
    }

    wind.handle(move |_, ev| {
        if ev == Event::Close {
            println!("[INFO] Window close => exit now");
            app::quit();
            true
        } else {
            false
        }
    });

    app.run().unwrap();
}
