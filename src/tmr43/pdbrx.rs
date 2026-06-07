#[doc = "Register `PDBRX` reader"]
pub type R = crate::R<PdbrxSpec>;
#[doc = "Register `PDBRX` writer"]
pub type W = crate::W<PdbrxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc PDBRX\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbrxSpec;
impl crate::RegisterSpec for PdbrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdbrx::R`](R) reader structure"]
impl crate::Readable for PdbrxSpec {}
#[doc = "`write(|w| ..)` method takes [`pdbrx::W`](W) writer structure"]
impl crate::Writable for PdbrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDBRX to value 0"]
impl crate::Resettable for PdbrxSpec {}
