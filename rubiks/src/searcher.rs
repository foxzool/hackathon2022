use crate::core::MOVE as M;

pub struct MovePruner;

impl MovePruner {
    /// 检查准备移动是否要跳过
    pub fn prune(check: M, last: M) -> bool {
        if (check == M::L || check == M::LPRIME || check == M::L2)
            && (last == M::L || last == M::LPRIME || last == M::L2)
        {
            return true;
        }

        if (check == M::R || check == M::RPRIME || check == M::R2)
            && (last == M::R || last == M::RPRIME || last == M::R2)
        {
            return true;
        }

        if (check == M::U || check == M::UPRIME || check == M::U2)
            && (last == M::U || last == M::UPRIME || last == M::U2)
        {
            return true;
        }

        if (check == M::D || check == M::DPRIME || check == M::D2)
            && (last == M::D || last == M::DPRIME || last == M::D2)
        {
            return true;
        }

        if (check == M::F || check == M::FPRIME || check == M::F2)
            && (last == M::F || last == M::FPRIME || last == M::F2)
        {
            return true;
        }

        if (check == M::B || check == M::BPRIME || check == M::B2)
            && (last == M::B || last == M::BPRIME || last == M::B2)
        {
            return true;
        }

        //
        if (check == M::F || check == M::FPRIME || check == M::F2)
            && (last == M::B || last == M::BPRIME || last == M::B2)
        {
            return true;
        }

        if (check == M::L || check == M::LPRIME || check == M::L2)
            && (last == M::R || last == M::RPRIME || last == M::R2)
        {
            return true;
        }

        if (check == M::U || check == M::UPRIME || check == M::U2)
            && (last == M::D || last == M::DPRIME || last == M::D2)
        {
            return true;
        }

        false
    }
}
