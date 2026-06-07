#[doc = "Register `DAT20` writer"]
pub type W = crate::W<Dat20Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat20Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT20\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat20::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat20Spec;
impl crate::RegisterSpec for Dat20Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat20::W`](W) writer structure"]
impl crate::Writable for Dat20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT20 to value 0"]
impl crate::Resettable for Dat20Spec {}
