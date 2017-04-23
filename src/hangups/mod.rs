#[path = "proto/hangouts.rs"] mod hangouts;

pub mod hangups {
    use hangups::hangouts::{MessageContent, Segment, SegmentType, RequestHeader,
                            EventRequestHeader, SendChatMessageRequest, DeliveryMediumType,
                            OffTheRecordStatus, EventType, SendChatMessageResponse,
                            DeliveryMedium, ConversationId};
    use protobuf::RepeatedField;
    use rand::random;
    use hyper::header::Cookie;
    use hyper::client::Request;
    use hyper::{Method, Uri};
    use hyper::error::Error;
    use protobuf::Message;
    use std::collections::HashMap;
    use std::time::*;
    use sha1::Sha1;
    use rustc_serialize::hex::ToHex;
    use rustc_serialize::base64::FromBase64;
    use protobuf::core::parse_from_bytes;
    use auth::get_auth;
    use helper::get_configs;
    use std;

    header! { (ContentType, "content-type") => [String] }
    header! {
              (XGoogEncodeResponseIfExecutable, "X-Goog-Encode-Response-If-Executable") => [String]
            }

    fn get_cookies() -> Result<HashMap<String, String>, Error> {
        let config = get_configs().unwrap();
        if (config.get_raw("SAPISID") == None) || (config.get_raw("HSID") == None) || (config.get_raw("SSID") == None) || (config.get_raw("APISID") == None) || (config.get_raw("SID") == None) {
            get_auth(String::from(""), String::from(""));
        }

        let sapisid_cookie = String::from(config.get_raw("SAPISID").unwrap().as_string().unwrap());
        let hsid_cookie = String::from(config.get_raw("HSID").unwrap().as_string().unwrap());
        let ssid_cookie = String::from(config.get_raw("SSID").unwrap().as_string().unwrap());
        let apisid_cookie = String::from(config.get_raw("APISID").unwrap().as_string().unwrap());
        let sid_cookie = String::from(config.get_raw("SID").unwrap().as_string().unwrap());

        let mut cookies = HashMap::new();
        cookies.insert(String::from("SAPISID"), sapisid_cookie);
        cookies.insert(String::from("HSID"), hsid_cookie);
        cookies.insert(String::from("SSID"), ssid_cookie);
        cookies.insert(String::from("APISID"), apisid_cookie);
        cookies.insert(String::from("SID"), sid_cookie);
        Ok(cookies)
    }

    fn get_auth_headers(sapisid: String) -> Result<HashMap<String, String>, Error> {
        let origin_url = String::from("https://talkgadget.google.com");
        let now = SystemTime::now();
        let dur = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp_msec = dur.as_secs();


        let auth_string = String::from(format!("{} {} {}", timestamp_msec, sapisid, origin_url));
        let mut hash = Sha1::new();
        hash.update(auth_string.as_bytes());
        let hash_bytes = hash.digest().bytes();
        let hex_sha1 = hash_bytes.to_hex();
        let sapisid_hash = format!("SAPISIDHASH {}_{}", timestamp_msec, hex_sha1);
        let mut auth_cookies = HashMap::new();
        auth_cookies.insert(String::from("authorization"), sapisid_hash);
        auth_cookies.insert(String::from("x-origin"), origin_url);
        auth_cookies.insert(String::from("x-goog-authuser"), String::from("0"));
        Ok(auth_cookies)
    }

    fn api_request(endpoint_url: String,
                   content_type: String,
                   response_type: String,
                   payload: std::vec::Vec<u8>)
                   -> Result<String, Error> {
       use hyper::Client;
       use tokio_core::reactor::Core;
       use futures::Future;

       let mut core = Core::new().unwrap();
       let handle = core.handle();
       let client = Client::configure()
                   .keep_alive(true)
                   .build(&handle);
        let url = format!("{}?key=AIzaSyAfFJCeph-euFSwtmqFZi0kaKk-cZ5wufM&alt={}",
                          endpoint_url,
                          response_type).parse::<Uri>().unwrap();;
        let auth_header = get_auth_headers(String::from("PLACEHOLDER"))
            .unwrap();
        let mut fresh_request = Request::new(Method::Post, url);
        let cookies = get_cookies().unwrap();
        let mut cookie_vec = Vec::new();
        for (key, val) in cookies {
            cookie_vec.push(format!("{}={}", key, val))
        }
        fresh_request.headers_mut().set(Cookie(cookie_vec));
        fresh_request.headers_mut().set(ContentType(String::from(content_type.to_owned())));
        fresh_request.headers_mut().set(XGoogEncodeResponseIfExecutable(String::from("base64".to_owned())));
        fresh_request.set_body(payload);
        for (key, val) in auth_header {
            fresh_request.headers_mut().append_raw(key, val.as_bytes().to_vec());
        }
        let body = String::new();

        let post = client.request(fresh_request)
                        .map(|res| {
                            let body = res.body();
                            println!("\n\nDone.");
                            body
                        });

        core.run(post).unwrap();

        Ok(body)
    }

    fn protobuf_api_request(api_endpoint: String,
                            request_struct: SendChatMessageRequest)
                            -> Result<SendChatMessageResponse, Error> {
        let url = format!("https://clients6.google.com/chat/v1/{}", api_endpoint);
        let payload = request_struct.write_to_bytes().unwrap();
        let output = api_request(url,
                                 String::from("application/x-protobuf"),
                                 String::from("proto"),
                                 payload);
        let decoded_output = output.unwrap().as_str().from_base64().unwrap();
        let response_proto = parse_from_bytes::<SendChatMessageResponse>(&decoded_output[..])
            .unwrap();
        Ok(response_proto)
    }

    pub fn send_message(message: String,
                        conv_id_raw: String)
                        -> Result<SendChatMessageResponse, Error> {
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
        let mut request_header = RequestHeader::new();
        request_header.set_language_code(String::from("en"));

        let mut delivery_medium = DeliveryMedium::new();
        delivery_medium.set_medium_type(DeliveryMediumType::DELIVERY_MEDIUM_BABEL);
        let expected_otr = OffTheRecordStatus::OFF_THE_RECORD_STATUS_ON_THE_RECORD;
        let client_generated_id = random::<u64>();
        let event_type = EventType::EVENT_TYPE_REGULAR_CHAT_MESSAGE;
        let mut event_request_header = EventRequestHeader::new();
        let mut conv_id = ConversationId::new();
        conv_id.set_id(conv_id_raw);
        event_request_header.set_conversation_id(conv_id);
        event_request_header.set_delivery_medium(delivery_medium);
        event_request_header.set_expected_otr(expected_otr);
        event_request_header.set_client_generated_id(client_generated_id);
        event_request_header.set_event_type(event_type);

        let mut request = SendChatMessageRequest::new();
        request.set_request_header(request_header);
        request.set_message_content(message_content);
        request.set_event_request_header(event_request_header);

        let output = protobuf_api_request(String::from("conversations/sendchatmessage"), request)
            .unwrap();
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_send_message() {
        use super::hangups::send_message;

        let test_message = String::from("Diese Nachricht sendet der Bot!");
        let test_conv = String::from("Ugxu9JRlbNPSqk5Ye1V4AaABAQ");
        send_message(test_message, test_conv);
    }
}
