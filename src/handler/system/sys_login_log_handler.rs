// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use rbatis::plugin::page::PageRequest;
use salvo::prelude::*;
use salvo::{Request, Response};

use crate::common::result::BaseResponse;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_login_log_vo::*;
use crate::RB;

/*
 *删除系统访问记录
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_login_log(req: &mut Request, res: &mut Response) {
    match req.parse_json::<DeleteLoginLogReq>().await {
        Ok(item) => {
            log::info!("delete sys_login_log params: {:?}", &item);

            let rb = &mut RB.clone();
            let result = LoginLog::delete_in_column(rb, "id", &item.ids).await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_login_log_detail(req: &mut Request, res: &mut Response) {
    match req.parse_json::<QueryLoginLogDetailReq>().await {
        Ok(item) => {
            log::info!("query sys_login_log_detail params: {:?}", &item);

            let rb = &mut RB.clone();
            let result = LoginLog::select_by_id(rb, &item.id).await;

            match result {
                Ok(opt_sys_login_log) => {
                    if opt_sys_login_log.is_none() {
                        return BaseResponse::<QueryLoginLogDetailResp>::err_result_data(
                            res,
                            QueryLoginLogDetailResp::new(),
                            "系统访问记录不存在".to_string(),
                        );
                    }
                    let x = opt_sys_login_log.unwrap();

                    let sys_login_log = QueryLoginLogDetailResp {
                        id: x.id.unwrap(),                        //访问ID
                        login_name: x.login_name,                 //登录账号
                        ipaddr: x.ipaddr,                         //登录IP地址
                        login_location: x.login_location,         //登录地点
                        platform: x.platform,                     //平台信息
                        browser: x.browser,                       //浏览器类型
                        version: x.version,                       //浏览器版本
                        os: x.os,                                 //操作系统
                        arch: x.arch,                             //体系结构信息
                        engine: x.engine,                         //渲染引擎信息
                        engine_details: x.engine_details,         //渲染引擎详细信息
                        extra: x.extra,                           //其他信息（可选）
                        status: x.status,                         //登录状态(0:失败,1:成功)
                        msg: x.msg,                               //提示消息
                        login_time: time_to_string(x.login_time), //访问时间
                    };

                    BaseResponse::<QueryLoginLogDetailResp>::ok_result_data(res, sys_login_log)
                }
                Err(err) => BaseResponse::<QueryLoginLogDetailResp>::err_result_data(
                    res,
                    QueryLoginLogDetailResp::new(),
                    err.to_string(),
                ),
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_login_log_list(req: &mut Request, res: &mut Response) {
    match req.parse_json::<QueryLoginLogListReq>().await {
        Ok(item) => {
            log::info!("query sys_login_log_list params: {:?}", &item);
            let rb = &mut RB.clone();

            let name = item.login_name.as_deref().unwrap_or_default(); //登录账号
            let ipaddr = item.ipaddr.as_deref().unwrap_or_default(); //登录IP地址
            let browser = item.browser.as_deref().unwrap_or_default(); //浏览器类型
            let os = item.os.as_deref().unwrap_or_default(); //操作系统
            let status = item.status.unwrap_or(2); //登录状态(0:失败,1:成功)

            let page = &PageRequest::new(item.page_no, item.page_size);

            let mut sys_login_log_list_data: Vec<LoginLogListDataResp> = Vec::new();
            match LoginLog::select_login_log_list(rb, page, name, ipaddr, browser, os, &status)
                .await
            {
                Ok(d) => {
                    let total = d.total;

                    for x in d.records {
                        sys_login_log_list_data.push(LoginLogListDataResp {
                            id: x.id.unwrap_or_default(),             //访问ID
                            login_name: x.login_name,                 //登录账号
                            ipaddr: x.ipaddr,                         //登录IP地址
                            login_location: x.login_location,         //登录地点
                            platform: x.platform,                     //平台信息
                            browser: x.browser,                       //浏览器类型
                            version: x.version,                       //浏览器版本
                            os: x.os,                                 //操作系统
                            arch: x.arch,                             //体系结构信息
                            engine: x.engine,                         //渲染引擎信息
                            engine_details: x.engine_details,         //渲染引擎详细信息
                            extra: x.extra,                           //其他信息（可选）
                            status: x.status,                         //登录状态(0:失败,1:成功)
                            msg: x.msg,                               //提示消息
                            login_time: time_to_string(x.login_time), //访问时间
                        })
                    }

                    BaseResponse::<Vec<LoginLogListDataResp>>::ok_result_page(
                        res,
                        sys_login_log_list_data,
                        total,
                    )
                }
                Err(err) => {
                    BaseResponse::<String>::err_result_msg(res, format!("数据库错误: {}", err))
                }
            }
        }
        Err(err) => {
            BaseResponse::<String>::err_result_msg(res, format!("解析请求参数失败: {}", err))
        }
    }
}
