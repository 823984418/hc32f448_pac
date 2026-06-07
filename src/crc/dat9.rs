#[doc = "Register `DAT9` writer"]
pub type W = crate::W<Dat9Spec>;
impl core::fmt::Debug for crate::generic::Reg<Dat9Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc DAT9\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dat9::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dat9Spec;
impl crate::RegisterSpec for Dat9Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dat9::W`](W) writer structure"]
impl crate::Writable for Dat9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAT9 to value 0"]
impl crate::Resettable for Dat9Spec {}
