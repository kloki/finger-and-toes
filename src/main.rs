use app::App;
mod app;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
