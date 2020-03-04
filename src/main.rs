use messages_actix::MessageApp;
fn main() -> std::io::Result<()> {
    std:env::set_var("RUS_LOG", "actix_web=info");
    env_logger::init();
    let app = messageApp::new(8080);
    app.run()
}
