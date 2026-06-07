#[doc = "Register `SRAMB_EIBIT0` reader"]
pub type R = crate::R<SrambEibit0Spec>;
#[doc = "Register `SRAMB_EIBIT0` writer"]
pub type W = crate::W<SrambEibit0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SRAMB_EIBIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eibit0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramb_eibit0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrambEibit0Spec;
impl crate::RegisterSpec for SrambEibit0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramb_eibit0::R`](R) reader structure"]
impl crate::Readable for SrambEibit0Spec {}
#[doc = "`write(|w| ..)` method takes [`sramb_eibit0::W`](W) writer structure"]
impl crate::Writable for SrambEibit0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAMB_EIBIT0 to value 0"]
impl crate::Resettable for SrambEibit0Spec {}
