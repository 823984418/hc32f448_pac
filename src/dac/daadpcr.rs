#[doc = "Register `DAADPCR` reader"]
pub type R = crate::R<DaadpcrSpec>;
#[doc = "Register `DAADPCR` writer"]
pub type W = crate::W<DaadpcrSpec>;
#[doc = "Field `ADCSL1` reader - desc ADCSL1"]
pub type Adcsl1R = crate::BitReader;
#[doc = "Field `ADCSL1` writer - desc ADCSL1"]
pub type Adcsl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSL2` reader - desc ADCSL2"]
pub type Adcsl2R = crate::BitReader;
#[doc = "Field `ADCSL2` writer - desc ADCSL2"]
pub type Adcsl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSL3` reader - desc ADCSL3"]
pub type Adcsl3R = crate::BitReader;
#[doc = "Field `ADCSL3` writer - desc ADCSL3"]
pub type Adcsl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA1SF` reader - desc DA1SF"]
pub type Da1sfR = crate::BitReader;
#[doc = "Field `DA2SF` reader - desc DA2SF"]
pub type Da2sfR = crate::BitReader;
#[doc = "Field `ADPEN` reader - desc ADPEN"]
pub type AdpenR = crate::BitReader;
#[doc = "Field `ADPEN` writer - desc ADPEN"]
pub type AdpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ADCSL1"]
    #[inline(always)]
    pub fn adcsl1(&self) -> Adcsl1R {
        Adcsl1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ADCSL2"]
    #[inline(always)]
    pub fn adcsl2(&self) -> Adcsl2R {
        Adcsl2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADCSL3"]
    #[inline(always)]
    pub fn adcsl3(&self) -> Adcsl3R {
        Adcsl3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DA1SF"]
    #[inline(always)]
    pub fn da1sf(&self) -> Da1sfR {
        Da1sfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DA2SF"]
    #[inline(always)]
    pub fn da2sf(&self) -> Da2sfR {
        Da2sfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADPEN"]
    #[inline(always)]
    pub fn adpen(&self) -> AdpenR {
        AdpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAADPCR")
            .field("adcsl1", &self.adcsl1())
            .field("adcsl2", &self.adcsl2())
            .field("adcsl3", &self.adcsl3())
            .field("da1sf", &self.da1sf())
            .field("da2sf", &self.da2sf())
            .field("adpen", &self.adpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc ADCSL1"]
    #[inline(always)]
    pub fn adcsl1(&mut self) -> Adcsl1W<'_, DaadpcrSpec> {
        Adcsl1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc ADCSL2"]
    #[inline(always)]
    pub fn adcsl2(&mut self) -> Adcsl2W<'_, DaadpcrSpec> {
        Adcsl2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc ADCSL3"]
    #[inline(always)]
    pub fn adcsl3(&mut self) -> Adcsl3W<'_, DaadpcrSpec> {
        Adcsl3W::new(self, 2)
    }
    #[doc = "Bit 15 - desc ADPEN"]
    #[inline(always)]
    pub fn adpen(&mut self) -> AdpenW<'_, DaadpcrSpec> {
        AdpenW::new(self, 15)
    }
}
#[doc = "desc DAADPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`daadpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaadpcrSpec;
impl crate::RegisterSpec for DaadpcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daadpcr::R`](R) reader structure"]
impl crate::Readable for DaadpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`daadpcr::W`](W) writer structure"]
impl crate::Writable for DaadpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAADPCR to value 0"]
impl crate::Resettable for DaadpcrSpec {}
