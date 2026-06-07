#[doc = "Register `PWRC3` reader"]
pub type R = crate::R<Pwrc3Spec>;
#[doc = "Register `PWRC3` writer"]
pub type W = crate::W<Pwrc3Spec>;
#[doc = "Field `DDAS` reader - desc DDAS"]
pub type DdasR = crate::FieldReader;
#[doc = "Field `DDAS` writer - desc DDAS"]
pub type DdasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc DDAS"]
    #[inline(always)]
    pub fn ddas(&self) -> DdasR {
        DdasR::new(self.bits & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC3").field("ddas", &self.ddas()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc DDAS"]
    #[inline(always)]
    pub fn ddas(&mut self) -> DdasW<'_, Pwrc3Spec> {
        DdasW::new(self, 0)
    }
}
#[doc = "desc PWRC3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc3Spec;
impl crate::RegisterSpec for Pwrc3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc3::R`](R) reader structure"]
impl crate::Readable for Pwrc3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc3::W`](W) writer structure"]
impl crate::Writable for Pwrc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC3 to value 0x0f"]
impl crate::Resettable for Pwrc3Spec {
    const RESET_VALUE: u8 = 0x0f;
}
