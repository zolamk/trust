use frank_jwt::Algorithm;

pub fn get_algorithm(alg: &str) -> Algorithm {
    return match alg {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        "RS256" => Algorithm::RS256,
        "RS384" => Algorithm::RS384,
        "RS512" => Algorithm::RS512,
        "ES256" => Algorithm::ES256,
        "ES384" => Algorithm::ES384,
        "ES512" => Algorithm::ES512,
        a => panic!("unsupported algorithm: {}", a),
    };
}
