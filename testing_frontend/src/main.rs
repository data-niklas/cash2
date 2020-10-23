use futures::executor::block_on;
use smol::{io, net, prelude::*, Unblock};

fn main() {
    let future = secret(); // Nothing is printed
    let task = smol::spawn(future);
    task.detach();
    println!("{}", smol::spawn(future)) 
}

async fn secret() -> usize{
    return secret2().await;
}

async fn secret2() -> usize{
    return 42;
}

