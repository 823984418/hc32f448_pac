#[doc = "Register `DAT5` writer"]
pub type W = crate::W<Dat5Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat5Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT5\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat5Spec;
impl crate::RegisterSpec for Dat5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat5::W`](W) writer structure"]
impl crate::Writable for Dat5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT5 to value 0"]
impl crate::Resettable for Dat5Spec {}
