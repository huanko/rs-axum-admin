use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    position::{ReqCreate, RespInfo, RespList,UpdateInfo},
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
    service::position::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(role_id): Path<u64>,
) -> Result<ApiOK<RespInfo>> {

    service::position::info(role_id).await
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {

    service::position::list(query).await
}


pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::position::update(req).await
}

pub async fn delete(
    Extension(identity): Extension<Identity>,
    Path(role_id): Path<u64>,
) -> Result<ApiOK<()>>  {

    service::position::delete(role_id).await
}
