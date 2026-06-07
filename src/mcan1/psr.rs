#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Field `LEC` reader - desc LEC"]
pub type LecR = crate::FieldReader;
#[doc = "Field `ACT` reader - desc ACT"]
pub type ActR = crate::FieldReader;
#[doc = "Field `EP` reader - desc EP"]
pub type EpR = crate::BitReader;
#[doc = "Field `EW` reader - desc EW"]
pub type EwR = crate::BitReader;
#[doc = "Field `BO` reader - desc BO"]
pub type BoR = crate::BitReader;
#[doc = "Field `DLEC` reader - desc DLEC"]
pub type DlecR = crate::FieldReader;
#[doc = "Field `RESI` reader - desc RESI"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RBRS` reader - desc RBRS"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RFDF` reader - desc RFDF"]
pub type RfdfR = crate::BitReader;
#[doc = "Field `PXE` reader - desc PXE"]
pub type PxeR = crate::BitReader;
#[doc = "Field `TDCV` reader - desc TDCV"]
pub type TdcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - desc LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - desc ACT"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - desc EP"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EW"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BO"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc DLEC"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc RESI"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc RFDF"]
    #[inline(always)]
    pub fn rfdf(&self) -> RfdfR {
        RfdfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - desc TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("rfdf", &self.rfdf())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
#[doc = "desc PSR\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}
