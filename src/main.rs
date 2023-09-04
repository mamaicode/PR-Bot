use lazy_static_include::*;

use reqwest::Client;
use lambda_runtime::Error;
use lambda_http::{service_fn, IntoResponse, Request};

lazy_static_include_str! {
  /// TEMPLATE
  TEMPLATE => "template.json",
}

const TEAM_URL: &str = "YOUR WEBHOOK URL";

async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
  let event_type = request
    .headers()
    .get("X-Event-Key")
    .ok_or(Error::from("NOT HEADER FOUND"))?
    .to_str()
    .map_err(Error::from)?;

  if event_type != "pullrequest:created" {
    return Ok("Not a pullrequest:created event");
  }

  let body = request.body().as_ref();

  let payload = serde_json::from_slice::<serde_json::Value>(body)?;
  // let team_url = env::var("TEAM_URL").map_err(Error::from)?;

  println!("Creating template !");

  let template = liquid::ParserBuilder::with_stdlib()
    .build()
    .map_err(Error::from)?
    .parse(&TEMPLATE)
    .map_err(Error::from)?;

  println!("Rendering template !");

  let obj = liquid::object!({ "data": payload });
  let team_payload = template.render(&obj).map_err(Error::from)?;

  let team_payload = serde_json::from_str::<serde_json::Value>(&team_payload)?;

  println!("Sending notification !");
  let client = Client::new();
  let res = client
    .post(TEAM_URL)
    .json(&team_payload)
    .send()
    .await
    .map_err(Error::from)?;

  println!("Notification sent! {res:#?}");

  match res.status().is_success() {
    false => {
      println!("Error sending notification: {}", res.status());
      Err(Error::from(format!(
        "Error sending notification: {}",
        res.status()
      )))
    }
    true => {
      println!("Notification sent to Microsoft Teams");
      Ok("Notification sent to Microsoft Teams")
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  println!("Starting lambda !");
  let func = service_fn(handler);
  lambda_http::run(func).await?;
  Ok(())
}
