use actix_web::{Error, HttpResponse, HttpServer, web, App, middleware};
use lazy_static::lazy_static;
use crate::schema::{create_schema, Schema};
use actix_cors::Cors;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

mod settings;
mod schema;

lazy_static! {
    pub static ref CONFIG: settings::Settings = settings::Settings::new().expect("config can be loaded");
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source(&*format!("http://{}:{}/graphql", CONFIG.server.url, CONFIG.server.port));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

// actix_web uses tokio for async
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // example changing env, which is reflected in the settings.rs
    // std::env::set_var("RUN_ENV", "Production");
    println!("env config {:?}", CONFIG.clone().env);
    println!("log config {:?}", CONFIG.clone().log);


    // create juniper schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        // .bind("127.0.0.1:8080")?
        .bind(format!("{}:{}", CONFIG.server.url, CONFIG.server.port))?
        .run()
        .await

}