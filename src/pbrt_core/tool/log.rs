use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn log_init() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[File] {d(%H:%M:%S)} {m}{n}\n",
        ))).append(false)
        .build("log/output.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
