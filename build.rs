use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    if let Ok(auth) = std::env::var("AUTH") {
        let resp = client
        .get(
            "https://api.collegefootballdata.com/games?year=2022&seasonType=regular&conference=PAC",
        )
        .header(
            "Authorization",
            format!(
                "Bearer {auth}",
            ),
        )
        .send()
        .expect("response should be successful");
        std::fs::write(
            "pac.json",
            resp.text().expect("response should have a body"),
        )
        .expect("write should be successful");
    }
}
