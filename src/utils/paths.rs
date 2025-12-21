//! Path normalization utilities

/// Normalize path to use forward slashes (cross-platform compatibility)
///
/// Engram archives always store paths with forward slashes internally,
/// regardless of the platform they were created on.
pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("foo/bar/baz.txt"), "foo/bar/baz.txt");
        assert_eq!(normalize_path("foo\\bar\\baz.txt"), "foo/bar/baz.txt");
        assert_eq!(normalize_path("foo\\bar/baz.txt"), "foo/bar/baz.txt");
    }
}
