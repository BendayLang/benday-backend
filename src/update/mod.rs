// service
// async fn update() -> Result<(), Box<dyn Error>> {
//     let mut service = Service::new();
//     service.update().await?;
//     Ok(())
// }

mod update_request;
pub use update_request::test_to_json;
