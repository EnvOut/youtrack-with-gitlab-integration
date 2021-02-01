use async_trait::async_trait;
use std::rc::Rc;

pub type Ideantifier = String;
pub type NameType = String;

#[async_trait]
pub trait BaseInfo {
    async fn name(&self) -> NameType;
    async fn id(&self) -> Ideantifier;
}

pub mod ops {
    use async_trait::async_trait;

    #[async_trait]
    pub trait BaseOps {
        async fn update(&mut self) -> &mut Self;
        // async fn delete(&self) -> Self;
        async fn save(&mut self) -> &mut Self;
    }
}

pub mod client;

pub mod wrap {
    use crate::rest_api::base::client::HttpClient;
    use std::rc::Rc;
    use std::ops::{Deref, DerefMut};
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct ActiveRecordWrap<DTO> {
        pub origin: Arc<DTO>,
        pub inner: Arc<DTO>,
        pub http_client: Arc<HttpClient>,
        pub project_id: String
    }

    impl<DTO> ActiveRecordWrap<DTO> {
        pub fn new(http_client: Arc<HttpClient>, origin: DTO, project_id: String) -> Self {
            let origin = Arc::new(origin);
            let inner = origin.clone();
            Self { origin, inner, http_client, project_id }
        }

        pub fn refresh(&mut self, new_origin: DTO) -> &Self {
            self.origin = Arc::new(new_origin);
            self.inner = self.origin.clone();
            self
        }
    }

    impl<DTO> Deref for ActiveRecordWrap<DTO> {
        type Target = DTO;

        fn deref(&self) -> &Self::Target {
            &*self.origin
        }
    }

    impl<DTO> DerefMut for ActiveRecordWrap<DTO>
        where DTO: Clone {
        fn deref_mut(&mut self) -> &mut Self::Target {
            Arc::make_mut(&mut self.inner)
        }
    }
}
