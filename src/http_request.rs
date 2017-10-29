use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
const MAX_HEADER_LEN: usize = 8192;

#[derive(Default)]//省略一些
//http://tools.jb51.net/table/http_header
pub struct HttpRequest {

    // HTTP请求方式
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub accept:Vec<String>,
    pub accept_charset:String,
    pub accept_encoding:Vec<String>,
    pub accept_language:String,
    pub accept_ranges:String,
    pub authorization:String,
    pub cache_control:String,
    pub cookie:String,
    pub connection:String,
    pub content_length :String,
    pub content_type:String,
    pub date:String,
    pub expect:String,
    pub form:String,
    pub host:String,
    pub if_match:String,
    pub if_modified_since:String,
    pub if_none_match:String,
    pub if_range:String,
    pub if_unmodified_since:String,
    pub max_forwards:String,
    pub pragma:String,
    pub proxy_authorization:String,
    pub range:String,
    pub referer:String,
    pub te:String,
    pub upgrade:String,
    pub user_agent:Vec<String>,
    pub via:String,
    pub warning:String,

    // // 请求参数
    pub get_params:  HashMap<String, String>,

    pub post_params: String,

    // // 客户端请求报文
    // content: &'a str,
    //TcpStream
    // stream: &'a mut TcpStream,
}

pub fn get_value<'a>(hash_map:&'a HashMap<&str,&str>,key:&str)->Result<&'a str,&'a str>{
        if hash_map.contains_key(key){
            Ok(hash_map[key])
         }
         else{
            Err("")
         }
}
pub fn build_string(content:Result<&str,& str>)->String{
        match content{
            Ok(v)=>{
                String::from(v)
            },
            Err(v)=>{
                 String::from("")
            }
        }
}
pub fn build_vec(content:Result<&str,&str>)->Vec<String>{
    match content{
            Ok(v)=>{
                v.split(", ").collect::<Vec<&str>>().iter().map(|&x| String::from(x)).collect::<Vec<String>>()
            },
            Err(v)=>{
                Vec::new()
            }
        }
}
impl HttpRequest{
    
    pub fn new( stream: &mut  TcpStream) -> HttpRequest {
        let  mut  buf = [0u8; MAX_HEADER_LEN];
        let length = stream.read(&mut buf);
        let  header = match str::from_utf8(&buf) {
            Ok(v) => v.trim(),
            Err(e) => panic!("Invalid UTF_8 sequence: {}", e),
        };
        let mut items: Vec<&str> = header.split("\r\n").collect();
       
        let line =items[0].split(' ').collect::<Vec<&str>>();
        

        let method=String::from(line[0]);let url=String::from(line[1]);let http_version=String::from(line[2]);
        
        items.remove(0);

        let post_params=String::from(items.pop().unwrap().trim());
        items.pop();

        let get_params={
            let content=url.split('?').collect::<Vec<&str>>();
            
            if content.len()>1{
                 content[1].split('&').collect::<Vec<&str>>().iter().map(|&x|
                 {
                        let params:Vec<&str>=x.split('=').collect();
                        (String::from(params[0]),String::from(params[1]))
                 }).collect::<HashMap<String,String>>()
            }
            else {
                 HashMap::new()
            }
        };

        let key_val=items.iter().map(|x| 
            {
                let content:Vec<&str>=x.split(": ").collect();
                (content[0],content[1])
            }).collect::<HashMap<&str,&str>>();
       
      

        HttpRequest {
            method:method,url:url,http_version:http_version,
            accept  :build_vec(get_value(&key_val,"Accept")),
            accept_charset :build_string(get_value(&key_val,"Accept_Charset")),
            accept_encoding :build_vec( get_value(&key_val,"Accept_Encoding")),
            accept_language :build_string( get_value(&key_val,"Accept_Language")),
            accept_ranges :build_string( get_value(&key_val,"Accept_Ranges")),
            authorization :build_string( get_value(&key_val,"Authorization")),
            cache_control :build_string( get_value(&key_val,"Cache_Control")),
            connection :build_string(get_value(&key_val,"Connection")),
            cookie :build_string( get_value(&key_val,"Cookie")),
            content_length :build_string( get_value(&key_val,"Content_Length")),
            content_type :build_string( get_value(&key_val,"Content_Type")),
            date :build_string( get_value(&key_val,"Date")),
            expect :build_string( get_value(&key_val,"Expect")),
            form :build_string( get_value(&key_val,"Form")),
            host :build_string( get_value(&key_val,"Host")),
            if_match :build_string( get_value(&key_val,"If_Match")),
            if_modified_since :build_string( get_value(&key_val,"if_Modified_Since")),
            if_none_match :build_string( get_value(&key_val,"If_None_Match")),
            if_range :build_string( get_value(&key_val,"If_Range")),
            if_unmodified_since :build_string( get_value(&key_val,"If_Unmodified_Since")),
            max_forwards :build_string( get_value(&key_val,"Max_Forwards")),
            pragma :build_string( get_value(&key_val,"Pragma")),
            proxy_authorization :build_string( get_value(&key_val,"Proxy_Authorization")),
            range :build_string( get_value(&key_val,"Range")),
            referer :build_string( get_value(&key_val,"Referer")),
            te :build_string( get_value(&key_val,"TE")),
            upgrade :build_string( get_value(&key_val,"Upgrade")),
            user_agent :build_vec(get_value(&key_val,"User_Agent")),
            via :build_string( get_value(&key_val,"Via")),
            warning :build_string( get_value(&key_val,"Warning")),

            post_params:post_params,
            get_params:get_params,

        }
       
    }
}
