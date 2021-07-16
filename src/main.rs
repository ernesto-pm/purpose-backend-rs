#[macro_use] extern crate rocket;
use mongodb::{Client, options::ClientOptions};
use rocket::{Rocket, Build};
use rocket::State;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/stuff")]
async fn stuff(mongo_client: &State<Client>) -> &'static str {
    let database_names = mongo_client.list_database_names(None, None).await.unwrap();
    for database_name in database_names {
        println!("{}", database_name)
    }

    "Yex this worked"
}


#[launch]
async fn rocket() -> _ {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    // Manually set an option.
    client_options.app_name = Some("Purpose Store".to_string());
    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    rocket::build()
        .manage(client)
        .mount("/api/v1", routes![index, stuff])
}
