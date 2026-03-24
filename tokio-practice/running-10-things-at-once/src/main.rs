use anyhow::Result;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles: Vec<JoinHandle<u64>> = Vec::new();
    let tasks = 10;
    let chunk_size = 1000;

    for i in 0..tasks {
        // Calculate the range for this specific task
        let start = (i * chunk_size) + 1;
        let end = (i + 1) * chunk_size;

        let handle = tokio::spawn(async move {
            let mut partial_sum = 0;
            for num in start..=end {
                partial_sum += num;
            }
            println!(
                "Task {} ({} to {}) finished with: {}",
                i, start, end, partial_sum
            );
            partial_sum
        });

        handles.push(handle);
    }

    let mut final_result = 0;

    // Join all handles and sum the results
    for handle in handles {
        // .await returns a Result<u64, JoinError>
        let task_result = handle.await?;
        final_result += task_result;
    }

    println!("---");
    println!("The total sum of 1 to 10,000 is: {}", final_result);

    Ok(())
}
