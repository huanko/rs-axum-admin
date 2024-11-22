use std::collections::HashMap;

use sea_orm::prelude::Expr;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set
};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use validator::Validate;

use pkg::crypto::hash::md5;
use pkg::{
    db,
    result::response::{ApiErr, ApiOK, Result},
    util,xtime,

};


use crate::ent::{prelude::TEmployee, t_employee};




/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate{
    #[validate(length(min = 1, message = "员工姓名必填"))]
    pub realname:String,
    #[validate(length(min = 1, message = "手机号码必填"))]
    pub phone:String,
    pub department_id:i64,
    #[validate(length(min = 1, message = "登录名必填"))]
    pub login_name: String,
    #[validate(length(min = 1, message = "邮箱必填"))]
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,

}


pub async fn create(req: ReqCreate) -> Result<ApiOK<()>> {
    // 验证登录名是否已存在
    let login_name_count = TEmployee::find()
        .filter(t_employee::Column::LoginName.eq(req.login_name.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count login_name");
            ApiErr::ErrSystem(None)
        })?;

    if login_name_count > 0 {
        return Err(ApiErr::ErrPerm(Some("登录名已重复".to_string())));
    }

    let phone_count = TEmployee::find()
        .filter(t_employee::Column::Phone.eq(req.phone.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count phone");
            ApiErr::ErrSystem(None)   
        })?;

    if phone_count > 0 {
        return Err(ApiErr::ErrPerm(Some("手机号码已重复".to_string())));
    }

    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_employee::ActiveModel {
        realname: Set(req.realname),
        phone: Set(req.phone),
        department_id: Set(req.department_id),
        login_name: Set(req.login_name.clone()),
        login_pwd: Set(md5(req.login_name.clone().as_bytes()).to_string()),
        email: Set(req.email),
        gender: Set(req.gender),
        disabled_flag: Set(req.disabled_flag),
        position_id: Set(req.position_id),
        create_time: Set(now),
        ..Default::default()
    };

    if let Err(e) = TEmployee::insert(model)
        .exec(db::conn())
        .await{
            tracing::error!(error = ?e, "error insert t_employee");
            return Err(ApiErr::ErrSystem(None));
        }

    Ok(ApiOK(None))     
}

#[derive(Debug, Serialize)]
pub struct RespInfo{
    pub employee_id:i64,
    pub realname:String,
    pub phone:String,
    pub department_id:i64,
    pub login_name: String,
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,
    pub create_time:i64,
    pub create_time_str:String,
}

/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}


pub async fn list(query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
    let mut builder = TEmployee::find();
    if let Some(disabled_flag) = query.get("disabled_flag") {
        if disabled_flag == "1" {
            builder = builder.filter(t_employee::Column::DisabledFlag.eq(1));
        } else {
            builder = builder.filter(t_employee::Column::DisabledFlag.eq(0));
        }
    }

    if let Some(login_name) = query.get("login_name") {
        if !login_name.is_empty() {
            builder = builder.filter(t_employee::Column::LoginName.contains(login_name));
        }
    }

    if let Some(phone) = query.get("phone") {
        if !phone.is_empty() {
            builder = builder.filter(t_employee::Column::Phone.contains(phone));  
        }
    }



    let mut total: i64 = 0;
    let (offset, limit) = util::query_page(&query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(t_employee::Column::EmployeeId.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count t_employee");
                ApiErr::ErrSystem(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(t_employee::Column::EmployeeId, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_employee");
            ApiErr::ErrSystem(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            employee_id: model.employee_id,
            login_name: model.login_name,
            realname: model.realname,
            phone: model.phone,
            email: model.email,
            gender: model.gender,
            disabled_flag: model.disabled_flag,
            position_id: model.position_id,
            department_id: model.department_id,
            create_time: model.create_time,
            create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
            .unwrap_or_default(),
        };
        resp.list.push(info);
    }

    Ok(ApiOK(Some(resp)))

}


pub async fn info(employee_id: i64) -> Result<ApiOK<RespInfo>> {
    let model = TEmployee::find_by_id(employee_id)
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_employee");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("员工信息不存在".to_string())))?;


    let mut resp = RespInfo {
        employee_id: model.employee_id,
        login_name: model.login_name,
        realname: model.realname,
        gender: model.gender,
        phone: model.phone,
        email: model.email,
        disabled_flag: model.disabled_flag,
        position_id: model.position_id,
        department_id: model.department_id,
        create_time: model.create_time,
        create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
        .unwrap_or_default(),
    };
    Ok(ApiOK(Some(resp)))
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateInfo{

    pub employee_id:i64,

    #[validate(length(min = 1, message = "员工姓名必填"))]
    pub realname:String,
    #[validate(length(min = 1, message = "手机号码必填"))]
    pub phone:String,
    pub department_id:i64,
    #[validate(length(min = 1, message = "登录名必填"))]
    pub login_name: String,
    #[validate(length(min = 1, message = "邮箱必填"))]
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,

}

pub async fn update(req: UpdateInfo) -> Result<ApiOK<()>> {
      /* 判断登录名或者手机号是否重复*/
      let count = TEmployee::find()
      .filter(Condition::any().add(t_employee::Column::LoginName.eq(req.login_name.clone())).add(t_employee::Column::Phone.eq(req.phone.clone())))
      .count(db::conn())
      .await
      .map_err(|e| {
          tracing::error!(error = ?e, "error count t_employee");
          ApiErr::ErrSystem(None)
      })?;

      if count > 0 {
        return Err(ApiErr::ErrPerm(Some("登录名称或手机号码重复".to_string())));
    }

    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_employee::ActiveModel {
        employee_id: Set(req.employee_id),
        login_name: Set(req.login_name),
        realname: Set(req.realname),
        phone: Set(req.phone),
        email: Set(req.email),
        gender: Set(req.gender),
        disabled_flag: Set(req.disabled_flag),
        position_id: Set(req.position_id),
        department_id: Set(req.department_id),
        update_time: Set(now),
        ..Default::default()
    };

    if let Err(e) = TEmployee::update(model)
            .exec(db::conn())
            .await{
                tracing::error!(error = ?e, "error update t_employee");
                return Err(ApiErr::ErrSystem(None));
            }
            Ok(ApiOK(None))
}

// 禁用
pub async fn disabled_flag(employee_id: i64, disabled_flag:u8) -> Result<ApiOK<()>> {
    let _update_model = TEmployee::update_many()
        .col_expr(t_employee::Column::DeletedFlag, Expr::value(disabled_flag))
        .filter(t_employee::Column::EmployeeId.eq(employee_id))
        .exec(db::conn())
        .await;   
    
    Ok(ApiOK(None))
}

// 重置密码
pub async fn reset_password(employee_id: i64) -> Result<ApiOK<()>> {
        let _update_model = TEmployee::update_many()
            .col_expr(t_employee::Column::LoginPwd, Expr::value(md5("123456".as_bytes()).to_string()))
            .filter(t_employee::Column::EmployeeId.eq(employee_id))
            .exec(db::conn())
            .await;     

    Ok(ApiOK(None))
}

// 调整部门
pub async fn change_department(employee_id: Vec<i64>, department_id:i64) -> Result<ApiOK<()>> {
         let _update_model = TEmployee::update_many()
                .col_expr(t_employee::Column::DepartmentId, Expr::value(department_id))
                .filter(t_employee::Column::EmployeeId.is_in(employee_id))
                .exec(db::conn())
                .await;    
        Ok(ApiOK(None))
}
