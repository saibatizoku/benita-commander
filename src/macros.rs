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

            req_fn_eval! { [ $( $request ),* ] }
        }
    };
}

macro_rules! req_fn_eval {
    ( [ $( $request:ty ),* ] ) => {
        pub fn eval(&self, s: &str) -> Result<String> {
            debug!("evaluating: {:?}", s);
            $(
                if let Ok(req) = s.parse::<$request>() {
                    let rep = SocketRequest::send(&req, &self.requester)
                        .chain_err(|| "bad REQ eval")?;
                    return Ok(SocketReply::to_string(&rep));
                } )*
            Ok("command not recognized".to_string())
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

            res_fn_eval! { [ $( $request ),* ] }
        }
    };
}

macro_rules! res_fn_eval {
    ( [ $( $request:ident ),* ] ) => {
        // Evaluate a conductivity command using the given responder. Returns a String.
        pub fn eval(&self, s: &str) -> Result<String> {
            debug!("evaluating: {:?}", s);
            $(
                if let Ok(req) = <$request as SocketRequest>::from_str(s) {
                    let rep = <$request as I2CCommand>::write(&req, &self.responder.sensor)
                        .chain_err(|| "bad REQ eval")?;
                    return Ok(I2CResponse::to_string(&rep));
                } )*
            Ok("command not recognized".to_string())
        }
    };
}
