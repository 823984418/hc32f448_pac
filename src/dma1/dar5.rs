#[doc = "Register `DAR5` reader"]
pub type R = crate::R<Dar5Spec>;
#[doc = "Register `DAR5` writer"]
pub type W = crate::W<Dar5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`dar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar5Spec;
impl crate::RegisterSpec for Dar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar5::R`](R) reader structure"]
impl crate::Readable for Dar5Spec {}
#[doc = "`write(|w| ..)` method takes [`dar5::W`](W) writer structure"]
impl crate::Writable for Dar5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR5 to value 0"]
impl crate::Resettable for Dar5Spec {}
