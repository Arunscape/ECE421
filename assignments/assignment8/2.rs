usehyper::rt::Future;
usehyper::service::service_fn_ok;
usehyper::{Body,Request,Response,Server};

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {            
            service_fn(service_router)
        })
    .map_err(|e| eprintln!("server error: {}", e));
    
    println!("Listening on http://{}", addr);17hyper::rt::run(server);
}

fn svc_wait(t: u64) -> impl Future<Item = (), Error = ()> {
    println!("[start] waiting...");
    let when = Instant::now() + Duration::from_millis(t);
    Delay::new(when)
        .map_err(|e| panic!("timer failed; err={:?}", e))
        .and_then(|_| {
            println!("[end] waiting");
            Ok(())
        })
}

fn fetch_data() -> impl Future<Item = future::FutureResult<RespStruct,String>, Error = ()> {
    let uri: Uri = "http://httpbin.org/get".parse().expect("Cannot parse 35URL");
    Client::new()
        .get(uri)
        // Future is polled here
        .and_then(|res| {
            res.into_body().concat2()
        })
    .map_err(|err| println!("error: {}", err))
    .map(|body| {
        let decoded: RespStruct =
            serde_json::from_slice(&body).expect("Couldn't deserialize");
        future::ok(decoded)
    })
}

type BoxFut= Box<dyn Future<Item = Response<Body>, Error = hyper::Error> 
+ Send>;

fn service_router(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/wait") => {
            let r = svc_wait(1500);
            hyper::rt::spawn(r);
            *response.body_mut() = Body::from(format!("Triggered waiting66{}ms", 1500));
        }
        (&Method::GET, "/fetch") => {
            let r = fetch_data().map(|x| {
                println!("got data: {:?}", x);
            });
            hyper::rt::spawn(r);
            *response.body_mut() = Body::from("Sent request to external 76webservice");
        }
        // ... more routers
        }
    eprintln!("Returning a response");
    Box::new(future::ok(response))
}
