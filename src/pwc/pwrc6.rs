#[doc = "Register `PWRC6` reader"]
pub type R = crate::R<Pwrc6Spec>;
#[doc = "Register `PWRC6` writer"]
pub type W = crate::W<Pwrc6Spec>;
#[doc = "Field `RTCCKSEL` reader - desc RTCCKSEL"]
pub type RtcckselR = crate::FieldReader;
#[doc = "Field `RTCCKSEL` writer - desc RTCCKSEL"]
pub type RtcckselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc RTCCKSEL"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RtcckselR {
        RtcckselR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC6")
            .field("rtccksel", &self.rtccksel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc RTCCKSEL"]
    #[inline(always)]
    pub fn rtccksel(&mut self) -> RtcckselW<'_, Pwrc6Spec> {
        RtcckselW::new(self, 0)
    }
}
#[doc = "desc PWRC6\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc6Spec;
impl crate::RegisterSpec for Pwrc6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc6::R`](R) reader structure"]
impl crate::Readable for Pwrc6Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc6::W`](W) writer structure"]
impl crate::Writable for Pwrc6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC6 to value 0"]
impl crate::Resettable for Pwrc6Spec {}
