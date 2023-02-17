slint::include_modules!();

pub fn _main() {}

fn main() {
    let main_window = MainWindow::new();
    let window_ref = main_window.as_weak();
    main_window.on_try_login(move || {
        let service = "guinix";
        let window_ref = window_ref.upgrade().unwrap();
        let username = window_ref.get_username();
        let password = window_ref.get_password();
        let mut auth = pam::Authenticator::with_password(service).unwrap();
        auth.get_handler().set_credentials(username, password);
        if auth.authenticate().is_ok() && auth.open_session().is_ok() {
            println!("Successfully opened a session!");
        } else {
            println!("Authentication failed =/");
        }
    });
    main_window.run();
}
