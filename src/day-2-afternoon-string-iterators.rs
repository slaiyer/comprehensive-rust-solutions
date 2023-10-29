pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut prefix_parts = prefix.split('/').peekable();
    let mut request_path_parts = request_path.split('/').peekable();

    loop {
        match (prefix_parts.peek(), request_path_parts.peek()) {
            (None, Some(_)) => return true,
            (Some(prefix_part), Some(request_path_part))
                if *prefix_part == *request_path_part => {
                    prefix_parts.next();
                    request_path_parts.next();
                },
            (Some(prefix_part), Some(request_path_part))
                if *prefix_part != "*" && *prefix_part != *request_path_part => return false,
            (Some(&"*"), Some(_)) => {
                prefix_parts.next();
                match prefix_parts.peek() {
                    Some(&prefix_part_needle) => {
                        loop {
                            match request_path_parts.next() {
                                Some(request_path_part_next) if prefix_part_needle == request_path_part_next => return true,
                                Some(_) => continue,
                                None => return false,
                            }
                        }
                    },
                    None => return true,
                }
            },
            (None, None) => return true,
            _ => return false,
        }
    }
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
