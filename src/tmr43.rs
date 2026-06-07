#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    occruh: Occruh,
    _reserved1: [u8; 0x02],
    occrul: Occrul,
    _reserved2: [u8; 0x02],
    occrvh: Occrvh,
    _reserved3: [u8; 0x02],
    occrvl: Occrvl,
    _reserved4: [u8; 0x02],
    occrwh: Occrwh,
    _reserved5: [u8; 0x02],
    occrwl: Occrwl,
    _reserved6: [u8; 0x02],
    occrxh: Occrxh,
    _reserved7: [u8; 0x02],
    occrxl: Occrxl,
    _reserved8: [u8; 0x02],
    ocsru: Ocsru,
    oceru: Oceru,
    ocsrv: Ocsrv,
    ocerv: Ocerv,
    ocsrw: Ocsrw,
    ocerw: Ocerw,
    ocsrx: Ocsrx,
    ocerx: Ocerx,
    ocmruh: Ocmruh,
    _reserved17: [u8; 0x02],
    ocmrul: Ocmrul,
    ocmrvh: Ocmrvh,
    _reserved19: [u8; 0x02],
    ocmrvl: Ocmrvl,
    ocmrwh: Ocmrwh,
    _reserved21: [u8; 0x02],
    ocmrwl: Ocmrwl,
    ocmrxh: Ocmrxh,
    _reserved23: [u8; 0x02],
    ocmrxl: Ocmrxl,
    cpsr: Cpsr,
    _reserved25: [u8; 0x02],
    cntr: Cntr,
    _reserved26: [u8; 0x02],
    ccsr: Ccsr,
    cvpr: Cvpr,
    pscr: Pscr,
    _reserved29: [u8; 0x22],
    pfsru: Pfsru,
    pdaru: Pdaru,
    pdbru: Pdbru,
    _reserved32: [u8; 0x02],
    pfsrv: Pfsrv,
    pdarv: Pdarv,
    pdbrv: Pdbrv,
    _reserved35: [u8; 0x02],
    pfsrw: Pfsrw,
    pdarw: Pdarw,
    pdbrw: Pdbrw,
    _reserved38: [u8; 0x02],
    pfsrx: Pfsrx,
    pdarx: Pdarx,
    pdbrx: Pdbrx,
    pocru: Pocru,
    _reserved42: [u8; 0x02],
    pocrv: Pocrv,
    _reserved43: [u8; 0x02],
    pocrw: Pocrw,
    _reserved44: [u8; 0x02],
    pocrx: Pocrx,
    _reserved45: [u8; 0x02],
    sccruh: Sccruh,
    _reserved46: [u8; 0x02],
    sccrul: Sccrul,
    _reserved47: [u8; 0x02],
    sccrvh: Sccrvh,
    _reserved48: [u8; 0x02],
    sccrvl: Sccrvl,
    _reserved49: [u8; 0x02],
    sccrwh: Sccrwh,
    _reserved50: [u8; 0x02],
    sccrwl: Sccrwl,
    _reserved51: [u8; 0x02],
    sccrxh: Sccrxh,
    _reserved52: [u8; 0x02],
    sccrxl: Sccrxl,
    _reserved53: [u8; 0x02],
    scsruh: Scsruh,
    scmruh: Scmruh,
    scsrul: Scsrul,
    scmrul: Scmrul,
    scsrvh: Scsrvh,
    scmrvh: Scmrvh,
    scsrvl: Scsrvl,
    scmrvl: Scmrvl,
    scsrwh: Scsrwh,
    scmrwh: Scmrwh,
    scsrwl: Scsrwl,
    scmrwl: Scmrwl,
    scsrxh: Scsrxh,
    scmrxh: Scmrxh,
    scsrxl: Scsrxl,
    scmrxl: Scmrxl,
    scer: Scer,
    _reserved70: [u8; 0x02],
    rcsr: Rcsr,
    scir: Scir,
    _reserved72: [u8; 0x02],
    scfr: Scfr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc OCCRUH"]
    #[inline(always)]
    pub const fn occruh(&self) -> &Occruh {
        &self.occruh
    }
    #[doc = "0x04 - desc OCCRUL"]
    #[inline(always)]
    pub const fn occrul(&self) -> &Occrul {
        &self.occrul
    }
    #[doc = "0x08 - desc OCCRVH"]
    #[inline(always)]
    pub const fn occrvh(&self) -> &Occrvh {
        &self.occrvh
    }
    #[doc = "0x0c - desc OCCRVL"]
    #[inline(always)]
    pub const fn occrvl(&self) -> &Occrvl {
        &self.occrvl
    }
    #[doc = "0x10 - desc OCCRWH"]
    #[inline(always)]
    pub const fn occrwh(&self) -> &Occrwh {
        &self.occrwh
    }
    #[doc = "0x14 - desc OCCRWL"]
    #[inline(always)]
    pub const fn occrwl(&self) -> &Occrwl {
        &self.occrwl
    }
    #[doc = "0x18 - desc OCCRXH"]
    #[inline(always)]
    pub const fn occrxh(&self) -> &Occrxh {
        &self.occrxh
    }
    #[doc = "0x1c - desc OCCRXL"]
    #[inline(always)]
    pub const fn occrxl(&self) -> &Occrxl {
        &self.occrxl
    }
    #[doc = "0x20 - desc OCSRU"]
    #[inline(always)]
    pub const fn ocsru(&self) -> &Ocsru {
        &self.ocsru
    }
    #[doc = "0x22 - desc OCERU"]
    #[inline(always)]
    pub const fn oceru(&self) -> &Oceru {
        &self.oceru
    }
    #[doc = "0x24 - desc OCSRV"]
    #[inline(always)]
    pub const fn ocsrv(&self) -> &Ocsrv {
        &self.ocsrv
    }
    #[doc = "0x26 - desc OCERV"]
    #[inline(always)]
    pub const fn ocerv(&self) -> &Ocerv {
        &self.ocerv
    }
    #[doc = "0x28 - desc OCSRW"]
    #[inline(always)]
    pub const fn ocsrw(&self) -> &Ocsrw {
        &self.ocsrw
    }
    #[doc = "0x2a - desc OCERW"]
    #[inline(always)]
    pub const fn ocerw(&self) -> &Ocerw {
        &self.ocerw
    }
    #[doc = "0x2c - desc OCSRX"]
    #[inline(always)]
    pub const fn ocsrx(&self) -> &Ocsrx {
        &self.ocsrx
    }
    #[doc = "0x2e - desc OCERX"]
    #[inline(always)]
    pub const fn ocerx(&self) -> &Ocerx {
        &self.ocerx
    }
    #[doc = "0x30 - desc OCMRUH"]
    #[inline(always)]
    pub const fn ocmruh(&self) -> &Ocmruh {
        &self.ocmruh
    }
    #[doc = "0x34 - desc OCMRUL"]
    #[inline(always)]
    pub const fn ocmrul(&self) -> &Ocmrul {
        &self.ocmrul
    }
    #[doc = "0x38 - desc OCMRVH"]
    #[inline(always)]
    pub const fn ocmrvh(&self) -> &Ocmrvh {
        &self.ocmrvh
    }
    #[doc = "0x3c - desc OCMRVL"]
    #[inline(always)]
    pub const fn ocmrvl(&self) -> &Ocmrvl {
        &self.ocmrvl
    }
    #[doc = "0x40 - desc OCMRWH"]
    #[inline(always)]
    pub const fn ocmrwh(&self) -> &Ocmrwh {
        &self.ocmrwh
    }
    #[doc = "0x44 - desc OCMRWL"]
    #[inline(always)]
    pub const fn ocmrwl(&self) -> &Ocmrwl {
        &self.ocmrwl
    }
    #[doc = "0x48 - desc OCMRXH"]
    #[inline(always)]
    pub const fn ocmrxh(&self) -> &Ocmrxh {
        &self.ocmrxh
    }
    #[doc = "0x4c - desc OCMRXL"]
    #[inline(always)]
    pub const fn ocmrxl(&self) -> &Ocmrxl {
        &self.ocmrxl
    }
    #[doc = "0x50 - desc CPSR"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
    #[doc = "0x54 - desc CNTR"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x58 - desc CCSR"]
    #[inline(always)]
    pub const fn ccsr(&self) -> &Ccsr {
        &self.ccsr
    }
    #[doc = "0x5a - desc CVPR"]
    #[inline(always)]
    pub const fn cvpr(&self) -> &Cvpr {
        &self.cvpr
    }
    #[doc = "0x5c - desc PSCR"]
    #[inline(always)]
    pub const fn pscr(&self) -> &Pscr {
        &self.pscr
    }
    #[doc = "0x82 - desc PFSRU"]
    #[inline(always)]
    pub const fn pfsru(&self) -> &Pfsru {
        &self.pfsru
    }
    #[doc = "0x84 - desc PDARU"]
    #[inline(always)]
    pub const fn pdaru(&self) -> &Pdaru {
        &self.pdaru
    }
    #[doc = "0x86 - desc PDBRU"]
    #[inline(always)]
    pub const fn pdbru(&self) -> &Pdbru {
        &self.pdbru
    }
    #[doc = "0x8a - desc PFSRV"]
    #[inline(always)]
    pub const fn pfsrv(&self) -> &Pfsrv {
        &self.pfsrv
    }
    #[doc = "0x8c - desc PDARV"]
    #[inline(always)]
    pub const fn pdarv(&self) -> &Pdarv {
        &self.pdarv
    }
    #[doc = "0x8e - desc PDBRV"]
    #[inline(always)]
    pub const fn pdbrv(&self) -> &Pdbrv {
        &self.pdbrv
    }
    #[doc = "0x92 - desc PFSRW"]
    #[inline(always)]
    pub const fn pfsrw(&self) -> &Pfsrw {
        &self.pfsrw
    }
    #[doc = "0x94 - desc PDARW"]
    #[inline(always)]
    pub const fn pdarw(&self) -> &Pdarw {
        &self.pdarw
    }
    #[doc = "0x96 - desc PDBRW"]
    #[inline(always)]
    pub const fn pdbrw(&self) -> &Pdbrw {
        &self.pdbrw
    }
    #[doc = "0x9a - desc PFSRX"]
    #[inline(always)]
    pub const fn pfsrx(&self) -> &Pfsrx {
        &self.pfsrx
    }
    #[doc = "0x9c - desc PDARX"]
    #[inline(always)]
    pub const fn pdarx(&self) -> &Pdarx {
        &self.pdarx
    }
    #[doc = "0x9e - desc PDBRX"]
    #[inline(always)]
    pub const fn pdbrx(&self) -> &Pdbrx {
        &self.pdbrx
    }
    #[doc = "0xa0 - desc POCRU"]
    #[inline(always)]
    pub const fn pocru(&self) -> &Pocru {
        &self.pocru
    }
    #[doc = "0xa4 - desc POCRV"]
    #[inline(always)]
    pub const fn pocrv(&self) -> &Pocrv {
        &self.pocrv
    }
    #[doc = "0xa8 - desc POCRW"]
    #[inline(always)]
    pub const fn pocrw(&self) -> &Pocrw {
        &self.pocrw
    }
    #[doc = "0xac - desc POCRX"]
    #[inline(always)]
    pub const fn pocrx(&self) -> &Pocrx {
        &self.pocrx
    }
    #[doc = "0xb0 - desc SCCRUH"]
    #[inline(always)]
    pub const fn sccruh(&self) -> &Sccruh {
        &self.sccruh
    }
    #[doc = "0xb4 - desc SCCRUL"]
    #[inline(always)]
    pub const fn sccrul(&self) -> &Sccrul {
        &self.sccrul
    }
    #[doc = "0xb8 - desc SCCRVH"]
    #[inline(always)]
    pub const fn sccrvh(&self) -> &Sccrvh {
        &self.sccrvh
    }
    #[doc = "0xbc - desc SCCRVL"]
    #[inline(always)]
    pub const fn sccrvl(&self) -> &Sccrvl {
        &self.sccrvl
    }
    #[doc = "0xc0 - desc SCCRWH"]
    #[inline(always)]
    pub const fn sccrwh(&self) -> &Sccrwh {
        &self.sccrwh
    }
    #[doc = "0xc4 - desc SCCRWL"]
    #[inline(always)]
    pub const fn sccrwl(&self) -> &Sccrwl {
        &self.sccrwl
    }
    #[doc = "0xc8 - desc SCCRXH"]
    #[inline(always)]
    pub const fn sccrxh(&self) -> &Sccrxh {
        &self.sccrxh
    }
    #[doc = "0xcc - desc SCCRXL"]
    #[inline(always)]
    pub const fn sccrxl(&self) -> &Sccrxl {
        &self.sccrxl
    }
    #[doc = "0xd0 - desc SCSRUH"]
    #[inline(always)]
    pub const fn scsruh(&self) -> &Scsruh {
        &self.scsruh
    }
    #[doc = "0xd2 - desc SCMRUH"]
    #[inline(always)]
    pub const fn scmruh(&self) -> &Scmruh {
        &self.scmruh
    }
    #[doc = "0xd4 - desc SCSRUL"]
    #[inline(always)]
    pub const fn scsrul(&self) -> &Scsrul {
        &self.scsrul
    }
    #[doc = "0xd6 - desc SCMRUL"]
    #[inline(always)]
    pub const fn scmrul(&self) -> &Scmrul {
        &self.scmrul
    }
    #[doc = "0xd8 - desc SCSRVH"]
    #[inline(always)]
    pub const fn scsrvh(&self) -> &Scsrvh {
        &self.scsrvh
    }
    #[doc = "0xda - desc SCMRVH"]
    #[inline(always)]
    pub const fn scmrvh(&self) -> &Scmrvh {
        &self.scmrvh
    }
    #[doc = "0xdc - desc SCSRVL"]
    #[inline(always)]
    pub const fn scsrvl(&self) -> &Scsrvl {
        &self.scsrvl
    }
    #[doc = "0xde - desc SCMRVL"]
    #[inline(always)]
    pub const fn scmrvl(&self) -> &Scmrvl {
        &self.scmrvl
    }
    #[doc = "0xe0 - desc SCSRWH"]
    #[inline(always)]
    pub const fn scsrwh(&self) -> &Scsrwh {
        &self.scsrwh
    }
    #[doc = "0xe2 - desc SCMRWH"]
    #[inline(always)]
    pub const fn scmrwh(&self) -> &Scmrwh {
        &self.scmrwh
    }
    #[doc = "0xe4 - desc SCSRWL"]
    #[inline(always)]
    pub const fn scsrwl(&self) -> &Scsrwl {
        &self.scsrwl
    }
    #[doc = "0xe6 - desc SCMRWL"]
    #[inline(always)]
    pub const fn scmrwl(&self) -> &Scmrwl {
        &self.scmrwl
    }
    #[doc = "0xe8 - desc SCSRXH"]
    #[inline(always)]
    pub const fn scsrxh(&self) -> &Scsrxh {
        &self.scsrxh
    }
    #[doc = "0xea - desc SCMRXH"]
    #[inline(always)]
    pub const fn scmrxh(&self) -> &Scmrxh {
        &self.scmrxh
    }
    #[doc = "0xec - desc SCSRXL"]
    #[inline(always)]
    pub const fn scsrxl(&self) -> &Scsrxl {
        &self.scsrxl
    }
    #[doc = "0xee - desc SCMRXL"]
    #[inline(always)]
    pub const fn scmrxl(&self) -> &Scmrxl {
        &self.scmrxl
    }
    #[doc = "0xf0 - desc SCER"]
    #[inline(always)]
    pub const fn scer(&self) -> &Scer {
        &self.scer
    }
    #[doc = "0xf4 - desc RCSR"]
    #[inline(always)]
    pub const fn rcsr(&self) -> &Rcsr {
        &self.rcsr
    }
    #[doc = "0xf8 - desc SCIR"]
    #[inline(always)]
    pub const fn scir(&self) -> &Scir {
        &self.scir
    }
    #[doc = "0xfc - desc SCFR"]
    #[inline(always)]
    pub const fn scfr(&self) -> &Scfr {
        &self.scfr
    }
}
#[doc = "OCCRUH (rw) register accessor: desc OCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`occruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occruh`] module"]
#[doc(alias = "OCCRUH")]
pub type Occruh = crate::Reg<occruh::OccruhSpec>;
#[doc = "desc OCCRUH"]
pub mod occruh;
#[doc = "OCCRUL (rw) register accessor: desc OCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrul`] module"]
#[doc(alias = "OCCRUL")]
pub type Occrul = crate::Reg<occrul::OccrulSpec>;
#[doc = "desc OCCRUL"]
pub mod occrul;
#[doc = "OCCRVH (rw) register accessor: desc OCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrvh`] module"]
#[doc(alias = "OCCRVH")]
pub type Occrvh = crate::Reg<occrvh::OccrvhSpec>;
#[doc = "desc OCCRVH"]
pub mod occrvh;
#[doc = "OCCRVL (rw) register accessor: desc OCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrvl`] module"]
#[doc(alias = "OCCRVL")]
pub type Occrvl = crate::Reg<occrvl::OccrvlSpec>;
#[doc = "desc OCCRVL"]
pub mod occrvl;
#[doc = "OCCRWH (rw) register accessor: desc OCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrwh`] module"]
#[doc(alias = "OCCRWH")]
pub type Occrwh = crate::Reg<occrwh::OccrwhSpec>;
#[doc = "desc OCCRWH"]
pub mod occrwh;
#[doc = "OCCRWL (rw) register accessor: desc OCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrwl`] module"]
#[doc(alias = "OCCRWL")]
pub type Occrwl = crate::Reg<occrwl::OccrwlSpec>;
#[doc = "desc OCCRWL"]
pub mod occrwl;
#[doc = "OCCRXH (rw) register accessor: desc OCCRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrxh`] module"]
#[doc(alias = "OCCRXH")]
pub type Occrxh = crate::Reg<occrxh::OccrxhSpec>;
#[doc = "desc OCCRXH"]
pub mod occrxh;
#[doc = "OCCRXL (rw) register accessor: desc OCCRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@occrxl`] module"]
#[doc(alias = "OCCRXL")]
pub type Occrxl = crate::Reg<occrxl::OccrxlSpec>;
#[doc = "desc OCCRXL"]
pub mod occrxl;
#[doc = "OCSRU (rw) register accessor: desc OCSRU\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsru`] module"]
#[doc(alias = "OCSRU")]
pub type Ocsru = crate::Reg<ocsru::OcsruSpec>;
#[doc = "desc OCSRU"]
pub mod ocsru;
#[doc = "OCERU (rw) register accessor: desc OCERU\n\nYou can [`read`](crate::Reg::read) this register and get [`oceru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oceru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oceru`] module"]
#[doc(alias = "OCERU")]
pub type Oceru = crate::Reg<oceru::OceruSpec>;
#[doc = "desc OCERU"]
pub mod oceru;
#[doc = "OCSRV (rw) register accessor: desc OCSRV\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsrv`] module"]
#[doc(alias = "OCSRV")]
pub type Ocsrv = crate::Reg<ocsrv::OcsrvSpec>;
#[doc = "desc OCSRV"]
pub mod ocsrv;
#[doc = "OCERV (rw) register accessor: desc OCERV\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocerv`] module"]
#[doc(alias = "OCERV")]
pub type Ocerv = crate::Reg<ocerv::OcervSpec>;
#[doc = "desc OCERV"]
pub mod ocerv;
#[doc = "OCSRW (rw) register accessor: desc OCSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsrw`] module"]
#[doc(alias = "OCSRW")]
pub type Ocsrw = crate::Reg<ocsrw::OcsrwSpec>;
#[doc = "desc OCSRW"]
pub mod ocsrw;
#[doc = "OCERW (rw) register accessor: desc OCERW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocerw`] module"]
#[doc(alias = "OCERW")]
pub type Ocerw = crate::Reg<ocerw::OcerwSpec>;
#[doc = "desc OCERW"]
pub mod ocerw;
#[doc = "OCSRX (rw) register accessor: desc OCSRX\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocsrx`] module"]
#[doc(alias = "OCSRX")]
pub type Ocsrx = crate::Reg<ocsrx::OcsrxSpec>;
#[doc = "desc OCSRX"]
pub mod ocsrx;
#[doc = "OCERX (rw) register accessor: desc OCERX\n\nYou can [`read`](crate::Reg::read) this register and get [`ocerx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocerx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocerx`] module"]
#[doc(alias = "OCERX")]
pub type Ocerx = crate::Reg<ocerx::OcerxSpec>;
#[doc = "desc OCERX"]
pub mod ocerx;
#[doc = "OCMRUH (rw) register accessor: desc OCMRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmruh`] module"]
#[doc(alias = "OCMRUH")]
pub type Ocmruh = crate::Reg<ocmruh::OcmruhSpec>;
#[doc = "desc OCMRUH"]
pub mod ocmruh;
#[doc = "OCMRUL (rw) register accessor: desc OCMRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrul`] module"]
#[doc(alias = "OCMRUL")]
pub type Ocmrul = crate::Reg<ocmrul::OcmrulSpec>;
#[doc = "desc OCMRUL"]
pub mod ocmrul;
#[doc = "OCMRVH (rw) register accessor: desc OCMRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrvh`] module"]
#[doc(alias = "OCMRVH")]
pub type Ocmrvh = crate::Reg<ocmrvh::OcmrvhSpec>;
#[doc = "desc OCMRVH"]
pub mod ocmrvh;
#[doc = "OCMRVL (rw) register accessor: desc OCMRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrvl`] module"]
#[doc(alias = "OCMRVL")]
pub type Ocmrvl = crate::Reg<ocmrvl::OcmrvlSpec>;
#[doc = "desc OCMRVL"]
pub mod ocmrvl;
#[doc = "OCMRWH (rw) register accessor: desc OCMRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrwh`] module"]
#[doc(alias = "OCMRWH")]
pub type Ocmrwh = crate::Reg<ocmrwh::OcmrwhSpec>;
#[doc = "desc OCMRWH"]
pub mod ocmrwh;
#[doc = "OCMRWL (rw) register accessor: desc OCMRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrwl`] module"]
#[doc(alias = "OCMRWL")]
pub type Ocmrwl = crate::Reg<ocmrwl::OcmrwlSpec>;
#[doc = "desc OCMRWL"]
pub mod ocmrwl;
#[doc = "OCMRXH (rw) register accessor: desc OCMRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrxh`] module"]
#[doc(alias = "OCMRXH")]
pub type Ocmrxh = crate::Reg<ocmrxh::OcmrxhSpec>;
#[doc = "desc OCMRXH"]
pub mod ocmrxh;
#[doc = "OCMRXL (rw) register accessor: desc OCMRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`ocmrxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocmrxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmrxl`] module"]
#[doc(alias = "OCMRXL")]
pub type Ocmrxl = crate::Reg<ocmrxl::OcmrxlSpec>;
#[doc = "desc OCMRXL"]
pub mod ocmrxl;
#[doc = "CPSR (rw) register accessor: desc CPSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`] module"]
#[doc(alias = "CPSR")]
pub type Cpsr = crate::Reg<cpsr::CpsrSpec>;
#[doc = "desc CPSR"]
pub mod cpsr;
#[doc = "CNTR (rw) register accessor: desc CNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`] module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "desc CNTR"]
pub mod cntr;
#[doc = "CCSR (rw) register accessor: desc CCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsr`] module"]
#[doc(alias = "CCSR")]
pub type Ccsr = crate::Reg<ccsr::CcsrSpec>;
#[doc = "desc CCSR"]
pub mod ccsr;
#[doc = "CVPR (rw) register accessor: desc CVPR\n\nYou can [`read`](crate::Reg::read) this register and get [`cvpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvpr`] module"]
#[doc(alias = "CVPR")]
pub type Cvpr = crate::Reg<cvpr::CvprSpec>;
#[doc = "desc CVPR"]
pub mod cvpr;
#[doc = "PSCR (rw) register accessor: desc PSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`] module"]
#[doc(alias = "PSCR")]
pub type Pscr = crate::Reg<pscr::PscrSpec>;
#[doc = "desc PSCR"]
pub mod pscr;
#[doc = "PFSRU (rw) register accessor: desc PFSRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsru`] module"]
#[doc(alias = "PFSRU")]
pub type Pfsru = crate::Reg<pfsru::PfsruSpec>;
#[doc = "desc PFSRU"]
pub mod pfsru;
#[doc = "PDARU (rw) register accessor: desc PDARU\n\nYou can [`read`](crate::Reg::read) this register and get [`pdaru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdaru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdaru`] module"]
#[doc(alias = "PDARU")]
pub type Pdaru = crate::Reg<pdaru::PdaruSpec>;
#[doc = "desc PDARU"]
pub mod pdaru;
#[doc = "PDBRU (rw) register accessor: desc PDBRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbru`] module"]
#[doc(alias = "PDBRU")]
pub type Pdbru = crate::Reg<pdbru::PdbruSpec>;
#[doc = "desc PDBRU"]
pub mod pdbru;
#[doc = "PFSRV (rw) register accessor: desc PFSRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrv`] module"]
#[doc(alias = "PFSRV")]
pub type Pfsrv = crate::Reg<pfsrv::PfsrvSpec>;
#[doc = "desc PFSRV"]
pub mod pfsrv;
#[doc = "PDARV (rw) register accessor: desc PDARV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdarv`] module"]
#[doc(alias = "PDARV")]
pub type Pdarv = crate::Reg<pdarv::PdarvSpec>;
#[doc = "desc PDARV"]
pub mod pdarv;
#[doc = "PDBRV (rw) register accessor: desc PDBRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbrv`] module"]
#[doc(alias = "PDBRV")]
pub type Pdbrv = crate::Reg<pdbrv::PdbrvSpec>;
#[doc = "desc PDBRV"]
pub mod pdbrv;
#[doc = "PFSRW (rw) register accessor: desc PFSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrw`] module"]
#[doc(alias = "PFSRW")]
pub type Pfsrw = crate::Reg<pfsrw::PfsrwSpec>;
#[doc = "desc PFSRW"]
pub mod pfsrw;
#[doc = "PDARW (rw) register accessor: desc PDARW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdarw`] module"]
#[doc(alias = "PDARW")]
pub type Pdarw = crate::Reg<pdarw::PdarwSpec>;
#[doc = "desc PDARW"]
pub mod pdarw;
#[doc = "PDBRW (rw) register accessor: desc PDBRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbrw`] module"]
#[doc(alias = "PDBRW")]
pub type Pdbrw = crate::Reg<pdbrw::PdbrwSpec>;
#[doc = "desc PDBRW"]
pub mod pdbrw;
#[doc = "PFSRX (rw) register accessor: desc PFSRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfsrx`] module"]
#[doc(alias = "PFSRX")]
pub type Pfsrx = crate::Reg<pfsrx::PfsrxSpec>;
#[doc = "desc PFSRX"]
pub mod pfsrx;
#[doc = "PDARX (rw) register accessor: desc PDARX\n\nYou can [`read`](crate::Reg::read) this register and get [`pdarx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdarx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdarx`] module"]
#[doc(alias = "PDARX")]
pub type Pdarx = crate::Reg<pdarx::PdarxSpec>;
#[doc = "desc PDARX"]
pub mod pdarx;
#[doc = "PDBRX (rw) register accessor: desc PDBRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbrx`] module"]
#[doc(alias = "PDBRX")]
pub type Pdbrx = crate::Reg<pdbrx::PdbrxSpec>;
#[doc = "desc PDBRX"]
pub mod pdbrx;
#[doc = "POCRU (rw) register accessor: desc POCRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pocru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocru`] module"]
#[doc(alias = "POCRU")]
pub type Pocru = crate::Reg<pocru::PocruSpec>;
#[doc = "desc POCRU"]
pub mod pocru;
#[doc = "POCRV (rw) register accessor: desc POCRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocrv`] module"]
#[doc(alias = "POCRV")]
pub type Pocrv = crate::Reg<pocrv::PocrvSpec>;
#[doc = "desc POCRV"]
pub mod pocrv;
#[doc = "POCRW (rw) register accessor: desc POCRW\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocrw`] module"]
#[doc(alias = "POCRW")]
pub type Pocrw = crate::Reg<pocrw::PocrwSpec>;
#[doc = "desc POCRW"]
pub mod pocrw;
#[doc = "POCRX (rw) register accessor: desc POCRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pocrx`] module"]
#[doc(alias = "POCRX")]
pub type Pocrx = crate::Reg<pocrx::PocrxSpec>;
#[doc = "desc POCRX"]
pub mod pocrx;
#[doc = "SCCRUH (rw) register accessor: desc SCCRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccruh`] module"]
#[doc(alias = "SCCRUH")]
pub type Sccruh = crate::Reg<sccruh::SccruhSpec>;
#[doc = "desc SCCRUH"]
pub mod sccruh;
#[doc = "SCCRUL (rw) register accessor: desc SCCRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrul`] module"]
#[doc(alias = "SCCRUL")]
pub type Sccrul = crate::Reg<sccrul::SccrulSpec>;
#[doc = "desc SCCRUL"]
pub mod sccrul;
#[doc = "SCCRVH (rw) register accessor: desc SCCRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrvh`] module"]
#[doc(alias = "SCCRVH")]
pub type Sccrvh = crate::Reg<sccrvh::SccrvhSpec>;
#[doc = "desc SCCRVH"]
pub mod sccrvh;
#[doc = "SCCRVL (rw) register accessor: desc SCCRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrvl`] module"]
#[doc(alias = "SCCRVL")]
pub type Sccrvl = crate::Reg<sccrvl::SccrvlSpec>;
#[doc = "desc SCCRVL"]
pub mod sccrvl;
#[doc = "SCCRWH (rw) register accessor: desc SCCRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrwh`] module"]
#[doc(alias = "SCCRWH")]
pub type Sccrwh = crate::Reg<sccrwh::SccrwhSpec>;
#[doc = "desc SCCRWH"]
pub mod sccrwh;
#[doc = "SCCRWL (rw) register accessor: desc SCCRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrwl`] module"]
#[doc(alias = "SCCRWL")]
pub type Sccrwl = crate::Reg<sccrwl::SccrwlSpec>;
#[doc = "desc SCCRWL"]
pub mod sccrwl;
#[doc = "SCCRXH (rw) register accessor: desc SCCRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrxh`] module"]
#[doc(alias = "SCCRXH")]
pub type Sccrxh = crate::Reg<sccrxh::SccrxhSpec>;
#[doc = "desc SCCRXH"]
pub mod sccrxh;
#[doc = "SCCRXL (rw) register accessor: desc SCCRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sccrxl`] module"]
#[doc(alias = "SCCRXL")]
pub type Sccrxl = crate::Reg<sccrxl::SccrxlSpec>;
#[doc = "desc SCCRXL"]
pub mod sccrxl;
#[doc = "SCSRUH (rw) register accessor: desc SCSRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsruh`] module"]
#[doc(alias = "SCSRUH")]
pub type Scsruh = crate::Reg<scsruh::ScsruhSpec>;
#[doc = "desc SCSRUH"]
pub mod scsruh;
#[doc = "SCMRUH (rw) register accessor: desc SCMRUH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmruh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmruh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmruh`] module"]
#[doc(alias = "SCMRUH")]
pub type Scmruh = crate::Reg<scmruh::ScmruhSpec>;
#[doc = "desc SCMRUH"]
pub mod scmruh;
#[doc = "SCSRUL (rw) register accessor: desc SCSRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrul`] module"]
#[doc(alias = "SCSRUL")]
pub type Scsrul = crate::Reg<scsrul::ScsrulSpec>;
#[doc = "desc SCSRUL"]
pub mod scsrul;
#[doc = "SCMRUL (rw) register accessor: desc SCMRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrul`] module"]
#[doc(alias = "SCMRUL")]
pub type Scmrul = crate::Reg<scmrul::ScmrulSpec>;
#[doc = "desc SCMRUL"]
pub mod scmrul;
#[doc = "SCSRVH (rw) register accessor: desc SCSRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrvh`] module"]
#[doc(alias = "SCSRVH")]
pub type Scsrvh = crate::Reg<scsrvh::ScsrvhSpec>;
#[doc = "desc SCSRVH"]
pub mod scsrvh;
#[doc = "SCMRVH (rw) register accessor: desc SCMRVH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrvh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrvh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrvh`] module"]
#[doc(alias = "SCMRVH")]
pub type Scmrvh = crate::Reg<scmrvh::ScmrvhSpec>;
#[doc = "desc SCMRVH"]
pub mod scmrvh;
#[doc = "SCSRVL (rw) register accessor: desc SCSRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrvl`] module"]
#[doc(alias = "SCSRVL")]
pub type Scsrvl = crate::Reg<scsrvl::ScsrvlSpec>;
#[doc = "desc SCSRVL"]
pub mod scsrvl;
#[doc = "SCMRVL (rw) register accessor: desc SCMRVL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrvl`] module"]
#[doc(alias = "SCMRVL")]
pub type Scmrvl = crate::Reg<scmrvl::ScmrvlSpec>;
#[doc = "desc SCMRVL"]
pub mod scmrvl;
#[doc = "SCSRWH (rw) register accessor: desc SCSRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrwh`] module"]
#[doc(alias = "SCSRWH")]
pub type Scsrwh = crate::Reg<scsrwh::ScsrwhSpec>;
#[doc = "desc SCSRWH"]
pub mod scsrwh;
#[doc = "SCMRWH (rw) register accessor: desc SCMRWH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrwh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrwh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrwh`] module"]
#[doc(alias = "SCMRWH")]
pub type Scmrwh = crate::Reg<scmrwh::ScmrwhSpec>;
#[doc = "desc SCMRWH"]
pub mod scmrwh;
#[doc = "SCSRWL (rw) register accessor: desc SCSRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrwl`] module"]
#[doc(alias = "SCSRWL")]
pub type Scsrwl = crate::Reg<scsrwl::ScsrwlSpec>;
#[doc = "desc SCSRWL"]
pub mod scsrwl;
#[doc = "SCMRWL (rw) register accessor: desc SCMRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrwl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrwl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrwl`] module"]
#[doc(alias = "SCMRWL")]
pub type Scmrwl = crate::Reg<scmrwl::ScmrwlSpec>;
#[doc = "desc SCMRWL"]
pub mod scmrwl;
#[doc = "SCSRXH (rw) register accessor: desc SCSRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrxh`] module"]
#[doc(alias = "SCSRXH")]
pub type Scsrxh = crate::Reg<scsrxh::ScsrxhSpec>;
#[doc = "desc SCSRXH"]
pub mod scsrxh;
#[doc = "SCMRXH (rw) register accessor: desc SCMRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrxh`] module"]
#[doc(alias = "SCMRXH")]
pub type Scmrxh = crate::Reg<scmrxh::ScmrxhSpec>;
#[doc = "desc SCMRXH"]
pub mod scmrxh;
#[doc = "SCSRXL (rw) register accessor: desc SCSRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`scsrxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsrxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsrxl`] module"]
#[doc(alias = "SCSRXL")]
pub type Scsrxl = crate::Reg<scsrxl::ScsrxlSpec>;
#[doc = "desc SCSRXL"]
pub mod scsrxl;
#[doc = "SCMRXL (rw) register accessor: desc SCMRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scmrxl`] module"]
#[doc(alias = "SCMRXL")]
pub type Scmrxl = crate::Reg<scmrxl::ScmrxlSpec>;
#[doc = "desc SCMRXL"]
pub mod scmrxl;
#[doc = "SCER (rw) register accessor: desc SCER\n\nYou can [`read`](crate::Reg::read) this register and get [`scer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scer`] module"]
#[doc(alias = "SCER")]
pub type Scer = crate::Reg<scer::ScerSpec>;
#[doc = "desc SCER"]
pub mod scer;
#[doc = "RCSR (rw) register accessor: desc RCSR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcsr`] module"]
#[doc(alias = "RCSR")]
pub type Rcsr = crate::Reg<rcsr::RcsrSpec>;
#[doc = "desc RCSR"]
pub mod rcsr;
#[doc = "SCIR (rw) register accessor: desc SCIR\n\nYou can [`read`](crate::Reg::read) this register and get [`scir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scir`] module"]
#[doc(alias = "SCIR")]
pub type Scir = crate::Reg<scir::ScirSpec>;
#[doc = "desc SCIR"]
pub mod scir;
#[doc = "SCFR (rw) register accessor: desc SCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`scfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfr`] module"]
#[doc(alias = "SCFR")]
pub type Scfr = crate::Reg<scfr::ScfrSpec>;
#[doc = "desc SCFR"]
pub mod scfr;
