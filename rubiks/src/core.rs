use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Display, Formatter};

use bevy::prelude::*;
use num_enum::TryFromPrimitive;

pub mod math;

/// 块
#[derive(Debug, Default, Component, Reflect, FromReflect, Clone, Copy)]
#[reflect(Component)]
pub struct Piece {
    /// 是几阶的魔方
    pub order: u8,
    /// x
    pub x: u8,
    /// y
    pub y: u8,
    /// z
    pub z: u8,
}

impl Piece {
    pub fn new(order: u8, x: u8, y: u8, z: u8) -> Self {
        Piece { order, x, y, z }
    }

    /// 判断是不是需要旋转的块
    pub fn is_selected(&self, command: &Command) -> bool {
        match command.0 {
            BaseMove::U => self.y == self.order - 1,
            BaseMove::L => self.x == 0,
            BaseMove::F => self.z == self.order - 1,
            BaseMove::R => self.x == self.order - 1,
            BaseMove::B => self.z == 0,
            BaseMove::D => self.y == 0,
            BaseMove::M => self.x == self.order / 2,
            BaseMove::E => self.y == self.order / 2,
            BaseMove::S => self.z == self.order / 2,
            BaseMove::X => true,
            BaseMove::Y => true,
            BaseMove::Z => true,
        }
    }
}

/// 表面
#[derive(Component, Reflect, FromReflect, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
#[reflect(Component)]
pub enum Surface {
    #[default]
    U,
    D,
    L,
    R,
    F,
    B,
}

/// 旋转方向
#[derive(Debug)]
pub enum Turns {
    /// 转第n个X轴多少度
    X(u8, f32),
    /// 转第n个Y轴多少度
    Y(u8, f32),
    /// 转第n个Z轴多少度
    Z(u8, f32),
}

impl Display for Turns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (n, axis, angle) = match self {
            Turns::X(n, angle) => (n, "x", angle),
            Turns::Y(n, angle) => (n, "y", angle),
            Turns::Z(n, angle) => (n, "z", angle),
        };
        write!(f, "the {} {} axis turn {}°", n, axis, angle.to_degrees())
    }
}

/// 基础的旋转指令集
/// - U: 顶面
/// - D: 底面
/// - L: 左面
/// - R: 右面
/// - F: 前面
/// - B: 后面
/// - M: L和R中间
/// - E: U和D中间
/// - S: F和B中间
/// - X: 顺时针沿着R旋转整个魔方
/// - Y: 顺时针沿着U旋转整个魔方
/// - Z: 顺时针沿着F旋转整个魔方
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum BaseMove {
    U,
    L,
    F,
    R,
    B,
    D,
    M,
    E,
    S,
    X,
    Y,
    Z,
}

impl Display for BaseMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub const MOVE_LIST: [BaseMove; 12] = [
    BaseMove::R,
    BaseMove::L,
    BaseMove::U,
    BaseMove::D,
    BaseMove::F,
    BaseMove::B,
    BaseMove::M,
    BaseMove::E,
    BaseMove::S,
    BaseMove::X,
    BaseMove::Y,
    BaseMove::Z,
];

/// 如何旋转面
/// - Normal 顺时针旋转
/// - Prime 逆时针旋转
/// - Double 顺时针旋转两次
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Normal,
    Prime,
    Double,
}

/// 旋转指令
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command(pub BaseMove, pub i8);
impl Command {
    pub fn prime(self) -> Self {
        Command(self.0, -self.1)
    }

    pub fn angle(&self) -> f32 {
        match self.1 {
            1 => FRAC_PI_2,
            2 => PI,
            -1 => -FRAC_PI_2,
            -2 => -PI,
            _ => 0.0,
        }
    }

    pub fn rotation(&self) -> Quat {
        match self.0 {
            BaseMove::U => Quat::from_rotation_y(-self.angle()),
            BaseMove::L => Quat::from_rotation_x(-self.angle()),
            BaseMove::F => Quat::from_rotation_z(-self.angle()),
            BaseMove::R => Quat::from_rotation_x(-self.angle()),
            BaseMove::B => Quat::from_rotation_z(self.angle()),
            BaseMove::D => Quat::from_rotation_y(self.angle()),
            BaseMove::M => Quat::from_rotation_x(self.angle()),
            BaseMove::E => Quat::from_rotation_y(self.angle()),
            BaseMove::S => Quat::from_rotation_z(self.angle()),
            BaseMove::X => Quat::from_rotation_x(self.angle()),
            BaseMove::Y => Quat::from_rotation_y(self.angle()),
            BaseMove::Z => Quat::from_rotation_z(self.angle()),
        }
    }

