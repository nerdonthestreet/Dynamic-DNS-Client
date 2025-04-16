// This application is meant to run on a Dynamic DNS client.
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Settings
  let linode_record_url = String::from("https://api.linode.com/v4/domains/YOUR-ACCOUNT/records/YOUR-RECORD");
  let linode_api_token = String::from("YOUR-TOKEN");
  let domain_name = String::from("YOURDOMAIN.EXAMPLE.COM");

  // Get current public IP address
  let ip_address = reqwest::get("https://api.ipify.org").await?.text().await?;
  
  // Get current DNS record
  let linode_client = Client::new();
  let get_current_dns_record = linode_client.get(&linode_record_url)
      .bearer_auth(&linode_api_token)
      .send().await?.text().await?;
  let current_dns_record: serde_json::Value = serde_json::from_str(&get_current_dns_record).expect("Linode API response was not valid JSON");
  let currently_recorded_address = current_dns_record["target"].as_str().unwrap();
  
  println!("Currently recorded address: {}", &currently_recorded_address);

  if &currently_recorded_address != &ip_address {
    println!("IP address changed. Updating DNS record to: {}", &ip_address);
    
    // Update Linode DNS record  
    let request_body = json!({"name": domain_name, "target": ip_address});
    let update_dns_record = linode_client.put(&linode_record_url)
                                         .bearer_auth(&linode_api_token)
                                         .json(&request_body)
                                         .send().await?;
    
    println!("Result status: {}", update_dns_record.status());
  } else {
    println!("IP address has not changed.");
  }
  
  Ok(())
}
