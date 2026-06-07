#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    nmier: Nmier,
    nmifr: Nmifr,
    nmifcr: Nmifcr,
    eirqcr0: Eirqcr0,
    eirqcr1: Eirqcr1,
    eirqcr2: Eirqcr2,
    eirqcr3: Eirqcr3,
    eirqcr4: Eirqcr4,
    eirqcr5: Eirqcr5,
    eirqcr6: Eirqcr6,
    eirqcr7: Eirqcr7,
    eirqcr8: Eirqcr8,
    eirqcr9: Eirqcr9,
    eirqcr10: Eirqcr10,
    eirqcr11: Eirqcr11,
    eirqcr12: Eirqcr12,
    eirqcr13: Eirqcr13,
    eirqcr14: Eirqcr14,
    eirqcr15: Eirqcr15,
    wken: Wken,
    eifr: Eifr,
    eifcr: Eifcr,
    intsel0: Intsel0,
    intsel1: Intsel1,
    intsel2: Intsel2,
    intsel3: Intsel3,
    intsel4: Intsel4,
    intsel5: Intsel5,
    intsel6: Intsel6,
    intsel7: Intsel7,
    intsel8: Intsel8,
    intsel9: Intsel9,
    intsel10: Intsel10,
    intsel11: Intsel11,
    intsel12: Intsel12,
    intsel13: Intsel13,
    intsel14: Intsel14,
    intsel15: Intsel15,
    inten0: Inten0,
    inten1: Inten1,
    inten2: Inten2,
    inten3: Inten3,
    inten4: Inten4,
    inten5: Inten5,
    inten6: Inten6,
    inten7: Inten7,
    inten8: Inten8,
    inten9: Inten9,
    inten10: Inten10,
    inten11: Inten11,
    inten12: Inten12,
    inten13: Inten13,
    inten14: Inten14,
    inten15: Inten15,
    swier: Swier,
    evter: Evter,
    ier: Ier,
}
impl RegisterBlock {
    #[doc = "0x04 - desc NMIER"]
    #[inline(always)]
    pub const fn nmier(&self) -> &Nmier {
        &self.nmier
    }
    #[doc = "0x08 - desc NMIFR"]
    #[inline(always)]
    pub const fn nmifr(&self) -> &Nmifr {
        &self.nmifr
    }
    #[doc = "0x0c - desc NMIFCR"]
    #[inline(always)]
    pub const fn nmifcr(&self) -> &Nmifcr {
        &self.nmifcr
    }
    #[doc = "0x10 - desc EIRQCR0"]
    #[inline(always)]
    pub const fn eirqcr0(&self) -> &Eirqcr0 {
        &self.eirqcr0
    }
    #[doc = "0x14 - desc EIRQCR1"]
    #[inline(always)]
    pub const fn eirqcr1(&self) -> &Eirqcr1 {
        &self.eirqcr1
    }
    #[doc = "0x18 - desc EIRQCR2"]
    #[inline(always)]
    pub const fn eirqcr2(&self) -> &Eirqcr2 {
        &self.eirqcr2
    }
    #[doc = "0x1c - desc EIRQCR3"]
    #[inline(always)]
    pub const fn eirqcr3(&self) -> &Eirqcr3 {
        &self.eirqcr3
    }
    #[doc = "0x20 - desc EIRQCR4"]
    #[inline(always)]
    pub const fn eirqcr4(&self) -> &Eirqcr4 {
        &self.eirqcr4
    }
    #[doc = "0x24 - desc EIRQCR5"]
    #[inline(always)]
    pub const fn eirqcr5(&self) -> &Eirqcr5 {
        &self.eirqcr5
    }
    #[doc = "0x28 - desc EIRQCR6"]
    #[inline(always)]
    pub const fn eirqcr6(&self) -> &Eirqcr6 {
        &self.eirqcr6
    }
    #[doc = "0x2c - desc EIRQCR7"]
    #[inline(always)]
    pub const fn eirqcr7(&self) -> &Eirqcr7 {
        &self.eirqcr7
    }
    #[doc = "0x30 - desc EIRQCR8"]
    #[inline(always)]
    pub const fn eirqcr8(&self) -> &Eirqcr8 {
        &self.eirqcr8
    }
    #[doc = "0x34 - desc EIRQCR9"]
    #[inline(always)]
    pub const fn eirqcr9(&self) -> &Eirqcr9 {
        &self.eirqcr9
    }
    #[doc = "0x38 - desc EIRQCR10"]
    #[inline(always)]
    pub const fn eirqcr10(&self) -> &Eirqcr10 {
        &self.eirqcr10
    }
    #[doc = "0x3c - desc EIRQCR11"]
    #[inline(always)]
    pub const fn eirqcr11(&self) -> &Eirqcr11 {
        &self.eirqcr11
    }
    #[doc = "0x40 - desc EIRQCR12"]
    #[inline(always)]
    pub const fn eirqcr12(&self) -> &Eirqcr12 {
        &self.eirqcr12
    }
    #[doc = "0x44 - desc EIRQCR13"]
    #[inline(always)]
    pub const fn eirqcr13(&self) -> &Eirqcr13 {
        &self.eirqcr13
    }
    #[doc = "0x48 - desc EIRQCR14"]
    #[inline(always)]
    pub const fn eirqcr14(&self) -> &Eirqcr14 {
        &self.eirqcr14
    }
    #[doc = "0x4c - desc EIRQCR15"]
    #[inline(always)]
    pub const fn eirqcr15(&self) -> &Eirqcr15 {
        &self.eirqcr15
    }
    #[doc = "0x50 - desc WKEN"]
    #[inline(always)]
    pub const fn wken(&self) -> &Wken {
        &self.wken
    }
    #[doc = "0x54 - desc EIFR"]
    #[inline(always)]
    pub const fn eifr(&self) -> &Eifr {
        &self.eifr
    }
    #[doc = "0x58 - desc EIFCR"]
    #[inline(always)]
    pub const fn eifcr(&self) -> &Eifcr {
        &self.eifcr
    }
    #[doc = "0x5c - desc INTSEL0"]
    #[inline(always)]
    pub const fn intsel0(&self) -> &Intsel0 {
        &self.intsel0
    }
    #[doc = "0x60 - desc INTSEL1"]
    #[inline(always)]
    pub const fn intsel1(&self) -> &Intsel1 {
        &self.intsel1
    }
    #[doc = "0x64 - desc INTSEL2"]
    #[inline(always)]
    pub const fn intsel2(&self) -> &Intsel2 {
        &self.intsel2
    }
    #[doc = "0x68 - desc INTSEL3"]
    #[inline(always)]
    pub const fn intsel3(&self) -> &Intsel3 {
        &self.intsel3
    }
    #[doc = "0x6c - desc INTSEL4"]
    #[inline(always)]
    pub const fn intsel4(&self) -> &Intsel4 {
        &self.intsel4
    }
    #[doc = "0x70 - desc INTSEL5"]
    #[inline(always)]
    pub const fn intsel5(&self) -> &Intsel5 {
        &self.intsel5
    }
    #[doc = "0x74 - desc INTSEL6"]
    #[inline(always)]
    pub const fn intsel6(&self) -> &Intsel6 {
        &self.intsel6
    }
    #[doc = "0x78 - desc INTSEL7"]
    #[inline(always)]
    pub const fn intsel7(&self) -> &Intsel7 {
        &self.intsel7
    }
    #[doc = "0x7c - desc INTSEL8"]
    #[inline(always)]
    pub const fn intsel8(&self) -> &Intsel8 {
        &self.intsel8
    }
    #[doc = "0x80 - desc INTSEL9"]
    #[inline(always)]
    pub const fn intsel9(&self) -> &Intsel9 {
        &self.intsel9
    }
    #[doc = "0x84 - desc INTSEL10"]
    #[inline(always)]
    pub const fn intsel10(&self) -> &Intsel10 {
        &self.intsel10
    }
    #[doc = "0x88 - desc INTSEL11"]
    #[inline(always)]
    pub const fn intsel11(&self) -> &Intsel11 {
        &self.intsel11
    }
    #[doc = "0x8c - desc INTSEL12"]
    #[inline(always)]
    pub const fn intsel12(&self) -> &Intsel12 {
        &self.intsel12
    }
    #[doc = "0x90 - desc INTSEL13"]
    #[inline(always)]
    pub const fn intsel13(&self) -> &Intsel13 {
        &self.intsel13
    }
    #[doc = "0x94 - desc INTSEL14"]
    #[inline(always)]
    pub const fn intsel14(&self) -> &Intsel14 {
        &self.intsel14
    }
    #[doc = "0x98 - desc INTSEL15"]
    #[inline(always)]
    pub const fn intsel15(&self) -> &Intsel15 {
        &self.intsel15
    }
    #[doc = "0x9c - desc INTEN0"]
    #[inline(always)]
    pub const fn inten0(&self) -> &Inten0 {
        &self.inten0
    }
    #[doc = "0xa0 - desc INTEN1"]
    #[inline(always)]
    pub const fn inten1(&self) -> &Inten1 {
        &self.inten1
    }
    #[doc = "0xa4 - desc INTEN2"]
    #[inline(always)]
    pub const fn inten2(&self) -> &Inten2 {
        &self.inten2
    }
    #[doc = "0xa8 - desc INTEN3"]
    #[inline(always)]
    pub const fn inten3(&self) -> &Inten3 {
        &self.inten3
    }
    #[doc = "0xac - desc INTEN4"]
    #[inline(always)]
    pub const fn inten4(&self) -> &Inten4 {
        &self.inten4
    }
    #[doc = "0xb0 - desc INTEN5"]
    #[inline(always)]
    pub const fn inten5(&self) -> &Inten5 {
        &self.inten5
    }
    #[doc = "0xb4 - desc INTEN6"]
    #[inline(always)]
    pub const fn inten6(&self) -> &Inten6 {
        &self.inten6
    }
    #[doc = "0xb8 - desc INTEN7"]
    #[inline(always)]
    pub const fn inten7(&self) -> &Inten7 {
        &self.inten7
    }
    #[doc = "0xbc - desc INTEN8"]
    #[inline(always)]
    pub const fn inten8(&self) -> &Inten8 {
        &self.inten8
    }
    #[doc = "0xc0 - desc INTEN9"]
    #[inline(always)]
    pub const fn inten9(&self) -> &Inten9 {
        &self.inten9
    }
    #[doc = "0xc4 - desc INTEN10"]
    #[inline(always)]
    pub const fn inten10(&self) -> &Inten10 {
        &self.inten10
    }
    #[doc = "0xc8 - desc INTEN11"]
    #[inline(always)]
    pub const fn inten11(&self) -> &Inten11 {
        &self.inten11
    }
    #[doc = "0xcc - desc INTEN12"]
    #[inline(always)]
    pub const fn inten12(&self) -> &Inten12 {
        &self.inten12
    }
    #[doc = "0xd0 - desc INTEN13"]
    #[inline(always)]
    pub const fn inten13(&self) -> &Inten13 {
        &self.inten13
    }
    #[doc = "0xd4 - desc INTEN14"]
    #[inline(always)]
    pub const fn inten14(&self) -> &Inten14 {
        &self.inten14
    }
    #[doc = "0xd8 - desc INTEN15"]
    #[inline(always)]
    pub const fn inten15(&self) -> &Inten15 {
        &self.inten15
    }
    #[doc = "0xdc - desc SWIER"]
    #[inline(always)]
    pub const fn swier(&self) -> &Swier {
        &self.swier
    }
    #[doc = "0xe0 - desc EVTER"]
    #[inline(always)]
    pub const fn evter(&self) -> &Evter {
        &self.evter
    }
    #[doc = "0xe4 - desc IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
}
#[doc = "NMIER (rw) register accessor: desc NMIER\n\nYou can [`read`](crate::Reg::read) this register and get [`nmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmier`] module"]
#[doc(alias = "NMIER")]
pub type Nmier = crate::Reg<nmier::NmierSpec>;
#[doc = "desc NMIER"]
pub mod nmier;
#[doc = "NMIFR (rw) register accessor: desc NMIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmifr`] module"]
#[doc(alias = "NMIFR")]
pub type Nmifr = crate::Reg<nmifr::NmifrSpec>;
#[doc = "desc NMIFR"]
pub mod nmifr;
#[doc = "NMIFCR (rw) register accessor: desc NMIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmifcr`] module"]
#[doc(alias = "NMIFCR")]
pub type Nmifcr = crate::Reg<nmifcr::NmifcrSpec>;
#[doc = "desc NMIFCR"]
pub mod nmifcr;
#[doc = "EIRQCR0 (rw) register accessor: desc EIRQCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr0`] module"]
#[doc(alias = "EIRQCR0")]
pub type Eirqcr0 = crate::Reg<eirqcr0::Eirqcr0Spec>;
#[doc = "desc EIRQCR0"]
pub mod eirqcr0;
#[doc = "EIRQCR1 (rw) register accessor: desc EIRQCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr1`] module"]
#[doc(alias = "EIRQCR1")]
pub type Eirqcr1 = crate::Reg<eirqcr1::Eirqcr1Spec>;
#[doc = "desc EIRQCR1"]
pub mod eirqcr1;
#[doc = "EIRQCR2 (rw) register accessor: desc EIRQCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr2`] module"]
#[doc(alias = "EIRQCR2")]
pub type Eirqcr2 = crate::Reg<eirqcr2::Eirqcr2Spec>;
#[doc = "desc EIRQCR2"]
pub mod eirqcr2;
#[doc = "EIRQCR3 (rw) register accessor: desc EIRQCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr3`] module"]
#[doc(alias = "EIRQCR3")]
pub type Eirqcr3 = crate::Reg<eirqcr3::Eirqcr3Spec>;
#[doc = "desc EIRQCR3"]
pub mod eirqcr3;
#[doc = "EIRQCR4 (rw) register accessor: desc EIRQCR4\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr4`] module"]
#[doc(alias = "EIRQCR4")]
pub type Eirqcr4 = crate::Reg<eirqcr4::Eirqcr4Spec>;
#[doc = "desc EIRQCR4"]
pub mod eirqcr4;
#[doc = "EIRQCR5 (rw) register accessor: desc EIRQCR5\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr5`] module"]
#[doc(alias = "EIRQCR5")]
pub type Eirqcr5 = crate::Reg<eirqcr5::Eirqcr5Spec>;
#[doc = "desc EIRQCR5"]
pub mod eirqcr5;
#[doc = "EIRQCR6 (rw) register accessor: desc EIRQCR6\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr6`] module"]
#[doc(alias = "EIRQCR6")]
pub type Eirqcr6 = crate::Reg<eirqcr6::Eirqcr6Spec>;
#[doc = "desc EIRQCR6"]
pub mod eirqcr6;
#[doc = "EIRQCR7 (rw) register accessor: desc EIRQCR7\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr7`] module"]
#[doc(alias = "EIRQCR7")]
pub type Eirqcr7 = crate::Reg<eirqcr7::Eirqcr7Spec>;
#[doc = "desc EIRQCR7"]
pub mod eirqcr7;
#[doc = "EIRQCR8 (rw) register accessor: desc EIRQCR8\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr8`] module"]
#[doc(alias = "EIRQCR8")]
pub type Eirqcr8 = crate::Reg<eirqcr8::Eirqcr8Spec>;
#[doc = "desc EIRQCR8"]
pub mod eirqcr8;
#[doc = "EIRQCR9 (rw) register accessor: desc EIRQCR9\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr9`] module"]
#[doc(alias = "EIRQCR9")]
pub type Eirqcr9 = crate::Reg<eirqcr9::Eirqcr9Spec>;
#[doc = "desc EIRQCR9"]
pub mod eirqcr9;
#[doc = "EIRQCR10 (rw) register accessor: desc EIRQCR10\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr10`] module"]
#[doc(alias = "EIRQCR10")]
pub type Eirqcr10 = crate::Reg<eirqcr10::Eirqcr10Spec>;
#[doc = "desc EIRQCR10"]
pub mod eirqcr10;
#[doc = "EIRQCR11 (rw) register accessor: desc EIRQCR11\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr11`] module"]
#[doc(alias = "EIRQCR11")]
pub type Eirqcr11 = crate::Reg<eirqcr11::Eirqcr11Spec>;
#[doc = "desc EIRQCR11"]
pub mod eirqcr11;
#[doc = "EIRQCR12 (rw) register accessor: desc EIRQCR12\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr12`] module"]
#[doc(alias = "EIRQCR12")]
pub type Eirqcr12 = crate::Reg<eirqcr12::Eirqcr12Spec>;
#[doc = "desc EIRQCR12"]
pub mod eirqcr12;
#[doc = "EIRQCR13 (rw) register accessor: desc EIRQCR13\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr13`] module"]
#[doc(alias = "EIRQCR13")]
pub type Eirqcr13 = crate::Reg<eirqcr13::Eirqcr13Spec>;
#[doc = "desc EIRQCR13"]
pub mod eirqcr13;
#[doc = "EIRQCR14 (rw) register accessor: desc EIRQCR14\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr14`] module"]
#[doc(alias = "EIRQCR14")]
pub type Eirqcr14 = crate::Reg<eirqcr14::Eirqcr14Spec>;
#[doc = "desc EIRQCR14"]
pub mod eirqcr14;
#[doc = "EIRQCR15 (rw) register accessor: desc EIRQCR15\n\nYou can [`read`](crate::Reg::read) this register and get [`eirqcr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eirqcr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eirqcr15`] module"]
#[doc(alias = "EIRQCR15")]
pub type Eirqcr15 = crate::Reg<eirqcr15::Eirqcr15Spec>;
#[doc = "desc EIRQCR15"]
pub mod eirqcr15;
#[doc = "WKEN (rw) register accessor: desc WKEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`] module"]
#[doc(alias = "WKEN")]
pub type Wken = crate::Reg<wken::WkenSpec>;
#[doc = "desc WKEN"]
pub mod wken;
#[doc = "EIFR (rw) register accessor: desc EIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifr`] module"]
#[doc(alias = "EIFR")]
pub type Eifr = crate::Reg<eifr::EifrSpec>;
#[doc = "desc EIFR"]
pub mod eifr;
#[doc = "EIFCR (rw) register accessor: desc EIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`eifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifcr`] module"]
#[doc(alias = "EIFCR")]
pub type Eifcr = crate::Reg<eifcr::EifcrSpec>;
#[doc = "desc EIFCR"]
pub mod eifcr;
#[doc = "INTSEL0 (rw) register accessor: desc INTSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel0`] module"]
#[doc(alias = "INTSEL0")]
pub type Intsel0 = crate::Reg<intsel0::Intsel0Spec>;
#[doc = "desc INTSEL0"]
pub mod intsel0;
#[doc = "INTSEL1 (rw) register accessor: desc INTSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel1`] module"]
#[doc(alias = "INTSEL1")]
pub type Intsel1 = crate::Reg<intsel1::Intsel1Spec>;
#[doc = "desc INTSEL1"]
pub mod intsel1;
#[doc = "INTSEL2 (rw) register accessor: desc INTSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel2`] module"]
#[doc(alias = "INTSEL2")]
pub type Intsel2 = crate::Reg<intsel2::Intsel2Spec>;
#[doc = "desc INTSEL2"]
pub mod intsel2;
#[doc = "INTSEL3 (rw) register accessor: desc INTSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel3`] module"]
#[doc(alias = "INTSEL3")]
pub type Intsel3 = crate::Reg<intsel3::Intsel3Spec>;
#[doc = "desc INTSEL3"]
pub mod intsel3;
#[doc = "INTSEL4 (rw) register accessor: desc INTSEL4\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel4`] module"]
#[doc(alias = "INTSEL4")]
pub type Intsel4 = crate::Reg<intsel4::Intsel4Spec>;
#[doc = "desc INTSEL4"]
pub mod intsel4;
#[doc = "INTSEL5 (rw) register accessor: desc INTSEL5\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel5`] module"]
#[doc(alias = "INTSEL5")]
pub type Intsel5 = crate::Reg<intsel5::Intsel5Spec>;
#[doc = "desc INTSEL5"]
pub mod intsel5;
#[doc = "INTSEL6 (rw) register accessor: desc INTSEL6\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel6`] module"]
#[doc(alias = "INTSEL6")]
pub type Intsel6 = crate::Reg<intsel6::Intsel6Spec>;
#[doc = "desc INTSEL6"]
pub mod intsel6;
#[doc = "INTSEL7 (rw) register accessor: desc INTSEL7\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel7`] module"]
#[doc(alias = "INTSEL7")]
pub type Intsel7 = crate::Reg<intsel7::Intsel7Spec>;
#[doc = "desc INTSEL7"]
pub mod intsel7;
#[doc = "INTSEL8 (rw) register accessor: desc INTSEL8\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel8`] module"]
#[doc(alias = "INTSEL8")]
pub type Intsel8 = crate::Reg<intsel8::Intsel8Spec>;
#[doc = "desc INTSEL8"]
pub mod intsel8;
#[doc = "INTSEL9 (rw) register accessor: desc INTSEL9\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel9`] module"]
#[doc(alias = "INTSEL9")]
pub type Intsel9 = crate::Reg<intsel9::Intsel9Spec>;
#[doc = "desc INTSEL9"]
pub mod intsel9;
#[doc = "INTSEL10 (rw) register accessor: desc INTSEL10\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel10`] module"]
#[doc(alias = "INTSEL10")]
pub type Intsel10 = crate::Reg<intsel10::Intsel10Spec>;
#[doc = "desc INTSEL10"]
pub mod intsel10;
#[doc = "INTSEL11 (rw) register accessor: desc INTSEL11\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel11`] module"]
#[doc(alias = "INTSEL11")]
pub type Intsel11 = crate::Reg<intsel11::Intsel11Spec>;
#[doc = "desc INTSEL11"]
pub mod intsel11;
#[doc = "INTSEL12 (rw) register accessor: desc INTSEL12\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel12`] module"]
#[doc(alias = "INTSEL12")]
pub type Intsel12 = crate::Reg<intsel12::Intsel12Spec>;
#[doc = "desc INTSEL12"]
pub mod intsel12;
#[doc = "INTSEL13 (rw) register accessor: desc INTSEL13\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel13`] module"]
#[doc(alias = "INTSEL13")]
pub type Intsel13 = crate::Reg<intsel13::Intsel13Spec>;
#[doc = "desc INTSEL13"]
pub mod intsel13;
#[doc = "INTSEL14 (rw) register accessor: desc INTSEL14\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel14`] module"]
#[doc(alias = "INTSEL14")]
pub type Intsel14 = crate::Reg<intsel14::Intsel14Spec>;
#[doc = "desc INTSEL14"]
pub mod intsel14;
#[doc = "INTSEL15 (rw) register accessor: desc INTSEL15\n\nYou can [`read`](crate::Reg::read) this register and get [`intsel15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsel15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsel15`] module"]
#[doc(alias = "INTSEL15")]
pub type Intsel15 = crate::Reg<intsel15::Intsel15Spec>;
#[doc = "desc INTSEL15"]
pub mod intsel15;
#[doc = "INTEN0 (rw) register accessor: desc INTEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`inten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten0`] module"]
#[doc(alias = "INTEN0")]
pub type Inten0 = crate::Reg<inten0::Inten0Spec>;
#[doc = "desc INTEN0"]
pub mod inten0;
#[doc = "INTEN1 (rw) register accessor: desc INTEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`inten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten1`] module"]
#[doc(alias = "INTEN1")]
pub type Inten1 = crate::Reg<inten1::Inten1Spec>;
#[doc = "desc INTEN1"]
pub mod inten1;
#[doc = "INTEN2 (rw) register accessor: desc INTEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`inten2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten2`] module"]
#[doc(alias = "INTEN2")]
pub type Inten2 = crate::Reg<inten2::Inten2Spec>;
#[doc = "desc INTEN2"]
pub mod inten2;
#[doc = "INTEN3 (rw) register accessor: desc INTEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`inten3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten3`] module"]
#[doc(alias = "INTEN3")]
pub type Inten3 = crate::Reg<inten3::Inten3Spec>;
#[doc = "desc INTEN3"]
pub mod inten3;
#[doc = "INTEN4 (rw) register accessor: desc INTEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`inten4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten4`] module"]
#[doc(alias = "INTEN4")]
pub type Inten4 = crate::Reg<inten4::Inten4Spec>;
#[doc = "desc INTEN4"]
pub mod inten4;
#[doc = "INTEN5 (rw) register accessor: desc INTEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`inten5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten5`] module"]
#[doc(alias = "INTEN5")]
pub type Inten5 = crate::Reg<inten5::Inten5Spec>;
#[doc = "desc INTEN5"]
pub mod inten5;
#[doc = "INTEN6 (rw) register accessor: desc INTEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`inten6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten6`] module"]
#[doc(alias = "INTEN6")]
pub type Inten6 = crate::Reg<inten6::Inten6Spec>;
#[doc = "desc INTEN6"]
pub mod inten6;
#[doc = "INTEN7 (rw) register accessor: desc INTEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`inten7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten7`] module"]
#[doc(alias = "INTEN7")]
pub type Inten7 = crate::Reg<inten7::Inten7Spec>;
#[doc = "desc INTEN7"]
pub mod inten7;
#[doc = "INTEN8 (rw) register accessor: desc INTEN8\n\nYou can [`read`](crate::Reg::read) this register and get [`inten8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten8`] module"]
#[doc(alias = "INTEN8")]
pub type Inten8 = crate::Reg<inten8::Inten8Spec>;
#[doc = "desc INTEN8"]
pub mod inten8;
#[doc = "INTEN9 (rw) register accessor: desc INTEN9\n\nYou can [`read`](crate::Reg::read) this register and get [`inten9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten9`] module"]
#[doc(alias = "INTEN9")]
pub type Inten9 = crate::Reg<inten9::Inten9Spec>;
#[doc = "desc INTEN9"]
pub mod inten9;
#[doc = "INTEN10 (rw) register accessor: desc INTEN10\n\nYou can [`read`](crate::Reg::read) this register and get [`inten10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten10`] module"]
#[doc(alias = "INTEN10")]
pub type Inten10 = crate::Reg<inten10::Inten10Spec>;
#[doc = "desc INTEN10"]
pub mod inten10;
#[doc = "INTEN11 (rw) register accessor: desc INTEN11\n\nYou can [`read`](crate::Reg::read) this register and get [`inten11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten11`] module"]
#[doc(alias = "INTEN11")]
pub type Inten11 = crate::Reg<inten11::Inten11Spec>;
#[doc = "desc INTEN11"]
pub mod inten11;
#[doc = "INTEN12 (rw) register accessor: desc INTEN12\n\nYou can [`read`](crate::Reg::read) this register and get [`inten12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten12`] module"]
#[doc(alias = "INTEN12")]
pub type Inten12 = crate::Reg<inten12::Inten12Spec>;
#[doc = "desc INTEN12"]
pub mod inten12;
#[doc = "INTEN13 (rw) register accessor: desc INTEN13\n\nYou can [`read`](crate::Reg::read) this register and get [`inten13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten13`] module"]
#[doc(alias = "INTEN13")]
pub type Inten13 = crate::Reg<inten13::Inten13Spec>;
#[doc = "desc INTEN13"]
pub mod inten13;
#[doc = "INTEN14 (rw) register accessor: desc INTEN14\n\nYou can [`read`](crate::Reg::read) this register and get [`inten14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten14`] module"]
#[doc(alias = "INTEN14")]
pub type Inten14 = crate::Reg<inten14::Inten14Spec>;
#[doc = "desc INTEN14"]
pub mod inten14;
#[doc = "INTEN15 (rw) register accessor: desc INTEN15\n\nYou can [`read`](crate::Reg::read) this register and get [`inten15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten15`] module"]
#[doc(alias = "INTEN15")]
pub type Inten15 = crate::Reg<inten15::Inten15Spec>;
#[doc = "desc INTEN15"]
pub mod inten15;
#[doc = "SWIER (rw) register accessor: desc SWIER\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`] module"]
#[doc(alias = "SWIER")]
pub type Swier = crate::Reg<swier::SwierSpec>;
#[doc = "desc SWIER"]
pub mod swier;
#[doc = "EVTER (rw) register accessor: desc EVTER\n\nYou can [`read`](crate::Reg::read) this register and get [`evter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evter`] module"]
#[doc(alias = "EVTER")]
pub type Evter = crate::Reg<evter::EvterSpec>;
#[doc = "desc EVTER"]
pub mod evter;
#[doc = "IER (rw) register accessor: desc IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "desc IER"]
pub mod ier;
