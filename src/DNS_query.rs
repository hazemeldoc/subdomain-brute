use std::str::FromStr;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_proto::op::response_code::ResponseCode;

use crate::race_cond;
use crate::ENT;

pub async fn DNS_q(mut client: AsyncClient, subdomain: String) {
    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str(&subdomain).unwrap();
    let query = client.query(name, DNSClass::IN, RecordType::A);
    let response = query.await.unwrap();
    let answers = response.response_code();
    let answer2 = response.answers();
    if answers != ResponseCode::NXDomain {
        if answer2.len() == 0 {
            unsafe {
                while race_cond != 1 {
                    //println!("inrace");
                    race_cond = 1;
                    ENT.push(subdomain.clone());
                    //println!("pushed");
                    race_cond = 0;
                    //println!("exit race");
                    break;
                }
            }
        } else {
            println!("{}", subdomain);
        }
    }
}
