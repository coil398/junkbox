pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

pub trait Coordinates {
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

impl Coordinates for PolarCoord {
    fn from_cartesian(cart: CartesianCoord) -> Self {
        let r = (cart.x * cart.x + cart.y * cart.y).sqrt();
        let theta = (cart.y / cart.x).atan();
        PolarCoord { r, theta }
    }
}

trait Dimension {
    const DIMENSION: u32;
}

impl Dimension for CartesianCoord {
    const DIMENSION: u32 = 2;
}

use std::str::FromStr;

trait Server {
    type Response;
    type Request: FromStr;

    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;

impl Server for EchoServer {
    type Response = String;
    type Request = String;

    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

fn main() {
    let c = CartesianCoord { x: 1.0, y: 0.0 };
    let p = PolarCoord::from_cartesian(c);

    let c = CartesianCoord { x: 1.0, y: 0.0 };
    let p: PolarCoord = Coordinates::from_cartesian(c);

    let dim = CartesianCoord::DIMENSION;
}
