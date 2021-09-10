mod run;

fn hello() {
    println!("Hello, world! My PID is {}", std::process::id());
}

#[tokio::main]
async fn main() -> run::DynResult {
    hello();

    run::run().await
}
