use futures::Future;

use crate::prelude::*;

impl GW2 {
    /// Retrieves the current build information.
    pub fn build(&self) -> impl Future<Item = Build, Error = APIError> {
        let client = self.get_http_client();   
        let endpoint = Endpoint::Build;
        crate::internal::http::request_without_ids(client, endpoint, None, None, None)
            .and_then(|res| crate::internal::http::convert_to_struct::<Build>(&res))
    }
}