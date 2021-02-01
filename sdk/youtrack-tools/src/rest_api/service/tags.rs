use crate::rest_api::base::client::HttpClient;
use crate::rest_api::base::Ideantifier;
use crate::rest_api::json_models::user::UserDto;
use crate::rest_api::json_models::issue::{IssueDto, IssueTagDto};
use hyper::{Uri, StatusCode, Body};
use hyper::http::uri;
use std::sync::Arc;
use std::ops::Deref;
use crate::rest_api::json_models::issue::field::{IssueStateType};
use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use hyper::body::Buf;
use hyper::http::response::Parts;
use std::num::NonZeroU16;
use crate::rest_api::error::{Result as Res, Error, RestError};
use crate::rest_api::error::Error as YoutrackError;
use std::borrow::Cow;

pub mod fetch {
    use crate::rest_api::base::client::HttpClient;
    use hyper::http::response::Parts;
    use crate::rest_api::base::Ideantifier;
    use crate::rest_api::json_models::issue::IssueTagDto;
    use hyper::Body;
    use hyper::body::Buf;
    use crate::rest_api::error::Result as Res;
    use crate::rest_api::error::Error as YoutrackError;
    use crate::rest_api::error::Error::RestError;

    async fn fetch_all_tags(client: &HttpClient, project_id: String) -> Res<Vec<IssueTagDto>> {
        // id = 0-2
        let uri = format!("/api/admin/projects/{project_id}/relevantTags?$skip=0&$top=50&fields=$type,color(id),id,isDeletable,isShareable,isUpdatable,isUsable,issuesUrl,name,owner($type,id,isLocked,login,name,ringId),pinned,query,readSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,login,name,ringId)),tagSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,login,name,ringId)),untagOnResolve,updateSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,isLocked,login,name,ringId))&sort=true",
                          project_id = project_id);
        let (Parts { status: status_code, .. }, body): (Parts, Body) = client.fetch_data(uri).await.unwrap().into_parts();
        let result = match status_code.as_u16() {
            _ if status_code.is_success() => hyper::body::to_bytes(body).await
                .map_err(|e| YoutrackError::HttpError(e))
                .and_then(|bytes| {
                    let body_str = String::from_utf8(bytes.to_vec()).unwrap();
                    log::trace!("fetched issue with body: {}", &body_str);
                    serde_json::from_str::<Vec<IssueTagDto>>(body_str.as_str()).map_err(|e| YoutrackError::ConverterError(e))
                }),
            status => Err(YoutrackError::empty_list()),
        };
        result
    }

    pub async fn fetch_tag_by_name(client: &HttpClient, project_id: String, name: String) -> Res<IssueTagDto> {
        fetch_all_tags(client, project_id).await
            .and_then(|tags| {
                tags.into_iter()
                    .filter(|tag| tag.name == name)
                    .next().ok_or_else(|| YoutrackError::not_found(name.clone()))
            })
    }
}

pub async fn persist_changes(client: &HttpClient, origin_dto: Arc<IssueTagDto>, modified_dto: Arc<IssueTagDto>) -> Res<IssueTagDto> {
    use std::str;

    let origin_dto = origin_dto.deref();
    let modified_dto = modified_dto.deref();

    // let mut tag_id = origin_dto.id.clone();
    let tag_dto: Res<IssueTagDto>;

    {
        // update path or create path
        let path = origin_dto.id.as_ref().map(|tag_id| format!("/api/issueFolders/{tag_id}?$top=-1&fields=$type,color(id),id,isDeletable,isShareable,isUpdatable,isUsable,issuesUrl,name,owner($type,id,isLocked,login,name,ringId),pinned,query,readSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,isLocked,login,name,ringId)),shortName,tagSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,isLocked,login,name,ringId)),untagOnResolve,updateSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,isLocked,login,name,ringId))",
                                           tag_id = tag_id))
            .unwrap_or("/api/issueFolders?$top=-1&fields=$type,color(id),id,isDeletable,isShareable,isUpdatable,isUsable,issuesUrl,name,owner($type,id,isLocked,login,name,ringId),pinned,query,readSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,login,name,ringId)),tagSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,login,name,ringId)),untagOnResolve,updateSharingSettings(permissionBasedTagAccess,permittedGroups(id,name),permittedUsers($type,id,isLocked,login,name,ringId))&sort=true".to_string());

        let (Parts { status: status_code, .. }, body): (Parts, Body) = client.post_data(path.clone(), modified_dto).await.unwrap().into_parts();

        let body_bytes_res = hyper::body::to_bytes(body).await
            .map_err(|e| YoutrackError::HttpError(e));

        tag_dto = match status_code.as_u16() {
            _ if status_code.is_success() => body_bytes_res.and_then(|bytes| {
                let body_str = String::from_utf8(bytes.to_vec()).unwrap();
                log::trace!("fetched issue with body: {}", &body_str);
                serde_json::from_str::<IssueTagDto>(&body_str).map_err(|e| YoutrackError::ConverterError(e))
            }),
            400u16 =>{
                let body = body_bytes_res
                    .as_ref()
                    .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
                    .unwrap_or("".to_string());
                let error = YoutrackError::RestError(RestError::Conflict(body));
                Err(error)
            },
            status => {
                let body = body_bytes_res
                    .as_ref()
                    .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
                    .unwrap_or("".to_string());
                let message = format!(r###"Can't update tag, status "{status}", path: "{path}", body: "{body}""###, status = status, path = path, body = body);
                let error = YoutrackError::rest_error(status, message);
                Err(error)
            },
        };
    }

    tag_dto
}