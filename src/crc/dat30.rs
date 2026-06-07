#[doc = "Register `DAT30` writer"]
pub type W = crate::W<Dat30Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat30Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT30\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat30::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat30Spec;
impl crate::RegisterSpec for Dat30Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat30::W`](W) writer structure"]
impl crate::Writable for Dat30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT30 to value 0"]
impl crate::Resettable for Dat30Spec {}
