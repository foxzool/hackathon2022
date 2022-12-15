use crate::core::{Cubie, COLOR, CORNER, EDGE, FACE};
use crate::model::RubiksCube;
use std::any::Any;

/// 快速排序模型, 注意 顶面红色， 前面白色
pub struct RubiksCubeIndexModel {
    edges: [Cubie; 12],
    corners: [Cubie; 8],
    centers: [COLOR; 6],
}

impl Default for RubiksCubeIndexModel {
    fn default() -> Self {
        let mut edges = [Cubie::default(); 12];
        let mut corners = [Cubie::default(); 8];
        for i in 0..12 {
            edges[i].index = i as u8;
        }
        for i in 0..8 {
            corners[i].index = i as u8;
        }
        Self {
            edges,
            corners,
            centers: [
                COLOR::WHITE,
                COLOR::GREEN,
                COLOR::RED,
                COLOR::BLUE,
                COLOR::ORANGE,
                COLOR::YELLOW,
            ],
        }
    }
}

impl RubiksCube for RubiksCubeIndexModel {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RubiksCubeIndexModel {
    pub fn get_color(&self, f: FACE, row: i32, col: i32) -> COLOR {
        if row == 1 && col == 1 {
            return self.centers[f as usize];
        }
        if f == FACE::UP {
            if row == 0 {
                if col == 0 {
                    self.get_corner_colors(CORNER::ULB)[0]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UB)[0]
                } else {
                    self.get_corner_colors(CORNER::URB)[0]
                }
            } else if row == 1 {
                if col == 0 {
                    self.get_edge_color(EDGE::UL)[0]
                } else {
                    self.get_edge_color(EDGE::UR)[0]
                }
            } else {
                if col == 0 {
                    self.get_corner_colors(CORNER::ULF)[0]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UF)[0]
                } else {
                    self.get_corner_colors(CORNER::URF)[0]
                }
            }
        } else if f == FACE::LEFT {
            if row == 0 {
                if col == 0 {
                    self.get_corner_colors(CORNER::ULB)[1]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UL)[1]
                } else {
                    self.get_corner_colors(CORNER::ULF)[1]
                }
            } else if row == 1 {
                if col == 0 {
                    self.get_edge_color(EDGE::BL)[1]
                } else {
                    self.get_edge_color(EDGE::FL)[1]
                }
            } else if col == 0 {
                self.get_corner_colors(CORNER::DLB)[1]
            } else if col == 1 {
                self.get_edge_color(EDGE::DL)[1]
            } else {
                self.get_corner_colors(CORNER::DLF)[1]
            }
        } else if f == FACE::FRONT {
            if row == 0 {
                if col == 0 {
                    self.get_corner_colors(CORNER::ULF)[2]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UF)[1]
                } else {
                    self.get_corner_colors(CORNER::URF)[2]
                }
            } else if row == 1 {
                if col == 0 {
                    self.get_edge_color(EDGE::FL)[0]
                } else {
                    self.get_edge_color(EDGE::FR)[0]
                }
            } else if col == 0 {
                self.get_corner_colors(CORNER::DLF)[2]
            } else if col == 1 {
                self.get_edge_color(EDGE::DF)[1]
            } else {
                self.get_corner_colors(CORNER::DRF)[2]
            }
        } else if f == FACE::RIGHT {
            if row == 0 {
                if col == 0 {
                    self.get_corner_colors(CORNER::URF)[1]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UR)[1]
                } else {
                    self.get_corner_colors(CORNER::URB)[1]
                }
            } else if row == 1 {
                if col == 0 {
                    self.get_edge_color(EDGE::FR)[1]
                } else {
                    self.get_edge_color(EDGE::BR)[1]
                }
            } else if col == 0 {
                self.get_corner_colors(CORNER::DRF)[1]
            } else if col == 1 {
                self.get_edge_color(EDGE::DR)[1]
            } else {
                self.get_corner_colors(CORNER::DRB)[1]
            }
        } else if f == FACE::BACK {
            if row == 0 {
                if col == 0 {
                    self.get_corner_colors(CORNER::URB)[2]
                } else if col == 1 {
                    self.get_edge_color(EDGE::UB)[1]
                } else {
                    self.get_corner_colors(CORNER::ULB)[2]
                }
            } else if row == 1 {
                if col == 0 {
                    self.get_edge_color(EDGE::BR)[0]
                } else {
                    self.get_edge_color(EDGE::BL)[0]
                }
            } else if col == 0 {
                self.get_corner_colors(CORNER::DRB)[2]
            } else if col == 1 {
                self.get_edge_color(EDGE::DB)[1]
            } else {
                self.get_corner_colors(CORNER::DLB)[2]
            }
        } else if row == 0 {
            if col == 0 {
                self.get_corner_colors(CORNER::DLF)[0]
            } else if col == 1 {
                self.get_edge_color(EDGE::DF)[0]
            } else {
                self.get_corner_colors(CORNER::DRF)[0]
            }
        } else if row == 1 {
            if col == 0 {
                self.get_edge_color(EDGE::DL)[0]
            } else {
                self.get_edge_color(EDGE::DR)[0]
            }
        } else if col == 0 {
            self.get_corner_colors(CORNER::DLB)[0]
        } else if col == 1 {
            self.get_edge_color(EDGE::DB)[0]
        } else {
            self.get_corner_colors(CORNER::DRB)[0]
        }
    }

    pub fn is_solved(&self) -> bool {
        for (i, cubies) in self.corners.iter().enumerate() {
            if cubies.index != i as u8 || cubies.orientation != 0 {
                return false;
            }
        }

        for (i, cubies) in self.edges.iter().enumerate() {
            if cubies.index != i as u8 || cubies.orientation != 0 {
                return false;
            }
        }

        true
    }

    pub fn get_edge_color(&self, e: EDGE) -> [COLOR; 2] {
        let mut colors = [COLOR::default(); 2];

        let edge = self.edges.get(e as usize).unwrap();
        match EDGE::try_from(edge.index).unwrap() {
            EDGE::UR => {
                colors[0] = COLOR::RED;
                colors[1] = COLOR::GREEN;
            }
            EDGE::UF => {
                colors[0] = COLOR::RED;
                colors[1] = COLOR::WHITE;
            }
            EDGE::UB => {
                colors[0] = COLOR::RED;
                colors[1] = COLOR::YELLOW;
            }
            EDGE::UL => {
                colors[0] = COLOR::RED;
                colors[1] = COLOR::BLUE;
            }
            EDGE::FR => {
                colors[0] = COLOR::WHITE;
                colors[1] = COLOR::GREEN;
            }
            EDGE::FL => {
                colors[0] = COLOR::WHITE;
                colors[1] = COLOR::BLUE;
            }
            EDGE::BL => {
                colors[0] = COLOR::YELLOW;
                colors[1] = COLOR::BLUE;
            }
            EDGE::BR => {
                colors[0] = COLOR::YELLOW;
                colors[1] = COLOR::GREEN;
            }
            EDGE::DF => {
                colors[0] = COLOR::ORANGE;
                colors[1] = COLOR::WHITE;
            }
            EDGE::DL => {
                colors[0] = COLOR::ORANGE;
                colors[1] = COLOR::BLUE;
            }
            EDGE::DB => {
                colors[0] = COLOR::ORANGE;
                colors[1] = COLOR::YELLOW;
            }
            EDGE::DR => {
                colors[0] = COLOR::ORANGE;
                colors[1] = COLOR::GREEN;
            }
        }

        if edge.orientation == 1 {
            colors.swap(0, 1);
        }

        colors
    }

    pub fn get_corner_colors(&self, ind: CORNER) -> [COLOR; 3] {
        let mut colors = [COLOR::default(); 3];

        let corner: &Cubie = self.corners.get(ind as usize).unwrap();

        let mut i0 = 0;
        let mut i1 = 0;
        let mut i2 = 0;
        if corner.orientation == 0 {
            i0 = 0;
            i1 = 1;
            i2 = 2;
            if (corner.index + ind as u8) % 2 == 1 {
                colors.swap(1, 2)
            }
        } else if corner.orientation == 1 {
            i0 = 1;
            i1 = 2;
            i2 = 0;
            if (corner.index + ind as u8) % 2 == 1 {
                colors.swap(0, 1)
            }
        } else {
            i0 = 2;
            i1 = 0;
            i2 = 1;
            if (corner.index + ind as u8) % 2 == 1 {
                colors.swap(0, 2)
            }
        }

        match CORNER::try_from(corner.index).unwrap() {
            CORNER::ULB => {
                colors[i0] = COLOR::RED;
                colors[i1] = COLOR::BLUE;
                colors[i2] = COLOR::YELLOW;
            }
            CORNER::URB => {
                colors[i0] = COLOR::RED;
                colors[i1] = COLOR::GREEN;
                colors[i2] = COLOR::YELLOW;
            }
            CORNER::URF => {
                colors[i0] = COLOR::RED;
                colors[i1] = COLOR::GREEN;
                colors[i2] = COLOR::WHITE;
            }
            CORNER::ULF => {
                colors[i0] = COLOR::RED;
                colors[i1] = COLOR::BLUE;
                colors[i2] = COLOR::WHITE;
            }
            CORNER::DLF => {
                colors[i0] = COLOR::ORANGE;
                colors[i1] = COLOR::BLUE;
                colors[i2] = COLOR::WHITE;
            }
            CORNER::DLB => {
                colors[i0] = COLOR::ORANGE;
                colors[i1] = COLOR::BLUE;
                colors[i2] = COLOR::YELLOW;
            }
            CORNER::DRB => {
                colors[i0] = COLOR::ORANGE;
                colors[i1] = COLOR::GREEN;
                colors[i2] = COLOR::YELLOW;
            }
            CORNER::DRF => {
                colors[i0] = COLOR::ORANGE;
                colors[i1] = COLOR::GREEN;
                colors[i2] = COLOR::WHITE;
            }
        }

        colors
    }

    pub const fn get_edge_index(&self, ind: EDGE) -> u8 {
        self.edges[ind as usize].index
    }

    pub const fn get_edge_orientation(&self, ind: EDGE) -> u8 {
        self.edges[ind as usize].index
    }

    pub const fn get_corner_index(&self, ind: CORNER) -> u8 {
        self.corners[ind as usize].index
    }

    pub const fn get_corner_orientation(&self, ind: CORNER) -> u8 {
        self.corners[ind as usize].orientation
    }

    /// orientation 朝向
    /// - 0 红色 或者 橘色 在顶层或者底层
    /// - 1 红色 或者 橘色 在最近的顶层或者底层顺时针旋转
    /// - 2 红色 或者 橘色 在最近的顶层或者底层逆时针旋转
    fn update_corner_orientation(&mut self, ind: CORNER, orientation: u8) {
        let mut corner = self.corners[ind as usize];
        corner.orientation += orientation;
        if corner.orientation > 2 {
            corner.orientation -= 3;
        }
    }

    /// 更新棱的朝向
    fn update_edge_orientation_z(&mut self, ind: EDGE) {
        self.edges[ind as usize].orientation = 1 - self.edges[ind as usize].orientation;
    }

    /// 顺时针转顶层
    pub fn u(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = hold;

        let hold = self.edges[EDGE::UL as usize];
        self.edges[EDGE::UL as usize] = self.edges[EDGE::UF as usize];
        self.edges[EDGE::UF as usize] = self.edges[EDGE::UR as usize];
        self.edges[EDGE::UR as usize] = self.edges[EDGE::UB as usize];
        self.edges[EDGE::UB as usize] = hold;

        self
    }

    /// 逆时针转顶层
    pub fn u_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = hold;

        let hold = self.edges[EDGE::UB as usize];
        self.edges[EDGE::UB as usize] = self.edges[EDGE::UR as usize];
        self.edges[EDGE::UR as usize] = self.edges[EDGE::UF as usize];
        self.edges[EDGE::UF as usize] = self.edges[EDGE::UL as usize];
        self.edges[EDGE::UL as usize] = hold;

        self
    }

    /// 顶层转2次
    pub fn u2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::ULB as usize, CORNER::URF as usize);
        self.corners
            .swap(CORNER::URB as usize, CORNER::ULB as usize);

        self.edges.swap(EDGE::UB as usize, EDGE::UF as usize);
        self.edges.swap(EDGE::UR as usize, EDGE::UL as usize);

        self
    }

    /// 左面转顺时针
    pub fn l(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = hold;

        let hold = self.edges[EDGE::BL as usize];
        self.edges[EDGE::BL as usize] = self.edges[EDGE::DL as usize];
        self.edges[EDGE::DL as usize] = self.edges[EDGE::FL as usize];
        self.edges[EDGE::FL as usize] = self.edges[EDGE::UL as usize];
        self.edges[EDGE::UL as usize] = hold;

        self.update_corner_orientation(CORNER::DLB, 1);
        self.update_corner_orientation(CORNER::DLF, 1);
        self.update_corner_orientation(CORNER::ULF, 1);
        self.update_corner_orientation(CORNER::ULB, 1);

        self
    }

    /// 左面转逆时针
    pub fn l_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = hold;

        let hold = self.edges[EDGE::BL as usize];
        self.edges[EDGE::BL as usize] = self.edges[EDGE::UL as usize];
        self.edges[EDGE::UL as usize] = self.edges[EDGE::FL as usize];
        self.edges[EDGE::FL as usize] = self.edges[EDGE::DL as usize];
        self.edges[EDGE::DL as usize] = hold;

        self.update_corner_orientation(CORNER::DLB, 1);
        self.update_corner_orientation(CORNER::DLF, 2);
        self.update_corner_orientation(CORNER::ULF, 1);
        self.update_corner_orientation(CORNER::ULB, 2);

        self
    }

    /// 左面转2次
    pub fn l2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::DLB as usize, CORNER::ULF as usize);
        self.corners
            .swap(CORNER::ULB as usize, CORNER::DLF as usize);

        self.edges.swap(EDGE::BL as usize, EDGE::FL as usize);
        self.edges.swap(EDGE::UL as usize, EDGE::DL as usize);

        self
    }

    /// 顺时针转动前面
    pub fn f(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = hold;

        let hold = self.edges[EDGE::UF as usize];
        self.edges[EDGE::UF as usize] = self.edges[EDGE::FL as usize];
        self.edges[EDGE::FL as usize] = self.edges[EDGE::DF as usize];
        self.edges[EDGE::DF as usize] = self.edges[EDGE::FR as usize];
        self.edges[EDGE::FR as usize] = hold;

        self.update_corner_orientation(CORNER::ULF, 2);
        self.update_corner_orientation(CORNER::URF, 1);
        self.update_corner_orientation(CORNER::DRF, 2);
        self.update_corner_orientation(CORNER::DLF, 1);

        self.update_edge_orientation_z(EDGE::UF);
        self.update_edge_orientation_z(EDGE::FL);
        self.update_edge_orientation_z(EDGE::DF);
        self.update_edge_orientation_z(EDGE::FR);

        self
    }

    /// 逆时针转动前面
    pub fn f_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULF as usize];
        self.corners[CORNER::ULF as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = hold;

        let hold = self.edges[EDGE::UF as usize];
        self.edges[EDGE::UF as usize] = self.edges[EDGE::FR as usize];
        self.edges[EDGE::FR as usize] = self.edges[EDGE::DF as usize];
        self.edges[EDGE::DF as usize] = self.edges[EDGE::FL as usize];
        self.edges[EDGE::FL as usize] = hold;

        self.update_corner_orientation(CORNER::ULF, 2);
        self.update_corner_orientation(CORNER::URF, 1);
        self.update_corner_orientation(CORNER::DRF, 2);
        self.update_corner_orientation(CORNER::DLF, 1);

        self.update_edge_orientation_z(EDGE::UF);
        self.update_edge_orientation_z(EDGE::FL);
        self.update_edge_orientation_z(EDGE::DF);
        self.update_edge_orientation_z(EDGE::FR);

        self
    }

    /// 前面转2次
    pub fn f2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::ULF as usize, CORNER::DRF as usize);
        self.corners
            .swap(CORNER::DLF as usize, CORNER::URF as usize);

        self.edges.swap(EDGE::UF as usize, EDGE::DF as usize);
        self.edges.swap(EDGE::FL as usize, EDGE::FR as usize);

        self
    }

    /// 顺时针转动右面
    pub fn r(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = hold;

        let hold = self.edges[EDGE::BR as usize];
        self.edges[EDGE::BR as usize] = self.edges[EDGE::UR as usize];
        self.edges[EDGE::UR as usize] = self.edges[EDGE::FR as usize];
        self.edges[EDGE::FR as usize] = self.edges[EDGE::DR as usize];
        self.edges[EDGE::DR as usize] = hold;

        self.update_corner_orientation(CORNER::URF, 2);
        self.update_corner_orientation(CORNER::URB, 1);
        self.update_corner_orientation(CORNER::DRB, 2);
        self.update_corner_orientation(CORNER::DRF, 1);

        self
    }

    /// 逆时针转动右面
    pub fn r_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = self.corners[CORNER::URF as usize];
        self.corners[CORNER::URF as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = hold;

        let hold = self.edges[EDGE::BR as usize];
        self.edges[EDGE::BR as usize] = self.edges[EDGE::DR as usize];
        self.edges[EDGE::DR as usize] = self.edges[EDGE::FR as usize];
        self.edges[EDGE::FR as usize] = self.edges[EDGE::UR as usize];
        self.edges[EDGE::UR as usize] = hold;

        self.update_corner_orientation(CORNER::URF, 2);
        self.update_corner_orientation(CORNER::URB, 1);
        self.update_corner_orientation(CORNER::DRB, 2);
        self.update_corner_orientation(CORNER::DRF, 1);

        self
    }

    /// 右面转2次
    pub fn r2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::URF as usize, CORNER::DRB as usize);
        self.corners
            .swap(CORNER::URB as usize, CORNER::DRF as usize);

        self.edges.swap(EDGE::UR as usize, EDGE::DR as usize);
        self.edges.swap(EDGE::FR as usize, EDGE::BR as usize);

        self
    }

    /// 顺时针转动后面
    pub fn b(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = hold;

        let hold = self.edges[EDGE::UB as usize];
        self.edges[EDGE::UB as usize] = self.edges[EDGE::BR as usize];
        self.edges[EDGE::BR as usize] = self.edges[EDGE::DB as usize];
        self.edges[EDGE::DB as usize] = self.edges[EDGE::BL as usize];
        self.edges[EDGE::BL as usize] = hold;

        self.update_corner_orientation(CORNER::URB, 2);
        self.update_corner_orientation(CORNER::ULB, 1);
        self.update_corner_orientation(CORNER::DRB, 1);
        self.update_corner_orientation(CORNER::DLB, 2);

        self.update_edge_orientation_z(EDGE::UB);
        self.update_edge_orientation_z(EDGE::BL);
        self.update_edge_orientation_z(EDGE::DB);
        self.update_edge_orientation_z(EDGE::BR);

        self
    }

    /// 逆时针转动后面
    pub fn b_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::ULB as usize];
        self.corners[CORNER::ULB as usize] = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::URB as usize];
        self.corners[CORNER::URB as usize] = hold;

        let hold = self.edges[EDGE::UB as usize];
        self.edges[EDGE::UB as usize] = self.edges[EDGE::BL as usize];
        self.edges[EDGE::BL as usize] = self.edges[EDGE::DB as usize];
        self.edges[EDGE::DB as usize] = self.edges[EDGE::BR as usize];
        self.edges[EDGE::BR as usize] = hold;

        self.update_corner_orientation(CORNER::URB, 2);
        self.update_corner_orientation(CORNER::ULB, 1);
        self.update_corner_orientation(CORNER::DRB, 1);
        self.update_corner_orientation(CORNER::DLB, 2);

        self.update_edge_orientation_z(EDGE::UB);
        self.update_edge_orientation_z(EDGE::BL);
        self.update_edge_orientation_z(EDGE::DB);
        self.update_edge_orientation_z(EDGE::BR);

        self
    }

    /// 后面转2次
    pub fn b2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::ULB as usize, CORNER::DRB as usize);
        self.corners
            .swap(CORNER::URB as usize, CORNER::DLB as usize);

        self.edges.swap(EDGE::UB as usize, EDGE::DB as usize);
        self.edges.swap(EDGE::BL as usize, EDGE::BR as usize);

        self
    }

    /// 顺时针转动下面
    pub fn d(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = hold;

        let hold = self.edges[EDGE::DB as usize];
        self.edges[EDGE::DB as usize] = self.edges[EDGE::DR as usize];
        self.edges[EDGE::DR as usize] = self.edges[EDGE::DF as usize];
        self.edges[EDGE::DF as usize] = self.edges[EDGE::DL as usize];
        self.edges[EDGE::DL as usize] = hold;

        self
    }

    /// 逆时针转动下面
    pub fn d_prime(&mut self) -> &mut Self {
        let hold = self.corners[CORNER::DLF as usize];
        self.corners[CORNER::DLF as usize] = self.corners[CORNER::DRF as usize];
        self.corners[CORNER::DRF as usize] = self.corners[CORNER::DRB as usize];
        self.corners[CORNER::DRB as usize] = self.corners[CORNER::DLB as usize];
        self.corners[CORNER::DLB as usize] = hold;

        let hold = self.edges[EDGE::DB as usize];
        self.edges[EDGE::DB as usize] = self.edges[EDGE::DL as usize];
        self.edges[EDGE::DL as usize] = self.edges[EDGE::DF as usize];
        self.edges[EDGE::DF as usize] = self.edges[EDGE::DR as usize];
        self.edges[EDGE::DR as usize] = hold;

        self
    }

    /// 下面转2次
    pub fn d2(&mut self) -> &mut Self {
        self.corners
            .swap(CORNER::DLB as usize, CORNER::DRF as usize);
        self.corners
            .swap(CORNER::DLF as usize, CORNER::DRB as usize);

        self.edges.swap(EDGE::DB as usize, EDGE::DF as usize);
        self.edges.swap(EDGE::DL as usize, EDGE::DR as usize);

        self
    }
}
