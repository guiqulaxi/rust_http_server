use std::net::TcpStream;
use std::io::Write;
pub struct HttpResponse {
    pub accept_ranges:String,
    pub age:String,
    pub allow:String,
    pub cache_control:String,
    pub content_encoding:String,
    pub content_language:String,
    pub content_length:String,
    pub content_location:String,
    pub content_md5:String,
    pub content_range:String,
    pub content_type:String,
    pub date:String,
    pub etag:String,
    pub expires:String,
    pub last_modified:String,
    pub location:String,
    pub pragma:String,
    pub proxy_authenticate:String,
    pub refresh:String,
    pub retry_after:String,
    pub server:String,
    pub set_cookie:String,
    pub trailer:String,
    pub transfer_encoding:String,
    pub vary:String,
    pub via:String,
    pub warning:String,
    pub www_authenticate:String,
    pub status_code: u16,
        //TcpStream
    pub stream: TcpStream,
}

impl HttpResponse {
     pub fn new(stream: TcpStream) -> HttpResponse {
          HttpResponse{
            accept_ranges:String::new(),
            age:String::new(),
            allow:String::new(),
            cache_control:String::new(),
            content_encoding:String::new(),
            content_language:String::new(),
            content_length:String::new(),
            content_location:String::new(),
            content_md5:String::new(),
            content_range:String::new(),
            content_type:String::new(),
            date:String::new(),
            etag:String::new(),
            expires:String::new(),
            last_modified:String::new(),
            location:String::new(),
            pragma:String::new(),
            proxy_authenticate:String::new(),
            refresh:String::new(),
            retry_after:String::new(),
            server:String::from("SunServer"),
            set_cookie:String::new(),
            trailer:String::new(),
            transfer_encoding:String::new(),
            vary:String::new(),
            via:String::new(),
            warning:String::new(),
            www_authenticate:String::new(),
           
            status_code: 200u16,
            stream: stream,
         }
     }
     fn build_header(&mut self) -> String {
            let mut header=String::new();
             header.push_str("HTTP/1.1 ");
             header.push_str(&(self.status_code.to_string()));
             header.push_str("\r\n");
             if !self.accept_ranges.is_empty(){
                 header.push_str("Accept_Ranges:");
                  header.push_str(&*(self.accept_ranges));
                 header.push_str("\r\n");
            }
            if !self.age.is_empty(){
                 header.push_str("Age:");
                  header.push_str(&*(self.age));
                 header.push_str("\r\n");
            }
            if !self.allow.is_empty(){
                 header.push_str("Allow:");
                  header.push_str(&*(self.allow));
                 header.push_str("\r\n");
            }
            if !self.cache_control.is_empty(){
                 header.push_str("Cache_Control:");
                  header.push_str(&*(self.cache_control));
                 header.push_str("\r\n");
            }
            if !self.content_encoding.is_empty(){
                 header.push_str("Content_Encoding:");
                  header.push_str(&*(self.content_encoding));
                 header.push_str("\r\n");
            }
            if !self.content_language.is_empty(){
                 header.push_str("Content_Language:");
                  header.push_str(&*(self.content_language));
                 header.push_str("\r\n");
            }
            if !self.content_length.is_empty(){
                 header.push_str("Content_Length:");
                  header.push_str(&*(self.content_length));
                 header.push_str("\r\n");
            }
            if !self.content_location.is_empty(){
                 header.push_str("Content_Location:");
                  header.push_str(&*(self.content_location));
                 header.push_str("\r\n");
            }
            if !self.content_md5.is_empty(){
                 header.push_str("Content_MD5:");
                  header.push_str(&*(self.content_md5));
                 header.push_str("\r\n");
            }
            if !self.content_range.is_empty(){
                 header.push_str("Content_Range:");
                  header.push_str(&*(self.content_range));
                 header.push_str("\r\n");
            }
            if !self.content_type.is_empty(){
                 header.push_str("Content_Type:");
                  header.push_str(&*(self.content_type));
                 header.push_str("\r\n");
            }
            if !self.date.is_empty(){
                 header.push_str("Date:");
                  header.push_str(&*(self.date));
                 header.push_str("\r\n");
            }
            if !self.etag.is_empty(){
                 header.push_str("ETag:");
                  header.push_str(&*(self.etag));
                 header.push_str("\r\n");
            }
            if !self.expires.is_empty(){
                 header.push_str("Expires:");
                  header.push_str(&*(self.expires));
                 header.push_str("\r\n");
            }
            if !self.last_modified.is_empty(){
                 header.push_str("Last_Modified:");
                  header.push_str(&*(self.last_modified));
                 header.push_str("\r\n");
            }
            if !self.location.is_empty(){
                 header.push_str("Pragma:");
                  header.push_str(&*(self.location));
                 header.push_str("\r\n");
            }
            if !self.pragma.is_empty(){
                 header.push_str("Pragma:");
                  header.push_str(&*(self.pragma));
                 header.push_str("\r\n");
            }
            if !self.proxy_authenticate.is_empty(){
                 header.push_str("Proxy_Authenticate:");
                  header.push_str(&*(self.proxy_authenticate));
                 header.push_str("\r\n");
            }
            if !self.refresh.is_empty(){
                 header.push_str("Refresh:");
                  header.push_str(&*(self.refresh));
                 header.push_str("\r\n");
            }
            if !self.retry_after.is_empty(){
                 header.push_str("Retry_After:");
                  header.push_str(&*(self.retry_after));
                 header.push_str("\r\n");
            }
            if !self.server.is_empty(){
                 header.push_str("Server:");
                  header.push_str(&*(self.server));
                 header.push_str("\r\n");
            }
            if !self.set_cookie.is_empty(){
                 header.push_str("Set_Cookie:");
                  header.push_str(&*(self.set_cookie));
                 header.push_str("\r\n");
            }
            if !self.trailer.is_empty(){
                 header.push_str("Trailer:");
                  header.push_str(&*(self.trailer));
                 header.push_str("\r\n");
            }
            if !self.transfer_encoding.is_empty(){
                 header.push_str("Transfer_Encoding:");
                  header.push_str(&*(self.transfer_encoding));
                 header.push_str("\r\n");
            }
            if !self.vary.is_empty(){
                 header.push_str("VaryAge:");
                  header.push_str(&*(self.trailer));
                 header.push_str("\r\n");
            }
             if !self.via.is_empty(){
                 header.push_str("Via:");
                  header.push_str(&*(self.via));
                 header.push_str("\r\n");
            }
            
             if !self.warning.is_empty(){
                 header.push_str("Warning:");
                  header.push_str(&*(self.warning));
                 header.push_str("\r\n");
            }
           
            header

     }
    pub fn set_status_code(& mut self,status_code:u16)-> &mut Self {
         self.status_code=status_code;
         self
     }
     
     pub fn send(& mut self,body:&str)  {
         let header=self.build_header();
         let _ =self.stream.write(header.as_bytes());
         let _=self.stream.write("\r\n".as_bytes());
         let _=self.stream.write(body.as_bytes());
     }
}