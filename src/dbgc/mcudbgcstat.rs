#[doc = "Register `MCUDBGCSTAT` reader"]
pub type R = crate::R<McudbgcstatSpec>;
#[doc = "Register `MCUDBGCSTAT` writer"]
pub type W = crate::W<McudbgcstatSpec>;
#[doc = "Field `CDBGPWRUPREQ` reader - desc CDBGPWRUPREQ"]
pub type CdbgpwrupreqR = crate::BitReader;
#[doc = "Field `CDBGPWRUPREQ` writer - desc CDBGPWRUPREQ"]
pub type CdbgpwrupreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDBGPWRUPACK` reader - desc CDBGPWRUPACK"]
pub type CdbgpwrupackR = crate::BitReader;
#[doc = "Field `CDBGPWRUPACK` writer - desc CDBGPWRUPACK"]
pub type CdbgpwrupackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CDBGPWRUPREQ"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&self) -> CdbgpwrupreqR {
        CdbgpwrupreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CDBGPWRUPACK"]
    #[inline(always)]
    pub fn cdbgpwrupack(&self) -> CdbgpwrupackR {
        CdbgpwrupackR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUDBGCSTAT")
            .field("cdbgpwrupreq", &self.cdbgpwrupreq())
            .field("cdbgpwrupack", &self.cdbgpwrupack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CDBGPWRUPREQ"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&mut self) -> CdbgpwrupreqW<'_, McudbgcstatSpec> {
        CdbgpwrupreqW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CDBGPWRUPACK"]
    #[inline(always)]
    pub fn cdbgpwrupack(&mut self) -> CdbgpwrupackW<'_, McudbgcstatSpec> {
        CdbgpwrupackW::new(self, 1)
    }
}
#[doc = "desc MCUDBGCSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`mcudbgcstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudbgcstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McudbgcstatSpec;
impl crate::RegisterSpec for McudbgcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcudbgcstat::R`](R) reader structure"]
impl crate::Readable for McudbgcstatSpec {}
#[doc = "`write(|w| ..)` method takes [`mcudbgcstat::W`](W) writer structure"]
impl crate::Writable for McudbgcstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUDBGCSTAT to value 0"]
impl crate::Resettable for McudbgcstatSpec {}
