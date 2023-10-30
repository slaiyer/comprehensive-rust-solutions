type PathFragIter<'a> = std::str::Split<'a, char>;
const PATH_SEP: char = '/';
const WILDCARD: &str = "*";

/// Supports glob matching on path fragments.
fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut p_it: PathFragIter = prefix.split(PATH_SEP);
    let mut r_it: PathFragIter = request_path.split(PATH_SEP);

    loop {
        match (p_it.next(), r_it.next()) {
            (None, _) => return true,
            (Some(WILDCARD), Some(_)) => {
                if match_glob(&mut p_it, &mut r_it) {
                    continue;
                } else {
                    return false;
                }
            }
            (Some(p_frag), Some(r_frag)) if p_frag == r_frag => advance(&mut p_it, &mut r_it),
            _ => return false,
        }
    }
}

fn advance(p_it: &mut PathFragIter, r_it: &mut PathFragIter) {
    p_it.next();
    r_it.next();
}

fn match_glob(p_it: &mut PathFragIter, r_it: &mut PathFragIter) -> bool {
    match p_it.next() {
        None => true,
        Some(p_frag) => match_lookahead(p_frag, r_it),
    }
}

/// Halts on first matching fragment in request path.
fn match_lookahead(p_frag: &str, r_it: &mut PathFragIter) -> bool {
    loop {
        match r_it.next() {
            Some(r_frag) if p_frag == r_frag => return true,
            Some(_) => continue,
            None => return false,
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
