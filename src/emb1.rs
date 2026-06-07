#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl1: Ctl1,
    ctl2: Ctl2,
    soe: Soe,
    stat: Stat,
    statclr: Statclr,
    inten: Inten,
    rlssel: Rlssel,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CTL1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x04 - desc CTL2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x08 - desc SOE"]
    #[inline(always)]
    pub const fn soe(&self) -> &Soe {
        &self.soe
    }
    #[doc = "0x0c - desc STAT"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - desc STATCLR"]
    #[inline(always)]
    pub const fn statclr(&self) -> &Statclr {
        &self.statclr
    }
    #[doc = "0x14 - desc INTEN"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x18 - desc RLSSEL"]
    #[inline(always)]
    pub const fn rlssel(&self) -> &Rlssel {
        &self.rlssel
    }
}
#[doc = "CTL1 (rw) register accessor: desc CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "desc CTL1"]
pub mod ctl1;
#[doc = "CTL2 (rw) register accessor: desc CTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`] module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "desc CTL2"]
pub mod ctl2;
#[doc = "SOE (rw) register accessor: desc SOE\n\nYou can [`read`](crate::Reg::read) this register and get [`soe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soe`] module"]
#[doc(alias = "SOE")]
pub type Soe = crate::Reg<soe::SoeSpec>;
#[doc = "desc SOE"]
pub mod soe;
#[doc = "STAT (r) register accessor: desc STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "desc STAT"]
pub mod stat;
#[doc = "STATCLR (w) register accessor: desc STATCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statclr`] module"]
#[doc(alias = "STATCLR")]
pub type Statclr = crate::Reg<statclr::StatclrSpec>;
#[doc = "desc STATCLR"]
pub mod statclr;
#[doc = "INTEN (rw) register accessor: desc INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "desc INTEN"]
pub mod inten;
#[doc = "RLSSEL (rw) register accessor: desc RLSSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`rlssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlssel`] module"]
#[doc(alias = "RLSSEL")]
pub type Rlssel = crate::Reg<rlssel::RlsselSpec>;
#[doc = "desc RLSSEL"]
pub mod rlssel;
