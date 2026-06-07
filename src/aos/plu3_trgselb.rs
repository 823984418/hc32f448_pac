#[doc = "Register `PLU3_TRGSELB` reader"]
pub type R = crate::R<Plu3TrgselbSpec>;
#[doc = "Register `PLU3_TRGSELB` writer"]
pub type W = crate::W<Plu3TrgselbSpec>;
#[doc = "Field `PLTRGSEL` reader - desc PLTRGSEL"]
pub type PltrgselR = crate::FieldReader<u16>;
#[doc = "Field `PLTRGSEL` writer - desc PLTRGSEL"]
pub type PltrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc PLTRGSEL"]
    #[inline(always)]
    pub fn pltrgsel(&self) -> PltrgselR {
        PltrgselR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLU3_TRGSELB")
            .field("pltrgsel", &self.pltrgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc PLTRGSEL"]
    #[inline(always)]
    pub fn pltrgsel(&mut self) -> PltrgselW<'_, Plu3TrgselbSpec> {
        PltrgselW::new(self, 0)
    }
}
#[doc = "desc PLU3_TRGSELB\n\nYou can [`read`](crate::Reg::read) this register and get [`plu3_trgselb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plu3_trgselb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plu3TrgselbSpec;
impl crate::RegisterSpec for Plu3TrgselbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plu3_trgselb::R`](R) reader structure"]
impl crate::Readable for Plu3TrgselbSpec {}
#[doc = "`write(|w| ..)` method takes [`plu3_trgselb::W`](W) writer structure"]
impl crate::Writable for Plu3TrgselbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLU3_TRGSELB to value 0x01ff"]
impl crate::Resettable for Plu3TrgselbSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
