use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use versi_platform::AppPaths;

pub fn init_logging(debug_enabled: bool) {
    if !debug_enabled {
        return;
    }

    let paths = AppPaths::new();
    let _ = paths.ensure_dirs();
    let log_path = paths.log_file();

    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .add_filter_allow_str("versi")
        .build();

    let file_logger = File::create(&log_path)
        .ok()
        .map(|file| WriteLogger::new(LevelFilter::Debug, config.clone(), file));

    #[cfg(debug_assertions)]
    {
        let term_logger = TermLogger::new(
            LevelFilter::Debug,
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );

        if let Some(file_logger) = file_logger {
            let _ = CombinedLogger::init(vec![term_logger, file_logger]);
        } else {
            let _ = CombinedLogger::init(vec![term_logger]);
        }
    }

    #[cfg(not(debug_assertions))]
    {
        if let Some(file_logger) = file_logger {
            let _ = CombinedLogger::init(vec![file_logger]);
        }
    }

    log::info!("Debug logging initialized, log file: {:?}", log_path);
}
