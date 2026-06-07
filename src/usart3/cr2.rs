#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `MPE` reader - desc MPE"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - desc MPE"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPIE` reader - desc WKUPIE"]
pub type WkupieR = crate::BitReader;
#[doc = "Field `WKUPIE` writer - desc WKUPIE"]
pub type WkupieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - desc BEIE"]
pub type BeieR = crate::BitReader;
#[doc = "Field `BEIE` writer - desc BEIE"]
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEE` reader - desc BEE"]
pub type BeeR = crate::BitReader;
#[doc = "Field `BEE` writer - desc BEE"]
pub type BeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - desc LBDIE"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - desc LBDIE"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - desc LBDL"]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - desc LBDL"]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKL` reader - desc SBKL"]
pub type SbklR = crate::FieldReader;
#[doc = "Field `SBKL` writer - desc SBKL"]
pub type SbklW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WKUPE` reader - desc WKUPE"]
pub type WkupeR = crate::BitReader;
#[doc = "Field `WKUPE` writer - desc WKUPE"]
pub type WkupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKC` reader - desc CLKC"]
pub type ClkcR = crate::FieldReader;
#[doc = "Field `CLKC` writer - desc CLKC"]
pub type ClkcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINEN` reader - desc LINEN"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - desc LINEN"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBK` reader - desc SBK"]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - desc SBK"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKM` reader - desc SBKM"]
pub type SbkmR = crate::BitReader;
#[doc = "Field `SBKM` writer - desc SBKM"]
pub type SbkmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc MPE"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&self) -> WkupieR {
        WkupieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BEE"]
    #[inline(always)]
    pub fn bee(&self) -> BeeR {
        BeeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LBDIE"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LBDL"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SBKL"]
    #[inline(always)]
    pub fn sbkl(&self) -> SbklR {
        SbklR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc WKUPE"]
    #[inline(always)]
    pub fn wkupe(&self) -> WkupeR {
        WkupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:12 - desc CLKC"]
    #[inline(always)]
    pub fn clkc(&self) -> ClkcR {
        ClkcR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LINEN"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SBK"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc SBKM"]
    #[inline(always)]
    pub fn sbkm(&self) -> SbkmR {
        SbkmR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("mpe", &self.mpe())
            .field("wkupie", &self.wkupie())
            .field("beie", &self.beie())
            .field("bee", &self.bee())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("sbkl", &self.sbkl())
            .field("wkupe", &self.wkupe())
            .field("clkc", &self.clkc())
            .field("stop", &self.stop())
            .field("linen", &self.linen())
            .field("sbk", &self.sbk())
            .field("sbkm", &self.sbkm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc MPE"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MpeW<'_, Cr2Spec> {
        MpeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WkupieW<'_, Cr2Spec> {
        WkupieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<'_, Cr2Spec> {
        BeieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc BEE"]
    #[inline(always)]
    pub fn bee(&mut self) -> BeeW<'_, Cr2Spec> {
        BeeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc LBDIE"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<'_, Cr2Spec> {
        LbdieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc LBDL"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LbdlW<'_, Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc SBKL"]
    #[inline(always)]
    pub fn sbkl(&mut self) -> SbklW<'_, Cr2Spec> {
        SbklW::new(self, 6)
    }
    #[doc = "Bit 8 - desc WKUPE"]
    #[inline(always)]
    pub fn wkupe(&mut self) -> WkupeW<'_, Cr2Spec> {
        WkupeW::new(self, 8)
    }
    #[doc = "Bits 11:12 - desc CLKC"]
    #[inline(always)]
    pub fn clkc(&mut self) -> ClkcW<'_, Cr2Spec> {
        ClkcW::new(self, 11)
    }
    #[doc = "Bit 13 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 13)
    }
    #[doc = "Bit 14 - desc LINEN"]
    #[inline(always)]
    pub fn linen(&mut self) -> LinenW<'_, Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bit 16 - desc SBK"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<'_, Cr2Spec> {
        SbkW::new(self, 16)
    }
    #[doc = "Bit 17 - desc SBKM"]
    #[inline(always)]
    pub fn sbkm(&mut self) -> SbkmW<'_, Cr2Spec> {
        SbkmW::new(self, 17)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0x0600"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0x0600;
}
