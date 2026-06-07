#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    emb_ctl1: EmbCtl1,
    emb_ctl2: EmbCtl2,
    emb_soe: EmbSoe,
    emb_stat: EmbStat,
    emb_statclr: EmbStatclr,
    emb_inten: EmbInten,
    emb_rlssel: EmbRlssel,
}
impl RegisterBlock {
    #[doc = "0x00 - desc EMB_CTL1"]
    #[inline(always)]
    pub const fn emb_ctl1(&self) -> &EmbCtl1 {
        &self.emb_ctl1
    }
    #[doc = "0x04 - desc EMB_CTL2"]
    #[inline(always)]
    pub const fn emb_ctl2(&self) -> &EmbCtl2 {
        &self.emb_ctl2
    }
    #[doc = "0x08 - desc EMB_SOE"]
    #[inline(always)]
    pub const fn emb_soe(&self) -> &EmbSoe {
        &self.emb_soe
    }
    #[doc = "0x0c - desc EMB_STAT"]
    #[inline(always)]
    pub const fn emb_stat(&self) -> &EmbStat {
        &self.emb_stat
    }
    #[doc = "0x10 - desc EMB_STATCLR"]
    #[inline(always)]
    pub const fn emb_statclr(&self) -> &EmbStatclr {
        &self.emb_statclr
    }
    #[doc = "0x14 - desc EMB_INTEN"]
    #[inline(always)]
    pub const fn emb_inten(&self) -> &EmbInten {
        &self.emb_inten
    }
    #[doc = "0x18 - desc EMB_RLSSEL"]
    #[inline(always)]
    pub const fn emb_rlssel(&self) -> &EmbRlssel {
        &self.emb_rlssel
    }
}
#[doc = "EMB_CTL1 (rw) register accessor: desc EMB_CTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_ctl1`] module"]
#[doc(alias = "EMB_CTL1")]
pub type EmbCtl1 = crate::Reg<emb_ctl1::EmbCtl1Spec>;
#[doc = "desc EMB_CTL1"]
pub mod emb_ctl1;
#[doc = "EMB_CTL2 (rw) register accessor: desc EMB_CTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_ctl2`] module"]
#[doc(alias = "EMB_CTL2")]
pub type EmbCtl2 = crate::Reg<emb_ctl2::EmbCtl2Spec>;
#[doc = "desc EMB_CTL2"]
pub mod emb_ctl2;
#[doc = "EMB_SOE (rw) register accessor: desc EMB_SOE\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_soe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_soe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_soe`] module"]
#[doc(alias = "EMB_SOE")]
pub type EmbSoe = crate::Reg<emb_soe::EmbSoeSpec>;
#[doc = "desc EMB_SOE"]
pub mod emb_soe;
#[doc = "EMB_STAT (r) register accessor: desc EMB_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_stat`] module"]
#[doc(alias = "EMB_STAT")]
pub type EmbStat = crate::Reg<emb_stat::EmbStatSpec>;
#[doc = "desc EMB_STAT"]
pub mod emb_stat;
#[doc = "EMB_STATCLR (w) register accessor: desc EMB_STATCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_statclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_statclr`] module"]
#[doc(alias = "EMB_STATCLR")]
pub type EmbStatclr = crate::Reg<emb_statclr::EmbStatclrSpec>;
#[doc = "desc EMB_STATCLR"]
pub mod emb_statclr;
#[doc = "EMB_INTEN (rw) register accessor: desc EMB_INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_inten`] module"]
#[doc(alias = "EMB_INTEN")]
pub type EmbInten = crate::Reg<emb_inten::EmbIntenSpec>;
#[doc = "desc EMB_INTEN"]
pub mod emb_inten;
#[doc = "EMB_RLSSEL (rw) register accessor: desc EMB_RLSSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_rlssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emb_rlssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emb_rlssel`] module"]
#[doc(alias = "EMB_RLSSEL")]
pub type EmbRlssel = crate::Reg<emb_rlssel::EmbRlsselSpec>;
#[doc = "desc EMB_RLSSEL"]
pub mod emb_rlssel;
