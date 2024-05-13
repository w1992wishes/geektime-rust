use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*};

fn main() {
    // 创建一个 Tracing subscriber，并将日志级别设置为 trace
    let subscriber: fmt::Subscriber = fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    // 初始化 Tracing 日志框架的格式化输出，并使用上面创建的 subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // 记录一些日志消息
    info!("This is an informational message");
    warn!("This is a warning message");
}
