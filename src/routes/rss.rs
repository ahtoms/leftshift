
use actix_web::{web, web::ServiceConfig, HttpRequest, HttpResponse};
use crate::db::{proxy::establish, proxy::get_events};

const HEADER : &'static str = "<rss version=\"2.0\"><channel>
    <title>Leftshift</title>
    <link>https://leftshift.xyz</link>
    <description>Some CS stuff</description>
    <language>en-au</language>
    ";
    
const FOOTER : &'static str = "</channel></rss>";
const ERROR : &'static str = "<error>Cannot Retrieve Items</error>";
const LIMIT : u32 = 50;


fn news(_req: HttpRequest) -> HttpResponse {
     match establish() { 
        Ok(connection) => {
            let mut events_buffer = String::new();
            let events = get_events(&connection, LIMIT);
            events_buffer.push_str(HEADER);
            for e in events {
                e.include_event_xml(&mut events_buffer);
            }
            events_buffer.push_str(FOOTER);
            HttpResponse::Ok().body(events_buffer)
        },
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::Ok().body(ERROR)
        }
    }
}

pub fn reg(app: &mut ServiceConfig) {
    app.route("/rss", web::get().to(news));
}
