#[path = "proto/hangouts.rs"] mod hangouts;

pub mod hangups {
    use hangups::hangouts::{MessageContent, Segment, SegmentType, RequestHeader, EventRequestHeader,
                         SendChatMessageRequest, DeliveryMediumType, OffTheRecordStatus,
                         EventType, SendChatMessageResponse, DeliveryMedium, ConversationId};
    use protobuf::RepeatedField;
    use rand::random;
    use hyper::header::Headers;
    use hyper::header::Cookie;
    use hyper::client::Client;
    header! { (ContentType, "content-type") => [String] }
    header! {
              (XGoogEncodeResponseIfExecutable, "X-Goog-Encode-Response-If-Executable") => [String]
            }

    fn get_auth_headers() {}

    fn api_request(endpoint_url: String,
                   content_type: String,
                   cookies: String,
                   payload: SendChatMessageRequest) {
        let mut headers = Headers::new();
        headers.set(Cookie(vec![String::from(cookies.to_owned())]));
        headers.set(ContentType(String::from(content_type.to_owned())));
        headers.set(XGoogEncodeResponseIfExecutable(String::from("base64".to_owned())));
        let client = Client::new();
        let post_request = client.post(&*endpoint_url).headers(headers);
    }

    fn protobuf_api_request(api_endpoint: String,
                            request_struct: SendChatMessageRequest,
                            response_struct: SendChatMessageResponse) {
        let url = format!("https://clients6.google.com/chat/v1/{}", api_endpoint);
        let output = api_request(url,
                                 String::from("application/x-protobuf"),
                                 String::from("proto"),
                                 request_struct);
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

        //Prepare response
        let response = SendChatMessageResponse::new();

        let err = protobuf_api_request(String::from("conversations/sendchatmessage"), request, response);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_send_message() {
        use super::hangups::send_message;

        let test_message = String::from("TEST!");
        let test_conv = String::from("Ugxu9JRlbNPSqk5Ye1V4AaABAQ");
        send_message(test_message, test_conv);
    }
}
