#[doc = "Register `DAT8` writer"]
pub type W = crate::W<Dat8Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat8Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat8Spec;
impl crate::RegisterSpec for Dat8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat8::W`](W) writer structure"]
impl crate::Writable for Dat8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT8 to value 0"]
impl crate::Resettable for Dat8Spec {}
