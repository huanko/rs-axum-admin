use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use time::macros::offset;
use validator::Validate;

use pkg::crypto::hash::md5;
use pkg::identity::Identity;
use pkg::result::response::{ApiErr, ApiOK, Result};
use pkg::{db, util, xtime};

use crate::ent::t_employee;
use crate::ent::prelude::TEmployee;

use crate::ent::t_role_employee;
use crate::ent::prelude::TRoleEmployee;

/** 封装输入参数 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqLogin {
    #[validate(length(min = 1, message = "用户名必填"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码必填"))]
    pub password: String,
}

/** 封装返回参数 */
#[derive(Debug, Deserialize, Serialize)]
pub struct RespLogin {
    pub name: String,
    pub role: i64,
    pub auth_token: String,
}

/**
 * 登录接口
 */
pub async fn login(req: ReqLogin) -> Result<ApiOK<RespLogin>> {
    /* 根据用户名查询sys_user表，返回用户对象 */
    let  model = TEmployee::find()
        .filter(t_employee::Column::LoginName.eq(req.username))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find t_employee");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrAuth(Some("账号不存在".to_string())))?;

        /* 根据用户ID查询 sys_user_role表，返回用户角色关系表对象 */
        let t_role_employee  = TRoleEmployee::find()
            .filter(t_role_employee::Column::EmployeeId.eq(model.employee_id))
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find sys_user_role");
                ApiErr::ErrSystem(None)
            })?
            .ok_or(ApiErr::ErrAuth(Some("账号角色关系不存在".to_string())))?;

        let pass = format!("{}", req.password);
        if md5(pass.as_bytes()) != model.login_pwd {
            return Err(ApiErr::ErrAuth(Some("密码错误".to_string())));
        }

        let now = xtime::now(offset!(+8)).unix_timestamp();
        //自定义token
        let login_token = md5(format!("auth.{}.{}.{}", model.employee_id, now, util::nonce(16)).as_bytes());
        // 加密token
        let auth_token = Identity::new(model.employee_id, login_token.clone())
        .to_auth_token()
        .map_err(|e| {
            tracing::error!(error = ?e, "error identity encrypt");
            ApiErr::ErrSystem(None)
        })?;

        // 封装修改model
        let update_model = t_employee::ActiveModel {
            login_at: Set(now),
            login_token: Set(login_token),
            update_time: Set(now),
            ..Default::default()
        };
        // 更新T_employee表数据
        let ret_update = TEmployee::update_many()
            .filter(t_employee::Column::EmployeeId.eq(model.employee_id))
            .set(update_model)
            .exec(db::conn())
            .await;
        if let Err(e) = ret_update {
            tracing::error!(error = ?e, "error update t_employee");
            return Err(ApiErr::ErrSystem(None));
        }
    
        let resp = RespLogin {
            name: model.realname,
            role: t_role_employee.role_id,
            auth_token,
        };
    
        Ok(ApiOK(Some(resp)))
}

/**退出接口 */
pub async fn logout(identity: Identity) -> Result<ApiOK<()>> {
    let ret: std::result::Result<_, _> = TEmployee::update_many()
        .filter(t_employee::Column::EmployeeId.eq(identity.id()))
        .col_expr(t_employee::Column::LoginToken, Expr::value(""))
        .col_expr(
            t_employee::Column::CreateTime,
            Expr::value(xtime::now(offset!(+8)).unix_timestamp()),
        )
        .exec(db::conn())
        .await;

    if let Err(e) = ret {
        tracing::error!(error = ?e, "error update t_employee");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}