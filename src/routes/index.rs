
use actix_web::{web, web::ServiceConfig, HttpRequest, HttpResponse};
use crate::db::{proxy::establish, proxy::get_events};

const HEADER : &'static str = include_str!("../html/header.html");
const FOOTER : &'static str = include_str!("../html/footer.html");
const ERROR : &'static str = include_str!("../html/error.html");
const LIMIT : u32  = 50; 

fn index(_req: HttpRequest) -> HttpResponse {
    match establish() { 
        Ok(connection) => {
            let mut events_buffer = String::new();
            let events = get_events(&connection, LIMIT);
            events_buffer.push_str(HEADER);
            events_buffer.push_str("<body>");
            for e in events {
                e.include_event_html(&mut events_buffer);
            }
            events_buffer.push_str("</body>");
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
    app.route("/", web::get().to(index));
}
