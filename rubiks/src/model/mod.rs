pub mod goal;
pub mod index_model;
pub mod move_store;
pub mod pattern_database;
pub mod searcher;

// pub trait RubiksCube {
//     fn as_any(&self) -> &dyn Any;
//
//     fn get_move(&self, ind: MOVE) -> String;
//
//     fn r#move(&mut self, ind: MOVE) -> &mut Self
//     where
//         Self: Sized,
//     {
//         match ind {
//             MOVE::L => self.l(),
//             MOVE::LPRIME => self.l_prime(),
//             MOVE::L2 => self.l2(),
//             MOVE::R => self.r(),
//             MOVE::RPRIME => self.r_prime(),
//             MOVE::R2 => self.r2(),
//             MOVE::U => self.u(),
//             MOVE::UPRIME => self.u_prime(),
//             MOVE::U2 => self.u2(),
//             MOVE::D => self.d(),
//             MOVE::DPRIME => self.d_prime(),
//             MOVE::D2 => self.d2(),
//             MOVE::F => self.f(),
//             MOVE::FPRIME => self.f_prime(),
//             MOVE::F2 => self.f2(),
//             MOVE::B => self.b(),
//             MOVE::BPRIME => self.b_prime(),
//             MOVE::B2 => self.b2(),
//             MOVE::X => self.x(),
//             MOVE::XPRIME => self.x_prime(),
//             MOVE::X2 => self.x2(),
//             MOVE::Y => self.y(),
//             MOVE::YPRIME => self.y_prime(),
//             MOVE::Y2 => self.y2(),
//             MOVE::Z => self.z(),
//             MOVE::ZPRIME => self.z_prime(),
//             MOVE::Z2 => self.z2(),
//             MOVE::M => self.m(),
//             MOVE::MPRIME => self.m_prime(),
//             MOVE::M2 => self.m2(),
//             MOVE::E => self.e(),
//             MOVE::EPRIME => self.e_prime(),
//             MOVE::E2 => self.e2(),
//             MOVE::S => self.s(),
//             MOVE::SPRIME => self.s_prime(),
//             MOVE::S2 => self.s2(),
//         }
//     }
//
//     fn u(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn u_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn u2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn l(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn l_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn l2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn f(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn f_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn f2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn r(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn r_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn r2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn b(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn b_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn b2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn d(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn d_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn d2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn m(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn m_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn m2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn e(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn e_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn e2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn s(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn s_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn s2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn y(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn y_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn y2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn x(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn x_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn x2(&mut self) -> &mut Self
//     where
//         Self: Sized;
//
//     fn z(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn z_prime(&mut self) -> &mut Self
//     where
//         Self: Sized;
//     fn z2(&mut self) -> &mut Self
//     where
//         Self: Sized;
// }
