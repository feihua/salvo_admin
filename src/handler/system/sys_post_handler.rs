// Code generated by https://github.com/feihua/code_cli
// author：刘飞华
// date：2025/01/08 13:51:14

use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use salvo::prelude::*;
use salvo::{Request, Response};

use crate::common::result::BaseResponse;
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_post_vo::*;
use crate::RB;

/*
 *添加岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn add_sys_post(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<AddPostReq>().await.unwrap();
    log::info!("add sys_post params: {:?}", &item);

    let res_by_name = Post::select_by_name(&mut RB.clone(), &item.post_name).await;
    match res_by_name {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "新增岗位失败,岗位名称已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }

    let res_by_code = Post::select_by_code(&mut RB.clone(), &item.post_code).await;
    match res_by_code {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "新增岗位失败,岗位编码已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }

    let sys_post = Post {
        id: None,                                //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    let result = Post::insert(&mut RB.clone(), &sys_post).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn delete_sys_post(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<DeletePostReq>().await.unwrap();
    log::info!("delete sys_post params: {:?}", &item);

    let ids = item.ids.clone();
    for id in ids {
        let post_by_id = Post::select_by_id(&mut RB.clone(), &id).await;
        let p = match post_by_id {
            Ok(p) => {
                if p.is_none() {
                    return BaseResponse::<String>::err_result_msg(
                        res,
                        "岗位不存在,不能删除".to_string(),
                    );
                } else {
                    p.unwrap()
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
        };

        let count = count_user_post_by_id(&mut RB.clone(), id).await;
        if count.unwrap_or_default() > 0 {
            let msg = format!("{}已分配,不能删除", p.post_name);
            return BaseResponse::<String>::err_result_msg(res, msg);
        }
    }

    let result = Post::delete_in_column(&mut RB.clone(), "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_post(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdatePostReq>().await.unwrap();
    log::info!("update sys_post params: {:?}", &item);

    let res_by_name = Post::select_by_name(&mut RB.clone(), &item.post_name).await;
    match res_by_name {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "更新岗位失败,岗位名称已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }

    let res_by_code = Post::select_by_code(&mut RB.clone(), &item.post_code).await;
    match res_by_code {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != item.id {
                return BaseResponse::<String>::err_result_msg(
                    res,
                    "更新岗位失败,岗位编码已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }

    let sys_post = Post {
        id: Some(item.id),                       //岗位id
        post_code: item.post_code,               //岗位编码
        post_name: item.post_name,               //岗位名称
        sort: item.sort,                         //显示顺序
        status: item.status,                     //部状态（0：停用，1:正常）
        remark: item.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    let result = Post::update_by_column(&mut RB.clone(), &sys_post, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn update_sys_post_status(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdatePostStatusReq>().await.unwrap();
    log::info!("update sys_post_status params: {:?}", &item);

    let update_sql = format!(
        "update sys_post set status = ? where id in ({})",
        item.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![to_value!(item.status)];
    param.extend(item.ids.iter().map(|&id| to_value!(id)));
    let result = &mut RB.clone().exec(&update_sql, param).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_post_detail(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryPostDetailReq>().await.unwrap();
    log::info!("query sys_post_detail params: {:?}", &item);

    let result = Post::select_by_id(&mut RB.clone(), &item.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<QueryPostDetailResp>::err_result_data(
                    res,
                    QueryPostDetailResp::new(),
                    "岗位不存在".to_string(),
                );
            }
            let x = d.unwrap();

            let sys_post = QueryPostDetailResp {
                id: x.id.unwrap_or_default(),               //岗位id
                post_code: x.post_code,                     //岗位编码
                post_name: x.post_name,                     //岗位名称
                sort: x.sort,                               //显示顺序
                status: x.status,                           //部状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //更新时间
            };

            BaseResponse::<QueryPostDetailResp>::ok_result_data(res, sys_post)
        }
        Err(err) => BaseResponse::<QueryPostDetailResp>::err_result_data(
            res,
            QueryPostDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2025/01/08 13:51:14
 */
#[handler]
pub async fn query_sys_post_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryPostListReq>().await.unwrap();
    log::info!("query sys_post_list params: {:?}", &item);

    let post_code = item.post_code.as_deref().unwrap_or_default(); //岗位编码
    let post_name = item.post_name.as_deref().unwrap_or_default(); //岗位名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Post::select_post_list(&mut RB.clone(), page, post_code, post_name, status).await;

    let mut sys_post_list_data: Vec<PostListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_post_list_data.push(PostListDataResp {
                    id: x.id.unwrap_or_default(),               //岗位id
                    post_code: x.post_code,                     //岗位编码
                    post_name: x.post_name,                     //岗位名称
                    sort: x.sort,                               //显示顺序
                    status: x.status,                           //部状态（0：停用，1:正常）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //更新时间
                })
            }

            BaseResponse::<Vec<PostListDataResp>>::ok_result_page(res, sys_post_list_data, total)
        }
        Err(err) => BaseResponse::<Vec<PostListDataResp>>::err_result_page(res, err.to_string()),
    }
}
