use chrono;

enum LogType 
{
    INFO,
    WARNING,
    ERROR,
}

fn main()
{
    log(LogType::INFO, "This is an info.");
    log(LogType::WARNING, "This is a warning.");
    log(LogType::ERROR, "This is an error.");
    log_with_timestamp(LogType::INFO, "This is an info with timestamp.");
    log_with_timestamp(LogType::WARNING, "This is a warning with timestamp.");
    log_with_timestamp(LogType::ERROR, "This is an error with timestamp.");

    log(LogType::INFO, "Functions with return value");
    let sum = sum(2, 2);
    println!("2+2={sum}");
    let div = div(2, 1);
    println!("2/1={div}");
}

fn log(log_type : LogType, message : &str)
{
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

fn sum(a: i32, b: i32) -> i32
{
    return a + b;
}

//This works too, because there is no semicolon
fn div(a: i32, b: i32) -> i32
{
    a / b
}