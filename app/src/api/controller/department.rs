use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    department::{self, ReqCreate, RespInfo, RespList, UpdateInfo},
};
use pkg::identity::Identity;
use pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

use pkg::tree;


pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::department::create(req).await
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(department_id): Path<i64>,
) -> Result<ApiOK<RespInfo>> {
    service::department::info(department_id).await
}


pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {

    service::department::list(query).await
}

pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::department::update(req).await
}

pub async fn delete(
    Extension(identity): Extension<Identity>,
    Path(department_id): Path<i64>,
) -> Result<ApiOK<()>>  {
    service::department::delete(department_id).await
}


pub async fn select_list(
    Extension(identity): Extension<Identity>
) -> Result<ApiOK<Vec<tree::TreeNode>>>{
    service::department::select_list().await
}