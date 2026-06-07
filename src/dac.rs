#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dadr1: Dadr1,
    dadr2: Dadr2,
    dacr: Dacr,
    daadpcr: Daadpcr,
    _reserved4: [u8; 0x14],
    daocr: Daocr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc DADR1"]
    #[inline(always)]
    pub const fn dadr1(&self) -> &Dadr1 {
        &self.dadr1
    }
    #[doc = "0x02 - desc DADR2"]
    #[inline(always)]
    pub const fn dadr2(&self) -> &Dadr2 {
        &self.dadr2
    }
    #[doc = "0x04 - desc DACR"]
    #[inline(always)]
    pub const fn dacr(&self) -> &Dacr {
        &self.dacr
    }
    #[doc = "0x06 - desc DAADPCR"]
    #[inline(always)]
    pub const fn daadpcr(&self) -> &Daadpcr {
        &self.daadpcr
    }
    #[doc = "0x1c - desc DAOCR"]
    #[inline(always)]
    pub const fn daocr(&self) -> &Daocr {
        &self.daocr
    }
}
#[doc = "DADR1 (rw) register accessor: desc DADR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr1`] module"]
#[doc(alias = "DADR1")]
pub type Dadr1 = crate::Reg<dadr1::Dadr1Spec>;
#[doc = "desc DADR1"]
pub mod dadr1;
#[doc = "DADR2 (rw) register accessor: desc DADR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadr2`] module"]
#[doc(alias = "DADR2")]
pub type Dadr2 = crate::Reg<dadr2::Dadr2Spec>;
#[doc = "desc DADR2"]
pub mod dadr2;
#[doc = "DACR (rw) register accessor: desc DACR\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacr`] module"]
#[doc(alias = "DACR")]
pub type Dacr = crate::Reg<dacr::DacrSpec>;
#[doc = "desc DACR"]
pub mod dacr;
#[doc = "DAADPCR (rw) register accessor: desc DAADPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`daadpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daadpcr`] module"]
#[doc(alias = "DAADPCR")]
pub type Daadpcr = crate::Reg<daadpcr::DaadpcrSpec>;
#[doc = "desc DAADPCR"]
pub mod daadpcr;
#[doc = "DAOCR (rw) register accessor: desc DAOCR\n\nYou can [`read`](crate::Reg::read) this register and get [`daocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daocr`] module"]
#[doc(alias = "DAOCR")]
pub type Daocr = crate::Reg<daocr::DaocrSpec>;
#[doc = "desc DAOCR"]
pub mod daocr;
