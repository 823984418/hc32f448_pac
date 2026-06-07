#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    chipid: Chipid,
    _reserved1: [u8; 0x0c],
    mcudbgcstat: Mcudbgcstat,
    mcustpctl: Mcustpctl,
    mcutracectl: Mcutracectl,
    mcustpctl2: Mcustpctl2,
}
impl RegisterBlock {
    #[doc = "0x0c - desc CHIPID"]
    #[inline(always)]
    pub const fn chipid(&self) -> &Chipid {
        &self.chipid
    }
    #[doc = "0x1c - desc MCUDBGCSTAT"]
    #[inline(always)]
    pub const fn mcudbgcstat(&self) -> &Mcudbgcstat {
        &self.mcudbgcstat
    }
    #[doc = "0x20 - desc MCUSTPCTL"]
    #[inline(always)]
    pub const fn mcustpctl(&self) -> &Mcustpctl {
        &self.mcustpctl
    }
    #[doc = "0x24 - desc MCUTRACECTL"]
    #[inline(always)]
    pub const fn mcutracectl(&self) -> &Mcutracectl {
        &self.mcutracectl
    }
    #[doc = "0x28 - desc MCUSTPCTL2"]
    #[inline(always)]
    pub const fn mcustpctl2(&self) -> &Mcustpctl2 {
        &self.mcustpctl2
    }
}
#[doc = "CHIPID (r) register accessor: desc CHIPID\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chipid`] module"]
#[doc(alias = "CHIPID")]
pub type Chipid = crate::Reg<chipid::ChipidSpec>;
#[doc = "desc CHIPID"]
pub mod chipid;
#[doc = "MCUDBGCSTAT (rw) register accessor: desc MCUDBGCSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcudbgcstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudbgcstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcudbgcstat`] module"]
#[doc(alias = "MCUDBGCSTAT")]
pub type Mcudbgcstat = crate::Reg<mcudbgcstat::McudbgcstatSpec>;
#[doc = "desc MCUDBGCSTAT"]
pub mod mcudbgcstat;
#[doc = "MCUSTPCTL (rw) register accessor: desc MCUSTPCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcustpctl`] module"]
#[doc(alias = "MCUSTPCTL")]
pub type Mcustpctl = crate::Reg<mcustpctl::McustpctlSpec>;
#[doc = "desc MCUSTPCTL"]
pub mod mcustpctl;
#[doc = "MCUTRACECTL (rw) register accessor: desc MCUTRACECTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcutracectl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcutracectl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcutracectl`] module"]
#[doc(alias = "MCUTRACECTL")]
pub type Mcutracectl = crate::Reg<mcutracectl::McutracectlSpec>;
#[doc = "desc MCUTRACECTL"]
pub mod mcutracectl;
#[doc = "MCUSTPCTL2 (rw) register accessor: desc MCUSTPCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcustpctl2`] module"]
#[doc(alias = "MCUSTPCTL2")]
pub type Mcustpctl2 = crate::Reg<mcustpctl2::Mcustpctl2Spec>;
#[doc = "desc MCUSTPCTL2"]
pub mod mcustpctl2;
