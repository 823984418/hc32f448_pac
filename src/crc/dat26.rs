#[doc = "Register `DAT26` writer"]
pub type W = crate::W<Dat26Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat26Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT26\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat26::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat26Spec;
impl crate::RegisterSpec for Dat26Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat26::W`](W) writer structure"]
impl crate::Writable for Dat26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT26 to value 0"]
impl crate::Resettable for Dat26Spec {}
