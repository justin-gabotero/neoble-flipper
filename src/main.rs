use std::error::Error;

mod request;

fn main() -> Result<(), Box<dyn Error>> {
    request::get_uuid_from_username("3nbi");
    Ok(())
}
