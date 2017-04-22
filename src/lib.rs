#[macro_use]
extern crate hyper;
extern crate protobuf;
extern crate rand;
extern crate sha1;
extern crate rustc_serialize;
extern crate jconfig;
extern crate regex;
extern crate url;
extern crate tokio_core;
extern crate futures;

pub mod auth {
    use hyper::{Client, Body, Uri};
    use hyper::client::{Response, HttpConnector};
    use tokio_core::reactor::Core;
    use hyper::header::Headers;
    fn do_auth_step(client: &Client<HttpConnector, Body>, url: Uri, resp: &Response, resp_body: &str, email: String, password: String, mut resp_headers_raw: &Headers) {
        use hyper::header::{Headers, SetCookie, UserAgent};
        use hyper::client::Request;
        use hyper::{Method, Uri};
        use regex::Regex;
        use std::str;
        use url::form_urlencoded;
        use std::str::FromStr;
        use futures::Future;
        use futures::Stream;
        use tokio_core::reactor::Core;

        let mut core = Core::new().unwrap();
        header! { (ContentType, "content-type") => [String] }
        header! { (GAPS, "GAPS") => [String] }
        header! { (GALX, "GALX") => [String] }

        let mut req_headers = Headers::new();
        if resp_headers_raw.has::<SetCookie>() {
            let resp_headers = resp_headers_raw.get_raw("Set-Cookie").unwrap();
            let mut resp_headers_string = String::new();
            for elem in resp_headers.iter() {
                let s = match str::from_utf8(elem) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                resp_headers_string.push_str(s);
            }
            let resp_cookie_gaps_re = Regex::new("(?:GAPS=)(.*?)(?:;)").unwrap();
            let resp_cookie_gaps_captures = resp_cookie_gaps_re.captures(resp_headers_string.as_str()).unwrap();
            let resp_cookie_gaps = resp_cookie_gaps_captures.get(1).map_or("", |m| m.as_str());

            let resp_cookie_galx_re = Regex::new("(?:GALX=)(.*?)(?:;)").unwrap();
            let resp_cookie_galx_captures = resp_cookie_galx_re.captures(resp_headers_string.as_str()).unwrap();
            let resp_cookie_galx = resp_cookie_galx_captures.get(1).map_or("", |m| m.as_str());

            if resp_cookie_gaps != "" {
                req_headers.set(GAPS(resp_cookie_gaps.to_owned()));
            }
            if resp_cookie_galx != "" {
                req_headers.set(GALX(resp_cookie_galx.to_owned()));
            }
        }

        let resp_page_re = Regex::new("(?:<input.*name=\"Page\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_page_captures = resp_page_re.captures(resp_body).unwrap();
        let resp_page = resp_page_captures.get(1).map_or("", |m| m.as_str());

        let resp_galx_re = Regex::new("(?:<input.*name=\"GALX\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_galx_captures = resp_galx_re.captures(resp_body).unwrap();
        let resp_galx = resp_galx_captures.get(1).map_or("", |m| m.as_str());

        let resp_gxf_re = Regex::new("(?:<input.*name=\"gxf\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_gxf_captures = resp_gxf_re.captures(resp_body).unwrap();
        let resp_gxf = resp_gxf_captures.get(1).map_or("", |m| m.as_str());

        let resp_continue_re = Regex::new("(?:<input.*name=\"continue\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_continue_captures = resp_continue_re.captures(resp_body).unwrap();
        let resp_gcontinue = resp_continue_captures.get(1).map_or("", |m| m.as_str());

        let resp_ltmpl_re = Regex::new("(?:<input.*name=\"ltmpl\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_ltmpl_captures = resp_ltmpl_re.captures(resp_body).unwrap();
        let resp_ltmpl = resp_ltmpl_captures.get(1).map_or("", |m| m.as_str());

        let resp_scc_re = Regex::new("(?:<input.*name=\"scc\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_scc_captures = resp_scc_re.captures(resp_body).unwrap();
        let resp_scc = resp_scc_captures.get(1).map_or("", |m| m.as_str());

        let resp_sarp_re = Regex::new("(?:<input.*name=\"sarp\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_sarp_captures = resp_sarp_re.captures(resp_body).unwrap();
        let resp_sarp = resp_sarp_captures.get(1).map_or("", |m| m.as_str());

        let resp_oauth_re = Regex::new("(?:<input.*name=\"oresp\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_oauth_captures = resp_oauth_re.captures(resp_body).unwrap();
        let resp_oauth = resp_oauth_captures.get(1).map_or("", |m| m.as_str());

        let resp_profile_information_re = Regex::new("(?:<input.*name=\"ProfileInformation\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_profile_information_captures = resp_profile_information_re.captures(resp_body).unwrap();
        let resp_profile_information = resp_profile_information_captures.get(1).map_or("", |m| m.as_str());

        let resp_session_state_re = Regex::new("(?:<input.*name=\"SessionState\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_session_state_captures = resp_session_state_re.captures(resp_body).unwrap();
        let resp_session_state = resp_session_state_captures.get(1).map_or("", |m| m.as_str());

        let resp__utf8_re = Regex::new("(?:<input.*name=\"_utf8\".*)(?:value=\"([^\"]*))").unwrap();
        let resp__utf8_captures = resp__utf8_re.captures(resp_body).unwrap();
        let resp__utf8 = resp__utf8_captures.get(1).map_or("", |m| m.as_str());

        let resp_bgresponse_re = Regex::new("(?:<input.*name=\"bgresponse\".*)(?:value=\"([^\"]*))").unwrap();
        let resp_bgresponse_captures = resp_bgresponse_re.captures(resp_body).unwrap();
        let resp_bgresponse = resp_bgresponse_captures.get(1).map_or("", |m| m.as_str());

        let body_send: String = form_urlencoded::Serializer::new(String::new())
                              .append_pair("Page", resp_page)
                              .append_pair("GALX", resp_galx)
                              .append_pair("gxf", resp_gxf)
                              .append_pair("continue", resp_gcontinue.replace("&amp;", "&").as_str())
                              .append_pair("ltmpl", resp_ltmpl)
                              .append_pair("scc", resp_scc)
                              .append_pair("sarp", resp_sarp)
                              .append_pair("oauth", resp_oauth)
                              .append_pair("ProfileInformation", resp_profile_information)
                              .append_pair("SessionState", resp_session_state)
                              .append_pair("_utf8", resp__utf8)
                              .append_pair("bgresponse", resp_bgresponse)
                              .append_pair("Email", email.as_str())
                              .append_pair("Passwd", password.as_str())
                              .append_pair("pstMsg", "0")
                              .append_pair("checkConnection", "")
                              .append_pair("checkedDomains", "youtube")
                              .append_pair("signIn", "Anmelden")
                              .append_pair("PersistentCookie", "no")
                              .finish();

        // Do Email Login
        let mut body = String::new();

        let mut fresh_request = Request::new(Method::Post, url);
        fresh_request.headers_mut().set(ContentType(String::from("application/x-www-form-urlencoded".to_owned())));
        fresh_request.headers_mut().set(UserAgent::new("Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2228.0 Safari/537.36".to_owned()));
        fresh_request.set_body(body_send);
        let mut post = client.request(fresh_request).map_err(|_| ()).and_then(|res| {
            println!("Response: {}", &res.status());
            println!("Headers: \n{}", &res.headers());

            &res.body().for_each(|chunk| {
                let body_chunk = String::from_utf8(chunk.to_vec()).unwrap();
                body.push_str(&body_chunk.as_str());
                Ok(())
            });
            Ok(())
        })
        .map(|_| {
            println!("\n\nDone.");
        });
        &core.run(post).unwrap();
        // let mut email_post = client.post(url)
        //                     .body(email_body_send.as_str())
        //                     .headers(req_headers)
        //                     .unwrap();

        // email_post.read_to_string(&mut email_body)
        //           .unwrap();
    }

    fn write_cookie_config() {
        //  let mut config = get_configs().unwrap();
         //
        //  config.set::<SAPISID>().unwrap();
        //  config.set::<HSID>().unwrap();
        //  config.set::<SSID>().unwrap();
        //  config.set::<SAPISID>().unwrap();
        //  config.set::<SAPISID>().unwrap();
        //  config.set_raw("SAPISID", )
        //  config.set_raw("HSID", )
        //  config.set_raw("SSID", )
        //  config.set_raw("APISID", )
        //  config.set_raw("SID", )
    }

    pub fn get_auth(mut client_email: String, mut client_passwd: String) {
        use hyper::Client;
        use hyper::header::{Headers, SetCookie, UserAgent};
        use hyper::client::Request;
        use hyper::{Method, Uri};
        use std::io::{stdin, stdout, Write, Read};
        use std::str;
        use url::form_urlencoded;
        use regex::Regex;
        use tokio_core::reactor::Core;
        use std::str::FromStr;
        use futures::Future;
        use futures::Stream;

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
                    .keep_alive(true)
                    .build(&handle);

        let auth_uri = "https://accounts.google.com/o/oauth2/programmatic_auth?scope=https://www.google.com/accounts/OAuthLogin+https://www.googleapis.com/auth/userinfo.email&client_id=936475272427.apps.googleusercontent.com".parse::<Uri>().unwrap();
        let email_uri = "https://accounts.google.com/signin/v1/lookup".parse::<Uri>().unwrap();
        let passwd_uri = "https://accounts.google.com/signin/challenge/sl/password".parse::<Uri>().unwrap();

        if client_email == String::from("") {
            println!("");
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

        if client_passwd == String::from("") {
            println!("Please enter Google Password: ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut client_passwd)
                .expect("Did not enter a correct Password");
            if let Some('\n') = client_passwd.chars().next_back() {
                client_passwd.pop();
            }
            if let Some('\r') = client_passwd.chars().next_back() {
                client_passwd.pop();
            }
        }
        println!("You typed: {}", client_passwd);

        // Login Step1
        let req = client.get(auth_uri).and_then(|res| {
                    do_auth_step(&client, email_uri, &res, &format!("{:?}", &res.body()).as_str(), client_email, client_passwd, &res.headers());
                    Ok(())
                })
                .map(|_| {
                    println!("\n\nDone.");
                });

        core.run(req).unwrap();
    }
}

pub mod hangups;

pub mod helper{
    use jconfig::config::Config;
    use jconfig::error::Error;

    fn check_generate_configs() {
        use std::fs::{File, create_dir_all};
        use std::env;
        use std::io::Write;
        // TODO Add handling if there is no home dir.
        // match std::env::home_dir() {
        //     Some(dir) => {
        //         //There is a home directory, do something with the dir variable
        //     },
        //     None => {
        //         //There is no home directory, do something else
        //     }
        // }

        //Memory File
        let mut memory_path = env::home_dir().unwrap();
        memory_path.push(".local/share/hangups/");
        create_dir_all(memory_path.as_path()).unwrap();
        memory_path.push("memory.json");
        let mut f = File::create(memory_path.as_path()).unwrap();
        f.write_all(b"{}").unwrap();
        f.sync_all().unwrap();

        //Config File
        let mut config_path = env::home_dir().unwrap();
        config_path.push(".local/share/hangups/");
        create_dir_all(config_path.as_path()).unwrap();
        config_path.push("config.json");
        let mut f = File::create(config_path.as_path()).unwrap();
        f.write_all(b"{}").unwrap();
        f.sync_all().unwrap();
    }
    pub fn get_configs() -> Result<Config, Error>{
        use std::env;

        check_generate_configs();

        let mut memory_path = env::home_dir().unwrap();
        memory_path.push(".local/share/hangups/");
        memory_path.push("memory.json");

        // Load the memory
        let config = Config::load(memory_path.as_path()).unwrap();

        Ok(config)
    }
}
