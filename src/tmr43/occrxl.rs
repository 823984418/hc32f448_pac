#[doc = "Register `OCCRXL` reader"]
pub type R = crate::R<OccrxlSpec>;
#[doc = "Register `OCCRXL` writer"]
pub type W = crate::W<OccrxlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRXL\n\nYou can [`read`](crate::Reg::read) this register and get [`occrxl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrxl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrxlSpec;
impl crate::RegisterSpec for OccrxlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrxl::R`](R) reader structure"]
impl crate::Readable for OccrxlSpec {}
#[doc = "`write(|w| ..)` method takes [`occrxl::W`](W) writer structure"]
impl crate::Writable for OccrxlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRXL to value 0"]
impl crate::Resettable for OccrxlSpec {}
