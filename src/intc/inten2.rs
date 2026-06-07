#[doc = "Register `INTEN2` reader"]
pub type R = crate::R<Inten2Spec>;
#[doc = "Register `INTEN2` writer"]
pub type W = crate::W<Inten2Spec>;
#[doc = "Field `INTEN` reader - desc INTEN"]
pub type IntenR = crate::FieldReader<u32>;
#[doc = "Field `INTEN` writer - desc INTEN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN2")
            .field("inten", &self.inten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, Inten2Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "desc INTEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`inten2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inten2Spec;
impl crate::RegisterSpec for Inten2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten2::R`](R) reader structure"]
impl crate::Readable for Inten2Spec {}
#[doc = "`write(|w| ..)` method takes [`inten2::W`](W) writer structure"]
impl crate::Writable for Inten2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN2 to value 0xffff_ffff"]
impl crate::Resettable for Inten2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
