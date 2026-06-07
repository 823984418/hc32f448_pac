#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stsr: Stsr,
    _reserved1: [u8; 0x04],
    stcr0: Stcr0,
    stcr1: Stcr1,
    cmdr: Cmdr,
    tmcr: Tmcr,
    cpcr: Cpcr,
    _reserved6: [u8; 0x04],
    rftr: Rftr,
    _reserved7: [u8; 0xdc],
    tmsr0: Tmsr0,
    cpsr0: Cpsr0,
    _reserved9: [u8; 0xf8],
    bacr: Bacr,
    _reserved10: [u8; 0x04],
    cscr0: Cscr0,
    cscr1: Cscr1,
}
impl RegisterBlock {
    #[doc = "0x00 - desc STSR"]
    #[inline(always)]
    pub const fn stsr(&self) -> &Stsr {
        &self.stsr
    }
    #[doc = "0x08 - desc STCR0"]
    #[inline(always)]
    pub const fn stcr0(&self) -> &Stcr0 {
        &self.stcr0
    }
    #[doc = "0x0c - desc STCR1"]
    #[inline(always)]
    pub const fn stcr1(&self) -> &Stcr1 {
        &self.stcr1
    }
    #[doc = "0x10 - desc CMDR"]
    #[inline(always)]
    pub const fn cmdr(&self) -> &Cmdr {
        &self.cmdr
    }
    #[doc = "0x14 - desc TMCR"]
    #[inline(always)]
    pub const fn tmcr(&self) -> &Tmcr {
        &self.tmcr
    }
    #[doc = "0x18 - desc CPCR"]
    #[inline(always)]
    pub const fn cpcr(&self) -> &Cpcr {
        &self.cpcr
    }
    #[doc = "0x20 - desc RFTR"]
    #[inline(always)]
    pub const fn rftr(&self) -> &Rftr {
        &self.rftr
    }
    #[doc = "0x100 - desc TMSR0"]
    #[inline(always)]
    pub const fn tmsr0(&self) -> &Tmsr0 {
        &self.tmsr0
    }
    #[doc = "0x104 - desc CPSR0"]
    #[inline(always)]
    pub const fn cpsr0(&self) -> &Cpsr0 {
        &self.cpsr0
    }
    #[doc = "0x200 - desc BACR"]
    #[inline(always)]
    pub const fn bacr(&self) -> &Bacr {
        &self.bacr
    }
    #[doc = "0x208 - desc CSCR0"]
    #[inline(always)]
    pub const fn cscr0(&self) -> &Cscr0 {
        &self.cscr0
    }
    #[doc = "0x20c - desc CSCR1"]
    #[inline(always)]
    pub const fn cscr1(&self) -> &Cscr1 {
        &self.cscr1
    }
}
#[doc = "STSR (r) register accessor: desc STSR\n\nYou can [`read`](crate::Reg::read) this register and get [`stsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stsr`] module"]
#[doc(alias = "STSR")]
pub type Stsr = crate::Reg<stsr::StsrSpec>;
#[doc = "desc STSR"]
pub mod stsr;
#[doc = "STCR0 (w) register accessor: desc STCR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcr0`] module"]
#[doc(alias = "STCR0")]
pub type Stcr0 = crate::Reg<stcr0::Stcr0Spec>;
#[doc = "desc STCR0"]
pub mod stcr0;
#[doc = "STCR1 (w) register accessor: desc STCR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcr1`] module"]
#[doc(alias = "STCR1")]
pub type Stcr1 = crate::Reg<stcr1::Stcr1Spec>;
#[doc = "desc STCR1"]
pub mod stcr1;
#[doc = "CMDR (w) register accessor: desc CMDR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`] module"]
#[doc(alias = "CMDR")]
pub type Cmdr = crate::Reg<cmdr::CmdrSpec>;
#[doc = "desc CMDR"]
pub mod cmdr;
#[doc = "TMCR (w) register accessor: desc TMCR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmcr`] module"]
#[doc(alias = "TMCR")]
pub type Tmcr = crate::Reg<tmcr::TmcrSpec>;
#[doc = "desc TMCR"]
pub mod tmcr;
#[doc = "CPCR (w) register accessor: desc CPCR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpcr`] module"]
#[doc(alias = "CPCR")]
pub type Cpcr = crate::Reg<cpcr::CpcrSpec>;
#[doc = "desc CPCR"]
pub mod cpcr;
#[doc = "RFTR (rw) register accessor: desc RFTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rftr`] module"]
#[doc(alias = "RFTR")]
pub type Rftr = crate::Reg<rftr::RftrSpec>;
#[doc = "desc RFTR"]
pub mod rftr;
#[doc = "TMSR0 (r) register accessor: desc TMSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmsr0`] module"]
#[doc(alias = "TMSR0")]
pub type Tmsr0 = crate::Reg<tmsr0::Tmsr0Spec>;
#[doc = "desc TMSR0"]
pub mod tmsr0;
#[doc = "CPSR0 (r) register accessor: desc CPSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr0`] module"]
#[doc(alias = "CPSR0")]
pub type Cpsr0 = crate::Reg<cpsr0::Cpsr0Spec>;
#[doc = "desc CPSR0"]
pub mod cpsr0;
#[doc = "BACR (rw) register accessor: desc BACR\n\nYou can [`read`](crate::Reg::read) this register and get [`bacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bacr`] module"]
#[doc(alias = "BACR")]
pub type Bacr = crate::Reg<bacr::BacrSpec>;
#[doc = "desc BACR"]
pub mod bacr;
#[doc = "CSCR0 (rw) register accessor: desc CSCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cscr0`] module"]
#[doc(alias = "CSCR0")]
pub type Cscr0 = crate::Reg<cscr0::Cscr0Spec>;
#[doc = "desc CSCR0"]
pub mod cscr0;
#[doc = "CSCR1 (rw) register accessor: desc CSCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cscr1`] module"]
#[doc(alias = "CSCR1")]
pub type Cscr1 = crate::Reg<cscr1::Cscr1Spec>;
#[doc = "desc CSCR1"]
pub mod cscr1;
