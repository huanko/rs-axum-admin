use std::collections::HashMap;

use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set
};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use validator::Validate;

use pkg::{
    db,
    result::response::{ApiErr, ApiOK, Result},
    util, xtime,
};

use crate::ent::{
    t_department, prelude::TDepartment
};

/** 封装添加数据对象 */
#[derive(Debug, Validate,  Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "部门名称必填"))]
    pub deptname: String,
    pub sort: i32,
    pub managerid: i64,
    pub parentid: i64,
}

/** 添加方法 */
pub async fn create(req: ReqCreate) -> Result<ApiOK<()>> {
    let count: u64 = TDepartment::find()
        .filter(t_department::Column::DepartmentName.eq(req.deptname.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_department");
            ApiErr::ErrSystem(None)
        })?;
    
    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("部门名称重复".to_string())));
    }

    /** 创建数据对象 */
    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_department::ActiveModel {
        department_name: Set(req.deptname),  
        sort: Set(req.sort),
        manager_id: Set(req.managerid),
        parent_id: Set(req.parentid),
        create_time: Set(now),
        ..Default::default()
    };
    /* 插入数据 */
    if let Err(e) = TDepartment::insert(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error insert t_department");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}