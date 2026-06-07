#[doc = "Register `EMB_STAT` reader"]
pub type R = crate::R<EmbStatSpec>;
#[doc = "Field `PWMSF` reader - desc PWMSF"]
pub type PwmsfR = crate::BitReader;
#[doc = "Field `CMPF` reader - desc CMPF"]
pub type CmpfR = crate::BitReader;
#[doc = "Field `SYSF` reader - desc SYSF"]
pub type SysfR = crate::BitReader;
#[doc = "Field `PWMST` reader - desc PWMST"]
pub type PwmstR = crate::BitReader;
#[doc = "Field `CMPST` reader - desc CMPST"]
pub type CmpstR = crate::BitReader;
#[doc = "Field `SYSST` reader - desc SYSST"]
pub type SysstR = crate::BitReader;
#[doc = "Field `PORTINF1` reader - desc PORTINF1"]
pub type Portinf1R = crate::BitReader;
#[doc = "Field `PORTINF2` reader - desc PORTINF2"]
pub type Portinf2R = crate::BitReader;
#[doc = "Field `PORTINF3` reader - desc PORTINF3"]
pub type Portinf3R = crate::BitReader;
#[doc = "Field `PORTINF4` reader - desc PORTINF4"]
pub type Portinf4R = crate::BitReader;
#[doc = "Field `PORTINST1` reader - desc PORTINST1"]
pub type Portinst1R = crate::BitReader;
#[doc = "Field `PORTINST2` reader - desc PORTINST2"]
pub type Portinst2R = crate::BitReader;
#[doc = "Field `PORTINST3` reader - desc PORTINST3"]
pub type Portinst3R = crate::BitReader;
#[doc = "Field `PORTINST4` reader - desc PORTINST4"]
pub type Portinst4R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - desc PWMSF"]
    #[inline(always)]
    pub fn pwmsf(&self) -> PwmsfR {
        PwmsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPF"]
    #[inline(always)]
    pub fn cmpf(&self) -> CmpfR {
        CmpfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SYSF"]
    #[inline(always)]
    pub fn sysf(&self) -> SysfR {
        SysfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PWMST"]
    #[inline(always)]
    pub fn pwmst(&self) -> PwmstR {
        PwmstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CMPST"]
    #[inline(always)]
    pub fn cmpst(&self) -> CmpstR {
        CmpstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SYSST"]
    #[inline(always)]
    pub fn sysst(&self) -> SysstR {
        SysstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PORTINF1"]
    #[inline(always)]
    pub fn portinf1(&self) -> Portinf1R {
        Portinf1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PORTINF2"]
    #[inline(always)]
    pub fn portinf2(&self) -> Portinf2R {
        Portinf2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PORTINF3"]
    #[inline(always)]
    pub fn portinf3(&self) -> Portinf3R {
        Portinf3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PORTINF4"]
    #[inline(always)]
    pub fn portinf4(&self) -> Portinf4R {
        Portinf4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PORTINST1"]
    #[inline(always)]
    pub fn portinst1(&self) -> Portinst1R {
        Portinst1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PORTINST2"]
    #[inline(always)]
    pub fn portinst2(&self) -> Portinst2R {
        Portinst2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc PORTINST3"]
    #[inline(always)]
    pub fn portinst3(&self) -> Portinst3R {
        Portinst3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc PORTINST4"]
    #[inline(always)]
    pub fn portinst4(&self) -> Portinst4R {
        Portinst4R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMB_STAT")
            .field("pwmsf", &self.pwmsf())
            .field("cmpf", &self.cmpf())
            .field("sysf", &self.sysf())
            .field("pwmst", &self.pwmst())
            .field("cmpst", &self.cmpst())
            .field("sysst", &self.sysst())
            .field("portinf1", &self.portinf1())
            .field("portinf2", &self.portinf2())
            .field("portinf3", &self.portinf3())
            .field("portinf4", &self.portinf4())
            .field("portinst1", &self.portinst1())
            .field("portinst2", &self.portinst2())
            .field("portinst3", &self.portinst3())
            .field("portinst4", &self.portinst4())
            .finish()
    }
}
#[doc = "desc EMB_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`emb_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmbStatSpec;
impl crate::RegisterSpec for EmbStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emb_stat::R`](R) reader structure"]
impl crate::Readable for EmbStatSpec {}
#[doc = "`reset()` method sets EMB_STAT to value 0"]
impl crate::Resettable for EmbStatSpec {}
