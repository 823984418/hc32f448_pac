#[doc = "Register `DACR` reader"]
pub type R = crate::R<DacrSpec>;
#[doc = "Register `DACR` writer"]
pub type W = crate::W<DacrSpec>;
#[doc = "Field `DAE` reader - desc DAE"]
pub type DaeR = crate::BitReader;
#[doc = "Field `DAE` writer - desc DAE"]
pub type DaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA1E` reader - desc DA1E"]
pub type Da1eR = crate::BitReader;
#[doc = "Field `DA1E` writer - desc DA1E"]
pub type Da1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA2E` reader - desc DA2E"]
pub type Da2eR = crate::BitReader;
#[doc = "Field `DA2E` writer - desc DA2E"]
pub type Da2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPSEL` reader - desc DPSEL"]
pub type DpselR = crate::BitReader;
#[doc = "Field `DPSEL` writer - desc DPSEL"]
pub type DpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAAMP1` reader - desc DAAMP1"]
pub type Daamp1R = crate::BitReader;
#[doc = "Field `DAAMP1` writer - desc DAAMP1"]
pub type Daamp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAAMP2` reader - desc DAAMP2"]
pub type Daamp2R = crate::BitReader;
#[doc = "Field `DAAMP2` writer - desc DAAMP2"]
pub type Daamp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTDSL1` reader - desc EXTDSL1"]
pub type Extdsl1R = crate::BitReader;
#[doc = "Field `EXTDSL1` writer - desc EXTDSL1"]
pub type Extdsl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTDSL2` reader - desc EXTDSL2"]
pub type Extdsl2R = crate::BitReader;
#[doc = "Field `EXTDSL2` writer - desc EXTDSL2"]
pub type Extdsl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DAE"]
    #[inline(always)]
    pub fn dae(&self) -> DaeR {
        DaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DA1E"]
    #[inline(always)]
    pub fn da1e(&self) -> Da1eR {
        Da1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc DA2E"]
    #[inline(always)]
    pub fn da2e(&self) -> Da2eR {
        Da2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DPSEL"]
    #[inline(always)]
    pub fn dpsel(&self) -> DpselR {
        DpselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DAAMP1"]
    #[inline(always)]
    pub fn daamp1(&self) -> Daamp1R {
        Daamp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DAAMP2"]
    #[inline(always)]
    pub fn daamp2(&self) -> Daamp2R {
        Daamp2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EXTDSL1"]
    #[inline(always)]
    pub fn extdsl1(&self) -> Extdsl1R {
        Extdsl1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EXTDSL2"]
    #[inline(always)]
    pub fn extdsl2(&self) -> Extdsl2R {
        Extdsl2R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DACR")
            .field("dae", &self.dae())
            .field("da1e", &self.da1e())
            .field("da2e", &self.da2e())
            .field("dpsel", &self.dpsel())
            .field("daamp1", &self.daamp1())
            .field("daamp2", &self.daamp2())
            .field("extdsl1", &self.extdsl1())
            .field("extdsl2", &self.extdsl2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc DAE"]
    #[inline(always)]
    pub fn dae(&mut self) -> DaeW<'_, DacrSpec> {
        DaeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc DA1E"]
    #[inline(always)]
    pub fn da1e(&mut self) -> Da1eW<'_, DacrSpec> {
        Da1eW::new(self, 1)
    }
    #[doc = "Bit 2 - desc DA2E"]
    #[inline(always)]
    pub fn da2e(&mut self) -> Da2eW<'_, DacrSpec> {
        Da2eW::new(self, 2)
    }
    #[doc = "Bit 8 - desc DPSEL"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DpselW<'_, DacrSpec> {
        DpselW::new(self, 8)
    }
    #[doc = "Bit 9 - desc DAAMP1"]
    #[inline(always)]
    pub fn daamp1(&mut self) -> Daamp1W<'_, DacrSpec> {
        Daamp1W::new(self, 9)
    }
    #[doc = "Bit 10 - desc DAAMP2"]
    #[inline(always)]
    pub fn daamp2(&mut self) -> Daamp2W<'_, DacrSpec> {
        Daamp2W::new(self, 10)
    }
    #[doc = "Bit 11 - desc EXTDSL1"]
    #[inline(always)]
    pub fn extdsl1(&mut self) -> Extdsl1W<'_, DacrSpec> {
        Extdsl1W::new(self, 11)
    }
    #[doc = "Bit 12 - desc EXTDSL2"]
    #[inline(always)]
    pub fn extdsl2(&mut self) -> Extdsl2W<'_, DacrSpec> {
        Extdsl2W::new(self, 12)
    }
}
#[doc = "desc DACR\n\nYou can [`read`](crate::Reg::read) this register and get [`dacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacrSpec;
impl crate::RegisterSpec for DacrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dacr::R`](R) reader structure"]
impl crate::Readable for DacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dacr::W`](W) writer structure"]
impl crate::Writable for DacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACR to value 0"]
impl crate::Resettable for DacrSpec {}
