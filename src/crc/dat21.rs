#[doc = "Register `DAT21` writer"]
pub type W = crate::W<Dat21Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat21Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT21\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat21::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat21Spec;
impl crate::RegisterSpec for Dat21Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat21::W`](W) writer structure"]
impl crate::Writable for Dat21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT21 to value 0"]
impl crate::Resettable for Dat21Spec {}
