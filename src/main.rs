#[tokio::main]
async fn main() {
    const FUTURES: usize = 50;
    const N_SEND_PER_FUTURE: usize = 1000;

    let (tx, rx) = kanal::unbounded_async();

    let _futures = (0..FUTURES)
        .map(move |_| {
            let tx = tx.clone();
            tokio::spawn(async move {
                for _ in 0..N_SEND_PER_FUTURE {
                    tx.send(()).await.unwrap();
                }
                /*
                // Works for FUTURES = 1
                tx.close();
                */
            })
        })
        .collect::<Vec<_>>();

    let mut counter = 0;
    loop {
        match rx.try_recv() {
            Ok(Some(_)) => {
                counter += 1;
            }
            Ok(None) => {}
            Err(_) => {
                break;
            }
        }
    }
    /*
    // This is OK
    while let Ok(_) = rx.recv().await {
        counter += 1;
    }
    */

    assert_eq!(counter, FUTURES * N_SEND_PER_FUTURE);
    dbg!(counter);
}
