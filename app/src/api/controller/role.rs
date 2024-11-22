use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    role::{ReqCreate, RespInfo, RespList,UpdateInfo, RespSelect},
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
    service::role::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(role_id): Path<u64>,
) -> Result<ApiOK<RespInfo>> {

    service::role::info(role_id).await
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {

    service::role::list(query).await
}


pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::role::update(req).await
}

pub async fn delete(
    Extension(identity): Extension<Identity>,
    Path(role_id): Path<u64>,
) -> Result<ApiOK<()>>  {

    service::role::delete(role_id).await
}


pub async fn select_list(
    Extension(identity): Extension<Identity>
) -> Result<ApiOK<Vec<RespSelect>>> {
    service::role::select_list().await
}