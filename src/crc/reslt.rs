#[doc = "Register `RESLT` reader"]
pub type R = crate::R<ResltSpec>;
#[doc = "Register `RESLT` writer"]
pub type W = crate::W<ResltSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc RESLT\n\nYou can [`read`](crate::Reg::read) this register and get [`reslt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reslt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResltSpec;
impl crate::RegisterSpec for ResltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reslt::R`](R) reader structure"]
impl crate::Readable for ResltSpec {}
#[doc = "`write(|w| ..)` method takes [`reslt::W`](W) writer structure"]
impl crate::Writable for ResltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESLT to value 0"]
impl crate::Resettable for ResltSpec {}
