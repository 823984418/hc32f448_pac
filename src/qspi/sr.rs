#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `XIPF` reader - desc XIPF"]
pub type XipfR = crate::BitReader;
#[doc = "Field `RAER` reader - desc RAER"]
pub type RaerR = crate::BitReader;
#[doc = "Field `PFNUM` reader - desc PFNUM"]
pub type PfnumR = crate::FieldReader;
#[doc = "Field `PFFUL` reader - desc PFFUL"]
pub type PffulR = crate::BitReader;
#[doc = "Field `PFAN` reader - desc PFAN"]
pub type PfanR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - desc XIPF"]
    #[inline(always)]
    pub fn xipf(&self) -> XipfR {
        XipfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RAER"]
    #[inline(always)]
    pub fn raer(&self) -> RaerR {
        RaerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - desc PFNUM"]
    #[inline(always)]
    pub fn pfnum(&self) -> PfnumR {
        PfnumR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - desc PFFUL"]
    #[inline(always)]
    pub fn pfful(&self) -> PffulR {
        PffulR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PFAN"]
    #[inline(always)]
    pub fn pfan(&self) -> PfanR {
        PfanR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("xipf", &self.xipf())
            .field("raer", &self.raer())
            .field("pfnum", &self.pfnum())
            .field("pfful", &self.pfful())
            .field("pfan", &self.pfan())
            .finish()
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x8000"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x8000;
}
