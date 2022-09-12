use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, HttpRequest};
use std::fs::File;
use xiaotserver::mods::{types::Config, tools::reg_fit};

#[get("/")]
async fn hello(req: HttpRequest) -> HttpResponse {
    let config = req
        .app_data::<Config>()
        .unwrap();
    let host = match req.headers().get("Host") {
        Some(host) => host.to_str().unwrap(),
        _ => match req.headers().get("authority") {
            Some(host) => host.to_str().unwrap(),
            _ => "",
        },
    };
    let mut names = match reg_fit(host).await {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("emmmmm出错了");
        }
    };
    for blackname in &config.blacklistnames {
        if names[1].contains(blackname) {
            names[1] = "你妈什么时候";
            break;
        }
    }
    
    let mut body_data = String::new();
    for item in names {
        body_data.push_str(item);
    }

    return HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(r#"<html><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><title>笑嘻了</title></head><body><div style="margin:0px auto;text-align:center;"><h1>{}</h1></div><div id="naptha_container0932014_0707"></div></body></html>"#,body_data));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_file = File::open("config.json").unwrap();
    let config: Config = serde_json::from_reader(config_file).unwrap();
    HttpServer::new(move || {

        App::new()
        .app_data(config.clone())
        .service(hello)
    })
        .bind(("127.0.0.1", 2664))?
        .run()
        .await
}
