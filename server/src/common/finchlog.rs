cfg_if::cfg_if! {
  if #[cfg(feature="development")] {
    fn get_log_level() -> log::LevelFilter {
      log::LevelFilter::Debug
    }
  } else {
    fn get_log_level() -> log::LevelFilter {
      log::LevelFilter::Info
    }
  }
}

pub fn init_log(module: &str) -> () {
  let log_level = get_log_level();
  simple_logger::SimpleLogger::new()
    .with_level(log::LevelFilter::Off)
    .with_module_level("actix_web", log::LevelFilter::Info)
    .with_module_level("pfp_server", log_level)
    .with_module_level(module, log_level)
    .init()
    .unwrap()
}
