
/// The macro responsible for defining API-centric functions in the GW2 module and struct.
///
/// THis module is responsible for defining the structure of all ArenaNet-focused API calls. Specifically,
/// it makes adding new API calls simple by abstracting the 99% of the function that remains the same away.
/// It reqires the following items:
///
/// - A constant identifier marking which case to use. Valid cases are: NIL, STR, STR_LOC, INT, INT_LOC, AUTH, QTY
/// - $fn_name: The name of the function to define
/// - $res_type: The response type of the function. The inner content of a GW2Result<T> variant.
/// - $endpoint: The `Endpoint` variant that maps to the endpoint this function should call to perform the data fetching.
macro_rules! gw2rs_api_wrapper {
    (NIL, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name(&self) -> GW2Result<$res_type> {
            let client = self.get_http_client();   
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                    .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (STR, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name<'a, I>(&self, ids: &'a I) -> GW2Result<$res_type>
        where
            I: Deref<Target=[&'a str]>,
        {
            let client = self.get_http_client();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::StrIds(ids) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                    .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (STR_LOC, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name<'a, I>(&self, ids: &'a I) -> GW2Result<$res_type>
        where
            I: Deref<Target=[&'a str]>,
        {
            let client = self.get_http_client();   
            let locale = self.locale(); 
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::StrIds(ids), Param::Locale(&locale) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                    .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (INT, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name<'a, I>(&self, ids: &'a I) -> GW2Result<$res_type>
        where
            I: Deref<Target=[u64]>,
        {
            let client = self.get_http_client();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::IntIds(ids) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                    .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (INT_LOC, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name<'a, I>(&self, ids: &'a I) -> GW2Result<$res_type>
        where
            I: Deref<Target=[u64]>,
        {
            let client = self.get_http_client();
            let locale = self.locale();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::IntIds(ids), Param::Locale(&locale) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                    .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (AUTH, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name(&self) -> GW2Result<$res_type> {
            let api_key = self.api_key();
            let client = self.get_http_client();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::AuthToken(api_key) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));
            
            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (AUTH_LOC, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name(&self) -> GW2Result<$res_type> {
            let api_key = self.api_key();
            let locale = self.locale();
            let client = self.get_http_client();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::AuthToken(api_key), Param::Locale(&locale) ];
            let request = crate::internal::http::http_request(client, endpoint, opts)
                .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));
            
            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };

    (QTY, $fn_name: ident, $res_type: ty, $endpoint: expr) => {
        pub fn $fn_name(&self, qty: u64) -> GW2Result<$res_type> {
            let client = self.get_http_client();
            let endpoint = $endpoint;
            let opts: Vec<Param> = vec![ Param::Quantity(qty) ];
            let request =
                crate::internal::http::http_request(client, endpoint, opts)
                .and_then(|res| crate::internal::http::convert_to_struct::<$res_type>(&res));

            #[cfg(feature = "futures-boxed")]
            Box::new(request)
        }
    };
}