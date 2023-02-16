slint::include_modules!();

pub fn _main() {
    let service = "guinix";
    let user = "dhilipsiva";
    let password = "mypass";

    let mut auth = pam::Authenticator::with_password(service).unwrap();
    auth.get_handler().set_credentials(user, password);
    if auth.authenticate().is_ok() && auth.open_session().is_ok() {
        println!("Successfully opened a session!");
    } else {
        println!("Authentication failed =/");
    }
}

fn main() {
    MainWindow::new().run();
}
