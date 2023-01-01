use aliri::jwt;
use aliri_oauth2::{oauth2::HasScope, Authority, AuthorityError, ScopePolicy};
use serde::Deserialize;

pub trait AliriOAuth2Extensions {
    fn verify_token_from_str<T>(self, s: &str) -> Result<T, AuthorityError>
    where
        T: for<'de> Deserialize<'de> + HasScope + jwt::CoreClaims;
}

impl AliriOAuth2Extensions for Authority {
    fn verify_token_from_str<T>(self, s: &str) -> Result<T, AuthorityError>
    where
        T: for<'de> Deserialize<'de> + HasScope + jwt::CoreClaims,
    {
        let jwt = aliri::jwt::Jwt::new(s.to_string());
        self.verify_token::<T>(jwt.as_ref(), &ScopePolicy::allow_any())
    }
}
