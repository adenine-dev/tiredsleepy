use tiredsleepy as ts;

fn main() {
    ts::set_log_level(ts::LogLevel::Trace);
    ts::platform::start();
}
