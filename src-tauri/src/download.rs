use reqwest::Client;

pub async fn download(url: String) -> Result<String, String> {
    let res = Client::new()
        .get(url)
        .header("user-agent","Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|_| "网络错误").unwrap();

    let res_len = res.content_length();

    println!("res_len:{:?}", res);

    Ok(String::from(""))
}
