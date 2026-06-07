#[doc = "Register `PLU1_TRGSELC` reader"]
pub type R = crate::R<Plu1TrgselcSpec>;
#[doc = "Register `PLU1_TRGSELC` writer"]
pub type W = crate::W<Plu1TrgselcSpec>;
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
        f.debug_struct("PLU1_TRGSELC")
            .field("pltrgsel", &self.pltrgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc PLTRGSEL"]
    #[inline(always)]
    pub fn pltrgsel(&mut self) -> PltrgselW<'_, Plu1TrgselcSpec> {
        PltrgselW::new(self, 0)
    }
}
#[doc = "desc PLU1_TRGSELC\n\nYou can [`read`](crate::Reg::read) this register and get [`plu1_trgselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plu1_trgselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plu1TrgselcSpec;
impl crate::RegisterSpec for Plu1TrgselcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plu1_trgselc::R`](R) reader structure"]
impl crate::Readable for Plu1TrgselcSpec {}
#[doc = "`write(|w| ..)` method takes [`plu1_trgselc::W`](W) writer structure"]
impl crate::Writable for Plu1TrgselcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLU1_TRGSELC to value 0x01ff"]
impl crate::Resettable for Plu1TrgselcSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
