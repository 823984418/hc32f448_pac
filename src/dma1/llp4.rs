#[doc = "Register `LLP4` reader"]
pub type R = crate::R<Llp4Spec>;
#[doc = "Register `LLP4` writer"]
pub type W = crate::W<Llp4Spec>;
#[doc = "Field `LLP` reader - desc LLP"]
pub type LlpR = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - desc LLP"]
pub type LlpW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - desc LLP"]
    #[inline(always)]
    pub fn llp(&self) -> LlpR {
        LlpR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLP4").field("llp", &self.llp()).finish()
    }
}
impl W {
    #[doc = "Bits 2:31 - desc LLP"]
    #[inline(always)]
    pub fn llp(&mut self) -> LlpW<'_, Llp4Spec> {
        LlpW::new(self, 2)
    }
}
#[doc = "desc LLP4\n\nYou can [`read`](crate::Reg::read) this register and get [`llp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp4Spec;
impl crate::RegisterSpec for Llp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp4::R`](R) reader structure"]
impl crate::Readable for Llp4Spec {}
#[doc = "`write(|w| ..)` method takes [`llp4::W`](W) writer structure"]
impl crate::Writable for Llp4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP4 to value 0"]
impl crate::Resettable for Llp4Spec {}
