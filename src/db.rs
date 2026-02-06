// -- Lógica de conexión a la base de datos
// use mongodb::{Collection, Database, Client, options::ClientOptions};
use std::env;

pub async fn conectar() -> Database {

    // -- EJEMPLO
    // Leer la URI de MongoDB desde las variables de entorno
    // let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI no está definida en el archivo .env");
    // let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME no está definida en el archivo .env");
    
    // // Configurar el cliente de MongoDB
    // let client_options = ClientOptions::parse(&mongo_uri).await.expect("Error al parsear la URI de MongoDB");
    // let client = Client::with_options(client_options).expect("Error al crear el cliente de MongoDB");

    // client.database(&mongo_db_name)
}


