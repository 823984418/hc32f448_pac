#[doc = "Register `INTEN11` reader"]
pub type R = crate::R<Inten11Spec>;
#[doc = "Register `INTEN11` writer"]
pub type W = crate::W<Inten11Spec>;
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
        f.debug_struct("INTEN11")
            .field("inten", &self.inten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - desc INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, Inten11Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "desc INTEN11\n\nYou can [`read`](crate::Reg::read) this register and get [`inten11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inten11Spec;
impl crate::RegisterSpec for Inten11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten11::R`](R) reader structure"]
impl crate::Readable for Inten11Spec {}
#[doc = "`write(|w| ..)` method takes [`inten11::W`](W) writer structure"]
impl crate::Writable for Inten11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN11 to value 0xffff_ffff"]
impl crate::Resettable for Inten11Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
