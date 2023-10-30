type PathFragIter<'a> = std::iter::Peekable<std::str::Split<'a, char>>;
const PATH_SEP: char = '/';
const WILDCARD: &str = "*";

fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut p_it: PathFragIter = prefix.split(PATH_SEP).peekable();
    let mut r_it: PathFragIter = request_path.split(PATH_SEP).peekable();

    loop {
        match (p_it.peek(), r_it.peek()) {
            (None, None) => return true,
            (None, Some(_)) => return true,
            (Some(&p_frag), Some(&r_frag)) if p_frag == r_frag => advance(&mut p_it, &mut r_it),
            (Some(&p_frag), Some(&r_frag)) if p_frag != WILDCARD && p_frag != r_frag => return false,
            (Some(&WILDCARD), Some(_)) => return glob_match(&mut p_it, &mut r_it),
            _ => return false,
        }
    }
}

fn advance(p_it: &mut PathFragIter, r_it: &mut PathFragIter) {
    p_it.next();
    r_it.next();
}

fn glob_match(p_it: &mut PathFragIter, r_it: &mut PathFragIter) -> bool {
    p_it.next();

    match p_it.peek() {
        None => return true,
        Some(&p_frag) => loop {
            match r_it.next() {
                Some(r_frag) if p_frag == r_frag => return true,
                Some(_) => continue,
                None => return false,
            }
        },
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
