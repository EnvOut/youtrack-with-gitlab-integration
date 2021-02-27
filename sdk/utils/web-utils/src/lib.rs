use std::error::Error;
use actix_web::HttpRequest;

pub trait HeaderGetter {
	fn get_header(&self, header_name: &str) -> Option<&str> ;
}

impl HeaderGetter for HttpRequest {
	fn get_header(&self, header_name: &str) -> Option<&str> {
		self.headers().get(header_name)
			.map(|h| h.to_str().ok())
			.flatten()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
