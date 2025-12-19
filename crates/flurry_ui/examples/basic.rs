use flurry_ui::ui;

fn submit() {
    println!("Submit clicked!");
}

fn main() {
    // Example from the guide - this should compile when Rust is available
    let _ui_tree = ui! {
        column(padding = 16, gap = 8) {
            text("Login")
            button(on_click = submit) {
                text("Sign In")
            }
        }
    };
}