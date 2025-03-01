use std::{cell::RefCell, env, fs::{self, File}, io::{self, BufRead, BufReader, Lines, Read}, path::Path, rc::Rc};
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

fn check_in_env_path(exec_command: &str) -> bool{
    let paths = env::split_paths(env::SplitPaths)
    return true;
}
fn check_valid_desktop_entry(exec_command: &str) -> bool{
    let path: &str; 
    let exec_field_codes = vec!["%f", "%S", "%u", "%U", "%d", "%D", "%N", "%n", "%i", "%c", "%k", "%v", "%m"];
    // Assume Flatpak is always right
    if exec_command.starts_with("flatpak "){ return true };
    if exec_command.contains(" wine ") { return true };

    exec_command.split(" ").any(|substring| check_in_env_path(substring));
    if exec_field_codes.iter().any(|code| exec_command.contains(code)) {
        return check_valid_desktop_entry(exec_command.split("%").nth(0).unwrap().trim());
    }

    if exec_command.contains("\""){
        path = exec_command.split("\"").nth(1).unwrap();  
        println!("{} contains \\", exec_command);
    }
    else{
        path = exec_command; 
    }

    // println!("Path={}", path);

    if path.contains("Thunderbird"){
        println!("{}", Path::new(path).exists());
    }
    if Path::new(path).exists(){  
        return true;
    }
    else {
        return false;
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn check_non_hidden_desktop_entry(path: &str) -> bool {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok){
            if (line.starts_with("NoDisplay") || line.starts_with("Hidden")) {
                let value = line.split("=").last().unwrap().trim();
                if value.to_lowercase() == "true" {return true} else {return false}
            }
        }
    }
    return true;
}

/// Walks through directory recursively searching for .desktop file 
fn walk_dir(start_dir: &str) -> (){
    let current_directory_items = fs::read_dir(start_dir).expect("Could Not find the supplied directory");
    for entry in current_directory_items{
        let entry_path = entry.unwrap();
        
        if fs::metadata(entry_path.path()).unwrap().is_file(){
            let contents = fs::read_to_string(entry_path.path())
                .expect(format!("Error: File {} could not be read", entry_path.path().to_str().unwrap()).as_str());

            let lines = contents.split("\n");
            for line in lines {
                let pair:Vec<&str> = line.split("=").collect() ;
                if pair.first().unwrap().to_string() == "Exec" {
                    let exec_command = pair.last().unwrap().to_string();
                    let validity = check_valid_desktop_entry(exec_command.as_str());
                    if validity {

                        println!("VALID {}: {}\n", exec_command, validity);
                    }
                }
                // let exec_command = line.split("Exec="); 
            }
        }
        // print!("Entry = {}\n", 
        //     entry_path.path().to_str().unwrap(),
        // );
        // print!("IsDir = {}\n",
        //     fs::metadata(entry_path.path()).unwrap().is_dir()
        // );
        // println!("");
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
            win_focus_clone.try_borrow_mut().unwrap().close();
        } 
    });

    event_controller.connect_key_pressed(move |_, key, _, _|{
        match key {
            gtk::gdk::Key::Escape => {
                println!("ESC Pressed");
                win_clone.try_borrow_mut().unwrap().close();
                // std::process::exit(0);
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
    app.connect_activate(build_ui);

    app.run();
}
