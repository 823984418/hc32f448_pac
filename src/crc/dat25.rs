#[doc = "Register `DAT25` writer"]
pub type W = crate::W<Dat25Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat25Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT25\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat25::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat25Spec;
impl crate::RegisterSpec for Dat25Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat25::W`](W) writer structure"]
impl crate::Writable for Dat25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT25 to value 0"]
impl crate::Resettable for Dat25Spec {}
