#[doc = "Register `DAT11` writer"]
pub type W = crate::W<Dat11Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat11Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT11\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat11::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat11Spec;
impl crate::RegisterSpec for Dat11Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat11::W`](W) writer structure"]
impl crate::Writable for Dat11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT11 to value 0"]
impl crate::Resettable for Dat11Spec {}
