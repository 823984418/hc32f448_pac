#[doc = "Register `SMC_ENAR` reader"]
pub type R = crate::R<SmcEnarSpec>;
#[doc = "Register `SMC_ENAR` writer"]
pub type W = crate::W<SmcEnarSpec>;
#[doc = "Field `SMCEN` reader - desc SMCEN"]
pub type SmcenR = crate::BitReader;
#[doc = "Field `SMCEN` writer - desc SMCEN"]
pub type SmcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc SMCEN"]
    #[inline(always)]
    pub fn smcen(&self) -> SmcenR {
        SmcenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMC_ENAR")
            .field("smcen", &self.smcen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc SMCEN"]
    #[inline(always)]
    pub fn smcen(&mut self) -> SmcenW<'_, SmcEnarSpec> {
        SmcenW::new(self, 1)
    }
}
#[doc = "desc SMC_ENAR\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_enar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_enar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcEnarSpec;
impl crate::RegisterSpec for SmcEnarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smc_enar::R`](R) reader structure"]
impl crate::Readable for SmcEnarSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_enar::W`](W) writer structure"]
impl crate::Writable for SmcEnarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMC_ENAR to value 0"]
impl crate::Resettable for SmcEnarSpec {}
