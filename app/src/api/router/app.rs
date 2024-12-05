use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::api::{
    controller::{department, login, position, role,employee},
    middleware,
};

pub fn init() -> Router{
    // 开放
    let open = Router::new().route("/login", post(login::login));

    // 需授权
    let auth = Router::new()
        .route("/logout", get(login::logout))
        .route("/roles", get(role::list).post(role::create))  
        .route("/roles/:role_id", get(role::info).delete(role::delete))
        .route("/roles/update", post(role::update))
        .route("/roles/select_list", get(role::select_list))
        .route("/roles/role_emp_list", get(role::role_emp_list))
        .route("/roles/role_func_list", get(role::role_func_list))
        .route("/roles/role_func_id", get(role::role_func_id))
        
        .route("/positions", get(position::list).post(position::create))
        .route("/positions/:post_id", get(position::info).delete(position::delete))
        .route("/positions/update", post(position::update))
        .route("/positions/select_list", get(position::select_list))
        
        
        .route("/departments", get(department::list).post(department::create))
        .route("/departments/:department_id", get(department::info).delete(department::delete))
        .route("/departments/update", post(department::update))
        .route("/departments/select_list", get(department::select_list))
        
        .route("/employees", get(employee::list).post(employee::create))
        .route("/employees/:employee_id", get(employee::info))
        .route("/employees/update", post(employee::update))
        .route("/employees/disabled_flag/:employee_id/:disabled_flag", get(employee::disabled_flag))
        .route("/employees/reset_password/:employee_id", get(employee::reset_password))
        .route("/employees/change_department/:employee_ids/:department_id", get(employee::change_department))
        .route("/employees/employee_select_list", get(employee::employee_select_list))

        .layer(axum::middleware::from_fn(middleware::auth::handle));

        Router::new()
            .route("/", get(|| async { "☺ welcome to Rust app" }))
            .nest("/v1", open.merge(auth))
            .layer(axum::middleware::from_fn(pkg::middleware::log::handle)) // 请求日志
            .layer(axum::middleware::from_fn(pkg::middleware::identity::handle))// 请求身份验证
            .layer(axum::middleware::from_fn(pkg::middleware::cors::handle))// 请求跨域
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                    let req_id = match request
                        .headers()
                        .get("x-request-id")
                        .and_then(|value| value.to_str().ok())
                    {
                        Some(v) => v.to_string(),
                        None => String::from("unknown"),
                    };

                    tracing::error_span!("request_id", id = req_id)
                }),
            )
            .layer(axum::middleware::from_fn(pkg::middleware::req_id::handle))
}