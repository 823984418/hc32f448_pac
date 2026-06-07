#[doc = "Register `OCCRXH` reader"]
pub type R = crate::R<OccrxhSpec>;
#[doc = "Register `OCCRXH` writer"]
pub type W = crate::W<OccrxhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc OCCRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`occrxh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occrxh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OccrxhSpec;
impl crate::RegisterSpec for OccrxhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`occrxh::R`](R) reader structure"]
impl crate::Readable for OccrxhSpec {}
#[doc = "`write(|w| ..)` method takes [`occrxh::W`](W) writer structure"]
impl crate::Writable for OccrxhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCRXH to value 0"]
impl crate::Resettable for OccrxhSpec {}
