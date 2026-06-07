#[doc = "Register `DAT14` writer"]
pub type W = crate::W<Dat14Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat14Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT14\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat14::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat14Spec;
impl crate::RegisterSpec for Dat14Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat14::W`](W) writer structure"]
impl crate::Writable for Dat14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT14 to value 0"]
impl crate::Resettable for Dat14Spec {}