    pub fn axis(&self) -> Vec3 {
        match self.0 {
            BaseMove::U => Vec3::Y,
            BaseMove::L => Vec3::X,
            BaseMove::F => Vec3::Z,
            BaseMove::R => Vec3::X,
            BaseMove::B => Vec3::Z,
            BaseMove::D => Vec3::Y,
            BaseMove::M => Vec3::X,
            BaseMove::E => Vec3::Y,
            BaseMove::S => Vec3::Z,
            BaseMove::X => Vec3::X,
            BaseMove::Y => Vec3::Y,
            BaseMove::Z => Vec3::Z,
        }
    }

    pub fn clockwise(&self) -> bool {
        self.1 > 0
    }
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.angle().to_degrees())
    }
}

/// 是一条命令还是一组命令
#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    One(Command),
    Group(Vec<Command>, i8),
}

/// 将旋转指令打平
pub fn flatten(elems: Vec<Elem>) -> Vec<Command> {
    let mut v = vec![];
    for e in elems {
        match e {
            Elem::One(c) => v.push(c),
            Elem::Group(cs, rep) => {
                if rep > 0 {
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c);
                        }
                    }
                } else {
                    let rep = -rep;
                    let mut cs = cs;
                    cs.reverse();
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c.prime())
                        }
                    }
                }
            }
        }
    }
    v
}
#[test]
fn test_flatten() {
    let e = Elem::Group(vec![Command(BaseMove::U, 1), Command(BaseMove::R, 1)], -1);
    let f = flatten(vec![e]);
    assert_eq!(f, vec![Command(BaseMove::R, -1), Command(BaseMove::U, -1)]);
}

pub fn random_command(n: usize) -> Vec<Command> {
    use rand::prelude::*;
    let mut rng = thread_rng();
    let mut v = vec![];
    for _ in 0..n {
        let mov: usize = rng.gen();
        let mov = MOVE_LIST[mov % 12];
        let ran: i64 = rng.gen();
        let rep = ran % 2 + 1;
        let rep = if ran % 2 == 0 { rep } else { -rep };
        v.push(Command(mov, rep as i8));
    }
    v
}

/// 面
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum FACE {
    UP,
    LEFT,
    FRONT,
    RIGHT,
    BACK,
    DOWN,
}

/// 贴纸颜色
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
#[repr(u8)]
pub enum COLOR {
    #[default]
    WHITE,
    GREEN,
    RED,
    BLUE,
    ORANGE,
    YELLOW,
}

///  棱
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[repr(u8)]
pub enum EDGE {
    UR,
    UF,
    UB,
    UL,
    FR,
    FL,
    BL,
    BR,
    DF,
    DL,
    DB,
    DR,
}

impl TryFrom<u8> for EDGE {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UR),
            1 => Ok(Self::UF),
            2 => Ok(Self::UB),
            3 => Ok(Self::UL),
            4 => Ok(Self::FR),
            5 => Ok(Self::FL),
            6 => Ok(Self::BL),
            7 => Ok(Self::BR),
            8 => Ok(Self::DF),
            9 => Ok(Self::DL),
            10 => Ok(Self::DB),
            11 => Ok(Self::DR),

            _ => Err(()),
        }
    }
}

///  角块
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[repr(u8)]
pub enum CORNER {
    ULB,
    URB,
    URF,
    ULF,
    DLF,
    DLB,
    DRB,
    DRF,
}

impl TryFrom<u8> for CORNER {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ULB),
            1 => Ok(Self::URB),
            2 => Ok(Self::URF),
            3 => Ok(Self::ULF),
            4 => Ok(Self::DLF),
            5 => Ok(Self::DLB),
            6 => Ok(Self::DRB),
            7 => Ok(Self::DRF),

            _ => Err(()),
        }
    }
}

/// 转动方法
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum MOVE {
    L,
    LPRIME,
    L2,
    R,
    RPRIME,
    R2,
    U,
    UPRIME,
    U2,
    D,
    DPRIME,
    D2,
    F,
    FPRIME,
    F2,
    B,
    BPRIME,
    B2,
    Y,
    YPRIME,
    Y2,
    X,
    XPRIME,
    X2,
    Z,
    ZPRIME,
    Z2,
    M,
    MPRIME,
    M2,
    E,
    EPRIME,
    E2,
    S,
    SPRIME,
    S2,
}

#[derive(Default, Clone, Copy)]
pub struct Cubie {
    // 0 - 11 棱, 0 - 7 角
    pub index: u8,
    // 0 - 1 棱,  0 - 2 角
    pub orientation: u8,
}
