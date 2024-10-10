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

use crate::ent::{t_position, prelude::TPosition};

/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "职务名称必填"))]
    pub postname: String,
    pub level: String,
    pub sort: i64,
    pub remark: String,
}

pub async fn create(req: ReqCreate) -> Result<ApiOK<()>> {
    
    let count = TPosition::find()
        .filter(t_position::Column::PositionName.eq(req.postname.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_position");
            ApiErr::ErrSystem(None)
        })?;
    
    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("职务名称重复".to_string())));
    }

    /** 创建数据对象 */
    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_position::ActiveModel {
        position_name: Set(req.postname),
        level: Set(req.level),
        sort: Set(req.sort),
        remark: Set(req.remark),
        deleted_flag: Set(0),
        create_time: Set(now),
        ..Default::default()
    };
    /* 插入数据 */
    if let Err(e) = TPosition::insert(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error insert t_position");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}

/** 封装返回数据对象 */
#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub postid: i64,
    pub postname: String,
    pub level: String,
    pub sort: i64,
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
    let mut builder = TPosition::find();
    if let Some(postname) = query.get("postname") {
        if !postname.is_empty() {
            builder = builder.filter(t_position::Column::PositionName.contains(postname));
        }
    }

    let mut total: i64 = 0;
    let (offset, limit) = util::query_page(&query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(t_position::Column::PositionId.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count t_position");
                ApiErr::ErrSystem(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(t_position::Column::PositionId, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_position");
            ApiErr::ErrSystem(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            postid: model.position_id,
            postname: model.position_name,
            level: model.level,
            sort: model.sort,
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
pub async fn info(postid: u64) -> Result<ApiOK<RespInfo>> {
    let model = TPosition::find_by_id(postid as i64)
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_position");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("职务信息不存在".to_string())))?;

   let mut resp = RespInfo {
        postid: model.position_id,
        postname: model.position_name,
        level: model.level,
        sort: model.sort,
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
    pub postid: i64,
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub postname: String,
    pub level: String,
    pub sort: i64,
    pub remark: String,
    pub create_time: i64,
    pub create_time_str: String,
}
/** 修改方法 */
pub async fn update(req: UpdateInfo) -> Result<ApiOK<()>> {
   
    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_position::ActiveModel {
        position_name: Set(req.postname),
        level: Set(req.level),
        sort: Set(req.sort),
        remark: Set(req.remark),
        update_time: Set(now),
        ..Default::default()
    };

    if let Err(e) = TPosition::update(model).exec(db::conn()).await {
        tracing::error!(error = ?e, "error update t_position");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}

/** 删除 */
pub async fn delete(postid: u64) -> Result<ApiOK<()>> {
    if let Err(e) = TPosition::delete_by_id(postid as i64).exec(db::conn()).await {
        tracing::error!(error = ?e, "error delete t_position");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}
