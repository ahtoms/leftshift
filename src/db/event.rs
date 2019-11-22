
use chrono::{DateTime, TimeZone, Local, Utc};

pub struct Event {
    id: u32,
    title: String,
    description: String,
    location: String,
    img: String,
    tstamp: i64,
    pub_tstamp: i64
}

impl Event {
    
    pub fn new(id: u32, title: String, description: String, 
        location: String, img: String, tstamp: i64, pub_tstamp: i64) -> Self {

        Event { 
            id : id,
            title : title,
            description : description,
            location : location,
            img: img,
            tstamp : tstamp,
            pub_tstamp : pub_tstamp
        }
    }

    pub fn include_event_html(&self, str_buf: &mut String) {
        let entry = format!("<div class='entry'><header class='entry_header'>{}</header><div class='entry_img'>{}</div>
            <article class='entry_description'>{}... at {}</article><div class='entry_datetime'>{}</div></div>",
            self.title, self.img, self.description, self.location,
            DateTime::<Local>::from(Utc.timestamp(self.tstamp, 0)).format("%Y-%m-%d %H:%M"));
        str_buf.push_str(entry.as_str());
    }

    pub fn include_event_xml(&self, str_buf: &mut String) {
        let entry = format!("<item><title>{}</title><link>{}</link><guid>{}</guid><desciption>{}... On {}
            </description><pubDate>{}</pubDate></item>",
            self.title, "/", self.id, self.description, 
            DateTime::<Local>::from(Utc.timestamp(self.tstamp, 0)).format("%Y-%m-%d %H:%M"),
            DateTime::<Local>::from(Utc.timestamp(self.pub_tstamp, 0)).format("%Y-%m-%d %H:%M"));
        str_buf.push_str(entry.as_str());
    }

    

}
