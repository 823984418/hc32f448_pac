#[doc = "Register `NMIFCR` reader"]
pub type R = crate::R<NmifcrSpec>;
#[doc = "Register `NMIFCR` writer"]
pub type W = crate::W<NmifcrSpec>;
#[doc = "Field `SWDTFCLR` reader - desc SWDTFCLR"]
pub type SwdtfclrR = crate::BitReader;
#[doc = "Field `SWDTFCLR` writer - desc SWDTFCLR"]
pub type SwdtfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1FCLR` reader - desc PVD1FCLR"]
pub type Pvd1fclrR = crate::BitReader;
#[doc = "Field `PVD1FCLR` writer - desc PVD1FCLR"]
pub type Pvd1fclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2FCLR` reader - desc PVD2FCLR"]
pub type Pvd2fclrR = crate::BitReader;
#[doc = "Field `PVD2FCLR` writer - desc PVD2FCLR"]
pub type Pvd2fclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPFCLR` reader - desc XTALSTPFCLR"]
pub type XtalstpfclrR = crate::BitReader;
#[doc = "Field `XTALSTPFCLR` writer - desc XTALSTPFCLR"]
pub type XtalstpfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPARERRFCLR` reader - desc RPARERRFCLR"]
pub type RparerrfclrR = crate::BitReader;
#[doc = "Field `RPARERRFCLR` writer - desc RPARERRFCLR"]
pub type RparerrfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCERRFCLR` reader - desc RECCERRFCLR"]
pub type ReccerrfclrR = crate::BitReader;
#[doc = "Field `RECCERRFCLR` writer - desc RECCERRFCLR"]
pub type ReccerrfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRFCLR` reader - desc BUSERRFCLR"]
pub type BuserrfclrR = crate::BitReader;
#[doc = "Field `BUSERRFCLR` writer - desc BUSERRFCLR"]
pub type BuserrfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTFCLR` reader - desc WDTFCLR"]
pub type WdtfclrR = crate::BitReader;
#[doc = "Field `WDTFCLR` writer - desc WDTFCLR"]
pub type WdtfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc SWDTFCLR"]
    #[inline(always)]
    pub fn swdtfclr(&self) -> SwdtfclrR {
        SwdtfclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1FCLR"]
    #[inline(always)]
    pub fn pvd1fclr(&self) -> Pvd1fclrR {
        Pvd1fclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2FCLR"]
    #[inline(always)]
    pub fn pvd2fclr(&self) -> Pvd2fclrR {
        Pvd2fclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPFCLR"]
    #[inline(always)]
    pub fn xtalstpfclr(&self) -> XtalstpfclrR {
        XtalstpfclrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RPARERRFCLR"]
    #[inline(always)]
    pub fn rparerrfclr(&self) -> RparerrfclrR {
        RparerrfclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCERRFCLR"]
    #[inline(always)]
    pub fn reccerrfclr(&self) -> ReccerrfclrR {
        ReccerrfclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSERRFCLR"]
    #[inline(always)]
    pub fn buserrfclr(&self) -> BuserrfclrR {
        BuserrfclrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTFCLR"]
    #[inline(always)]
    pub fn wdtfclr(&self) -> WdtfclrR {
        WdtfclrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIFCR")
            .field("swdtfclr", &self.swdtfclr())
            .field("pvd1fclr", &self.pvd1fclr())
            .field("pvd2fclr", &self.pvd2fclr())
            .field("xtalstpfclr", &self.xtalstpfclr())
            .field("rparerrfclr", &self.rparerrfclr())
            .field("reccerrfclr", &self.reccerrfclr())
            .field("buserrfclr", &self.buserrfclr())
            .field("wdtfclr", &self.wdtfclr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDTFCLR"]
    #[inline(always)]
    pub fn swdtfclr(&mut self) -> SwdtfclrW<'_, NmifcrSpec> {
        SwdtfclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1FCLR"]
    #[inline(always)]
    pub fn pvd1fclr(&mut self) -> Pvd1fclrW<'_, NmifcrSpec> {
        Pvd1fclrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2FCLR"]
    #[inline(always)]
    pub fn pvd2fclr(&mut self) -> Pvd2fclrW<'_, NmifcrSpec> {
        Pvd2fclrW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPFCLR"]
    #[inline(always)]
    pub fn xtalstpfclr(&mut self) -> XtalstpfclrW<'_, NmifcrSpec> {
        XtalstpfclrW::new(self, 5)
    }
    #[doc = "Bit 8 - desc RPARERRFCLR"]
    #[inline(always)]
    pub fn rparerrfclr(&mut self) -> RparerrfclrW<'_, NmifcrSpec> {
        RparerrfclrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCERRFCLR"]
    #[inline(always)]
    pub fn reccerrfclr(&mut self) -> ReccerrfclrW<'_, NmifcrSpec> {
        ReccerrfclrW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSERRFCLR"]
    #[inline(always)]
    pub fn buserrfclr(&mut self) -> BuserrfclrW<'_, NmifcrSpec> {
        BuserrfclrW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTFCLR"]
    #[inline(always)]
    pub fn wdtfclr(&mut self) -> WdtfclrW<'_, NmifcrSpec> {
        WdtfclrW::new(self, 11)
    }
}
#[doc = "desc NMIFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmifcrSpec;
impl crate::RegisterSpec for NmifcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmifcr::R`](R) reader structure"]
impl crate::Readable for NmifcrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmifcr::W`](W) writer structure"]
impl crate::Writable for NmifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIFCR to value 0"]
impl crate::Resettable for NmifcrSpec {}
