#[doc = "Register `XTALDIVCR` reader"]
pub type R = crate::R<XtaldivcrSpec>;
#[doc = "Register `XTALDIVCR` writer"]
pub type W = crate::W<XtaldivcrSpec>;
#[doc = "Field `FRADIVEN` reader - desc FRADIVEN"]
pub type FradivenR = crate::BitReader;
#[doc = "Field `FRADIVEN` writer - desc FRADIVEN"]
pub type FradivenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc FRADIVEN"]
    #[inline(always)]
    pub fn fradiven(&self) -> FradivenR {
        FradivenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTALDIVCR")
            .field("fradiven", &self.fradiven())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc FRADIVEN"]
    #[inline(always)]
    pub fn fradiven(&mut self) -> FradivenW<'_, XtaldivcrSpec> {
        FradivenW::new(self, 0)
    }
}
#[doc = "desc XTALDIVCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtaldivcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtaldivcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtaldivcrSpec;
impl crate::RegisterSpec for XtaldivcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtaldivcr::R`](R) reader structure"]
impl crate::Readable for XtaldivcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtaldivcr::W`](W) writer structure"]
impl crate::Writable for XtaldivcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALDIVCR to value 0"]
impl crate::Resettable for XtaldivcrSpec {}
