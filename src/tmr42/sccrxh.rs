#[doc = "Register `SCCRXH` reader"]
pub type R = crate::R<SccrxhSpec>;
#[doc = "Register `SCCRXH` writer"]
pub type W = crate::W<SccrxhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SCCRXH\n\nYou can [`read`](crate::Reg::read) this register and get [`sccrxh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccrxh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrxhSpec;
impl crate::RegisterSpec for SccrxhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sccrxh::R`](R) reader structure"]
impl crate::Readable for SccrxhSpec {}
#[doc = "`write(|w| ..)` method takes [`sccrxh::W`](W) writer structure"]
impl crate::Writable for SccrxhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCRXH to value 0"]
impl crate::Resettable for SccrxhSpec {}
