#[doc = "Register `NMIFR` reader"]
pub type R = crate::R<NmifrSpec>;
#[doc = "Register `NMIFR` writer"]
pub type W = crate::W<NmifrSpec>;
#[doc = "Field `SWDTF` reader - desc SWDTF"]
pub type SwdtfR = crate::BitReader;
#[doc = "Field `SWDTF` writer - desc SWDTF"]
pub type SwdtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1F` reader - desc PVD1F"]
pub type Pvd1fR = crate::BitReader;
#[doc = "Field `PVD1F` writer - desc PVD1F"]
pub type Pvd1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2F` reader - desc PVD2F"]
pub type Pvd2fR = crate::BitReader;
#[doc = "Field `PVD2F` writer - desc PVD2F"]
pub type Pvd2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALSTPF` reader - desc XTALSTPF"]
pub type XtalstpfR = crate::BitReader;
#[doc = "Field `XTALSTPF` writer - desc XTALSTPF"]
pub type XtalstpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPARERRF` reader - desc RPARERRF"]
pub type RparerrfR = crate::BitReader;
#[doc = "Field `RPARERRF` writer - desc RPARERRF"]
pub type RparerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECCERRF` reader - desc RECCERRF"]
pub type ReccerrfR = crate::BitReader;
#[doc = "Field `RECCERRF` writer - desc RECCERRF"]
pub type ReccerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRF` reader - desc BUSERRF"]
pub type BuserrfR = crate::BitReader;
#[doc = "Field `BUSERRF` writer - desc BUSERRF"]
pub type BuserrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTF` reader - desc WDTF"]
pub type WdtfR = crate::BitReader;
#[doc = "Field `WDTF` writer - desc WDTF"]
pub type WdtfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc SWDTF"]
    #[inline(always)]
    pub fn swdtf(&self) -> SwdtfR {
        SwdtfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PVD1F"]
    #[inline(always)]
    pub fn pvd1f(&self) -> Pvd1fR {
        Pvd1fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PVD2F"]
    #[inline(always)]
    pub fn pvd2f(&self) -> Pvd2fR {
        Pvd2fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc XTALSTPF"]
    #[inline(always)]
    pub fn xtalstpf(&self) -> XtalstpfR {
        XtalstpfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RPARERRF"]
    #[inline(always)]
    pub fn rparerrf(&self) -> RparerrfR {
        RparerrfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RECCERRF"]
    #[inline(always)]
    pub fn reccerrf(&self) -> ReccerrfR {
        ReccerrfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BUSERRF"]
    #[inline(always)]
    pub fn buserrf(&self) -> BuserrfR {
        BuserrfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WDTF"]
    #[inline(always)]
    pub fn wdtf(&self) -> WdtfR {
        WdtfR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIFR")
            .field("swdtf", &self.swdtf())
            .field("pvd1f", &self.pvd1f())
            .field("pvd2f", &self.pvd2f())
            .field("xtalstpf", &self.xtalstpf())
            .field("rparerrf", &self.rparerrf())
            .field("reccerrf", &self.reccerrf())
            .field("buserrf", &self.buserrf())
            .field("wdtf", &self.wdtf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDTF"]
    #[inline(always)]
    pub fn swdtf(&mut self) -> SwdtfW<'_, NmifrSpec> {
        SwdtfW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PVD1F"]
    #[inline(always)]
    pub fn pvd1f(&mut self) -> Pvd1fW<'_, NmifrSpec> {
        Pvd1fW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PVD2F"]
    #[inline(always)]
    pub fn pvd2f(&mut self) -> Pvd2fW<'_, NmifrSpec> {
        Pvd2fW::new(self, 3)
    }
    #[doc = "Bit 5 - desc XTALSTPF"]
    #[inline(always)]
    pub fn xtalstpf(&mut self) -> XtalstpfW<'_, NmifrSpec> {
        XtalstpfW::new(self, 5)
    }
    #[doc = "Bit 8 - desc RPARERRF"]
    #[inline(always)]
    pub fn rparerrf(&mut self) -> RparerrfW<'_, NmifrSpec> {
        RparerrfW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RECCERRF"]
    #[inline(always)]
    pub fn reccerrf(&mut self) -> ReccerrfW<'_, NmifrSpec> {
        ReccerrfW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BUSERRF"]
    #[inline(always)]
    pub fn buserrf(&mut self) -> BuserrfW<'_, NmifrSpec> {
        BuserrfW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WDTF"]
    #[inline(always)]
    pub fn wdtf(&mut self) -> WdtfW<'_, NmifrSpec> {
        WdtfW::new(self, 11)
    }
}
#[doc = "desc NMIFR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmifrSpec;
impl crate::RegisterSpec for NmifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmifr::R`](R) reader structure"]
impl crate::Readable for NmifrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmifr::W`](W) writer structure"]
impl crate::Writable for NmifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIFR to value 0"]
impl crate::Resettable for NmifrSpec {}
