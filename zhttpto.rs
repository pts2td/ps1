//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::*;
use std::io::buffered::BufferedReader;
use std::io::File;
use std::io::Path;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut VCOUNT: int = 0;

fn main() {
    //let mut vcount: int = 0;
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    

	let mut count = 0;
    for stream1 in acceptor.incoming() {
	
        // Spawn a task to handle the connection
	// vcount +=1;
	let mut viewnum = 0;
	unsafe {viewnum = addviewer();}
	viewnum = viewnum/2;
	println!("visitors: {:d}", viewnum);
        
            let mut stream = stream1;
	 
            
	    
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
		println(format!("Received request :{:s}", request_str));
		let strget = request_str.slice_to(3);
		let strlink = request_str.slice_to(14);

		println("HOMELINK:" + strlink);
		if strlink == "GET / HTTP/1.1"{
			let numresponse = format!("<h1>Greetings, Peter {:d} </h1>", 12);
		let response: ~str = 	 ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n                  <doctype !html><html><head><title>Hello, Rust!</title>                  <style>body { background-color: #FFF; color: #FFFFFF }                         h1 { font-size:2cm; text-align: center; color: black;}                </style></head>                 <body>                 <h1>Greetings, Krusty!</h1>         "+numresponse+"       </body></html>\r\n";
			 stream.write(response.as_bytes());
		}
		
					

		else if strget == "GET"{
			//println("YEAH GET");
			//println("File: "+ strhttp);
			let rqlen = request_str.len();
			
			
			println!("Request Len:{:u}", rqlen);
			let file_name = request_str.slice(5, rqlen - 9);
			println!("File Len:{:u}", file_name.len());
			println("Actual File:" + file_name);
			let practice_file = ~"hello.txt";
			let path = Path::new(practice_file.clone());
			let request_file = File::open(&path);
			//check to see if file
		 	  
			let mut reader = BufferedReader::new(request_file);
		
			for line in reader.lines() {
       		        			
                			//print(line);
          			  	stream.write(line.as_bytes());
			
			}
	    		
	  		 
			
	
			}
			
			
		}
		
	    
            
            
        
    

}
unsafe fn addviewer() ->int{
	
	VCOUNT += 1;
	let num = VCOUNT;
	//println("visitors: " + int::str(num));
	return num;
}

