#[doc = "Register `PFSRX` reader"]
pub type R = crate::R<PfsrxSpec>;
#[doc = "Register `PFSRX` writer"]
pub type W = crate::W<PfsrxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PFSRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfsrxSpec;
impl crate::RegisterSpec for PfsrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrx::R`](R) reader structure"]
impl crate::Readable for PfsrxSpec {}
#[doc = "`write(|w| ..)` method takes [`pfsrx::W`](W) writer structure"]
impl crate::Writable for PfsrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRX to value 0"]
impl crate::Resettable for PfsrxSpec {}
