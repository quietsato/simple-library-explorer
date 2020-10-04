use std::io::Result;

pub(crate) struct Book {

}

pub(crate) struct ApiResponse {
    session: String,
    books: Book,
    continue: u8,
}

pub(crate) fn access_api() -> Result<ApiResponse> {
    unimplemented!();
}
