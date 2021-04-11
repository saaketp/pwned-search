use sha1::{Digest, Sha1};
use std::env;
use std::error::Error;
use std::io::{self, BufRead};

async fn check(password: &str) -> Result<(String, u32), Box<dyn Error>> {
    let hash = format!("{:X}", Sha1::digest(password.as_bytes()));
    let (head, tail) = (&hash[0..5], &hash[5..]);
    let url = format!("https://api.pwnedpasswords.com/range/{}", head);
    let resp = reqwest::get(url).await?.text().await?;
    let mut times: u32 = 0;
    for line in resp.lines() {
        let mut line = line.split(":");
        if line.next().unwrap() == tail {
            times = line.next().unwrap().parse().unwrap();
            break;
        }
    }
    Ok((hash, times))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut passwords: Vec<String> = env::args().skip(1).collect();
    if passwords.len() == 0 {
        passwords = io::stdin().lock().lines().map(|s| s.unwrap()).collect();
    } else {
        println!(concat!(
            "Warning: passwords provided though command line arguments are probably saved in shell history.\n",
            "Don't forget to clear shell history if you passed important passwords here."
        ))
    }
    for password in passwords {
        let (hash, times) = check(&password).await?;
        if times > 0 {
            println!("{} was found {} times (hash: {})", password, times, hash);
        } else {
            println!("{} was not found (hash: {})", password, hash);
        }
    }
    Ok(())
}
