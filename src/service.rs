use chrono::*;
use uuid::Uuid;

use crate::db_connection::establish_connection;

use tonic::{Request, Response, Status, Code};

// Compare it with user.proto file, imported from the main.rs file
use crate::user::{
    crud_server::Crud, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
};

#[derive(Default)]
pub struct User {}

#[tonic::async_trait]
impl Crud for User {
    // Compare it with the Crud service definition in user.proto file
    // The method GetUser becomes get_user etc
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let UserRequest { id } = &request.into_inner();

        let conn = establish_connection();

        // 1.
        let rows = &conn
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .unwrap();

        // println!("{:#?}", rows);
        // println!("{:#?}", rows.get(0));
        // https://docs.rs/postgres/0.17.0-alpha.1/postgres/row/struct.Row.html

        if rows.len() < 1 {
            println!("No data found.");
            return Err(Status::new(Code::NotFound, "No data found."));
        }

        // else:
        let row = rows.get(0);
        println!("{:#?}", &row);

        // 2.
        let date_of_birth: NaiveDate = row.get(3);

        let reply = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            // 2.
            date_of_birth: date_of_birth.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);
        let conn = establish_connection();

        // 3.
        let mut v: Vec<UserReply> = Vec::new();
        for row in &conn.query("SELECT * FROM users", &[]).unwrap() {
            let date_of_birth: NaiveDate = row.get(3);
            let user = UserReply {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                date_of_birth: date_of_birth.to_string(),
            };
            v.push(user);
        }

        let reply = Users { users: v };

        Ok(Response::new(reply))
    }

    // Test with create_users, Rust compiler shows errors to help you.
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // 4.
        let user_id = Uuid::new_v4().to_hyphenated().to_string();
        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        // 5.
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let conn = establish_connection();
        // 6.
        let number_of_rows_affected = &conn.execute(
                "INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ($1, $2, $3, $4)",
                &[
                    &user_id,
                    &first_name,
                    &last_name,
                    &serialize_date_of_birth,
                ]
            )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            CreateUserReply {
                message: format!(
                    "Fail to create user with id {}.",
                    &user_id
                ),
            }
        } else {
            CreateUserReply {
                message: format!(
                    "Create {} user with id {}.",
                    &number_of_rows_affected, &user_id
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        // 3.
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let conn = establish_connection();

        let number_of_rows_affected = &conn
            .execute(
                "UPDATE users SET first_name = $2, last_name = $3, date_of_birth = $4 WHERE id = $1",
                &[
                    &id,
                    &first_name,
                    &last_name,
                    &serialize_date_of_birth,
                ]
            )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", id),
            }
        } else {
            UpdateUserReply {
                message: format!("Update {} user with id {}", &number_of_rows_affected, &id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let conn = establish_connection();

        let number_of_rows_affected = &conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
            DeleteUserReply {
                message: format!("Remove the user with id {}.", id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let conn = establish_connection();

        let rows = &conn.query("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!("Remove {} user data from the database.", rows.len()),
        };

        Ok(Response::new(reply))
    }
}
