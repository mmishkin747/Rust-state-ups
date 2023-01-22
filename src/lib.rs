

use clap::Parser;
use json::JsonValue;
use std::error::Error;
use std::io::{BufReader, LineWriter, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::ops::RangeInclusive;
use std::collections::HashMap;
use std::time::Duration;


type MyResult<T> = Result<T, Box<dyn Error>>;




#[derive(Parser)]
#[command(name = "Rups")]
#[command(author = "Mikhail V. <mmishkin747@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Rust check state UPS")]
struct Cli {
    /// User's name for connecting ups
    #[arg(short, long)]
    user: Option<String>,
    /// Password for connecting ups
    #[arg(short, long)]
    password: Option<String>,
    /// Network ipv4 address server
    address_server: IpAddr,
    /// Network port to use
    #[arg( long, value_parser = port_in_range, default_value_t = 2001)]
    port: u16,
}
#[derive(Debug)]
pub struct Config {
    addr_server: SocketAddr,
    user: String,
    passw: String,
    commands: HashMap<String, String>
}



#[derive(Debug)]
pub struct Connecter {
    writer: LineWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}
impl Connecter {

    pub fn new(config: &Config) -> MyResult<Self> {
        let stream = TcpStream::connect_timeout(&config.addr_server, Duration::from_secs(2))?;
        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
        //stream.set_write_timeout(Some(config.timeout)).unwrap();
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_mes(&mut self, message: &str) -> MyResult<()> {

        let mes = message.to_string() + &"\r\n".to_string();
        self.writer
            .write_all(&mes.as_bytes())
            .expect("didn't send messg");
        Ok(())
    }

    pub fn read_mes(&mut self) -> MyResult<String> {  
        let mut buf = Vec::new();
        _ = self.reader.read_to_end(&mut buf);
        let res = String::from_utf8_lossy(&buf);
        Ok(res.to_string())
    }
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    let addr_server = SocketAddr::new(cli.address_server, cli.port);

    let mut user = String::new();
    let mut passw = String::new();
    if let Some(ref user_v) = cli.user {
        if let Some(ref passw_v) = cli.password {
            user = user_v.to_string();
            passw = passw_v.to_string();
        }
    }

    let commands = HashMap::from([
        ("main_voltage".to_string(), "O".to_string()),
        ("load".to_string(), "P".to_string()),
        ("temperature".to_string(), "C".to_string()),
        ("charge_battaries".to_string(), "0".to_string()),
        ("workin_hour".to_string(), "j".to_string()),
    ]);


    Ok(Config {
        addr_server,
        user,
        passw,
        commands,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(&config);
    let mut connect = Connecter::new(&config)?;
    auth(&mut connect, &config.user, &config.passw)?;
    let state = send_commands(&mut connect, &config.commands)?;
    
    println!("{}", state);


    
    
    Ok(())
}

fn auth (connect :&mut Connecter, user: &String, passw: &String) -> MyResult<()>{
    println!("auth");
    let check_auth = connect.read_mes()?;
    println!("auth1");
    if check_auth.contains("Username:") {
        connect.send_mes(user.as_str())?;
        connect.send_mes(passw.as_str())?;
    }
    Ok(())
}

fn send_commands(connect: &mut Connecter, commands: &HashMap<String, String>) -> MyResult<JsonValue> {
    println!("senf");

    let mut data = json::JsonValue::new_object();
    for (name, command) in commands{
            connect.send_mes(command)?;
            data[name] = connect.read_mes()?.as_str().into();
    } 
    
    Ok(data)
}




/// This func check valid number port
fn port_in_range(s: &str) -> Result<u16, String> {
    let port_range: RangeInclusive<usize> = 1..=65535;
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;
    if port_range.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "Port not in range {}-{}",
            port_range.start(),
            port_range.end()
        ))
    }
}




// #[macro_use]
// extern crate json;

// fn main() {
//     let parsed = json::parse(r#"

// {
//     "code": 200,
//     "success": true,
//     "payload": {
//         "features": [
//             "awesome",
//             "easyAPI",
//             "lowLearningCurve"
//         ]
//     }
// }

// "#).unwrap();

// let instantiated = object!{
//     // quotes on keys are optional
//     "code": 200,
//     success: true,
//     payload: {
//         features: [
//             "awesome",
//             "easyAPI",
//             "lowLearningCurve"
//         ]
//     }
// };

// assert_eq!(parsed, instantiated);
// println!("{}", instantiated);
// }
