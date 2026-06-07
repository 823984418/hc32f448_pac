#[doc = "Register `DAT12` writer"]
pub type W = crate::W<Dat12Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat12Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat12Spec;
impl crate::RegisterSpec for Dat12Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat12::W`](W) writer structure"]
impl crate::Writable for Dat12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT12 to value 0"]
impl crate::Resettable for Dat12Spec {}
