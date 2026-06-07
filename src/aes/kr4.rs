#[doc = "Register `KR4` reader"]
pub type R = crate::R<Kr4Spec>;
#[doc = "Register `KR4` writer"]
pub type W = crate::W<Kr4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR4\n\nYou can [`read`](crate::Reg::read) this register and get [`kr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr4Spec;
impl crate::RegisterSpec for Kr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr4::R`](R) reader structure"]
impl crate::Readable for Kr4Spec {}
#[doc = "`write(|w| ..)` method takes [`kr4::W`](W) writer structure"]
impl crate::Writable for Kr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR4 to value 0"]
impl crate::Resettable for Kr4Spec {}
