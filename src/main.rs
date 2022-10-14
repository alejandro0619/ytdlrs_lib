// Import fetch.rs
use ytdlrs_lib::api::fetch;

// tokio runtime
#[tokio::main]
async fn main(){
  let resp = fetch::request(&String::from("https://www.youtube.com/watch?v=EfeOL6RD9zI")).await;

  // handle the result
  match resp {
    Ok(res) => {
      println!("{:#?}", res);
    }
    Err(e) => {
      println!("Error: {}", e);
    }
  }
}