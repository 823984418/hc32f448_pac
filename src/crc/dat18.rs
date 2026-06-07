#[doc = "Register `DAT18` writer"]
pub type W = crate::W<Dat18Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat18Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT18\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat18::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat18Spec;
impl crate::RegisterSpec for Dat18Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat18::W`](W) writer structure"]
impl crate::Writable for Dat18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT18 to value 0"]
impl crate::Resettable for Dat18Spec {}
