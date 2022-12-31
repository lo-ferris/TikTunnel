
/*
Plan 0

- Visiting the website, and accepting the cookie (also saving it)

- making requests to the backend instead from scraping from the HTML!

- 

ways to overcome the ip/request  blocking:
- proxies (a lot)
- tor nodes (slow :/)
- vpns

- randomizing header & queryy/param data.
    like:
        - User-Agent
        - Cookie
        - count
        - device_platform
        - channel
        - language
        - tz_name
        - region


gotta randomize request between these

*/


/*
proxy stuff:
since it wouldn't be anonymous if the we sent the user the raw links of 
the response from the tiktok api

like: https://p77-sign-va-lite.tiktokcdn.com/obj/tos-maliva-p-0068/842232d3e72a4f0c8ec72388e627b02c_1669583400?x-expires=1672466400&x-signature=mfoG5%2BlIZsbwbOoixNPKrl3FnKU%3D

we gotta create a rust proxy for it that actually requests it.
sad

https://rocket.rs 
*/
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::{Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    
    let client:Client = reqwest::Client::new();
    let highway: String = String::from("https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&count=9&device_id=7181251537108059653&device_platform=web_mobile&focus_state=true&from_page=fyp&history_len=2&is_fullscreen=false&is_page_visible=true&os=android&priority_region=&referer=&region=DE&screen_height=883&screen_width=412&tz_name=Europe%2FBudapest&webcast_language=en&msToken=64EFVpph0PFaRtXb68BPf7TiffeAkxUE-6uxgAfQeBBMHeu717tNpRGaMi8J8x2aP91jLzMoKYby3kcLbOTROCAn2ehVWCoSwclPmRhznT6Yr6K9XU-ErWWCfhZ6W6ClSIJM7zK8HJdFtFbbtw==&X-Bogus=DFSzKIVOxczANeMeSkV9EGO8kJ0E&_signature=_02B4Z6wo00001GjktBwAAIDBdKcKo9ZSJwxo9bCAAHmb35");
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent",  String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0").parse().unwrap());

  
	
    let response = client.get(highway)
    .headers(headers)
    .send()
    .await?
    .text()
    .await?;
    
    
    // spent like 2 hours figuring this out btw
    // i mean how to convert a string to json
    let data: Value = serde_json::from_str(&response.as_str())?;


    println!("{}",data["itemList"][0]);

/* 
    let response_data: Value = serde_json::from_str(&response)?; */


/*     println!("{:?}", response_data["itemList"]); */

    Ok(())

}