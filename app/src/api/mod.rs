use actix_web::{get, post, web, Responder, HttpServer, middleware, App, HttpRequest, HttpResponse, error};
use actix_web::dev::Server;
use crate::settings;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use listenfd::ListenFd;
use tokio::stream::StreamExt;
use gitlab_tools::models::hooks::GitlabHookRequest;
use web_utils::HeaderGetter;
use crate::service::youtrack_service::YoutrackService;
use youtrack_tools::rest_api::client::YoutrackClientImpl;
use actix_web::web::Json;
use actix_web::body::Body;
use actix_web::client::HttpError;
use crate::service::webhook_service::SimpleWebhookService;
use crate::service::Service;
use crate::service::pattern_builder_service::mustache::MustachePatternBuilderService;
use crate::service::grok_service::GrokService;
use crate::service::operation_service::OperationService;

const MAX_SIZE: usize = 262_144;

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}


#[post("/api/gitlab/hooks")]
pub async fn merge_request(request: HttpRequest, hook: web::Json<GitlabHookRequest>, hook_service: web::Data<Service<SimpleWebhookService>>) -> impl Responder {
    let gitlab_event_header = request.get_header("X-Gitlab-Event");
    let gitlab_token_header = request.get_header("x-gitlab-token");

    let expected_token = settings::get_str("gitlab.token").unwrap();

    if let (Some(gitlab_event), Some(gitlab_token)) = (gitlab_event_header, gitlab_token_header) {
        if gitlab_token == expected_token {
            match hook.0 {
                GitlabHookRequest::MergeRequest(merge_request_hook) => {
                    let mut hook_service = hook_service.write().await;
                    hook_service.process_merge_request_hook(merge_request_hook).await;
                    HttpResponse::Ok().body("done")
                }
                GitlabHookRequest::Note(note_hook) => {
                    let mut hook_service = hook_service.write().await;
                    hook_service.process_note_hook(note_hook).await;
                    HttpResponse::Ok().body("done")
                }
                GitlabHookRequest::Pipeline(_) => HttpResponse::BadRequest().body(r#"Pipeline hook not impelemnted"#),
            }.await
        } else {
            HttpResponse::BadRequest().body(r#"Wrong value of header: "X-Gitlab-Token""#).await
        }
    } else {
        HttpResponse::BadRequest().body(r#"Missed requared headers: "X-Gitlab-Event", "X-Gitlab-Token""#).await
    }
}

pub async fn server() -> Server {
    let mut listenfd = ListenFd::from_env();

    let grok_service = {
        let custom_patterns = crate::settings::get_map("app.grok.patterns").unwrap();
        let merge_request_title_pattern = crate::settings::get_str("gitlab.merge-request.title-pattern").unwrap();
        let service = GrokService::new(custom_patterns, merge_request_title_pattern);
        crate::service::new_service(service)
    };

    let youtrack_service = {
        let base_url = settings::get_str("youtrack.url").unwrap();
        let token = settings::get_str("youtrack.token").unwrap();
        let client = YoutrackClientImpl::new(base_url, token).await.unwrap();
        let service = YoutrackService::new(client);
        crate::service::new_service(service)
    };

    let pattern_builder_service = {
        let service = MustachePatternBuilderService::new();
        crate::service::new_service(service)
    };

    let operationService = {
        let service = OperationService::new(youtrack_service.clone(), pattern_builder_service.clone(), grok_service.clone());
        crate::service::new_service(service)
    };

    let webhook_service = {
        let service = SimpleWebhookService::new(
            youtrack_service.clone(), pattern_builder_service.clone(),
            grok_service.clone(), operationService.clone(),
        );
        crate::service::new_service(service)
    };

    let mut server = HttpServer::new(move || {
        App::new()
            // .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            // .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(merge_request)
            .data(youtrack_service.clone())
            .data(webhook_service.clone())
        // .service(load_image_base64)
        // .service(load_image_multipart)
        // .service(load_image_link)
        // .service(get_image)
        // .service(get_images)
        // .service(index)
        // .service(ping)
        // .data(api_version.clone())
        // .data(image_service.clone())
        // .service(
        // 	web::resource("/resource2/index.html")
        // 		.wrap(
        // 			middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"),
        // 		)
        // 		.default_service(
        // 			web::route().to(|| HttpResponse::MethodNotAllowed()),
        // 		)
        // 		.route(web::get().to(index_async)),
        // )
        // .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    });

    let server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        let application_port = settings::get_int("app.port").unwrap() as u16;
        let socket = ("0.0.0.0", application_port);
        server.bind(socket).unwrap()
    };
    println!("Selected sockets for web: {:?}", server.addrs());
    server.run()
}
