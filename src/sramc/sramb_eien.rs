#[doc = "Register `SRAMB_EIEN` reader"]
pub type R = crate::R<SrambEienSpec>;
#[doc = "Register `SRAMB_EIEN` writer"]
pub type W = crate::W<SrambEienSpec>;
#[doc = "Field `EIEN` reader - desc EIEN"]
pub type EienR = crate::BitReader;
#[doc = "Field `EIEN` writer - desc EIEN"]
pub type EienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EIEN"]
    #[inline(always)]
    pub fn eien(&self) -> EienR {
        EienR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMB_EIEN")
            .field("eien", &self.eien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc EIEN"]
    #[inline(always)]
    pub fn eien(&mut self) -> EienW<'_, SrambEienSpec> {
        EienW::new(self, 0)
    }
}
#[doc = "desc SRAMB_EIEN\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramb_eien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrambEienSpec;
impl crate::RegisterSpec for SrambEienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramb_eien::R`](R) reader structure"]
impl crate::Readable for SrambEienSpec {}
#[doc = "`write(|w| ..)` method takes [`sramb_eien::W`](W) writer structure"]
impl crate::Writable for SrambEienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAMB_EIEN to value 0"]
impl crate::Resettable for SrambEienSpec {}
