#[doc = "Register `DAT28` writer"]
pub type W = crate::W<Dat28Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat28Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT28\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat28::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat28Spec;
impl crate::RegisterSpec for Dat28Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat28::W`](W) writer structure"]
impl crate::Writable for Dat28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT28 to value 0"]
impl crate::Resettable for Dat28Spec {}
