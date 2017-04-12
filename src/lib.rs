extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2 as oauth2;
extern crate serde;
extern crate serde_json;
pub mod auth {
    pub fn get_auth(client_email: String) {
        use hyper::Client;
        use hyper::net::HttpsConnector;
        use hyper_native_tls::NativeTlsClient;
        use oauth2::{Authenticator, DefaultAuthenticatorDelegate, MemoryStorage, GetToken, ApplicationSecret};
        use std::io::{stdin,stdout,Write};

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let client_id = String::from("936475272427.apps.googleusercontent.com");
        let client_secret = String::from("KWsJlkaMn1jGLxQpWxMnOox-");
        let token_uri = String::from("https://accounts.google.com/o/oauth2/token");
        let auth_uri = String::from("https://accounts.google.com/o/oauth2/programmatic_auth");
        let redirect_uri = String::from("");
        if client_email == String::from("") {
            let mut client_email = String::new();
            println!("Please enter Google Email: ");
            let _=stdout().flush();
            stdin().read_line(&mut client_email).expect("Did not enter a correct Email");
            if let Some('\n')=client_email.chars().next_back() {
                client_email.pop();
            }
            if let Some('\r')=client_email.chars().next_back() {
                client_email.pop();
            }
        }
        println!("You typed: {}",client_email);

        let secret = ApplicationSecret {client_id: client_id, client_secret: client_secret, token_uri: token_uri, auth_uri: auth_uri, redirect_uris: vec![redirect_uri], project_id: None, client_email: None, auth_provider_x509_cert_url: None, client_x509_cert_url: None};
        let res = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                client,
                                <MemoryStorage as Default>::default(), None)
                                .token(&["https://www.googleapis.com/auth/userinfo.email"]);
        match res {
            Ok(t) => {
                // now you can use t.access_token to authenticate API calls within your
                // given scopes. It will not be valid forever, but Authenticator will automatically
                // refresh the token for you.
                println!("Acquired access_token: {}", t.access_token);
                println!("Acquired access_token: {}", t.refresh_token);
            },
            Err(err) => println!("Failed to acquire token: {}", err),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::auth::get_auth;
    use std::io::{stdin,stdout,Write};
    use std::env;
    #[test]
    fn get_auth_token() {
        match env::var_os("CI") {
            Some() => println!("CI's currently not supported at this test"),
            None => {
                let mut client_email = String::new();
                println!("Please enter Google Email: ");
                let _=stdout().flush();
                stdin().read_line(&mut client_email).expect("Did not enter a correct Email");
                if let Some('\n')=client_email.chars().next_back() {
                    client_email.pop();
                }
                if let Some('\r')=client_email.chars().next_back() {
                    client_email.pop();
                }
                get_auth(client_email);
            }
        }
    }
}
