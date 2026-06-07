#[doc = "Register `DAT0` writer"]
pub type W = crate::W<Dat0Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat0Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat0Spec;
impl crate::RegisterSpec for Dat0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat0::W`](W) writer structure"]
impl crate::Writable for Dat0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT0 to value 0"]
impl crate::Resettable for Dat0Spec {}
