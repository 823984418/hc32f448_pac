#[doc = "Register `LLP5` reader"]
pub type R = crate::R<Llp5Spec>;
#[doc = "Register `LLP5` writer"]
pub type W = crate::W<Llp5Spec>;
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
        f.debug_struct("LLP5").field("llp", &self.llp()).finish()
    }
}
impl W {
    #[doc = "Bits 2:31 - desc LLP"]
    #[inline(always)]
    pub fn llp(&mut self) -> LlpW<'_, Llp5Spec> {
        LlpW::new(self, 2)
    }
}
#[doc = "desc LLP5\n\nYou can [`read`](crate::Reg::read) this register and get [`llp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp5Spec;
impl crate::RegisterSpec for Llp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp5::R`](R) reader structure"]
impl crate::Readable for Llp5Spec {}
#[doc = "`write(|w| ..)` method takes [`llp5::W`](W) writer structure"]
impl crate::Writable for Llp5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP5 to value 0"]
impl crate::Resettable for Llp5Spec {}
