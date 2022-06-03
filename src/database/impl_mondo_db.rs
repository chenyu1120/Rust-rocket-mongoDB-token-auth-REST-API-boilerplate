use crate::database::connect_to_db::MongoDB;
use crate::helper::hash_text;
use crate::routes::routes_models::registration_request::RegistrationRequest;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use rocket::serde::json::Json;
use crate::models::model_user::User;

impl MongoDB {
    pub fn new(database: Database) -> Self {
        MongoDB { database }
    }

    pub async fn find_user_by_login(&self, login: String) -> mongodb::error::Result<Option<User>> {
        let collection_user = self.database.collection::<User>("user");

        Ok(collection_user
            .find_one(bson::doc! { "login": login }, None)
            .await?)
    }

    pub async fn registration(
        &self,
        registration_request: Json<RegistrationRequest>,
    ) -> mongodb::error::Result<bool> {
        let collection_user = self.database.collection::<User>("user");

        match hash_text(&registration_request.password, 4) {
            Ok(hash_password) => {
                collection_user
                    .insert_one(
                        User {
                            _id: ObjectId::new(),
                            login: registration_request.login.clone(),
                            password: hash_password,
                        },
                        None,
                    )
                    .await?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
