#![allow(unused_extern_crates)]
extern crate serde_json;
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate urlencoded;
extern crate uuid;
extern crate chrono;
extern crate multipart;

use futures::Future;
use futures::future;
use futures::{stream, Stream};
use hyper;
use hyper::header::{Headers, ContentType};
use self::iron::prelude::*;
use self::iron::{status, modifiers, BeforeMiddleware};
use self::router::Router;
use self::urlencoded::UrlEncodedQuery;
use multipart::server::{Multipart, SaveResult};

#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use swagger;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::BTreeSet;

use swagger::auth::{Authorization, AuthData, Scopes};
use swagger::{ApiError, Context, XSpanId};

use {Api,
     FakeOuterBooleanSerializeResponse,
     FakeOuterCompositeSerializeResponse,
     FakeOuterNumberSerializeResponse,
     FakeOuterStringSerializeResponse,
     TestClientModelResponse,
     TestEndpointParametersResponse,
     TestEnumParametersResponse,
     TestJsonFormDataResponse,
     TestClassnameResponse,
     AddPetResponse,
     DeletePetResponse,
     FindPetsByStatusResponse,
     FindPetsByTagsResponse,
     GetPetByIdResponse,
     UpdatePetResponse,
     UpdatePetWithFormResponse,
     UploadFileResponse,
     DeleteOrderResponse,
     GetInventoryResponse,
     GetOrderByIdResponse,
     PlaceOrderResponse,
     CreateUserResponse,
     CreateUsersWithArrayInputResponse,
     CreateUsersWithListInputResponse,
     DeleteUserResponse,
     GetUserByNameResponse,
     LoginUserResponse,
     LogoutUserResponse,
     UpdateUserResponse
     };
#[allow(unused_imports)]
use models;

