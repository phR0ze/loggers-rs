pub const APP_NAME: &'static str = "loggers-rs";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_GIT_COMMIT: &'static str = env!("APP_GIT_COMMIT");
pub const APP_BUILD_DATE: &'static str = env!("APP_BUILD_DATE");

fn main() {
   println!("{} v{} - {}", APP_NAME, APP_VERSION, APP_DESCRIPTION);
   println!("{:->w$}", "-", w = 60);
   println!("{:<w$} {}", "Build Date:", APP_BUILD_DATE, w = 18);
   println!("{:<w$} {}", "Git Commit:", APP_GIT_COMMIT, w = 18);
}
