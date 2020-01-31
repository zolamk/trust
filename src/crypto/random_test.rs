#[cfg(test)]
use crate::crypto::secure_token;

#[allow(dead_code)]
#[test]
fn test_secure_token() {
    let mut token = secure_token(100);

    assert_eq!(token.len(), 100, "unexpected secure token length: expected 100 found {}", token.len());

    token = secure_token(1);

    assert_eq!(token.len(), 1, "unexpected secure token length: expected 1 found {}", token.len());

    token = secure_token(7);

    assert_eq!(token.len(), 7, "unexpected secure token length: expected 7 found {}", token.len());
}
