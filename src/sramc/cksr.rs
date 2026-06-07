#[doc = "Register `CKSR` reader"]
pub type R = crate::R<CksrSpec>;
#[doc = "Register `CKSR` writer"]
pub type W = crate::W<CksrSpec>;
#[doc = "Field `SRAMH_PYERR` reader - desc SRAMH_PYERR"]
pub type SramhPyerrR = crate::BitReader;
#[doc = "Field `SRAMH_PYERR` writer - desc SRAMH_PYERR"]
pub type SramhPyerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM0_1ERR` reader - desc SRAM0_1ERR"]
pub type Sram0_1errR = crate::BitReader;
#[doc = "Field `SRAM0_1ERR` writer - desc SRAM0_1ERR"]
pub type Sram0_1errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM0_2ERR` reader - desc SRAM0_2ERR"]
pub type Sram0_2errR = crate::BitReader;
#[doc = "Field `SRAM0_2ERR` writer - desc SRAM0_2ERR"]
pub type Sram0_2errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMB_1ERR` reader - desc SRAMB_1ERR"]
pub type Sramb1errR = crate::BitReader;
#[doc = "Field `SRAMB_1ERR` writer - desc SRAMB_1ERR"]
pub type Sramb1errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMB_2ERR` reader - desc SRAMB_2ERR"]
pub type Sramb2errR = crate::BitReader;
#[doc = "Field `SRAMB_2ERR` writer - desc SRAMB_2ERR"]
pub type Sramb2errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PYERR` reader - desc CACHE_PYERR"]
pub type CachePyerrR = crate::BitReader;
#[doc = "Field `CACHE_PYERR` writer - desc CACHE_PYERR"]
pub type CachePyerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - desc SRAMH_PYERR"]
    #[inline(always)]
    pub fn sramh_pyerr(&self) -> SramhPyerrR {
        SramhPyerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRAM0_1ERR"]
    #[inline(always)]
    pub fn sram0_1err(&self) -> Sram0_1errR {
        Sram0_1errR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SRAM0_2ERR"]
    #[inline(always)]
    pub fn sram0_2err(&self) -> Sram0_2errR {
        Sram0_2errR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SRAMB_1ERR"]
    #[inline(always)]
    pub fn sramb_1err(&self) -> Sramb1errR {
        Sramb1errR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SRAMB_2ERR"]
    #[inline(always)]
    pub fn sramb_2err(&self) -> Sramb2errR {
        Sramb2errR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CACHE_PYERR"]
    #[inline(always)]
    pub fn cache_pyerr(&self) -> CachePyerrR {
        CachePyerrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKSR")
            .field("sramh_pyerr", &self.sramh_pyerr())
            .field("sram0_1err", &self.sram0_1err())
            .field("sram0_2err", &self.sram0_2err())
            .field("sramb_1err", &self.sramb_1err())
            .field("sramb_2err", &self.sramb_2err())
            .field("cache_pyerr", &self.cache_pyerr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - desc SRAMH_PYERR"]
    #[inline(always)]
    pub fn sramh_pyerr(&mut self) -> SramhPyerrW<'_, CksrSpec> {
        SramhPyerrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SRAM0_1ERR"]
    #[inline(always)]
    pub fn sram0_1err(&mut self) -> Sram0_1errW<'_, CksrSpec> {
        Sram0_1errW::new(self, 4)
    }
    #[doc = "Bit 5 - desc SRAM0_2ERR"]
    #[inline(always)]
    pub fn sram0_2err(&mut self) -> Sram0_2errW<'_, CksrSpec> {
        Sram0_2errW::new(self, 5)
    }
    #[doc = "Bit 6 - desc SRAMB_1ERR"]
    #[inline(always)]
    pub fn sramb_1err(&mut self) -> Sramb1errW<'_, CksrSpec> {
        Sramb1errW::new(self, 6)
    }
    #[doc = "Bit 7 - desc SRAMB_2ERR"]
    #[inline(always)]
    pub fn sramb_2err(&mut self) -> Sramb2errW<'_, CksrSpec> {
        Sramb2errW::new(self, 7)
    }
    #[doc = "Bit 8 - desc CACHE_PYERR"]
    #[inline(always)]
    pub fn cache_pyerr(&mut self) -> CachePyerrW<'_, CksrSpec> {
        CachePyerrW::new(self, 8)
    }
}
#[doc = "desc CKSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cksr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cksr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CksrSpec;
impl crate::RegisterSpec for CksrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cksr::R`](R) reader structure"]
impl crate::Readable for CksrSpec {}
#[doc = "`write(|w| ..)` method takes [`cksr::W`](W) writer structure"]
impl crate::Writable for CksrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKSR to value 0"]
impl crate::Resettable for CksrSpec {}
