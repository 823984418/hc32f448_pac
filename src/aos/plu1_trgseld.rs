#[doc = "Register `PLU1_TRGSELD` reader"]
pub type R = crate::R<Plu1TrgseldSpec>;
#[doc = "Register `PLU1_TRGSELD` writer"]
pub type W = crate::W<Plu1TrgseldSpec>;
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
        f.debug_struct("PLU1_TRGSELD")
            .field("pltrgsel", &self.pltrgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc PLTRGSEL"]
    #[inline(always)]
    pub fn pltrgsel(&mut self) -> PltrgselW<'_, Plu1TrgseldSpec> {
        PltrgselW::new(self, 0)
    }
}
#[doc = "desc PLU1_TRGSELD\n\nYou can [`read`](crate::Reg::read) this register and get [`plu1_trgseld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plu1_trgseld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Plu1TrgseldSpec;
impl crate::RegisterSpec for Plu1TrgseldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plu1_trgseld::R`](R) reader structure"]
impl crate::Readable for Plu1TrgseldSpec {}
#[doc = "`write(|w| ..)` method takes [`plu1_trgseld::W`](W) writer structure"]
impl crate::Writable for Plu1TrgseldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLU1_TRGSELD to value 0x01ff"]
impl crate::Resettable for Plu1TrgseldSpec {
    const RESET_VALUE: u32 = 0x01ff;
}
