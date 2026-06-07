#[doc = "Register `DAT29` writer"]
pub type W = crate::W<Dat29Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat29Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat29::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat29Spec;
impl crate::RegisterSpec for Dat29Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat29::W`](W) writer structure"]
impl crate::Writable for Dat29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT29 to value 0"]
impl crate::Resettable for Dat29Spec {}
