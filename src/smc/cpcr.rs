#[doc = "Register `CPCR` writer"]
pub type W = crate::W<CpcrSpec>;
#[doc = "Field `RSYN` writer - desc RSYN"]
pub type RsynW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WSYN` writer - desc WSYN"]
pub type WsynW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MW` writer - desc MW"]
pub type MwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BAAS` writer - desc BAAS"]
pub type BaasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVS` writer - desc ADVS"]
pub type AdvsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLSS` writer - desc BLSS"]
pub type BlssW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CpcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - desc RSYN"]
    #[inline(always)]
    pub fn rsyn(&mut self) -> RsynW<'_, CpcrSpec> {
        RsynW::new(self, 0)
    }
    #[doc = "Bit 4 - desc WSYN"]
    #[inline(always)]
    pub fn wsyn(&mut self) -> WsynW<'_, CpcrSpec> {
        WsynW::new(self, 4)
    }
    #[doc = "Bits 8:9 - desc MW"]
    #[inline(always)]
    pub fn mw(&mut self) -> MwW<'_, CpcrSpec> {
        MwW::new(self, 8)
    }
    #[doc = "Bit 10 - desc BAAS"]
    #[inline(always)]
    pub fn baas(&mut self) -> BaasW<'_, CpcrSpec> {
        BaasW::new(self, 10)
    }
    #[doc = "Bit 11 - desc ADVS"]
    #[inline(always)]
    pub fn advs(&mut self) -> AdvsW<'_, CpcrSpec> {
        AdvsW::new(self, 11)
    }
    #[doc = "Bit 12 - desc BLSS"]
    #[inline(always)]
    pub fn blss(&mut self) -> BlssW<'_, CpcrSpec> {
        BlssW::new(self, 12)
    }
}
#[doc = "desc CPCR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpcrSpec;
impl crate::RegisterSpec for CpcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cpcr::W`](W) writer structure"]
impl crate::Writable for CpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPCR to value 0"]
impl crate::Resettable for CpcrSpec {}
