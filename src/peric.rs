#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    smc_enar: SmcEnar,
    _reserved1: [u8; 0x04],
    tmr_synenr: TmrSynenr,
    _reserved2: [u8; 0x04],
    usart1_nfc: Usart1Nfc,
}
impl RegisterBlock {
    #[doc = "0x0c - desc SMC_ENAR"]
    #[inline(always)]
    pub const fn smc_enar(&self) -> &SmcEnar {
        &self.smc_enar
    }
    #[doc = "0x14 - desc TMR_SYNENR"]
    #[inline(always)]
    pub const fn tmr_synenr(&self) -> &TmrSynenr {
        &self.tmr_synenr
    }
    #[doc = "0x1c - desc USART1_NFC"]
    #[inline(always)]
    pub const fn usart1_nfc(&self) -> &Usart1Nfc {
        &self.usart1_nfc
    }
}
#[doc = "SMC_ENAR (rw) register accessor: desc SMC_ENAR\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_enar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_enar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_enar`] module"]
#[doc(alias = "SMC_ENAR")]
pub type SmcEnar = crate::Reg<smc_enar::SmcEnarSpec>;
#[doc = "desc SMC_ENAR"]
pub mod smc_enar;
#[doc = "TMR_SYNENR (rw) register accessor: desc TMR_SYNENR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_synenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_synenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_synenr`] module"]
#[doc(alias = "TMR_SYNENR")]
pub type TmrSynenr = crate::Reg<tmr_synenr::TmrSynenrSpec>;
#[doc = "desc TMR_SYNENR"]
pub mod tmr_synenr;
#[doc = "USART1_NFC (rw) register accessor: desc USART1_NFC\n\nYou can [`read`](crate::Reg::read) this register and get [`usart1_nfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_nfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart1_nfc`] module"]
#[doc(alias = "USART1_NFC")]
pub type Usart1Nfc = crate::Reg<usart1_nfc::Usart1NfcSpec>;
#[doc = "desc USART1_NFC"]
pub mod usart1_nfc;
