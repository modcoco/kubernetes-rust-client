use common::chrono::Local;
use common::tokio::{self, time::Instant};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

pub type ReloadLogLevelHandle =
    tracing_subscriber::reload::Handle<tracing_subscriber::EnvFilter, tracing_subscriber::Registry>;

pub fn setup_logger() -> Arc<tokio::time::Instant> {
    let start_time = Instant::now();
    let start_time: Arc<Instant> = Arc::new(start_time);

    pub struct LocalTimer;
    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("%FT%T%.3f"))
        }
    }

    let format = tracing_subscriber::fmt::format()
        .with_line_number(true)
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        // .with_writer(io::stdout) // 写入标准输出
        // .with_writer(non_blocking)
        .with_ansi(true) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init();

    info!("init");

    start_time
}

pub fn init_logger() -> ReloadLogLevelHandle {
    pub struct LocalTimer;
    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("%FT%T%.3f"))
        }
    }

    let default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()), // sqlx::query=debug
    );
    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(default_filter);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_ansi(false)
                .with_timer(LocalTimer),
        )
        .try_init();
    reload_handle
}

#[allow(dead_code)]
#[test]
fn main() {
    setup_logger();
    tracing::info!("This is not an example");
}

// WEB
// #[derive(serde::Deserialize)]
// pub struct LevelFlag {
//     level: String,
// }
// pub async fn change_log_level(
//     Query(flag): Query<LevelFlag>,
//     Extension(reload_handle): Extension<logger::ReloadLogLevelHandle>,
// ) -> String {
//     match flag.level.parse::<logger::EnvFilter>() {
//         Ok(env_filter) => {
//             reload_handle.modify(|filter| *filter = env_filter).unwrap();
//             "ok".to_string()
//         }
//         Err(err) => err.to_string(),
//     }
// }

// CMD
// export RUST_LOG=debug