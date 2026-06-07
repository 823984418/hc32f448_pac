#[doc = "Register `DAT22` writer"]
pub type W = crate::W<Dat22Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat22Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT22\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat22::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat22Spec;
impl crate::RegisterSpec for Dat22Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat22::W`](W) writer structure"]
impl crate::Writable for Dat22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT22 to value 0"]
impl crate::Resettable for Dat22Spec {}
