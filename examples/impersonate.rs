use reqwest_impersonate as reqwest;

fn main() {
    // Build a client to mimic OkHttpAndroid13
    let client = reqwest::blocking::Client::builder()
        .impersonate(reqwest::impersonate::Impersonate::Chrome116)
        .proxy(reqwest::Proxy::all("socks5://192.168.1.1:1080").unwrap())
        // .enable_ech_grease(true)
        .permute_extensions(true)
        .cookie_store(true)
        .tls_info(true)
        .build()
        .unwrap();

    // Use the API you're already familiar with
    // https://tls.peet.ws/api/all
    // https://chat.openai.com/backend-api/models
    // https://chat.openai.com/backend-api/conversation
    // https://order.surfshark.com/api/v1/account/users?source=surfshark
    match client.get("https://tls.peet.ws/api/all").send() {
        Ok(res) => {
            println!("{}", res.text().unwrap());
        }
        Err(err) => {
            dbg!(err);
        }
    };

    match client.get("https://chat.openai.com/api/auth/csrf").send() {
        Ok(res) => {
            println!("{}", res.text().unwrap());
        }
        Err(err) => {
            dbg!(err);
        }
    };
}
