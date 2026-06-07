#[doc = "Register `DAT4` writer"]
pub type W = crate::W<Dat4Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat4Spec;
impl crate::RegisterSpec for Dat4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat4::W`](W) writer structure"]
impl crate::Writable for Dat4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT4 to value 0"]
impl crate::Resettable for Dat4Spec {}
