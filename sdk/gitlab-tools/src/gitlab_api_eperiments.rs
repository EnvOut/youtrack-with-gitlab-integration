extern crate gitlab;

pub mod models;

use gitlab::{Gitlab, api};
use gitlab::api::projects::merge_requests::CreateMergeRequest;
use gitlab::api::Query;

pub fn create_client() -> Gitlab {
    Gitlab::new("git.let4.me", "token").unwrap()
}

pub fn prepare_merge_request<'a>() -> CreateMergeRequest<'a> {
    CreateMergeRequest::builder()
        .project("serge/proj2")
        .title("mr title")
        .description("description")
        .remove_source_branch(true)
        .squash(true)
        // .project(NameOrId::Name(Cow::from("proj2")))
        .source_branch("branch1")
        .target_branch("master")
        .build()
        .unwrap()
}

pub fn create_merge_request(client: &Gitlab, merge_request:CreateMergeRequest){
    api::ignore(merge_request).query(client).unwrap();
}

#[cfg(test)]
mod tests {
    use gitlab::{Gitlab, Project, api, MergeRequest};
    use gitlab::api::{projects, Query};
    use gitlab::api::projects::merge_requests::CreateMergeRequest;
    use gitlab::api::common::NameOrId;
    use gitlab::api::endpoint_prelude::Cow;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    pub fn some() {
        let client = Gitlab::new("git.let4.me", "token").unwrap();
        // Create a simple endpoint. This one gets the "gitlab-org/gitlab" project information.
        let endpoint = projects::Project::builder().project("serge/proj2").build().unwrap();
        // client.graphql()
        // MergeRequest::

        let merge_request = CreateMergeRequest::builder()
            .project("serge/proj2")
            .title("mr title 2")
            .description("description")
            .remove_source_branch(true)
            .squash(true)
            // .project(NameOrId::Name(Cow::from("proj2")))
            .source_branch("branch1")
            .target_branch("master")
            .build()
            .unwrap();
        // merge_request.query(&client).unwrap();
// Call the endpoint. The return type decides how to represent the value.
        api::ignore(merge_request).query(&client).unwrap();

        // let merge_request_response = merge_request.query(&client).unwrap();
        let project: Project = endpoint.query(&client).unwrap();
// For some endpoints (mainly `POST` endpoints), you may want to ignore the result.
// `rest_api::ignore` can be used to do this.
        let _: () = api::ignore(endpoint).query(&client).unwrap();

// Some endpoints support pagination. They work on their own or via the `rest_api::paged` function
// to get further results.
        let pageable_endpoint = projects::Projects::builder().build().unwrap();
// The endpoint on its own is just the first page of results (usually 20 entries).
        let first_page: Vec<Project> = pageable_endpoint.query(&client).unwrap();
// `rest_api::paged` can be used to get results up to some count or all results.
        let first_200_projects: Vec<Project> = api::paged(pageable_endpoint, api::Pagination::Limit(200)).query(&client).unwrap();

// Builders accept strings or integers for some fields. This is done wherever GitLab supports
// either IDs or names being used.
        let endpoint = projects::Project::builder().project(278964).build().unwrap();
// The `rest_api::raw` function can be used to return the raw data from the endpoint. This is
// usually meant for endpoints which represent file contents, pipeline artifacts, etc., but may
// be used with any endpoint.
        let raw_data: Vec<u8> = api::raw(endpoint).query(&client).unwrap();
    }
}
