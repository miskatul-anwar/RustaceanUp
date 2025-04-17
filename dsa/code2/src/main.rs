#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        "return value"
    });

    let v = vec![1, 2, 3];

    let printval = tokio::spawn(async move {
        println!("{:?}", v)
    });

    let out = handle.await.unwrap();
    printval.await;

    println!("We've got: {}", out)
}
