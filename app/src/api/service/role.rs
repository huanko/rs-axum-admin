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
    tree,
};

use crate::ent::{
        t_role, prelude::TRole,
        t_role_employee, prelude::TRoleEmployee, 
        t_employee, prelude::TEmployee, 
        t_menu, prelude::TMenu, 
        t_role_menu, prelude::TRoleMenu
};

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
    let count= TRole::find()
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
    /** 封装查询条件 */
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



#[derive(Debug, Deserialize, Serialize)]
pub struct RespSelect {
    pub roleid: i64,
    pub rolename: String,
}


pub async fn select_list() -> Result<ApiOK<Vec<RespSelect>>> {
    let models = TRole::find()
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_role");
            ApiErr::ErrSystem(None)
        })?;
    let mut list: Vec<RespSelect> = Vec::with_capacity(models.len());
    for model in models {
        list.push(RespSelect {
            roleid: model.role_id,
            rolename: model.role_name,
        });
    }
    Ok(ApiOK(Some(list)))
}


#[derive(Debug, Serialize)]
pub struct RespEmpInfo{
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
pub struct RespEmpList {
    pub total: i64,
    pub list: Vec<RespEmpInfo>,
}
// 根据用户点击的角色id获取该角色下的员工列表
pub async fn role_emp_list(query: HashMap<String, String>) -> Result<ApiOK<RespEmpList>> {
    // 获取用户参数中的 roleid
    let roleid = convert_string_to_i64(query.get("roleid"));
    
    let default_roleid = 1;
    // 根据roleid获取角色员工表里的员工ID列表
    let emp_id_list = TRoleEmployee::find_by_id(roleid.unwrap_or(default_roleid))
        .select_only()
        .column(t_role_employee::Column::EmployeeId)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_role_employee");
            ApiErr::ErrSystem(None)
        })?;

    // 将获取的员工ID列表转为Vec<i64>
    let emp_id_list = emp_id_list.iter().map(|i| i.employee_id).collect::<Vec<_>>();
    
    // 根据员工ID列表封装到查询条件中
    let mut builder = TEmployee::find();
    builder = builder.filter(t_employee::Column::EmployeeId.is_in(emp_id_list));

    // 封装查询条件
    if let Some(realname) = query.get("realname") {
        if !realname.is_empty() {
            builder = builder.filter(t_employee::Column::Realname.contains(realname));
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
    let mut resp = RespEmpList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespEmpInfo {
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



fn convert_string_to_i64(opt_str: Option<&String>) -> Option<i64> {
    match opt_str {
        Some(s) => Some(s.parse::<i64>().unwrap_or_default()),
        None => None
    }
}


/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespMenuSelect {
    pub menu_id: i64,
    pub menu_name: String,
    pub parent_id: i64,
}

//查询所有访问资源
pub async fn menu_list() -> Result<ApiOK<Vec<tree::TreeNode>>> {
    let menu_list = TMenu::find()
    .select_only()
    .column(t_menu::Column::MenuId)
    .column(t_menu::Column::MenuName)
    .column(t_menu::Column::ParentId)
    .all(db::conn())
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "error find t_department");
        ApiErr::ErrSystem(None)
    })?;

    let mut list = Vec::new();
    for menu in &menu_list{
        list.push(RespMenuSelect{
            menu_id:menu.menu_id,
            menu_name: menu.menu_name.clone(),
            parent_id: menu.parent_id,
        });
    }

    let tuple_list = list.iter().map(|i| (i.menu_id, i.menu_name.clone(), Some(i.parent_id))).collect::<Vec<_>>();
    let tuple_node = tree::build_tree(tuple_list);
    Ok(ApiOK(tuple_node))
}


#[derive(Debug, Serialize)]
pub struct RespRoleMenu {
    pub role_id: i64,
    pub menu_id: i64,
}

//根据角色查询该角色可以访问的资源ID
pub async fn role_menu(roleid: i64) -> Result<ApiOK<Vec<RespRoleMenu>>> {
    let role_menu = TRoleMenu::find()
    .filter(t_role_menu::Column::RoleId.eq(roleid))
    .select_only()
    .column(t_role_menu::Column::MenuId)
    .column(t_role_menu::Column::RoleId)
    .all(db::conn())
    .await
    .map_err(|e| {
        tracing::error!(error = ?e, "error find t_role_menu");
        ApiErr::ErrSystem(None)
    })?;

    let mut list = Vec::new();
    for menu in &role_menu{
        list.push(RespRoleMenu{
            role_id: menu.role_id,
            menu_id: menu.menu_id,
        });
    }
    Ok(ApiOK(Some(list)))
}