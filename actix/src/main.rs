#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    RunQueryDsl,
};
use actix_web::{get, post, web, App, web::Data, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateUserDto {
    name: String,
    email: String,
}

#[post("/user")]
async fn create_user(
    db_pool: web::Data<DbPool>,
    form: web::Form<CreateUserDto>
) -> impl Responder {
    let CreateUserDto { name, email } = form.into_inner();
    let new_user = models::NewUser {
        name,
        email,
    };
    let conn = db_pool.get().unwrap();
    let rec = web::block(
        move || {
            use crate::schema::users;
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result::<models::User>(&conn)
        }
    )
    .await
    .unwrap().unwrap();

    web::Json(rec)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePostDto {
    title: String,
    content: String,
    author_email: String,
}

#[post("/post")]
async fn create_post(
    db_pool: web::Data<DbPool>,
    form: web::Form<CreatePostDto>
) -> impl Responder {
    let CreatePostDto { title, content, author_email } = form.into_inner();
    let conn = db_pool.get().unwrap();
    let rec = web::block(
        move || {
            // TODO query author_id
            let new_post = models::NewPost {
                title,
                content,
                published: false,
                author_id: 1,
            };

            use crate::schema::posts;
            diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result::<models::Post>(&conn)
        }
    )
    .await
    .unwrap().unwrap();

    web::Json(rec)
}

#[get("/post")]
async fn post_list(
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = db_pool.get().unwrap();
    let rec = web::block(
        move || {
            use crate::schema::posts;
            posts::table.load::<models::Post>(&conn)
            // posts::table.select((
            //     posts::id,
            //     posts::title,
            //     posts::content,
            //     posts::published,
            //     posts::author_id,
            // )).load::<models::Post>(&conn)
        }
    ).await
    .unwrap().unwrap();

    web::Json(rec)
}

// diesel::update(
//     score::table
//     .filter(score::id.eq(id))
//     .filter(score::user_id.eq(user_id))
// )
// .set((
//     &change_set,
//     score::updated_at.eq(now.naive_local()),
// ))
// .execute(&conn)

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Must set DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    let db_pool = r2d2::Pool::builder()
        .build(manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(hello)
            .service(create_post)
            .service(create_user)
            .service(post_list)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}