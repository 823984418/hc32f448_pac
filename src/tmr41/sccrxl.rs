#[doc = "Register `SCCRXL` reader"]
pub type R = crate::R<SccrxlSpec>;
#[doc = "Register `SCCRXL` writer"]
pub type W = crate::W<SccrxlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrxl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrxl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrxlSpec;
impl crate::RegisterSpec for SccrxlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrxl::R`](R) reader structure"]
impl crate::Readable for SccrxlSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrxl::W`](W) writer structure"]
impl crate::Writable for SccrxlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRXL to value 0"]
impl crate::Resettable for SccrxlSpec {}
