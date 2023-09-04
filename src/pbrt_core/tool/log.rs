use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn log_init() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[File] {d(%Y-%m-%d %H:%M:%S)} - {l} - {t} - {m}{n}\n",
        )))
        .build("log/output.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
