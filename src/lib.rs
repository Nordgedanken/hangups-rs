#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2 as oauth2;
extern crate serde;
extern crate serde_json;
extern crate protobuf;
extern crate rand;

pub mod auth {
    pub fn get_auth(client_email: String) {
        use hyper::Client;
        use hyper::net::HttpsConnector;
        use hyper_native_tls::NativeTlsClient;
        use oauth2::{Authenticator, DefaultAuthenticatorDelegate, MemoryStorage, GetToken,
                     ApplicationSecret};
        use std::io::{stdin, stdout, Write};

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
            let _ = stdout().flush();
            stdin()
                .read_line(&mut client_email)
                .expect("Did not enter a correct Email");
            if let Some('\n') = client_email.chars().next_back() {
                client_email.pop();
            }
            if let Some('\r') = client_email.chars().next_back() {
                client_email.pop();
            }
        }
        println!("You typed: {}", client_email);

        let secret = ApplicationSecret {
            client_id: client_id,
            client_secret: client_secret,
            token_uri: token_uri,
            auth_uri: auth_uri,
            redirect_uris: vec![redirect_uri],
            project_id: None,
            client_email: None,
            auth_provider_x509_cert_url: None,
            client_x509_cert_url: None,
        };
        let res = Authenticator::new(&secret,
                                     DefaultAuthenticatorDelegate,
                                     client,
                                     <MemoryStorage as Default>::default(),
                                     None)
                .token(&["https://www.googleapis.com/auth/userinfo.email"]);
        match res {
            Ok(t) => {
                println!("Acquired access_token: {}", t.access_token);
                println!("Acquired access_token: {}", t.refresh_token);
            }
            Err(err) => println!("Failed to acquire token: {}", err),
        }

    }
}

#[path = "."]
pub mod hangups {
    #[path = "proto/hangouts.rs"] mod hangouts;
    use self::hangouts::{MessageContent, Segment, SegmentType, RequestHeader, EventRequestHeader,
                         SendChatMessageRequest, DeliveryMediumType, OffTheRecordStatus,
                         EventType, SendChatMessageResponse, DeliveryMedium, ConversationId};
    use protobuf::RepeatedField;
    use rand::random;
    use hyper::header::Headers;
    use hyper::header::Cookie;
    use hyper::client::{Client, IntoUrl};
    use hyper::client::RequestBuilder;
    use hyper::method::Method;
    header! { (Content_Type, "content-type") => [String] }
    header! {
              (XGoogEncodeResponseIfExecutable, "X-Goog-Encode-Response-If-Executable") => [String]
            }

    fn GetAuthHeaders() {}

    fn ApiRequest(endpointUrl: String, contentType: String, cookies: String, payload: SendChatMessageRequest) {
        let mut headers = Headers::new();
        headers.set(Cookie(vec![String::from(cookies.to_owned())]));
        headers.set(Content_Type(String::from(contentType.to_owned())));
        headers.set(XGoogEncodeResponseIfExecutable(String::from("base64".to_owned())));
        let client = Client::new();
        let custom_request = RequestBuilder {
            client: &client,
            url: endpointUrl.into_url(),
            headers: Some(headers),
            method: Method::Post,
            body: payload
        };
    }

    fn ProtobufApiRequest(apiEndpoint: String,
                          requestStruct: SendChatMessageRequest,
                          responseStruct: SendChatMessageResponse) {
        let url = format!("https://clients6.google.com/chat/v1/{}", apiEndpoint);
        let output = self::ApiRequest(url,
                                      String::from("application/x-protobuf"),
                                      String::from("proto"),
                                      requestStruct);
    }

    pub fn send_message(message: String, conv_id_raw: String) {
        //Prepare message
        let segment_type = SegmentType::SEGMENT_TYPE_TEXT;
        let mut segment_raw = Segment::new();
        segment_raw.set_field_type(segment_type);
        segment_raw.set_text(message);
        let mut segment = RepeatedField::new();
        segment.push(segment_raw);
        let mut message_content = MessageContent::new();
        message_content.set_segment(segment);


        //Prepare Request
        let request_header = RequestHeader::new();
        request_header.set_language_code(String::from("en"));

        let deliveryMedium = DeliveryMedium::new();
        deliveryMedium.set_medium_type(DeliveryMediumType::DELIVERY_MEDIUM_BABEL);
        let expectedOtr = OffTheRecordStatus::OFF_THE_RECORD_STATUS_ON_THE_RECORD;
        let clientGeneratedId = random::<u64>();
        let eventType = EventType::EVENT_TYPE_REGULAR_CHAT_MESSAGE;
        let event_request_header = EventRequestHeader::new();
        let conv_id = ConversationId::new();
        conv_id.set_id(conv_id_raw);
        event_request_header.set_conversation_id(conv_id);
        event_request_header.set_delivery_medium(deliveryMedium);
        event_request_header.set_expected_otr(expectedOtr);
        event_request_header.set_client_generated_id(clientGeneratedId);
        event_request_header.set_event_type(eventType);

        let request = SendChatMessageRequest::new();
        request.set_request_header(request_header);
        request.set_message_content(message_content);
        request.set_event_request_header(event_request_header);

        //Prepare response
        let response = SendChatMessageResponse::new();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_auth_token() {
        use super::auth::get_auth;
        use std::io::{stdin, stdout, Write};
        use std::env;

        match env::var_os("CI") {
            #[allow(unused_variables)]
            Some(env) => println!("CI's currently not supported at this test"),
            None => {
                let mut client_email = String::new();
                println!("Please enter Google Email: ");
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut client_email)
                    .expect("Did not enter a correct Email");
                if let Some('\n') = client_email.chars().next_back() {
                    client_email.pop();
                }
                if let Some('\r') = client_email.chars().next_back() {
                    client_email.pop();
                }
                get_auth(client_email);
            }
        }
    }

    #[test]
    fn test_send_message() {
        use super::hangups::send_message;

        let test_message = String::from("TEST!");
        let test_conv = String::from("Ugxu9JRlbNPSqk5Ye1V4AaABAQ");
        send_message(test_message, test_conv);
    }
}
