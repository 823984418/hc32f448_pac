#[doc = "Register `DAR4` reader"]
pub type R = crate::R<Dar4Spec>;
#[doc = "Register `DAR4` writer"]
pub type W = crate::W<Dar4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`dar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar4Spec;
impl crate::RegisterSpec for Dar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar4::R`](R) reader structure"]
impl crate::Readable for Dar4Spec {}
#[doc = "`write(|w| ..)` method takes [`dar4::W`](W) writer structure"]
impl crate::Writable for Dar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR4 to value 0"]
impl crate::Resettable for Dar4Spec {}
