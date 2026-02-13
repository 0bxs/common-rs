use chrono::Local;
use tracing::{Level, subscriber};
use tracing_rolling_file::{RollingConditionBase, RollingFileAppender};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{FmtSubscriber, fmt};

struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, w: &mut fmt::format::Writer) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn init(env: String) {
    match env.as_str() {
        "dev" => {
            tracing_subscriber::fmt()
                .with_span_events(FmtSpan::CLOSE)
                .with_max_level(Level::DEBUG)
                .with_timer(LocalTime)
                .with_line_number(true)
                .with_ansi(true)
                .init();
        }
        _ => {
            let appender = RollingFileAppender::new(
                "./logs/log.log",
                RollingConditionBase::new().max_size(10 * 1024 * 1024),
                100,
            )
            .expect("create rolling file appender failed");
            let (non_blocking, guard) = tracing_appender::non_blocking(appender);
            let subscriber = FmtSubscriber::builder()
                .with_span_events(FmtSpan::CLOSE)
                .with_max_level(Level::DEBUG)
                .with_timer(LocalTime)
                .with_line_number(true)
                .with_writer(non_blocking)
                .with_ansi(false)
                .finish();
            subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
            Box::leak(Box::new(guard));
        }
    }
}
