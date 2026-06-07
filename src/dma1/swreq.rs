#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SwreqSpec>;
#[doc = "Field `SWREQ` writer - desc SWREQ"]
pub type SwreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWRCFGREQ` writer - desc SWRCFGREQ"]
pub type SwrcfgreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWREQWP` writer - desc SWREQWP"]
pub type SwreqwpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SWRCFGWP` writer - desc SWRCFGWP"]
pub type SwrcfgwpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<SwreqSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - desc SWREQ"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<'_, SwreqSpec> {
        SwreqW::new(self, 0)
    }
    #[doc = "Bit 15 - desc SWRCFGREQ"]
    #[inline(always)]
    pub fn swrcfgreq(&mut self) -> SwrcfgreqW<'_, SwreqSpec> {
        SwrcfgreqW::new(self, 15)
    }
    #[doc = "Bits 16:23 - desc SWREQWP"]
    #[inline(always)]
    pub fn swreqwp(&mut self) -> SwreqwpW<'_, SwreqSpec> {
        SwreqwpW::new(self, 16)
    }
    #[doc = "Bits 24:31 - desc SWRCFGWP"]
    #[inline(always)]
    pub fn swrcfgwp(&mut self) -> SwrcfgwpW<'_, SwreqSpec> {
        SwrcfgwpW::new(self, 24)
    }
}
#[doc = "desc SWREQ\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwreqSpec;
impl crate::RegisterSpec for SwreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SwreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SwreqSpec {}
