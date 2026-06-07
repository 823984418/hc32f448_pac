#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x18],
    xtaldivr: Xtaldivr,
    xtaldivcr: Xtaldivcr,
    _reserved2: [u8; 0x4c30],
    xtalcfgr: Xtalcfgr,
    _reserved3: [u8; 0x03],
    xtal32cr: Xtal32cr,
    _reserved4: [u8; 0x03],
    xtal32cfgr: Xtal32cfgr,
    _reserved5: [u8; 0x0f],
    xtal32nfr: Xtal32nfr,
    _reserved6: [u8; 0x03],
    lrccr: Lrccr,
    _reserved7: [u8; 0x07],
    lrctrm: Lrctrm,
    _reserved8: [u8; 0x739b],
    pericksel: Pericksel,
    _reserved9: [u8; 0x06],
    canckcfgr: Canckcfgr,
    _reserved10: [u8; 0x06],
    scfgr: Scfgr,
    _reserved11: [u8; 0x02],
    ckswr: Ckswr,
    _reserved12: [u8; 0x03],
    pllhcr: Pllhcr,
    _reserved13: [u8; 0x07],
    xtalcr: Xtalcr,
    _reserved14: [u8; 0x03],
    hrccr: Hrccr,
    _reserved15: [u8; 0x01],
    mrccr: Mrccr,
    _reserved16: [u8; 0x03],
    oscstbsr: Oscstbsr,
    mco1cfgr: Mco1cfgr,
    mco2cfgr: Mco2cfgr,
    tpiuckcfgr: Tpiuckcfgr,
    xtalstdcr: Xtalstdcr,
    xtalstdsr: Xtalstdsr,
    _reserved22: [u8; 0x1f],
    mrctrm: Mrctrm,
    hrctrm: Hrctrm,
    _reserved24: [u8; 0x3f],
    xtalstbcr: Xtalstbcr,
    _reserved25: [u8; 0x5d],
    pllhcfgr: Pllhcfgr,
}
impl RegisterBlock {
    #[doc = "0x18 - desc XTALDIVR"]
    #[inline(always)]
    pub const fn xtaldivr(&self) -> &Xtaldivr {
        &self.xtaldivr
    }
    #[doc = "0x1c - desc XTALDIVCR"]
    #[inline(always)]
    pub const fn xtaldivcr(&self) -> &Xtaldivcr {
        &self.xtaldivcr
    }
    #[doc = "0x4c50 - desc XTALCFGR"]
    #[inline(always)]
    pub const fn xtalcfgr(&self) -> &Xtalcfgr {
        &self.xtalcfgr
    }
    #[doc = "0x4c54 - desc XTAL32CR"]
    #[inline(always)]
    pub const fn xtal32cr(&self) -> &Xtal32cr {
        &self.xtal32cr
    }
    #[doc = "0x4c58 - desc XTAL32CFGR"]
    #[inline(always)]
    pub const fn xtal32cfgr(&self) -> &Xtal32cfgr {
        &self.xtal32cfgr
    }
    #[doc = "0x4c68 - desc XTAL32NFR"]
    #[inline(always)]
    pub const fn xtal32nfr(&self) -> &Xtal32nfr {
        &self.xtal32nfr
    }
    #[doc = "0x4c6c - desc LRCCR"]
    #[inline(always)]
    pub const fn lrccr(&self) -> &Lrccr {
        &self.lrccr
    }
    #[doc = "0x4c74 - desc LRCTRM"]
    #[inline(always)]
    pub const fn lrctrm(&self) -> &Lrctrm {
        &self.lrctrm
    }
    #[doc = "0xc010 - desc PERICKSEL"]
    #[inline(always)]
    pub const fn pericksel(&self) -> &Pericksel {
        &self.pericksel
    }
    #[doc = "0xc018 - desc CANCKCFGR"]
    #[inline(always)]
    pub const fn canckcfgr(&self) -> &Canckcfgr {
        &self.canckcfgr
    }
    #[doc = "0xc020 - desc SCFGR"]
    #[inline(always)]
    pub const fn scfgr(&self) -> &Scfgr {
        &self.scfgr
    }
    #[doc = "0xc026 - desc CKSWR"]
    #[inline(always)]
    pub const fn ckswr(&self) -> &Ckswr {
        &self.ckswr
    }
    #[doc = "0xc02a - desc PLLHCR"]
    #[inline(always)]
    pub const fn pllhcr(&self) -> &Pllhcr {
        &self.pllhcr
    }
    #[doc = "0xc032 - desc XTALCR"]
    #[inline(always)]
    pub const fn xtalcr(&self) -> &Xtalcr {
        &self.xtalcr
    }
    #[doc = "0xc036 - desc HRCCR"]
    #[inline(always)]
    pub const fn hrccr(&self) -> &Hrccr {
        &self.hrccr
    }
    #[doc = "0xc038 - desc MRCCR"]
    #[inline(always)]
    pub const fn mrccr(&self) -> &Mrccr {
        &self.mrccr
    }
    #[doc = "0xc03c - desc OSCSTBSR"]
    #[inline(always)]
    pub const fn oscstbsr(&self) -> &Oscstbsr {
        &self.oscstbsr
    }
    #[doc = "0xc03d - desc MCO1CFGR"]
    #[inline(always)]
    pub const fn mco1cfgr(&self) -> &Mco1cfgr {
        &self.mco1cfgr
    }
    #[doc = "0xc03e - desc MCO2CFGR"]
    #[inline(always)]
    pub const fn mco2cfgr(&self) -> &Mco2cfgr {
        &self.mco2cfgr
    }
    #[doc = "0xc03f - desc TPIUCKCFGR"]
    #[inline(always)]
    pub const fn tpiuckcfgr(&self) -> &Tpiuckcfgr {
        &self.tpiuckcfgr
    }
    #[doc = "0xc040 - desc XTALSTDCR"]
    #[inline(always)]
    pub const fn xtalstdcr(&self) -> &Xtalstdcr {
        &self.xtalstdcr
    }
    #[doc = "0xc041 - desc XTALSTDSR"]
    #[inline(always)]
    pub const fn xtalstdsr(&self) -> &Xtalstdsr {
        &self.xtalstdsr
    }
    #[doc = "0xc061 - desc MRCTRM"]
    #[inline(always)]
    pub const fn mrctrm(&self) -> &Mrctrm {
        &self.mrctrm
    }
    #[doc = "0xc062 - desc HRCTRM"]
    #[inline(always)]
    pub const fn hrctrm(&self) -> &Hrctrm {
        &self.hrctrm
    }
    #[doc = "0xc0a2 - desc XTALSTBCR"]
    #[inline(always)]
    pub const fn xtalstbcr(&self) -> &Xtalstbcr {
        &self.xtalstbcr
    }
    #[doc = "0xc100 - desc PLLHCFGR"]
    #[inline(always)]
    pub const fn pllhcfgr(&self) -> &Pllhcfgr {
        &self.pllhcfgr
    }
}
#[doc = "XTALDIVR (rw) register accessor: desc XTALDIVR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtaldivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtaldivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtaldivr`] module"]
#[doc(alias = "XTALDIVR")]
pub type Xtaldivr = crate::Reg<xtaldivr::XtaldivrSpec>;
#[doc = "desc XTALDIVR"]
pub mod xtaldivr;
#[doc = "XTALDIVCR (rw) register accessor: desc XTALDIVCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtaldivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtaldivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtaldivcr`] module"]
#[doc(alias = "XTALDIVCR")]
pub type Xtaldivcr = crate::Reg<xtaldivcr::XtaldivcrSpec>;
#[doc = "desc XTALDIVCR"]
pub mod xtaldivcr;
#[doc = "XTALCFGR (rw) register accessor: desc XTALCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcfgr`] module"]
#[doc(alias = "XTALCFGR")]
pub type Xtalcfgr = crate::Reg<xtalcfgr::XtalcfgrSpec>;
#[doc = "desc XTALCFGR"]
pub mod xtalcfgr;
#[doc = "XTAL32CR (rw) register accessor: desc XTAL32CR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32cr`] module"]
#[doc(alias = "XTAL32CR")]
pub type Xtal32cr = crate::Reg<xtal32cr::Xtal32crSpec>;
#[doc = "desc XTAL32CR"]
pub mod xtal32cr;
#[doc = "XTAL32CFGR (rw) register accessor: desc XTAL32CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32cfgr`] module"]
#[doc(alias = "XTAL32CFGR")]
pub type Xtal32cfgr = crate::Reg<xtal32cfgr::Xtal32cfgrSpec>;
#[doc = "desc XTAL32CFGR"]
pub mod xtal32cfgr;
#[doc = "XTAL32NFR (rw) register accessor: desc XTAL32NFR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32nfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32nfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32nfr`] module"]
#[doc(alias = "XTAL32NFR")]
pub type Xtal32nfr = crate::Reg<xtal32nfr::Xtal32nfrSpec>;
#[doc = "desc XTAL32NFR"]
pub mod xtal32nfr;
#[doc = "LRCCR (rw) register accessor: desc LRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`lrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrccr`] module"]
#[doc(alias = "LRCCR")]
pub type Lrccr = crate::Reg<lrccr::LrccrSpec>;
#[doc = "desc LRCCR"]
pub mod lrccr;
#[doc = "LRCTRM (rw) register accessor: desc LRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`lrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrctrm`] module"]
#[doc(alias = "LRCTRM")]
pub type Lrctrm = crate::Reg<lrctrm::LrctrmSpec>;
#[doc = "desc LRCTRM"]
pub mod lrctrm;
#[doc = "PERICKSEL (rw) register accessor: desc PERICKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`pericksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pericksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pericksel`] module"]
#[doc(alias = "PERICKSEL")]
pub type Pericksel = crate::Reg<pericksel::PerickselSpec>;
#[doc = "desc PERICKSEL"]
pub mod pericksel;
#[doc = "CANCKCFGR (rw) register accessor: desc CANCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`canckcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canckcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canckcfgr`] module"]
#[doc(alias = "CANCKCFGR")]
pub type Canckcfgr = crate::Reg<canckcfgr::CanckcfgrSpec>;
#[doc = "desc CANCKCFGR"]
pub mod canckcfgr;
#[doc = "SCFGR (rw) register accessor: desc SCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`scfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfgr`] module"]
#[doc(alias = "SCFGR")]
pub type Scfgr = crate::Reg<scfgr::ScfgrSpec>;
#[doc = "desc SCFGR"]
pub mod scfgr;
#[doc = "CKSWR (rw) register accessor: desc CKSWR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckswr`] module"]
#[doc(alias = "CKSWR")]
pub type Ckswr = crate::Reg<ckswr::CkswrSpec>;
#[doc = "desc CKSWR"]
pub mod ckswr;
#[doc = "PLLHCR (rw) register accessor: desc PLLHCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllhcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllhcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllhcr`] module"]
#[doc(alias = "PLLHCR")]
pub type Pllhcr = crate::Reg<pllhcr::PllhcrSpec>;
#[doc = "desc PLLHCR"]
pub mod pllhcr;
#[doc = "XTALCR (rw) register accessor: desc XTALCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcr`] module"]
#[doc(alias = "XTALCR")]
pub type Xtalcr = crate::Reg<xtalcr::XtalcrSpec>;
#[doc = "desc XTALCR"]
pub mod xtalcr;
#[doc = "HRCCR (rw) register accessor: desc HRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrccr`] module"]
#[doc(alias = "HRCCR")]
pub type Hrccr = crate::Reg<hrccr::HrccrSpec>;
#[doc = "desc HRCCR"]
pub mod hrccr;
#[doc = "MRCCR (rw) register accessor: desc MRCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrccr`] module"]
#[doc(alias = "MRCCR")]
pub type Mrccr = crate::Reg<mrccr::MrccrSpec>;
#[doc = "desc MRCCR"]
pub mod mrccr;
#[doc = "OSCSTBSR (rw) register accessor: desc OSCSTBSR\n\nYou can [`read`](crate::Reg::read) this register and get [`oscstbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscstbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscstbsr`] module"]
#[doc(alias = "OSCSTBSR")]
pub type Oscstbsr = crate::Reg<oscstbsr::OscstbsrSpec>;
#[doc = "desc OSCSTBSR"]
pub mod oscstbsr;
#[doc = "MCO1CFGR (rw) register accessor: desc MCO1CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`mco1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco1cfgr`] module"]
#[doc(alias = "MCO1CFGR")]
pub type Mco1cfgr = crate::Reg<mco1cfgr::Mco1cfgrSpec>;
#[doc = "desc MCO1CFGR"]
pub mod mco1cfgr;
#[doc = "MCO2CFGR (rw) register accessor: desc MCO2CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`mco2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco2cfgr`] module"]
#[doc(alias = "MCO2CFGR")]
pub type Mco2cfgr = crate::Reg<mco2cfgr::Mco2cfgrSpec>;
#[doc = "desc MCO2CFGR"]
pub mod mco2cfgr;
#[doc = "TPIUCKCFGR (rw) register accessor: desc TPIUCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`tpiuckcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpiuckcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpiuckcfgr`] module"]
#[doc(alias = "TPIUCKCFGR")]
pub type Tpiuckcfgr = crate::Reg<tpiuckcfgr::TpiuckcfgrSpec>;
#[doc = "desc TPIUCKCFGR"]
pub mod tpiuckcfgr;
#[doc = "XTALSTDCR (rw) register accessor: desc XTALSTDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstdcr`] module"]
#[doc(alias = "XTALSTDCR")]
pub type Xtalstdcr = crate::Reg<xtalstdcr::XtalstdcrSpec>;
#[doc = "desc XTALSTDCR"]
pub mod xtalstdcr;
#[doc = "XTALSTDSR (rw) register accessor: desc XTALSTDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstdsr`] module"]
#[doc(alias = "XTALSTDSR")]
pub type Xtalstdsr = crate::Reg<xtalstdsr::XtalstdsrSpec>;
#[doc = "desc XTALSTDSR"]
pub mod xtalstdsr;
#[doc = "MRCTRM (rw) register accessor: desc MRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`mrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrctrm`] module"]
#[doc(alias = "MRCTRM")]
pub type Mrctrm = crate::Reg<mrctrm::MrctrmSpec>;
#[doc = "desc MRCTRM"]
pub mod mrctrm;
#[doc = "HRCTRM (rw) register accessor: desc HRCTRM\n\nYou can [`read`](crate::Reg::read) this register and get [`hrctrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrctrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrctrm`] module"]
#[doc(alias = "HRCTRM")]
pub type Hrctrm = crate::Reg<hrctrm::HrctrmSpec>;
#[doc = "desc HRCTRM"]
pub mod hrctrm;
#[doc = "XTALSTBCR (rw) register accessor: desc XTALSTBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalstbcr`] module"]
#[doc(alias = "XTALSTBCR")]
pub type Xtalstbcr = crate::Reg<xtalstbcr::XtalstbcrSpec>;
#[doc = "desc XTALSTBCR"]
pub mod xtalstbcr;
#[doc = "PLLHCFGR (rw) register accessor: desc PLLHCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`pllhcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllhcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllhcfgr`] module"]
#[doc(alias = "PLLHCFGR")]
pub type Pllhcfgr = crate::Reg<pllhcfgr::PllhcfgrSpec>;
#[doc = "desc PLLHCFGR"]
pub mod pllhcfgr;
