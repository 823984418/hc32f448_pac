#[doc = "Register `PCNBR` reader"]
pub type R = crate::R<PcnbrSpec>;
#[doc = "Register `PCNBR` writer"]
pub type W = crate::W<PcnbrSpec>;
#[doc = "Field `STACB` reader - desc STACB"]
pub type StacbR = crate::FieldReader;
#[doc = "Field `STACB` writer - desc STACB"]
pub type StacbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STPCB` reader - desc STPCB"]
pub type StpcbR = crate::FieldReader;
#[doc = "Field `STPCB` writer - desc STPCB"]
pub type StpcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVFCB` reader - desc OVFCB"]
pub type OvfcbR = crate::FieldReader;
#[doc = "Field `OVFCB` writer - desc OVFCB"]
pub type OvfcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UDFCB` reader - desc UDFCB"]
pub type UdfcbR = crate::FieldReader;
#[doc = "Field `UDFCB` writer - desc UDFCB"]
pub type UdfcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMAUCB` reader - desc CMAUCB"]
pub type CmaucbR = crate::FieldReader;
#[doc = "Field `CMAUCB` writer - desc CMAUCB"]
pub type CmaucbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMADCB` reader - desc CMADCB"]
pub type CmadcbR = crate::FieldReader;
#[doc = "Field `CMADCB` writer - desc CMADCB"]
pub type CmadcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMBUCB` reader - desc CMBUCB"]
pub type CmbucbR = crate::FieldReader;
#[doc = "Field `CMBUCB` writer - desc CMBUCB"]
pub type CmbucbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMBDCB` reader - desc CMBDCB"]
pub type CmbdcbR = crate::FieldReader;
#[doc = "Field `CMBDCB` writer - desc CMBDCB"]
pub type CmbdcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCB` reader - desc FORCB"]
pub type ForcbR = crate::FieldReader;
#[doc = "Field `FORCB` writer - desc FORCB"]
pub type ForcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBCB` reader - desc EMBCB"]
pub type EmbcbR = crate::FieldReader;
#[doc = "Field `EMBCB` writer - desc EMBCB"]
pub type EmbcbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBRB` reader - desc EMBRB"]
pub type EmbrbR = crate::FieldReader;
#[doc = "Field `EMBRB` writer - desc EMBRB"]
pub type EmbrbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBSB` reader - desc EMBSB"]
pub type EmbsbR = crate::FieldReader;
#[doc = "Field `EMBSB` writer - desc EMBSB"]
pub type EmbsbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTENB` reader - desc OUTENB"]
pub type OutenbR = crate::BitReader;
#[doc = "Field `OUTENB` writer - desc OUTENB"]
pub type OutenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPMDB` reader - desc CAPMDB"]
pub type CapmdbR = crate::BitReader;
#[doc = "Field `CAPMDB` writer - desc CAPMDB"]
pub type CapmdbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&self) -> StacbR {
        StacbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&self) -> StpcbR {
        StpcbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc OVFCB"]
    #[inline(always)]
    pub fn ovfcb(&self) -> OvfcbR {
        OvfcbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc UDFCB"]
    #[inline(always)]
    pub fn udfcb(&self) -> UdfcbR {
        UdfcbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc CMAUCB"]
    #[inline(always)]
    pub fn cmaucb(&self) -> CmaucbR {
        CmaucbR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc CMADCB"]
    #[inline(always)]
    pub fn cmadcb(&self) -> CmadcbR {
        CmadcbR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc CMBUCB"]
    #[inline(always)]
    pub fn cmbucb(&self) -> CmbucbR {
        CmbucbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc CMBDCB"]
    #[inline(always)]
    pub fn cmbdcb(&self) -> CmbdcbR {
        CmbdcbR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc FORCB"]
    #[inline(always)]
    pub fn forcb(&self) -> ForcbR {
        ForcbR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc EMBCB"]
    #[inline(always)]
    pub fn embcb(&self) -> EmbcbR {
        EmbcbR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc EMBRB"]
    #[inline(always)]
    pub fn embrb(&self) -> EmbrbR {
        EmbrbR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - desc EMBSB"]
    #[inline(always)]
    pub fn embsb(&self) -> EmbsbR {
        EmbsbR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&self) -> OutenbR {
        OutenbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&self) -> CapmdbR {
        CapmdbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNBR")
            .field("stacb", &self.stacb())
            .field("stpcb", &self.stpcb())
            .field("ovfcb", &self.ovfcb())
            .field("udfcb", &self.udfcb())
            .field("cmaucb", &self.cmaucb())
            .field("cmadcb", &self.cmadcb())
            .field("cmbucb", &self.cmbucb())
            .field("cmbdcb", &self.cmbdcb())
            .field("forcb", &self.forcb())
            .field("embcb", &self.embcb())
            .field("embrb", &self.embrb())
            .field("embsb", &self.embsb())
            .field("outenb", &self.outenb())
            .field("capmdb", &self.capmdb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&mut self) -> StacbW<'_, PcnbrSpec> {
        StacbW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&mut self) -> StpcbW<'_, PcnbrSpec> {
        StpcbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc OVFCB"]
    #[inline(always)]
    pub fn ovfcb(&mut self) -> OvfcbW<'_, PcnbrSpec> {
        OvfcbW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc UDFCB"]
    #[inline(always)]
    pub fn udfcb(&mut self) -> UdfcbW<'_, PcnbrSpec> {
        UdfcbW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc CMAUCB"]
    #[inline(always)]
    pub fn cmaucb(&mut self) -> CmaucbW<'_, PcnbrSpec> {
        CmaucbW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc CMADCB"]
    #[inline(always)]
    pub fn cmadcb(&mut self) -> CmadcbW<'_, PcnbrSpec> {
        CmadcbW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc CMBUCB"]
    #[inline(always)]
    pub fn cmbucb(&mut self) -> CmbucbW<'_, PcnbrSpec> {
        CmbucbW::new(self, 12)
    }
    #[doc = "Bits 14:15 - desc CMBDCB"]
    #[inline(always)]
    pub fn cmbdcb(&mut self) -> CmbdcbW<'_, PcnbrSpec> {
        CmbdcbW::new(self, 14)
    }
    #[doc = "Bits 16:17 - desc FORCB"]
    #[inline(always)]
    pub fn forcb(&mut self) -> ForcbW<'_, PcnbrSpec> {
        ForcbW::new(self, 16)
    }
    #[doc = "Bits 20:21 - desc EMBCB"]
    #[inline(always)]
    pub fn embcb(&mut self) -> EmbcbW<'_, PcnbrSpec> {
        EmbcbW::new(self, 20)
    }
    #[doc = "Bits 22:23 - desc EMBRB"]
    #[inline(always)]
    pub fn embrb(&mut self) -> EmbrbW<'_, PcnbrSpec> {
        EmbrbW::new(self, 22)
    }
    #[doc = "Bits 24:25 - desc EMBSB"]
    #[inline(always)]
    pub fn embsb(&mut self) -> EmbsbW<'_, PcnbrSpec> {
        EmbsbW::new(self, 24)
    }
    #[doc = "Bit 28 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&mut self) -> OutenbW<'_, PcnbrSpec> {
        OutenbW::new(self, 28)
    }
    #[doc = "Bit 31 - desc CAPMDB"]
    #[inline(always)]
    pub fn capmdb(&mut self) -> CapmdbW<'_, PcnbrSpec> {
        CapmdbW::new(self, 31)
    }
}
#[doc = "desc PCNBR\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcnbrSpec;
impl crate::RegisterSpec for PcnbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnbr::R`](R) reader structure"]
impl crate::Readable for PcnbrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcnbr::W`](W) writer structure"]
impl crate::Writable for PcnbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNBR to value 0"]
impl crate::Resettable for PcnbrSpec {}
