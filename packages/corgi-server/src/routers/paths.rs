pub struct Paths;

impl Paths {
    pub const OPENAPI_JSON: &'static str = "/api-docs/openapi.json";
    pub const SWAGGER_UI: &'static str = "/api-docs/swagger-ui";
}

impl Paths {
    pub const SYSTEM_PING: &'static str = "/system/ping";
    pub const SYSTEM_STATUS: &'static str = "/system/status";
    pub const SYSTEM_INFO: &'static str = "/system/info";
    pub const SYSTEM_FS_DIRECTORIES: &'static str = "/system/fs/directories";
    pub const SYSTEM_FS_FILES: &'static str = "/system/fs/files";
}

impl Paths {
    pub const ACCOUNT: &'static str = "/account";
}

impl Paths {
    pub const AUTHENTICATION_ENDPOINTS_PASSWORD: &'static str = "/auth/endpoints/password";
}

impl Paths {
    pub const USERS: &'static str = "/users";
}

impl Paths {
    pub const LIBRARIES: &'static str = "/libraries";
    pub const LIBRARIES_ID: &'static str = "/libraries/{id}";

    pub const LIBRARIES_DIRECTORIES: &'static str = "/libraries/{library_id}/directories";
    pub const LIBRARIES_DIRECTORIES_ID: &'static str = "/libraries/{library_id}/directories/{id}";
}
