#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdr: Mdr,
    fir: Fir,
    ocr: Ocr,
    _reserved3: [u8; 0x01],
    pmsr: Pmsr,
    _reserved4: [u8; 0x08],
    bwsr1: Bwsr1,
    bwsr2: Bwsr2,
    _reserved6: [u8; 0x02],
    sccr: Sccr,
    scmr: Scmr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc MDR"]
    #[inline(always)]
    pub const fn mdr(&self) -> &Mdr {
        &self.mdr
    }
    #[doc = "0x01 - desc FIR"]
    #[inline(always)]
    pub const fn fir(&self) -> &Fir {
        &self.fir
    }
    #[doc = "0x02 - desc OCR"]
    #[inline(always)]
    pub const fn ocr(&self) -> &Ocr {
        &self.ocr
    }
    #[doc = "0x04 - desc PMSR"]
    #[inline(always)]
    pub const fn pmsr(&self) -> &Pmsr {
        &self.pmsr
    }
    #[doc = "0x10 - desc BWSR1"]
    #[inline(always)]
    pub const fn bwsr1(&self) -> &Bwsr1 {
        &self.bwsr1
    }
    #[doc = "0x14 - desc BWSR2"]
    #[inline(always)]
    pub const fn bwsr2(&self) -> &Bwsr2 {
        &self.bwsr2
    }
    #[doc = "0x18 - desc SCCR"]
    #[inline(always)]
    pub const fn sccr(&self) -> &Sccr {
        &self.sccr
    }
    #[doc = "0x1c - desc SCMR"]
    #[inline(always)]
    pub const fn scmr(&self) -> &Scmr {
        &self.scmr
    }
}
#[doc = "MDR (rw) register accessor: desc MDR\n\nYou can [`read`](crate::Reg::read) this register and get [`mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`] module"]
#[doc(alias = "MDR")]
pub type Mdr = crate::Reg<mdr::MdrSpec>;
#[doc = "desc MDR"]
pub mod mdr;
#[doc = "FIR (rw) register accessor: desc FIR\n\nYou can [`read`](crate::Reg::read) this register and get [`fir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fir`] module"]
#[doc(alias = "FIR")]
pub type Fir = crate::Reg<fir::FirSpec>;
#[doc = "desc FIR"]
pub mod fir;
#[doc = "OCR (rw) register accessor: desc OCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr`] module"]
#[doc(alias = "OCR")]
pub type Ocr = crate::Reg<ocr::OcrSpec>;
#[doc = "desc OCR"]
pub mod ocr;
#[doc = "PMSR (rw) register accessor: desc PMSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmsr`] module"]
#[doc(alias = "PMSR")]
pub type Pmsr = crate::Reg<pmsr::PmsrSpec>;
#[doc = "desc PMSR"]
pub mod pmsr;
#[doc = "BWSR1 (rw) register accessor: desc BWSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`bwsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwsr1`] module"]
#[doc(alias = "BWSR1")]
pub type Bwsr1 = crate::Reg<bwsr1::Bwsr1Spec>;
#[doc = "desc BWSR1"]
pub mod bwsr1;
#[doc = "BWSR2 (rw) register accessor: desc BWSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`bwsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwsr2`] module"]
#[doc(alias = "BWSR2")]
pub type Bwsr2 = crate::Reg<bwsr2::Bwsr2Spec>;
#[doc = "desc BWSR2"]
pub mod bwsr2;
#[doc = "SCCR (rw) register accessor: desc SCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccr`] module"]
#[doc(alias = "SCCR")]
pub type Sccr = crate::Reg<sccr::SccrSpec>;
#[doc = "desc SCCR"]
pub mod sccr;
#[doc = "SCMR (rw) register accessor: desc SCMR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmr`] module"]
#[doc(alias = "SCMR")]
pub type Scmr = crate::Reg<scmr::ScmrSpec>;
#[doc = "desc SCMR"]
pub mod scmr;
