use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    role::{ReqCreate, RespInfo, RespList,UpdateInfo, RespSelect, RespEmpList,RespRoleMenu},
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


// 根据角色Id查询对应角色下的员工列表,参数包含角色Id、员工姓名、员工手机号、登录名
pub async fn role_emp_list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>
) -> Result<ApiOK<RespEmpList>> {
    service::role::role_emp_list(query).await
}

//功能权限-查询所有功能权限
pub async fn role_func_list(
    Extension(identity): Extension<Identity>,
) -> Result<ApiOK<Vec<tree::TreeNode>>>{
    service::role::menu_list().await
}

//功能权限-根据角色Id查询对应角色下的功能ID列表
pub async fn role_func_id(
    Extension(identity): Extension<Identity>,
    Path(role_id): Path<i64>
) -> Result<ApiOK<Vec<RespRoleMenu>>>{
    service::role::role_menu(role_id).await
}
