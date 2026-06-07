#[doc = "Register `KR6` reader"]
pub type R = crate::R<Kr6Spec>;
#[doc = "Register `KR6` writer"]
pub type W = crate::W<Kr6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc KR6\n\nYou can [`read`](crate::Reg::read) this register and get [`kr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Kr6Spec;
impl crate::RegisterSpec for Kr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kr6::R`](R) reader structure"]
impl crate::Readable for Kr6Spec {}
#[doc = "`write(|w| ..)` method takes [`kr6::W`](W) writer structure"]
impl crate::Writable for Kr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR6 to value 0"]
impl crate::Resettable for Kr6Spec {}
