#[doc = "Register `DAT17` writer"]
pub type W = crate::W<Dat17Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat17Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT17\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat17::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat17Spec;
impl crate::RegisterSpec for Dat17Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat17::W`](W) writer structure"]
impl crate::Writable for Dat17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT17 to value 0"]
impl crate::Resettable for Dat17Spec {}
