#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ckcr: Ckcr,
    ckpr: Ckpr,
    cksr: Cksr,
    sram0_eien: Sram0Eien,
    sram0_eibit0: Sram0Eibit0,
    sram0_eibit1: Sram0Eibit1,
    sram0_eccerraddr: Sram0Eccerraddr,
    sramb_eien: SrambEien,
    sramb_eibit0: SrambEibit0,
    sramb_eibit1: SrambEibit1,
    sramb_eccerraddr: SrambEccerraddr,
}
impl RegisterBlock {
    #[doc = "0x08 - desc CKCR"]
    #[inline(always)]
    pub const fn ckcr(&self) -> &Ckcr {
        &self.ckcr
    }
    #[doc = "0x0c - desc CKPR"]
    #[inline(always)]
    pub const fn ckpr(&self) -> &Ckpr {
        &self.ckpr
    }
    #[doc = "0x10 - desc CKSR"]
    #[inline(always)]
    pub const fn cksr(&self) -> &Cksr {
        &self.cksr
    }
    #[doc = "0x14 - desc SRAM0_EIEN"]
    #[inline(always)]
    pub const fn sram0_eien(&self) -> &Sram0Eien {
        &self.sram0_eien
    }
    #[doc = "0x18 - desc SRAM0_EIBIT0"]
    #[inline(always)]
    pub const fn sram0_eibit0(&self) -> &Sram0Eibit0 {
        &self.sram0_eibit0
    }
    #[doc = "0x1c - desc SRAM0_EIBIT1"]
    #[inline(always)]
    pub const fn sram0_eibit1(&self) -> &Sram0Eibit1 {
        &self.sram0_eibit1
    }
    #[doc = "0x20 - desc SRAM0_ECCERRADDR"]
    #[inline(always)]
    pub const fn sram0_eccerraddr(&self) -> &Sram0Eccerraddr {
        &self.sram0_eccerraddr
    }
    #[doc = "0x24 - desc SRAMB_EIEN"]
    #[inline(always)]
    pub const fn sramb_eien(&self) -> &SrambEien {
        &self.sramb_eien
    }
    #[doc = "0x28 - desc SRAMB_EIBIT0"]
    #[inline(always)]
    pub const fn sramb_eibit0(&self) -> &SrambEibit0 {
        &self.sramb_eibit0
    }
    #[doc = "0x2c - desc SRAMB_EIBIT1"]
    #[inline(always)]
    pub const fn sramb_eibit1(&self) -> &SrambEibit1 {
        &self.sramb_eibit1
    }
    #[doc = "0x30 - desc SRAMB_ECCERRADDR"]
    #[inline(always)]
    pub const fn sramb_eccerraddr(&self) -> &SrambEccerraddr {
        &self.sramb_eccerraddr
    }
}
#[doc = "CKCR (rw) register accessor: desc CKCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckcr`] module"]
#[doc(alias = "CKCR")]
pub type Ckcr = crate::Reg<ckcr::CkcrSpec>;
#[doc = "desc CKCR"]
pub mod ckcr;
#[doc = "CKPR (rw) register accessor: desc CKPR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckpr`] module"]
#[doc(alias = "CKPR")]
pub type Ckpr = crate::Reg<ckpr::CkprSpec>;
#[doc = "desc CKPR"]
pub mod ckpr;
#[doc = "CKSR (rw) register accessor: desc CKSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cksr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cksr`] module"]
#[doc(alias = "CKSR")]
pub type Cksr = crate::Reg<cksr::CksrSpec>;
#[doc = "desc CKSR"]
pub mod cksr;
#[doc = "SRAM0_EIEN (rw) register accessor: desc SRAM0_EIEN\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0_eien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram0_eien`] module"]
#[doc(alias = "SRAM0_EIEN")]
pub type Sram0Eien = crate::Reg<sram0_eien::Sram0EienSpec>;
#[doc = "desc SRAM0_EIEN"]
pub mod sram0_eien;
#[doc = "SRAM0_EIBIT0 (rw) register accessor: desc SRAM0_EIBIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eibit0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0_eibit0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram0_eibit0`] module"]
#[doc(alias = "SRAM0_EIBIT0")]
pub type Sram0Eibit0 = crate::Reg<sram0_eibit0::Sram0Eibit0Spec>;
#[doc = "desc SRAM0_EIBIT0"]
pub mod sram0_eibit0;
#[doc = "SRAM0_EIBIT1 (rw) register accessor: desc SRAM0_EIBIT1\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eibit1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0_eibit1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram0_eibit1`] module"]
#[doc(alias = "SRAM0_EIBIT1")]
pub type Sram0Eibit1 = crate::Reg<sram0_eibit1::Sram0Eibit1Spec>;
#[doc = "desc SRAM0_EIBIT1"]
pub mod sram0_eibit1;
#[doc = "SRAM0_ECCERRADDR (r) register accessor: desc SRAM0_ECCERRADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eccerraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram0_eccerraddr`] module"]
#[doc(alias = "SRAM0_ECCERRADDR")]
pub type Sram0Eccerraddr = crate::Reg<sram0_eccerraddr::Sram0EccerraddrSpec>;
#[doc = "desc SRAM0_ECCERRADDR"]
pub mod sram0_eccerraddr;
#[doc = "SRAMB_EIEN (rw) register accessor: desc SRAMB_EIEN\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramb_eien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramb_eien`] module"]
#[doc(alias = "SRAMB_EIEN")]
pub type SrambEien = crate::Reg<sramb_eien::SrambEienSpec>;
#[doc = "desc SRAMB_EIEN"]
pub mod sramb_eien;
#[doc = "SRAMB_EIBIT0 (rw) register accessor: desc SRAMB_EIBIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eibit0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramb_eibit0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramb_eibit0`] module"]
#[doc(alias = "SRAMB_EIBIT0")]
pub type SrambEibit0 = crate::Reg<sramb_eibit0::SrambEibit0Spec>;
#[doc = "desc SRAMB_EIBIT0"]
pub mod sramb_eibit0;
#[doc = "SRAMB_EIBIT1 (rw) register accessor: desc SRAMB_EIBIT1\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eibit1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramb_eibit1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramb_eibit1`] module"]
#[doc(alias = "SRAMB_EIBIT1")]
pub type SrambEibit1 = crate::Reg<sramb_eibit1::SrambEibit1Spec>;
#[doc = "desc SRAMB_EIBIT1"]
pub mod sramb_eibit1;
#[doc = "SRAMB_ECCERRADDR (r) register accessor: desc SRAMB_ECCERRADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eccerraddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sramb_eccerraddr`] module"]
#[doc(alias = "SRAMB_ECCERRADDR")]
pub type SrambEccerraddr = crate::Reg<sramb_eccerraddr::SrambEccerraddrSpec>;
#[doc = "desc SRAMB_ECCERRADDR"]
pub mod sramb_eccerraddr;
