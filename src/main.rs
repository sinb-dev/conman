use rweblet::{HttpListener, Context, Response};
use std::env;
use std::process::Command;
//extern crate rs_docker;
//use rs_docker::Docker;
//use rs_docker::container::{ Container };
extern crate docker;
use docker::Docker
use docker::container::{ Container };
use serde::{Serialize};
use serde_json::Result;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut threads:usize = 1;
    if args.len() > 1 {
        threads = args[1].parse::<usize>().unwrap();
    }
    let mut webserver = HttpListener::new();
    webserver.route("^/list/$", rq_list);
    webserver.route("^/run/[a-z0-9]+/$", rq_run);
    webserver.route("^/$", request_root);
    webserver.webroot = String::from("client/");
    
    webserver.threads(threads);

    println!("Starting server on 0.0.0.0:8080 with {} threads", threads);
    webserver.start("0.0.0.0:8080",1);
}

fn rq_run(context: &Context) -> Response {
    let folders: Vec<&str> = context.request.url.split("/").collect();
    for f in &folders {
        println!("Folder: {}",f);
    }
    let api_key: String = folders[2].to_string();
    
    let msg = format!("Invalid api key: {}",api_key);
    let msg2: &str = msg.as_ref();
    let json = Reply::json(msg2);

    Response::ok_json(json.as_ref())
}

fn rq_list(context: &Context) -> Response {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
    	Ok(docker) => docker,
        Err(e) => { panic!("{}", e); }
    };
    let containers = match docker.get_containers(false) {
        Ok(containers) => containers,
        Err(e) => { panic!("{}", e); }
    };
    println!("Found {} containers", containers.len());
    for cont in &containers {
        for name in &cont.Names {
            println!("{}", name);
        }
    }

    let serialized = serde_json::to_string(&containers).unwrap();
    Response::ok_text(&serialized)
}
fn request_root(context: &Context) -> Response {
    Response::ok_text("fine")
}

#[derive(Serialize)]
struct Con {
    name: String,
}

impl Con {
    fn from_docker(container: &Container) -> Con {
        let name: &str = container.Names[0].as_ref();
        Con {
            name : name.to_string()
        }
    }
}

#[derive(Serialize)]
struct Reply {
    message: String
}
impl Reply {
    fn new(msg: &str) -> Reply {
        Reply {
            message : String::from(msg),
        }
    }
    fn json(msg: &str) -> String {
        let reply = Reply::new(msg);
        return serde_json::to_string(&reply).unwrap()
    }
}