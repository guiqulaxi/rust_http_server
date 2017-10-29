use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::str;
use http_request::HttpRequest;
use http_response::HttpResponse;

// pub trait Handler {
//     fn on_post(request:HttpRequest,response:HttpResponse);
//     fn on_get(request:HttpRequest,response:HttpResponse);
//     fn on_defualt(request:HttpRequest,response:HttpResponse);
// 

pub struct Server{
    addr: String,
    root_dir: String,
}
fn on_post(request:HttpRequest, response:HttpResponse){
    
}
fn on_get(request:HttpRequest, mut  response:HttpResponse){
    response.send("get respose");
}
fn on_defualt(request:HttpRequest, response:HttpResponse){

}

 
impl Server  {

    pub fn new( addr:&str, root_dir: &str ) -> Server{
        Server {
            addr: addr.to_string(),
            root_dir: root_dir.to_string(),

        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&*(self.addr)).unwrap();
        
        for stream in listener.incoming() {
            thread::spawn(move || {
                let mut tcp_stream = match stream {
                    Ok(stream) => stream ,
                    Err(e) => { panic!("/* connection failed */"); }
                    };
                let request=HttpRequest::new(&mut tcp_stream);

                let  response=HttpResponse::new(tcp_stream);

                if request.method=="GET"{
                            
                    on_get(request,response);
                                
                }
                else if request.method=="Post"{
                    on_post(request,response);
                }
                else{
                    on_defualt(request,response);
                } 

            }); 
        }

       
    }
  
    
}

