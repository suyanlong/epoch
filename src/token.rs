use chrono::Utc;
use jwt::errors::Result;
use jwt::{decode, encode, Header, Validation};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Token {
    id: usize,
    exp: usize,
}

impl Token {
    pub fn new(id: usize) -> Self {
        Token {
            id,
            exp: (Utc::now().timestamp() + 2 * 24 * 60) as usize,
        }
    }

    pub fn encode(&self) -> Result<String> {
        encode(&Header::default(), &self, "epoch-secret".as_ref())
    }

    pub fn decode(jwt: &str) -> Result<usize> {
        let validation = Validation::default();
        // token is a struct with 2 params: header and claims
        match decode::<Self>(jwt, "epoch-secret".as_ref(), &validation) {
            Ok(token) => Ok(token.claims.id),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_claims() {
        let my_claims = Token {
            id: 123456789,
            exp: (Utc::now().timestamp() + 2 * 24 * 60) as usize,
        };

        let token = encode(&Header::default(), &my_claims, "epoch-secret".as_ref()).unwrap();
        println!("{:?}", token);

        let validation = Validation::default();

        // token is a struct with 2 params: header and claims
        let claims = decode::<Token>(&token, "epoch-secret".as_ref(), &validation);
        println!("{:?}", claims);
        assert_eq!(claims.unwrap().claims.id, 123456789);
    }

    #[test]
    fn test_token() {
        let jwt = Token::new(123456789).encode().unwrap();
        let id = Token::decode(&jwt).unwrap();
        assert_eq!(id, 123456789);
    }
}
