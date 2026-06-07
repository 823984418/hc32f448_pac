#[doc = "Register `COMTRGSEL2` reader"]
pub type R = crate::R<Comtrgsel2Spec>;
#[doc = "Register `COMTRGSEL2` writer"]
pub type W = crate::W<Comtrgsel2Spec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMTRGSEL2")
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Comtrgsel2Spec> {
        TrgselW::new(self, 0)
    }
}
#[doc = "desc COMTRGSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`comtrgsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comtrgsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comtrgsel2Spec;
impl crate::RegisterSpec for Comtrgsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comtrgsel2::R`](R) reader structure"]
impl crate::Readable for Comtrgsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`comtrgsel2::W`](W) writer structure"]
impl crate::Writable for Comtrgsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMTRGSEL2 to value 0x01ff"]
impl crate::Resettable for Comtrgsel2Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
