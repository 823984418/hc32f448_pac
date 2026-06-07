#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CccrSpec>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CccrSpec>;
#[doc = "Field `INIT` reader - desc INIT"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - desc INIT"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - desc CCE"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - desc CCE"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - desc ASM"]
pub type AsmR = crate::BitReader;
#[doc = "Field `ASM` writer - desc ASM"]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - desc CSA"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSR` reader - desc CSR"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - desc CSR"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - desc MON"]
pub type MonR = crate::BitReader;
#[doc = "Field `MON` writer - desc MON"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - desc DAR"]
pub type DarR = crate::BitReader;
#[doc = "Field `DAR` writer - desc DAR"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - desc TEST"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - desc TEST"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - desc FDOE"]
pub type FdoeR = crate::BitReader;
#[doc = "Field `FDOE` writer - desc FDOE"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSE` reader - desc BRSE"]
pub type BrseR = crate::BitReader;
#[doc = "Field `BRSE` writer - desc BRSE"]
pub type BrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTSU` reader - desc UTSU"]
pub type UtsuR = crate::BitReader;
#[doc = "Field `UTSU` writer - desc UTSU"]
pub type UtsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMM` reader - desc WMM"]
pub type WmmR = crate::BitReader;
#[doc = "Field `WMM` writer - desc WMM"]
pub type WmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - desc PXHD"]
pub type PxhdR = crate::BitReader;
#[doc = "Field `PXHD` writer - desc PXHD"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - desc EFBI"]
pub type EfbiR = crate::BitReader;
#[doc = "Field `EFBI` writer - desc EFBI"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - desc TXP"]
pub type TxpR = crate::BitReader;
#[doc = "Field `TXP` writer - desc TXP"]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - desc NISO"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - desc NISO"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc INIT"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCE"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ASM"]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc MON"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TEST"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc FDOE"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BRSE"]
    #[inline(always)]
    pub fn brse(&self) -> BrseR {
        BrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UTSU"]
    #[inline(always)]
    pub fn utsu(&self) -> UtsuR {
        UtsuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc WMM"]
    #[inline(always)]
    pub fn wmm(&self) -> WmmR {
        WmmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PXHD"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EFBI"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc NISO"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("utsu", &self.utsu())
            .field("wmm", &self.wmm())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, CccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CCE"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, CccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ASM"]
    #[inline(always)]
    pub fn asm(&mut self) -> AsmW<'_, CccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 4 - desc CSR"]
    #[inline(always)]
    pub fn csr(&mut self) -> CsrW<'_, CccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - desc MON"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<'_, CccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DAR"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<'_, CccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TEST"]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<'_, CccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - desc FDOE"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FdoeW<'_, CccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - desc BRSE"]
    #[inline(always)]
    pub fn brse(&mut self) -> BrseW<'_, CccrSpec> {
        BrseW::new(self, 9)
    }
    #[doc = "Bit 10 - desc UTSU"]
    #[inline(always)]
    pub fn utsu(&mut self) -> UtsuW<'_, CccrSpec> {
        UtsuW::new(self, 10)
    }
    #[doc = "Bit 11 - desc WMM"]
    #[inline(always)]
    pub fn wmm(&mut self) -> WmmW<'_, CccrSpec> {
        WmmW::new(self, 11)
    }
    #[doc = "Bit 12 - desc PXHD"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PxhdW<'_, CccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - desc EFBI"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EfbiW<'_, CccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - desc TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TxpW<'_, CccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - desc NISO"]
    #[inline(always)]
    pub fn niso(&mut self) -> NisoW<'_, CccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "desc CCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccrSpec;
impl crate::RegisterSpec for CccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CccrSpec {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CccrSpec {
    const RESET_VALUE: u32 = 0x01;
}
