use maplibre::{MapBuilder, ScheduleMethod, TokioScheduleMethod};
use maplibre::window::FromWindow;
pub use std::time::Instant;

#[cfg(not(target_os = "android"))]
compile_error!("maplibre-android works only on android.");

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    MapBuilder::from_window("A fantastic window!")
        .with_schedule_method(ScheduleMethod::Tokio(TokioScheduleMethod::new()))
        .build()
        .run_sync();
}
