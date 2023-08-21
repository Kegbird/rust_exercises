use chrono;

enum LogType {
    INFO,
    WARNING,
    ERROR,
}

fn main(){
    log(LogType::INFO, "This is an info.");
    log(LogType::WARNING, "This is a warning.");
    log(LogType::ERROR, "This is an error.");
    log_with_timestamp(LogType::INFO, "This is an info with timestamp.");
    log_with_timestamp(LogType::WARNING, "This is a warning with timestamp.");
    log_with_timestamp(LogType::ERROR, "This is an error with timestamp.");
}

fn log(log_type : LogType, message : &str){
    match log_type
    {
        LogType::INFO => println!("Info: {message}"),
        LogType::WARNING => println!("Warning: {message}"),
        LogType::ERROR => println!("Error: {message}"),
    }   
}

fn log_with_timestamp(log_type : LogType, message : &str){
    let timestamp = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");
    print!("{timestamp} - ");
    log(log_type, message);
}