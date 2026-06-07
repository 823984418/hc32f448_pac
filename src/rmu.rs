#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frst0: Frst0,
    frst1: Frst1,
    frst2: Frst2,
    frst3: Frst3,
    prstcr0: Prstcr0,
    _reserved5: [u8; 0x03],
    rstf0: Rstf0,
}
impl RegisterBlock {
    #[doc = "0x00 - desc FRST0"]
    #[inline(always)]
    pub const fn frst0(&self) -> &Frst0 {
        &self.frst0
    }
    #[doc = "0x04 - desc FRST1"]
    #[inline(always)]
    pub const fn frst1(&self) -> &Frst1 {
        &self.frst1
    }
    #[doc = "0x08 - desc FRST2"]
    #[inline(always)]
    pub const fn frst2(&self) -> &Frst2 {
        &self.frst2
    }
    #[doc = "0x0c - desc FRST3"]
    #[inline(always)]
    pub const fn frst3(&self) -> &Frst3 {
        &self.frst3
    }
    #[doc = "0x10 - desc PRSTCR0"]
    #[inline(always)]
    pub const fn prstcr0(&self) -> &Prstcr0 {
        &self.prstcr0
    }
    #[doc = "0x14 - desc RSTF0"]
    #[inline(always)]
    pub const fn rstf0(&self) -> &Rstf0 {
        &self.rstf0
    }
}
#[doc = "FRST0 (rw) register accessor: desc FRST0\n\nYou can [`read`](crate::Reg::read) this register and get [`frst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frst0`] module"]
#[doc(alias = "FRST0")]
pub type Frst0 = crate::Reg<frst0::Frst0Spec>;
#[doc = "desc FRST0"]
pub mod frst0;
#[doc = "FRST1 (rw) register accessor: desc FRST1\n\nYou can [`read`](crate::Reg::read) this register and get [`frst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frst1`] module"]
#[doc(alias = "FRST1")]
pub type Frst1 = crate::Reg<frst1::Frst1Spec>;
#[doc = "desc FRST1"]
pub mod frst1;
#[doc = "FRST2 (rw) register accessor: desc FRST2\n\nYou can [`read`](crate::Reg::read) this register and get [`frst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frst2`] module"]
#[doc(alias = "FRST2")]
pub type Frst2 = crate::Reg<frst2::Frst2Spec>;
#[doc = "desc FRST2"]
pub mod frst2;
#[doc = "FRST3 (rw) register accessor: desc FRST3\n\nYou can [`read`](crate::Reg::read) this register and get [`frst3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frst3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frst3`] module"]
#[doc(alias = "FRST3")]
pub type Frst3 = crate::Reg<frst3::Frst3Spec>;
#[doc = "desc FRST3"]
pub mod frst3;
#[doc = "PRSTCR0 (rw) register accessor: desc PRSTCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`prstcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prstcr0`] module"]
#[doc(alias = "PRSTCR0")]
pub type Prstcr0 = crate::Reg<prstcr0::Prstcr0Spec>;
#[doc = "desc PRSTCR0"]
pub mod prstcr0;
#[doc = "RSTF0 (rw) register accessor: desc RSTF0\n\nYou can [`read`](crate::Reg::read) this register and get [`rstf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstf0`] module"]
#[doc(alias = "RSTF0")]
pub type Rstf0 = crate::Reg<rstf0::Rstf0Spec>;
#[doc = "desc RSTF0"]
pub mod rstf0;
