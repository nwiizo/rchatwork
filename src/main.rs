use std::process::Command;

fn main() {
    let output = Command::new("curl")
        .arg("-X POST")
        .arg("-H")
        .arg("X-Chatworktoken: $token")
        .arg("-d")
        .arg("body=$data")
        .arg("https://api.chatwork.com/v2/rooms/$room/messages")
        .env("token","*****************************")
        .env("room","*******")
        .env("data","sample")
        .output()
        .expect("curl command failed to start");
    
    let hello = output.stdout;
    println!("{}", std::str::from_utf8(&hello).unwrap());
}
