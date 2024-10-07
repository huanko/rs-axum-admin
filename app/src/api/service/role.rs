use std::collections::HashMap;

use sea_orm::{
    ColumnTrait, Condition, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set
};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use validator::Validate;

use pkg::{
    db,
    result::response::{ApiErr, ApiOK, Result},
    util, xtime,
};

use crate::ent::{t_role, prelude::TRole,
t_role_employee, prelude::TRoleEmployee};

/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub rolename: String,
    #[validate(length(min = 1, message = "角色编码必填"))]
    pub rolecode: String,
    pub remark: String,
}

/** 添加方法 */
pub async fn create(req: ReqCreate) -> Result<ApiOK<()>> {
    let count = TRole::find()
        .filter(Condition::any().add(t_role::Column::RoleName.eq(req.rolename.clone())).add(t_role::Column::RoleCode.eq(req.rolecode.clone())))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_role");
            ApiErr::ErrSystem(None)
        })?;
    
    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("角色名称或角色编码重复".to_string())));
    }

    /** 创建数据对象 */
    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_role::ActiveModel {
        role_name: Set(req.rolename),
        role_code: Set(req.rolecode),
        remark: Set(req.remark),
        create_time: Set(now),
        ..Default::default()
    };
    /* 插入数据 */
    if let Err(e) = TRole::insert(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error insert t_role");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}


/** 封装返回数据对象 */
#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub roleid: i64,
    pub rolename: String,
    pub rolecode: String,
    pub remark: String,
    pub create_time: i64,
    pub create_time_str: String,
}

/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

/** 获取列表 */
pub async fn list(query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
    /** 查询条件 */
    let mut builder = TRole::find();
    if let Some(rolename) = query.get("rolename") {
        if !rolename.is_empty() {
            builder = builder.filter(t_role::Column::RoleName.contains(rolename));
        }
    }

    let mut total: i64 = 0;
    let (offset, limit) = util::query_page(&query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(t_role::Column::RoleId.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count t_role");
                ApiErr::ErrSystem(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(t_role::Column::RoleId, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_role");
            ApiErr::ErrSystem(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            roleid: model.role_id,
            rolename: model.role_name,
            rolecode: model.role_code,
            remark: model.remark,
            create_time: model.create_time,
            create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
            .unwrap_or_default(),
        };
        resp.list.push(info);
    }

    Ok(ApiOK(Some(resp)))
}

/** 获取详情 */
pub async fn info(roleid: u64) -> Result<ApiOK<RespInfo>> {
    let model = TRole::find_by_id(roleid as i64)
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_role");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("角色信息不存在".to_string())))?;

   let mut resp = RespInfo {
       roleid: model.role_id,
       rolename: model.role_name,
       rolecode: model.role_code,
       remark: model.remark,
       create_time: model.create_time,
       create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
       .unwrap_or_default(),
   };
   Ok(ApiOK(Some(resp)))
}


/** 封装修改数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateInfo {
    pub roleid: i64,
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub rolename: String,
    #[validate(length(min = 1, message = "角色编码必填"))]
    pub rolecode: String,
    pub remark: String,
    pub create_time: i64,
    pub create_time_str: String,
}
/** 修改方法 */
pub async fn update(req: UpdateInfo) -> Result<ApiOK<()>> {
    /* 判断角色名称或角色编码是否重复*/
    let count = TRole::find()
        .filter(Condition::any().add(t_role::Column::RoleName.eq(req.rolename.clone())).add(t_role::Column::RoleCode.eq(req.rolecode.clone())))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_role");
            ApiErr::ErrSystem(None)
        })?;
    
    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("角色名称或角色编码重复".to_string())));
    }

    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_role::ActiveModel {
        role_name: Set(req.rolename),
        role_code: Set(req.rolecode),
        remark: Set(req.remark),
        update_time: Set(now),
        ..Default::default()
    };

    if let Err(e) = TRole::update(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error update t_role");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}

/** 删除 */
pub async fn delete(roleid: u64) -> Result<ApiOK<()>> {
    /* 判断是否已分配人员 */
    let count = TRoleEmployee::find()
    .filter(t_role_employee::Column::RoleId.eq(roleid))
    .count(db::conn())
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "error find t_role_employee");
        ApiErr::ErrSystem(None)
    })?;

    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("该角色下存在员工，无法删除".to_string())));
    } 
    if let Err(e) = TRole::delete_by_id(roleid as i64).exec(db::conn()).await {
        tracing::error!(error = ?e, "error delete t_role");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}