mod response_mimetypes {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for FakeOuterBooleanSerialize
    lazy_static! { 
    }
    /// Create Mime objects for the response content types for FakeOuterCompositeSerialize
    lazy_static! { 
    }
    /// Create Mime objects for the response content types for FakeOuterNumberSerialize
    lazy_static! { 
    }
    /// Create Mime objects for the response content types for FakeOuterStringSerialize
    lazy_static! { 
    }
    /// Create Mime objects for the response content types for TestClientModel
    lazy_static! { 
        pub static ref TEST_CLIENT_MODEL_SUCCESSFUL_OPERATION: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for TestEndpointParameters
    lazy_static! { 
        pub static ref TEST_ENDPOINT_PARAMETERS_INVALID_USERNAME_SUPPLIED: Mime = mime!(Application/Xml; ("charset")=("utf-8"));
        pub static ref TEST_ENDPOINT_PARAMETERS_USER_NOT_FOUND: Mime = mime!(Application/Xml; ("charset")=("utf-8"));
    }
    /// Create Mime objects for the response content types for TestEnumParameters
    lazy_static! { 
        pub static ref TEST_ENUM_PARAMETERS_INVALID_REQUEST: Mime = mime!(Star/Star);
        pub static ref TEST_ENUM_PARAMETERS_NOT_FOUND: Mime = mime!(Star/Star);
    }
    /// Create Mime objects for the response content types for TestJsonFormData
    lazy_static! { 
    }
    /// Create Mime objects for the response content types for TestClassname
    lazy_static! { 
        pub static ref TEST_CLASSNAME_SUCCESSFUL_OPERATION: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for AddPet
    lazy_static! { 
        pub static ref ADD_PET_INVALID_INPUT: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for DeletePet
    lazy_static! { 
        pub static ref DELETE_PET_INVALID_PET_VALUE: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for FindPetsByStatus
    lazy_static! { 
        pub static ref FIND_PETS_BY_STATUS_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref FIND_PETS_BY_STATUS_INVALID_STATUS_VALUE: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for FindPetsByTags
    lazy_static! { 
        pub static ref FIND_PETS_BY_TAGS_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref FIND_PETS_BY_TAGS_INVALID_TAG_VALUE: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for GetPetById
    lazy_static! { 
        pub static ref GET_PET_BY_ID_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref GET_PET_BY_ID_INVALID_ID_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref GET_PET_BY_ID_PET_NOT_FOUND: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for UpdatePet
    lazy_static! { 
        pub static ref UPDATE_PET_INVALID_ID_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref UPDATE_PET_PET_NOT_FOUND: Mime = mime!(Application/Xml);
        pub static ref UPDATE_PET_VALIDATION_EXCEPTION: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for UpdatePetWithForm
    lazy_static! { 
        pub static ref UPDATE_PET_WITH_FORM_INVALID_INPUT: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for UploadFile
    lazy_static! { 
        pub static ref UPLOAD_FILE_SUCCESSFUL_OPERATION: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for DeleteOrder
    lazy_static! { 
        pub static ref DELETE_ORDER_INVALID_ID_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref DELETE_ORDER_ORDER_NOT_FOUND: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for GetInventory
    lazy_static! { 
        pub static ref GET_INVENTORY_SUCCESSFUL_OPERATION: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for GetOrderById
    lazy_static! { 
        pub static ref GET_ORDER_BY_ID_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref GET_ORDER_BY_ID_INVALID_ID_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref GET_ORDER_BY_ID_ORDER_NOT_FOUND: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for PlaceOrder
    lazy_static! { 
        pub static ref PLACE_ORDER_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref PLACE_ORDER_INVALID_ORDER: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for CreateUser
    lazy_static! { 
        pub static ref CREATE_USER_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for CreateUsersWithArrayInput
    lazy_static! { 
        pub static ref CREATE_USERS_WITH_ARRAY_INPUT_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for CreateUsersWithListInput
    lazy_static! { 
        pub static ref CREATE_USERS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for DeleteUser
    lazy_static! { 
        pub static ref DELETE_USER_INVALID_USERNAME_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref DELETE_USER_USER_NOT_FOUND: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for GetUserByName
    lazy_static! { 
        pub static ref GET_USER_BY_NAME_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref GET_USER_BY_NAME_INVALID_USERNAME_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref GET_USER_BY_NAME_USER_NOT_FOUND: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for LoginUser
    lazy_static! { 
        pub static ref LOGIN_USER_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
        pub static ref LOGIN_USER_INVALID_USERNAME: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for LogoutUser
    lazy_static! { 
        pub static ref LOGOUT_USER_SUCCESSFUL_OPERATION: Mime = mime!(Application/Xml);
    }
    /// Create Mime objects for the response content types for UpdateUser
    lazy_static! { 
        pub static ref UPDATE_USER_INVALID_USER_SUPPLIED: Mime = mime!(Application/Xml);
        pub static ref UPDATE_USER_USER_NOT_FOUND: Mime = mime!(Application/Xml);
    }

}


/// Create a new router for `Api`
pub fn router<T>(api: T) -> Router where T: Api + Send + Sync + Clone + 'static {
    let mut router = Router::new();
    add_routes(&mut router, api);
    router
}

/// Add routes for `Api` to a provided router.
///
/// Note that these routes are added straight onto the router. This means that if the router
/// already has a route for an endpoint which clashes with those provided by this API, then the
/// old route will be lost.
///
/// It is generally a bad idea to add routes in this way to an existing router, which may have
/// routes on it for other APIs. Distinct APIs should be behind distinct paths to encourage
/// separation of interfaces, which this function does not enforce. APIs should not overlap.
///
/// Alternative approaches include:
///
/// - generate an `iron::middleware::Handler` (usually a `router::Router` or
///   `iron::middleware::chain`) for each interface, and add those handlers inside an existing
///   router, mounted at different paths - so the interfaces are separated by path
/// - use a different instance of `iron::Iron` for each interface - so the interfaces are
///   separated by the address/port they listen on
///
/// This function exists to allow legacy code, which doesn't separate its APIs properly, to make
/// use of this crate.
#[deprecated(note="APIs should not overlap - only for use in legacy code.")]
pub fn route<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {
    add_routes(router, api)
}

/// Add routes for `Api` to a provided router
fn add_routes<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {

    let api_clone = api.clone();
    router.post(
        "/v2/fake/outer/boolean",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let param_body = req.get::<bodyparser::Struct<models::OuterBoolean>>().unwrap_or(None);


                match api.fake_outer_boolean_serialize(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        FakeOuterBooleanSerializeResponse::OutputBoolean(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FakeOuterBooleanSerialize");

    let api_clone = api.clone();
    router.post(
        "/v2/fake/outer/composite",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let param_body = req.get::<bodyparser::Struct<models::OuterComposite>>().unwrap_or(None);


                match api.fake_outer_composite_serialize(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        FakeOuterCompositeSerializeResponse::OutputComposite(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FakeOuterCompositeSerialize");

    let api_clone = api.clone();
    router.post(
        "/v2/fake/outer/number",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let param_body = req.get::<bodyparser::Struct<models::OuterNumber>>().unwrap_or(None);


                match api.fake_outer_number_serialize(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        FakeOuterNumberSerializeResponse::OutputNumber(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FakeOuterNumberSerialize");

    let api_clone = api.clone();
    router.post(
        "/v2/fake/outer/string",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let param_body = req.get::<bodyparser::Struct<models::OuterString>>().unwrap_or(None);


                match api.fake_outer_string_serialize(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        FakeOuterStringSerializeResponse::OutputString(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FakeOuterStringSerialize");

    let api_clone = api.clone();
    router.patch(
        "/v2/fake",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::Client>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.test_client_model(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        TestClientModelResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_CLIENT_MODEL_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "TestClientModel");

    let api_clone = api.clone();
    router.post(
        "/v2/fake",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;





                // Form parameters
                let param_number = 3.4;
                let param_double = 1.2;
                let param_pattern_without_delimiter = "pattern_without_delimiter_example".to_string();
                let param_byte = swagger::ByteArray("byte_example".to_string().into_bytes());
                let param_integer = Some(56);
                let param_int32 = Some(56);
                let param_int64 = Some(789);
                let param_float = Some(3.4);
                let param_string = Some("string_example".to_string());
                let param_binary = Some(swagger::ByteArray(Vec::from("B")));
                let param_date = None;
                let param_date_time = None;
                let param_password = Some("password_example".to_string());
                let param_callback = Some("callback_example".to_string());

                match api.test_endpoint_parameters(param_number, param_double, param_pattern_without_delimiter, param_byte, param_integer, param_int32, param_int64, param_float, param_string, param_binary, param_date, param_date_time, param_password, param_callback, context).wait() {
                    Ok(rsp) => match rsp {
                        TestEndpointParametersResponse::InvalidUsernameSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_ENDPOINT_PARAMETERS_INVALID_USERNAME_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        TestEndpointParametersResponse::UserNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_ENDPOINT_PARAMETERS_USER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "TestEndpointParameters");

    let api_clone = api.clone();
    router.get(
        "/v2/fake",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Header parameters
                header! { (RequestEnumHeaderStringArray, "enum_header_string_array") => (String)* }
                let param_enum_header_string_array = req.headers.get::<RequestEnumHeaderStringArray>().map(|header| header.0.clone());
                header! { (RequestEnumHeaderString, "enum_header_string") => [String] }
                let param_enum_header_string = req.headers.get::<RequestEnumHeaderString>().map(|header| header.0.clone());


                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = req.get::<UrlEncodedQuery>().unwrap_or_default();
                let param_enum_query_string_array = query_params.get("enum_query_string_array")
                    .map(|list| list.iter().flat_map(|x| x.parse::<String>()).collect::<Vec<_>>());
                let param_enum_query_string = query_params.get("enum_query_string")
                    .and_then(|list| list.first()).and_then(|x| x.parse::<String>().ok());
                let param_enum_query_integer = query_params.get("enum_query_integer")
                    .and_then(|list| list.first()).and_then(|x| x.parse::<i32>().ok());


                // Form parameters
                let param_enum_form_string_array = None;
                let param_enum_form_string = Some("enum_form_string_example".to_string());
                let param_enum_query_double = Some(1.2);

                match api.test_enum_parameters(param_enum_form_string_array.as_ref(), param_enum_form_string, param_enum_header_string_array.as_ref(), param_enum_header_string, param_enum_query_string_array.as_ref(), param_enum_query_string, param_enum_query_integer, param_enum_query_double, context).wait() {
                    Ok(rsp) => match rsp {
                        TestEnumParametersResponse::InvalidRequest => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_ENUM_PARAMETERS_INVALID_REQUEST.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        TestEnumParametersResponse::NotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_ENUM_PARAMETERS_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "TestEnumParameters");

    let api_clone = api.clone();
    router.get(
        "/v2/fake/jsonFormData",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Form parameters
                let param_param = "param_example".to_string();
                let param_param2 = "param2_example".to_string();

                match api.test_json_form_data(param_param, param_param2, context).wait() {
                    Ok(rsp) => match rsp {
                        TestJsonFormDataResponse::SuccessfulOperation => {
                            let mut response = Response::with((status::Status::from_u16(200)));
    
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "TestJsonFormData");

    let api_clone = api.clone();
    router.patch(
        "/v2/fake_classname_test",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::Client>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.test_classname(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        TestClassnameResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::TEST_CLASSNAME_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "TestClassname");

    let api_clone = api.clone();
    router.post(
        "/v2/pet",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }




                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::Pet>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.add_pet(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        AddPetResponse::InvalidInput => {
                            let mut response = Response::with((status::Status::from_u16(405)));
    
                            response.headers.set(ContentType(response_mimetypes::ADD_PET_INVALID_INPUT.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "AddPet");

    let api_clone = api.clone();
    router.delete(
        "/v2/pet/:petId",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }


                // Path parameters
                let param_pet_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("petId").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter petId".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter petId: {}", e))))?;

                // Header parameters
                header! { (RequestApiKey, "api_key") => [String] }
                let param_api_key = req.headers.get::<RequestApiKey>().map(|header| header.0.clone());




                match api.delete_pet(param_pet_id, param_api_key, context).wait() {
                    Ok(rsp) => match rsp {
                        DeletePetResponse::InvalidPetValue => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::DELETE_PET_INVALID_PET_VALUE.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "DeletePet");

    let api_clone = api.clone();
    router.get(
        "/v2/pet/findByStatus",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }



                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = req.get::<UrlEncodedQuery>().unwrap_or_default();
                let param_status = query_params.get("status").ok_or_else(|| Response::with((status::BadRequest, "Missing required query parameter status".to_string())))?
                    .iter().flat_map(|x| x.parse::<String>()).collect::<Vec<_>>();



                match api.find_pets_by_status(param_status.as_ref(), context).wait() {
                    Ok(rsp) => match rsp {
                        FindPetsByStatusResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::FIND_PETS_BY_STATUS_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        FindPetsByStatusResponse::InvalidStatusValue => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::FIND_PETS_BY_STATUS_INVALID_STATUS_VALUE.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FindPetsByStatus");

    let api_clone = api.clone();
    router.get(
        "/v2/pet/findByTags",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }



                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = req.get::<UrlEncodedQuery>().unwrap_or_default();
                let param_tags = query_params.get("tags").ok_or_else(|| Response::with((status::BadRequest, "Missing required query parameter tags".to_string())))?
                    .iter().flat_map(|x| x.parse::<String>()).collect::<Vec<_>>();



                match api.find_pets_by_tags(param_tags.as_ref(), context).wait() {
                    Ok(rsp) => match rsp {
                        FindPetsByTagsResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::FIND_PETS_BY_TAGS_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        FindPetsByTagsResponse::InvalidTagValue => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::FIND_PETS_BY_TAGS_INVALID_TAG_VALUE.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "FindPetsByTags");

    let api_clone = api.clone();
    router.get(
        "/v2/pet/:petId",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;



                // Path parameters
                let param_pet_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("petId").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter petId".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter petId: {}", e))))?;




                match api.get_pet_by_id(param_pet_id, context).wait() {
                    Ok(rsp) => match rsp {
                        GetPetByIdResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::GET_PET_BY_ID_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetPetByIdResponse::InvalidIDSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_PET_BY_ID_INVALID_ID_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetPetByIdResponse::PetNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_PET_BY_ID_PET_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "GetPetById");

    let api_clone = api.clone();
    router.put(
        "/v2/pet",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }




                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::Pet>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.update_pet(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        UpdatePetResponse::InvalidIDSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_PET_INVALID_ID_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        UpdatePetResponse::PetNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_PET_PET_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        UpdatePetResponse::ValidationException => {
                            let mut response = Response::with((status::Status::from_u16(405)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_PET_VALIDATION_EXCEPTION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "UpdatePet");

    let api_clone = api.clone();
    router.post(
        "/v2/pet/:petId",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }


                // Path parameters
                let param_pet_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("petId").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter petId".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter petId: {}", e))))?;



                // Form parameters
                let param_name = Some("name_example".to_string());
                let param_status = Some("status_example".to_string());

                match api.update_pet_with_form(param_pet_id, param_name, param_status, context).wait() {
                    Ok(rsp) => match rsp {
                        UpdatePetWithFormResponse::InvalidInput => {
                            let mut response = Response::with((status::Status::from_u16(405)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_PET_WITH_FORM_INVALID_INPUT.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "UpdatePetWithForm");

    let api_clone = api.clone();
    router.post(
        "/v2/pet/:petId/uploadImage",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;

                // Authorization
                if let Scopes::Some(ref scopes) = authorization.scopes {
                    let required_scopes: BTreeSet<String> = vec![
                        "write:pets".to_string(), // modify pets in your account
                        "read:pets".to_string(), // read your pets
                    ].into_iter().collect();

                    if !required_scopes.is_subset(scopes) {
                        let missing_scopes = required_scopes.difference(scopes);
                        return Err(Response::with((
                            status::Forbidden,
                            missing_scopes.fold(
                                "Insufficient authorization, missing scopes".to_string(),
                                |s, scope| format!("{} {}", s, scope)
                            )
                        )));
                    }
                }


                // Path parameters
                let param_pet_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("petId").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter petId".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter petId: {}", e))))?;



                // Form parameters

                // Expecting a multipart form, extract and parse it now.
                let mut entries = match Multipart::from_request(req) {
                    Ok(mut multipart) => {

                        match multipart.save().temp() {
                            SaveResult::Full(entries) => {
                                Ok(entries)
                            },
                            _ => {
                                Err(Response::with((status::InternalServerError, format!("Unable to process all message parts"))))
                            },
                        }
                    },
                    Err(e) => {
                        // Unable to parse as multipart
                        Err(Response::with((status::BadRequest, format!("Couldn't parse body as multipart"))))
                    }
                }?;

                let param_additional_metadata = Some("additional_metadata_example".to_string());


                let param_file = entries.fields.remove("file");

                let param_file = match param_file {
                    Some(body) => {
                        Ok({let bytes = body.as_bytes();
                           Some(
                                Box::new(stream::once(Ok(bytes.to_vec()))) as Box<Stream<Item=Vec<u8>, Error=Error> + Send>
                            )}
                        )
                    }
                    None => {Err(Response::with((status::BadRequest, format!("Body part not found!"))))}
                }?;
                let param_file = Box::new(future::ok(param_file));

                match api.upload_file(param_pet_id, param_additional_metadata, param_file, context).wait() {
                    Ok(rsp) => match rsp {
                        UploadFileResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::UPLOAD_FILE_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "UploadFile");

    let api_clone = api.clone();
    router.delete(
        "/v2/store/order/:order_id",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_order_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("order_id").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter order_id".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter order_id: {}", e))))?;




                match api.delete_order(param_order_id, context).wait() {
                    Ok(rsp) => match rsp {
                        DeleteOrderResponse::InvalidIDSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::DELETE_ORDER_INVALID_ID_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        DeleteOrderResponse::OrderNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::DELETE_ORDER_ORDER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "DeleteOrder");

    let api_clone = api.clone();
    router.get(
        "/v2/store/inventory",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();

                let authorization = context.authorization.as_ref().ok_or_else(|| {
                    Response::with((
                        status::Forbidden,
                        "Unauthenticated".to_string()
                    ))
                })?;






                match api.get_inventory(context).wait() {
                    Ok(rsp) => match rsp {
                        GetInventoryResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::GET_INVENTORY_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "GetInventory");

    let api_clone = api.clone();
    router.get(
        "/v2/store/order/:order_id",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_order_id = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("order_id").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter order_id".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter order_id: {}", e))))?;




                match api.get_order_by_id(param_order_id, context).wait() {
                    Ok(rsp) => match rsp {
                        GetOrderByIdResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::GET_ORDER_BY_ID_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetOrderByIdResponse::InvalidIDSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_ORDER_BY_ID_INVALID_ID_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetOrderByIdResponse::OrderNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_ORDER_BY_ID_ORDER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "GetOrderById");

    let api_clone = api.clone();
    router.post(
        "/v2/store/order",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::Order>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.place_order(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        PlaceOrderResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::PLACE_ORDER_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        PlaceOrderResponse::InvalidOrder => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::PLACE_ORDER_INVALID_ORDER.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "PlaceOrder");

    let api_clone = api.clone();
    router.post(
        "/v2/user",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::User>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.create_user(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        CreateUserResponse::SuccessfulOperation => {
                            let mut response = Response::with((status::Status::from_u16(0)));
    
                            response.headers.set(ContentType(response_mimetypes::CREATE_USER_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "CreateUser");

    let api_clone = api.clone();
    router.post(
        "/v2/user/createWithArray",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<Vec<models::User>>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.create_users_with_array_input(param_body.as_ref(), context).wait() {
                    Ok(rsp) => match rsp {
                        CreateUsersWithArrayInputResponse::SuccessfulOperation => {
                            let mut response = Response::with((status::Status::from_u16(0)));
    
                            response.headers.set(ContentType(response_mimetypes::CREATE_USERS_WITH_ARRAY_INPUT_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "CreateUsersWithArrayInput");

    let api_clone = api.clone();
    router.post(
        "/v2/user/createWithList",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();





                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<Vec<models::User>>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.create_users_with_list_input(param_body.as_ref(), context).wait() {
                    Ok(rsp) => match rsp {
                        CreateUsersWithListInputResponse::SuccessfulOperation => {
                            let mut response = Response::with((status::Status::from_u16(0)));
    
                            response.headers.set(ContentType(response_mimetypes::CREATE_USERS_WITH_LIST_INPUT_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "CreateUsersWithListInput");

    let api_clone = api.clone();
    router.delete(
        "/v2/user/:username",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_username = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("username").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter username".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter username: {}", e))))?;




                match api.delete_user(param_username, context).wait() {
                    Ok(rsp) => match rsp {
                        DeleteUserResponse::InvalidUsernameSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::DELETE_USER_INVALID_USERNAME_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        DeleteUserResponse::UserNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::DELETE_USER_USER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "DeleteUser");

    let api_clone = api.clone();
    router.get(
        "/v2/user/:username",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_username = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("username").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter username".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter username: {}", e))))?;




                match api.get_user_by_name(param_username, context).wait() {
                    Ok(rsp) => match rsp {
                        GetUserByNameResponse::SuccessfulOperation(body) => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
    
                            response.headers.set(ContentType(response_mimetypes::GET_USER_BY_NAME_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetUserByNameResponse::InvalidUsernameSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_USER_BY_NAME_INVALID_USERNAME_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        GetUserByNameResponse::UserNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::GET_USER_BY_NAME_USER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "GetUserByName");

    let api_clone = api.clone();
    router.get(
        "/v2/user/login",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();




                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = req.get::<UrlEncodedQuery>().unwrap_or_default();
                let param_username = query_params.get("username").ok_or_else(|| Response::with((status::BadRequest, "Missing required query parameter username".to_string())))?
                    .first().ok_or_else(|| Response::with((status::BadRequest, "Required query parameter username was empty".to_string())))?
                    .parse::<String>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse query parameter username - doesn't match schema: {}", e))))?;
                let param_password = query_params.get("password").ok_or_else(|| Response::with((status::BadRequest, "Missing required query parameter password".to_string())))?
                    .first().ok_or_else(|| Response::with((status::BadRequest, "Required query parameter password was empty".to_string())))?
                    .parse::<String>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse query parameter password - doesn't match schema: {}", e))))?;



                match api.login_user(param_username, param_password, context).wait() {
                    Ok(rsp) => match rsp {
                        LoginUserResponse::SuccessfulOperation{ body, x_rate_limit, x_expires_after } => {
                            let mut response = Response::with((status::Status::from_u16(200), serde_json::to_string(&body).expect("Impossible to fail to serialize")));
                            header! { (ResponseXRateLimit, "X-Rate-Limit") => [i32] }
                            response.headers.set(ResponseXRateLimit(x_rate_limit));
                            header! { (ResponseXExpiresAfter, "X-Expires-After") => [chrono::DateTime<chrono::Utc>] }
                            response.headers.set(ResponseXExpiresAfter(x_expires_after));
    
                            response.headers.set(ContentType(response_mimetypes::LOGIN_USER_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        LoginUserResponse::InvalidUsername => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::LOGIN_USER_INVALID_USERNAME.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "LoginUser");

    let api_clone = api.clone();
    router.get(
        "/v2/user/logout",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();






                match api.logout_user(context).wait() {
                    Ok(rsp) => match rsp {
                        LogoutUserResponse::SuccessfulOperation => {
                            let mut response = Response::with((status::Status::from_u16(0)));
    
                            response.headers.set(ContentType(response_mimetypes::LOGOUT_USER_SUCCESSFUL_OPERATION.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "LogoutUser");

    let api_clone = api.clone();
    router.put(
        "/v2/user/:username",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_username = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                    .find("username").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter username".to_string())))?
                    .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter username: {}", e))))?;



                // Body parameters (note that non-required body parameters will ignore garbage values, rather than causing a 400 response)
                let body = req.get::<bodyparser::Struct<models::User>>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;
                let param_body = body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.update_user(param_username, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        UpdateUserResponse::InvalidUserSupplied => {
                            let mut response = Response::with((status::Status::from_u16(400)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_USER_INVALID_USER_SUPPLIED.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                        UpdateUserResponse::UserNotFound => {
                            let mut response = Response::with((status::Status::from_u16(404)));
    
                            response.headers.set(ContentType(response_mimetypes::UPDATE_USER_USER_NOT_FOUND.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "UpdateUser");

}

/// Middleware to extract authentication data from request
pub struct ExtractAuthData;

impl BeforeMiddleware for ExtractAuthData {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        {
            header! { (ApiKey1, "api_key") => [String] }
            if let Some(header) = req.headers.get::<ApiKey1>() {
                req.extensions.insert::<AuthData>(AuthData::ApiKey(header.0.clone()));
                return Ok(());
            }
        }
        {
            let header = match req.get_ref::<UrlEncodedQuery>() {
                Ok(query) => query.get("api_key_query").map(|v| v[0].clone()),
                _ => None
            };
            if let Some(key) = header {
                req.extensions.insert::<AuthData>(AuthData::ApiKey(key));
                return Ok(());
            }
        }
        {
            use hyper::header::{Authorization, Basic, Bearer};
            use std::ops::Deref;
            if let Some(basic) = req.headers.get::<Authorization<Basic>>() {
                req.extensions.insert::<AuthData>(AuthData::Basic(basic.deref().clone()));
                return Ok(());
            }
        }
        {
            use hyper::header::{Authorization, Basic, Bearer};
            use std::ops::Deref;
            if let Some(bearer) = req.headers.get::<Authorization<Bearer>>() {
                req.extensions.insert::<AuthData>(AuthData::Bearer(bearer.deref().clone()));
                return Ok(());
            }
        }

        Ok(())
    }
}
