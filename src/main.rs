use std::{cell::RefCell, env, fs, rc::Rc};
use log::{LevelFilter, trace, info};
use simple_logging::log_to_file;
use gtk::{gdk::Display, glib, prelude::*, Application, ApplicationWindow, CssProvider, EventControllerFocus};

const APP_ID: &str = "org.woew.woew";

#[allow(dead_code)]
struct AppDetail {
    name: String,
    icon: String,
    exec: String,
    categories: Vec<String>,
    keyword: Vec<String>
}

/// Walks through directory recursively searching for .desktop file 
fn walk_dir(start_dir: &str) -> (){
    let current_directory_items = fs::read_dir(start_dir).expect("Could Not find the supplied directory");
    for entry in current_directory_items{
        let entry_path = entry.unwrap();
        print!("{}: ", 
            entry_path.path().to_str().unwrap(),
        );
        print!("{}",
            fs::metadata(entry_path.path()).unwrap().is_dir()
        );
        println!("");
    }
}

/// startup_call indexes the application list to the struct 
/// AppDetail
///
/// # Returns
/// 
/// Vec<AppDetail>: List of Indexed application


fn startup_call() -> AppDetail{

    let _app_details: Vec<AppDetail> = vec![] ; 

    #[allow(deprecated)]
    let user_desktop_entries = format!("{}/.local/share/applications", env::home_dir().unwrap().to_str().unwrap());

    walk_dir(&user_desktop_entries);
    return AppDetail {
        name: "".to_string(),
        icon: "".to_string(),
        exec: "".to_string(),
        keyword: vec![String::from("")],
        categories: vec![String::from("")],
    };
}

fn build_ui(app: &Application) ->  (){
    // TODO: Set Decorated -> false
    let height_mult = 1;
    let provider = CssProvider::new();
    provider.load_from_path("./style/woew_window_style.css");
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    ); 

    let input_field = gtk::Entry::builder().build();
    let event_controller = gtk::EventControllerKey::builder().build();

    let focus_controller = EventControllerFocus::builder().build();


    log_to_file("/home/ojash/Projects/rust/woew/woew.log", LevelFilter::Info).unwrap();

    println!("App Starting...");


    let window = Rc::new(RefCell::new(ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .default_height(80 * height_mult)
        .default_width(520)
        .modal(true)
        .resizable(false)
        .title("Woew")
        .can_focus(true)
        .name("Woew")
        .child(&input_field)
        .hide_on_close(true)
        .build()));



    let win_clone = window.clone();
    let win_focus_clone = window.clone();

    focus_controller.connect_leave(move |focus| {
        if !focus.is_focus() { 
            info!("Focus Gained");
            win_focus_clone.try_borrow_mut().unwrap().close();
        } 
    });

    event_controller.connect_key_pressed(move |_, key, _, _|{
        match key {
            gtk::gdk::Key::Escape => {
                println!("ESC Pressed");
                info!("ESC pressed");
                win_clone.try_borrow_mut().unwrap().close();
                //std::process::exit(0);
            }
            _ => ()
        }
        return glib::Propagation::Proceed;
    });
    window.try_borrow_mut().unwrap().add_controller(event_controller);
    window.try_borrow_mut().unwrap().add_controller(focus_controller);

    input_field.connect_changed(|_text| {
    });

    window.try_borrow_mut().unwrap().present();
}

fn main() -> (){
    startup_call();
    let app = Application::builder().application_id(APP_ID).build();
    println!("WEEEEE");
    app.connect_activate(build_ui);

    app.run();
}
