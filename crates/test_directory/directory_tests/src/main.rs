use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use directories::directory_service::directory_service_directory::DirectoryServiceDirectory;
use directories::directory_type::Directory;

#[tokio::main]
async fn main() {
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();
    collector.init();
    for _ in 0..5 {
        tokio::spawn(async move {
            let directory_service_connector = DirectoryServiceDirectory::load(()).await.unwrap();
            let account = directory_service_connector
                .get_account("will_always_exist".to_string())
                .await
                .unwrap();
            println!("{:?}", account);
            let account = directory_service_connector
                .login_account("will_always_exist".to_string(), String::new())
                .await
                .unwrap();
            println!("{:?}", account);
            drop(directory_service_connector)
        });
    }
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
