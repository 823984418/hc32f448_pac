#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xec],
    fcntr: Fcntr,
    sstar: Sstar,
    sstpr: Sstpr,
    sclrr: Sclrr,
    supdr: Supdr,
}
impl RegisterBlock {
    #[doc = "0xec - desc FCNTR"]
    #[inline(always)]
    pub const fn fcntr(&self) -> &Fcntr {
        &self.fcntr
    }
    #[doc = "0xf0 - desc SSTAR"]
    #[inline(always)]
    pub const fn sstar(&self) -> &Sstar {
        &self.sstar
    }
    #[doc = "0xf4 - desc SSTPR"]
    #[inline(always)]
    pub const fn sstpr(&self) -> &Sstpr {
        &self.sstpr
    }
    #[doc = "0xf8 - desc SCLRR"]
    #[inline(always)]
    pub const fn sclrr(&self) -> &Sclrr {
        &self.sclrr
    }
    #[doc = "0xfc - desc SUPDR"]
    #[inline(always)]
    pub const fn supdr(&self) -> &Supdr {
        &self.supdr
    }
}
#[doc = "FCNTR (rw) register accessor: desc FCNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fcntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcntr`] module"]
#[doc(alias = "FCNTR")]
pub type Fcntr = crate::Reg<fcntr::FcntrSpec>;
#[doc = "desc FCNTR"]
pub mod fcntr;
#[doc = "SSTAR (rw) register accessor: desc SSTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`sstar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstar`] module"]
#[doc(alias = "SSTAR")]
pub type Sstar = crate::Reg<sstar::SstarSpec>;
#[doc = "desc SSTAR"]
pub mod sstar;
#[doc = "SSTPR (rw) register accessor: desc SSTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`sstpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstpr`] module"]
#[doc(alias = "SSTPR")]
pub type Sstpr = crate::Reg<sstpr::SstprSpec>;
#[doc = "desc SSTPR"]
pub mod sstpr;
#[doc = "SCLRR (rw) register accessor: desc SCLRR\n\nYou can [`read`](crate::Reg::read) this register and get [`sclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sclrr`] module"]
#[doc(alias = "SCLRR")]
pub type Sclrr = crate::Reg<sclrr::SclrrSpec>;
#[doc = "desc SCLRR"]
pub mod sclrr;
#[doc = "SUPDR (rw) register accessor: desc SUPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`supdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supdr`] module"]
#[doc(alias = "SUPDR")]
pub type Supdr = crate::Reg<supdr::SupdrSpec>;
#[doc = "desc SUPDR"]
pub mod supdr;
