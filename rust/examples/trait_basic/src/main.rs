trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

struct PolarCoord {
    r: f64,
    theta: f64,
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn print_point<P: Coordinates>(point: P) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
    point.clone().to_cartesian()
}

fn double_point<P: Coordinates>(point: P) -> P {
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart)
}

fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T>,
{
    i.convert()
}

struct Matrix([[f64; 2]; 2]);

trait LinearTransform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized,
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;

        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * y + m[1][1] * y;
        Self::from_cartesian(cart)
    }

    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }
}

impl LinearTransform for CartesianCoord {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }
}

impl LinearTransform for PolarCoord {
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

trait As<T> {
    fn cast(self) -> T;
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

trait Overload {
    fn call(&self) -> &str;
}

impl Overload for i32 {
    fn call(&self) -> &str {
        "i32"
    }
}

impl Overload for str {
    fn call(&self) -> &str {
        "str"
    }
}

trait Overload1<T> {
    fn call1(&self, t: T) -> &str;
}

impl Overload1<i32> for i32 {
    fn call1(&self, _: i32) -> &str {
        "(i32, i32)"
    }
}

impl Overload1<char> for i32 {
    fn call1(&self, _: char) -> &str {
        "(i32, char)"
    }
}

fn main() {
    let point = (1.0, 1.0);

    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);

    let p = PolarCoord::from_cartesian(c);
    println!("r = {}, theta = {}", p.r, p.theta);

    print_point((0.0, 1.0));
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });

    let p = (1.0, 0.0).to_cartesian();
    print_point(p.rotate(std::f64::consts::PI));

    let data = Box::init("foo");
    let data = Box::<f32>::init(0.1);
    let data: Box<f32> = Init::init(0.1);
    let data: Box<_> = Init::<f32>::init(0.1);

    let one_u32: u32 = 1.cast();
    let one_u64: u64 = 1.cast();

    assert_eq!(1i32.call(), "i32");
    assert_eq!("str".call(), "str");
    assert_eq!(Overload::call(&1i32), "i32");
    assert_eq!(Overload::call("str"), "str");

    assert_eq!(1i32.call1(2i32), "(i32, i32)");
    assert_eq!(1i32.call1('c'), "(i32, char)");
}
