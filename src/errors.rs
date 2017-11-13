error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links {
        Hyper(::reqwest::Error);
        Io(::std::io::Error);
        Image(::image::ImageError);
    }

    errors {
        InvalidUri(url: ::url::Url) {
            description("URL doesn't parse to a valid URI")
            display("URL doesn't parse to a valid URI: {}", &url)
        }
        BadStatusCode(response: ::reqwest::Response) {
            description("Bad status code")
            display("Bad status code: {}", response.status())
        }
        NoContentType(response: ::reqwest::Response) {
            description("No Content-Type found.")
        }
        BadContentType(response: ::reqwest::Response) {
            description("Invalid Content-Type for image.")
        }
    }
}
