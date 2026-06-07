#[doc = "Register `DAT16` writer"]
pub type W = crate::W<Dat16Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat16Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT16\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat16::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat16Spec;
impl crate::RegisterSpec for Dat16Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat16::W`](W) writer structure"]
impl crate::Writable for Dat16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT16 to value 0"]
impl crate::Resettable for Dat16Spec {}
