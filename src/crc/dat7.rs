#[doc = "Register `DAT7` writer"]
pub type W = crate::W<Dat7Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat7Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT7\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat7Spec;
impl crate::RegisterSpec for Dat7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat7::W`](W) writer structure"]
impl crate::Writable for Dat7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT7 to value 0"]
impl crate::Resettable for Dat7Spec {}
