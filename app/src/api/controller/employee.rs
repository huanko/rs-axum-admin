use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    employee::{self, ReqCreate, RespInfo, RespList, UpdateInfo},
};
use pkg::identity::Identity;
use pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::employee::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(employee_id): Path<i64>,
) -> Result<ApiOK<RespInfo>> {

    service::employee::info(employee_id).await
}


pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {

    service::employee::list(query).await
}


pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::employee::update(req).await
}


pub async fn disabled_flag(
    Extension(identity): Extension<Identity>,
    Path(employee_id): Path<i64>,
    Path(disabled_flag): Path<u8>,
)-> Result<ApiOK<()>> {
    service::employee::disabled_flag(employee_id, disabled_flag).await
}


pub async fn reset_password(
    Extension(identity): Extension<Identity>,
    Path(employee_id): Path<i64>,
)-> Result<ApiOK<()>> {
    service::employee::reset_password(employee_id).await
}

pub async fn change_department(
    Extension(identity): Extension<Identity>,
    Path(employee_id): Path<Vec<i64>>,
    Path(department_id): Path<i64>,
)-> Result<ApiOK<()>> {
    service::employee::change_department(employee_id, department_id).await
}