#[doc = "Register `PLU2_TRGSELD` reader"]
pub type R = crate::R<Plu2TrgseldSpec>;
#[doc = "Register `PLU2_TRGSELD` writer"]
pub type W = crate::W<Plu2TrgseldSpec>;
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
        f.debug_struct("PLU2_TRGSELD")
            .field("pltrgsel", &self.pltrgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc PLTRGSEL"]
    #[inline(always)]
    pub fn pltrgsel(&mut self) -> PltrgselW<'_, Plu2TrgseldSpec> {
        PltrgselW::new(self, 0)
    }
}
#[doc = "desc PLU2_TRGSELD\n\nYou can [`read`](crate::Reg::read) this register and get [`plu2_trgseld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plu2_trgseld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plu2TrgseldSpec;
impl crate::RegisterSpec for Plu2TrgseldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plu2_trgseld::R`](R) reader structure"]
impl crate::Readable for Plu2TrgseldSpec {}
#[doc = "`write(|w| ..)` method takes [`plu2_trgseld::W`](W) writer structure"]
impl crate::Writable for Plu2TrgseldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLU2_TRGSELD to value 0x01ff"]
impl crate::Resettable for Plu2TrgseldSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
