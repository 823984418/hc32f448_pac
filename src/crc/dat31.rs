#[doc = "Register `DAT31` writer"]
pub type W = crate::W<Dat31Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat31Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat31::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat31Spec;
impl crate::RegisterSpec for Dat31Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat31::W`](W) writer structure"]
impl crate::Writable for Dat31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT31 to value 0"]
impl crate::Resettable for Dat31Spec {}
