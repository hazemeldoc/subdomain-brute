use clap::{App, Arg};
use futures::{stream, StreamExt};
use tokio::net::TcpStream as TokioTcpStream;
use trust_dns_client::client::AsyncClient;
use trust_dns_client::proto::iocompat::AsyncIoTokioAsStd;
use trust_dns_client::tcp::TcpClientStream;

mod dns_query;
mod file_read;

static mut ENT: Vec<String> = vec![];
static mut race_cond: i8 = 0;

#[tokio::main]
async fn main() {
    let param = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("domain")
                .help("domain to preform bruteforce on")
                .short("d")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("20"),
        )
        .arg(
            Arg::with_name("wordlist")
                .help("wordlist path")
                .long("wordlist")
                .short("w")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .required(false)
                .takes_value(false)
                .short("r")
                .help("bruteforce on ENT entities"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let concurrency = param
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(20);
    let domain = param.value_of("domain").unwrap();
    let wordlist = param.value_of("wordlist").unwrap();

    let mut queue_: Vec<String> = vec![];
    file_read::read_file_line_by_line(wordlist, &mut queue_, domain);

    /*
    let (stream, sender) =
    TcpClientStream::<AsyncIoTokioAsStd<TokioTcpStream>>::new(([8, 8, 8, 8], 53).into());
    let client = AsyncClient::new(stream, sender, None);
    let (mut client, bg) = client.await.expect("connection failed");
    tokio::spawn(bg);
    //9.9.9.9
    //109.228.47.218
     */
    let (stream, sender) =
        TcpClientStream::<AsyncIoTokioAsStd<TokioTcpStream>>::new(([1, 1, 1, 1], 53).into());
    let client2 = AsyncClient::new(stream, sender, None);
    let (client2, bg2) = client2.await.expect("connection failed");
    tokio::spawn(bg2);

    let q = stream::iter(queue_.clone());

    q.for_each_concurrent(concurrency, |subdomain| {
        dns_query::dns_q(client2.clone(), subdomain.to_string())
    })
    .await;

    unsafe {
        if race_cond == 0 {
            println!("--------------------------------------------ENT------------------------------------------------");
            race_cond = 1;
            for _i in 0..ENT.len() {
                let ent = ENT.pop().unwrap();
                println!("{}", ent);
                if param.is_present("recursive") {
                    queue_ = vec![];
                    file_read::read_file_line_by_line(wordlist, &mut queue_, &ent);
                    let q = stream::iter(queue_.clone());
                    q.for_each_concurrent(concurrency, |subdomain| {
                        dns_query::dns_q(client2.clone(), subdomain.to_string())
                    })
                    .await;
                }
            }
            race_cond = 0;
        }
    }
}
