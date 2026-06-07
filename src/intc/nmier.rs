#[doc = "Register `NMIER` reader"]
pub type R = crate::R<NmierSpec>;
#[doc = "Register `NMIER` writer"]
pub type W = crate::W<NmierSpec>;
#[doc = "Field `SWDTEN` reader - desc SWDTEN"]
pub type SwdtenR = crate::BitReader;
#[doc = "Field `SWDTEN` writer - desc SWDTEN"]
pub type SwdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1EN` reader - desc PVD1EN"]
pub type Pvd1enR = crate::BitReader;
#[doc = "Field `PVD1EN` writer - desc PVD1EN"]
pub type Pvd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2EN` reader - desc PVD2EN"]
pub type Pvd2enR = crate::BitReader;
#[doc = "Field `PVD2EN` writer - desc PVD2EN"]
pub type Pvd2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPEN` reader - desc XTALSTPEN"]
pub type XtalstpenR = crate::BitReader;
#[doc = "Field `XTALSTPEN` writer - desc XTALSTPEN"]
pub type XtalstpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPARERREN` reader - desc RPARERREN"]
pub type RparerrenR = crate::BitReader;
#[doc = "Field `RPARERREN` writer - desc RPARERREN"]
pub type RparerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCERREN` reader - desc RECCERREN"]
pub type ReccerrenR = crate::BitReader;
#[doc = "Field `RECCERREN` writer - desc RECCERREN"]
pub type ReccerrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERREN` reader - desc BUSERREN"]
pub type BuserrenR = crate::BitReader;
#[doc = "Field `BUSERREN` writer - desc BUSERREN"]
pub type BuserrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTEN` reader - desc WDTEN"]
pub type WdtenR = crate::BitReader;
#[doc = "Field `WDTEN` writer - desc WDTEN"]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc SWDTEN"]
    #[inline(always)]
    pub fn swdten(&self) -> SwdtenR {
        SwdtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1EN"]
    #[inline(always)]
    pub fn pvd1en(&self) -> Pvd1enR {
        Pvd1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2EN"]
    #[inline(always)]
    pub fn pvd2en(&self) -> Pvd2enR {
        Pvd2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPEN"]
    #[inline(always)]
    pub fn xtalstpen(&self) -> XtalstpenR {
        XtalstpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RPARERREN"]
    #[inline(always)]
    pub fn rparerren(&self) -> RparerrenR {
        RparerrenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCERREN"]
    #[inline(always)]
    pub fn reccerren(&self) -> ReccerrenR {
        ReccerrenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSERREN"]
    #[inline(always)]
    pub fn buserren(&self) -> BuserrenR {
        BuserrenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIER")
            .field("swdten", &self.swdten())
            .field("pvd1en", &self.pvd1en())
            .field("pvd2en", &self.pvd2en())
            .field("xtalstpen", &self.xtalstpen())
            .field("rparerren", &self.rparerren())
            .field("reccerren", &self.reccerren())
            .field("buserren", &self.buserren())
            .field("wdten", &self.wdten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDTEN"]
    #[inline(always)]
    pub fn swdten(&mut self) -> SwdtenW<'_, NmierSpec> {
        SwdtenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1EN"]
    #[inline(always)]
    pub fn pvd1en(&mut self) -> Pvd1enW<'_, NmierSpec> {
        Pvd1enW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2EN"]
    #[inline(always)]
    pub fn pvd2en(&mut self) -> Pvd2enW<'_, NmierSpec> {
        Pvd2enW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPEN"]
    #[inline(always)]
    pub fn xtalstpen(&mut self) -> XtalstpenW<'_, NmierSpec> {
        XtalstpenW::new(self, 5)
    }
    #[doc = "Bit 8 - desc RPARERREN"]
    #[inline(always)]
    pub fn rparerren(&mut self) -> RparerrenW<'_, NmierSpec> {
        RparerrenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCERREN"]
    #[inline(always)]
    pub fn reccerren(&mut self) -> ReccerrenW<'_, NmierSpec> {
        ReccerrenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSERREN"]
    #[inline(always)]
    pub fn buserren(&mut self) -> BuserrenW<'_, NmierSpec> {
        BuserrenW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTEN"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WdtenW<'_, NmierSpec> {
        WdtenW::new(self, 11)
    }
}
#[doc = "desc NMIER\n\nYou can [`read`](crate::Reg::read) this register and get [`nmier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmierSpec;
impl crate::RegisterSpec for NmierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmier::R`](R) reader structure"]
impl crate::Readable for NmierSpec {}
#[doc = "`write(|w| ..)` method takes [`nmier::W`](W) writer structure"]
impl crate::Writable for NmierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIER to value 0"]
impl crate::Resettable for NmierSpec {}
