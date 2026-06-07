#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    icg0: Icg0,
    icg1: Icg1,
    _reserved2: [u8; 0x04],
    icg3: Icg3,
}
impl RegisterBlock {
    #[doc = "0x00 - desc ICG0"]
    #[inline(always)]
    pub const fn icg0(&self) -> &Icg0 {
        &self.icg0
    }
    #[doc = "0x04 - desc ICG1"]
    #[inline(always)]
    pub const fn icg1(&self) -> &Icg1 {
        &self.icg1
    }
    #[doc = "0x0c - desc ICG3"]
    #[inline(always)]
    pub const fn icg3(&self) -> &Icg3 {
        &self.icg3
    }
}
#[doc = "ICG0 (r) register accessor: desc ICG0\n\nYou can [`read`](crate::Reg::read) this register and get [`icg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg0`] module"]
#[doc(alias = "ICG0")]
pub type Icg0 = crate::Reg<icg0::Icg0Spec>;
#[doc = "desc ICG0"]
pub mod icg0;
#[doc = "ICG1 (r) register accessor: desc ICG1\n\nYou can [`read`](crate::Reg::read) this register and get [`icg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg1`] module"]
#[doc(alias = "ICG1")]
pub type Icg1 = crate::Reg<icg1::Icg1Spec>;
#[doc = "desc ICG1"]
pub mod icg1;
#[doc = "ICG3 (r) register accessor: desc ICG3\n\nYou can [`read`](crate::Reg::read) this register and get [`icg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icg3`] module"]
#[doc(alias = "ICG3")]
pub type Icg3 = crate::Reg<icg3::Icg3Spec>;
#[doc = "desc ICG3"]
pub mod icg3;
