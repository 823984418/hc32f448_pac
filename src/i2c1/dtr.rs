#[doc = "Register `DTR` reader"]
pub type R = crate::R<DtrSpec>;
#[doc = "Register `DTR` writer"]
pub type W = crate::W<DtrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc DTR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtrSpec;
impl crate::RegisterSpec for DtrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtr::R`](R) reader structure"]
impl crate::Readable for DtrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtr::W`](W) writer structure"]
impl crate::Writable for DtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTR to value 0xff"]
impl crate::Resettable for DtrSpec {
    const RESET_VALUE: u8 = 0xff;
}
