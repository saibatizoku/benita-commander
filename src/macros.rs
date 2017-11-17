//! Macros used throughout.

/// Creates a Requester capable of:
///
/// * parsing command-line input from a list of known requests
/// * and, evaluating.
macro_rules! requester {
    ( $name:ident , $doc:tt ,  $req:ty,  $req_expr:ident , [ $( $request:ty ),* ] ) => {
        #[ doc = $doc ]
        pub struct $name {
            pub requester: $req,
        }

        impl $name {
            pub fn new(url: &str) -> Result<$name> {
                let req_socket =
                    create_and_connect_requester(url).chain_err(|| "unable to setup req_socket")?;
                let requester = $req_expr::new(req_socket)
                    .chain_err(|| "requester artifact failed to start")?;
                let _connect = requester
                    .connect(url)
                    .chain_err(|| "requester failed to connect")?;

                Ok($name { requester })
            }

            pub fn request<T: SocketRequest + Command>(&self, cmd: T) -> Option<String>
                where
                    <T as SocketRequest>::Response: SocketReply + fmt::Debug,
                {
                    println!("REQ: {}", cmd.to_request_string());
                    match cmd.send_to(&self.requester) {
                        Ok(rep) => Some(format!("{}", rep.to_reply_string())),
                        _ => None,
                    }
                }

            req_fn_eval! { [ $( $request ),* ] }
        }
    };
}

macro_rules! req_fn_eval {
    ( [ $( $request:ty ),* ] ) => {
        pub fn eval(&self, s: &str) -> Result<String> {
            debug!("evaluating: {:?}", s);
            let resp = match s {
                $( a if s.parse::<$request>().is_ok() =>
                   self.request(a.parse::<$request>().unwrap()), )*
                _ => None,
            };
            match resp {
                Some(rep) => Ok(rep),
                None => Ok("command not recognized".to_string()),
            }
        }
    };
}

/// Creates a Responder capable of evaluating a list of explicit requests (commands).
macro_rules! responder {
    ( $name:ident , $doc:tt ,  $sensor:ident, $res:ty,  $res_expr:ident ,
      [ $( $request:ident ),* ] ) => {
        #[ doc = $doc ]
        pub struct $name {
            pub responder: $res,
        }

        impl $name {
            pub fn new(url: &str, path: &str, addr: u16) -> Result<$name> {
                let req_socket =
                    create_and_bind_responder(url).chain_err(|| "unable to setup req_socket")?;
                let sensor = $sensor::new(path, addr)
                    .chain_err(|| "could not start sensor")?;
                let responder = $res_expr::new(req_socket, sensor)
                    .chain_err(|| "responder artifact failed to start")?;
                let _connect = responder
                    .connect(url)
                    .chain_err(|| "responder failed to connect")?;

                Ok($name { responder })
            }

            // Evaluate a conductivity command using the given responder. Returns a String.
            pub fn respond<T: SocketRequest + Command>(&self, cmd: T) -> Option<String>
                where <T as Command>::Response: fmt::Debug {
                    println!("REQ: {}", cmd.to_request_string());
                    match cmd.run(&mut self.responder.sensor.i2cdev.borrow_mut()) {
                        Ok(rep) => {
                            println!("REP: {:?}", rep);
                            Some(format!("{:?}", rep))
                        }
                        _ => None,
                    }
                }

            res_fn_eval! { [ $( $request ),* ] }
        }
    };
}

macro_rules! res_fn_eval {
    ( [ $( $request:ident ),* ] ) => {
        pub fn eval(&self, s: &str) -> Result<String> {
            debug!("evaluating: {:?}", s);
            let resp = match s {
                $( a if $request::from_request_str(a).is_ok() =>
                   self.respond($request::from_request_str(a).unwrap()), )*
                _ => None,
            };
            match resp {
                Some(rep) => Ok(rep),
                None => Ok("command not recognized".to_string()),
            }
        }
    };
}
