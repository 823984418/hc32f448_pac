#[doc = "Register `DAT6` writer"]
pub type W = crate::W<Dat6Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat6Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT6\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat6Spec;
impl crate::RegisterSpec for Dat6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat6::W`](W) writer structure"]
impl crate::Writable for Dat6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT6 to value 0"]
impl crate::Resettable for Dat6Spec {}
