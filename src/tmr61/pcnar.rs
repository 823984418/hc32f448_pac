#[doc = "Register `PCNAR` reader"]
pub type R = crate::R<PcnarSpec>;
#[doc = "Register `PCNAR` writer"]
pub type W = crate::W<PcnarSpec>;
#[doc = "Field `STACA` reader - desc STACA"]
pub type StacaR = crate::FieldReader;
#[doc = "Field `STACA` writer - desc STACA"]
pub type StacaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STPCA` reader - desc STPCA"]
pub type StpcaR = crate::FieldReader;
#[doc = "Field `STPCA` writer - desc STPCA"]
pub type StpcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVFCA` reader - desc OVFCA"]
pub type OvfcaR = crate::FieldReader;
#[doc = "Field `OVFCA` writer - desc OVFCA"]
pub type OvfcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UDFCA` reader - desc UDFCA"]
pub type UdfcaR = crate::FieldReader;
#[doc = "Field `UDFCA` writer - desc UDFCA"]
pub type UdfcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMAUCA` reader - desc CMAUCA"]
pub type CmaucaR = crate::FieldReader;
#[doc = "Field `CMAUCA` writer - desc CMAUCA"]
pub type CmaucaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMADCA` reader - desc CMADCA"]
pub type CmadcaR = crate::FieldReader;
#[doc = "Field `CMADCA` writer - desc CMADCA"]
pub type CmadcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMBUCA` reader - desc CMBUCA"]
pub type CmbucaR = crate::FieldReader;
#[doc = "Field `CMBUCA` writer - desc CMBUCA"]
pub type CmbucaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMBDCA` reader - desc CMBDCA"]
pub type CmbdcaR = crate::FieldReader;
#[doc = "Field `CMBDCA` writer - desc CMBDCA"]
pub type CmbdcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCA` reader - desc FORCA"]
pub type ForcaR = crate::FieldReader;
#[doc = "Field `FORCA` writer - desc FORCA"]
pub type ForcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBCA` reader - desc EMBCA"]
pub type EmbcaR = crate::FieldReader;
#[doc = "Field `EMBCA` writer - desc EMBCA"]
pub type EmbcaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBRA` reader - desc EMBRA"]
pub type EmbraR = crate::FieldReader;
#[doc = "Field `EMBRA` writer - desc EMBRA"]
pub type EmbraW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMBSA` reader - desc EMBSA"]
pub type EmbsaR = crate::FieldReader;
#[doc = "Field `EMBSA` writer - desc EMBSA"]
pub type EmbsaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUTENA` reader - desc OUTENA"]
pub type OutenaR = crate::BitReader;
#[doc = "Field `OUTENA` writer - desc OUTENA"]
pub type OutenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPMDA` reader - desc CAPMDA"]
pub type CapmdaR = crate::BitReader;
#[doc = "Field `CAPMDA` writer - desc CAPMDA"]
pub type CapmdaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&self) -> StacaR {
        StacaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&self) -> StpcaR {
        StpcaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc OVFCA"]
    #[inline(always)]
    pub fn ovfca(&self) -> OvfcaR {
        OvfcaR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc UDFCA"]
    #[inline(always)]
    pub fn udfca(&self) -> UdfcaR {
        UdfcaR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - desc CMAUCA"]
    #[inline(always)]
    pub fn cmauca(&self) -> CmaucaR {
        CmaucaR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc CMADCA"]
    #[inline(always)]
    pub fn cmadca(&self) -> CmadcaR {
        CmadcaR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc CMBUCA"]
    #[inline(always)]
    pub fn cmbuca(&self) -> CmbucaR {
        CmbucaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc CMBDCA"]
    #[inline(always)]
    pub fn cmbdca(&self) -> CmbdcaR {
        CmbdcaR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc FORCA"]
    #[inline(always)]
    pub fn forca(&self) -> ForcaR {
        ForcaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - desc EMBCA"]
    #[inline(always)]
    pub fn embca(&self) -> EmbcaR {
        EmbcaR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc EMBRA"]
    #[inline(always)]
    pub fn embra(&self) -> EmbraR {
        EmbraR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - desc EMBSA"]
    #[inline(always)]
    pub fn embsa(&self) -> EmbsaR {
        EmbsaR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&self) -> OutenaR {
        OutenaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&self) -> CapmdaR {
        CapmdaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNAR")
            .field("staca", &self.staca())
            .field("stpca", &self.stpca())
            .field("ovfca", &self.ovfca())
            .field("udfca", &self.udfca())
            .field("cmauca", &self.cmauca())
            .field("cmadca", &self.cmadca())
            .field("cmbuca", &self.cmbuca())
            .field("cmbdca", &self.cmbdca())
            .field("forca", &self.forca())
            .field("embca", &self.embca())
            .field("embra", &self.embra())
            .field("embsa", &self.embsa())
            .field("outena", &self.outena())
            .field("capmda", &self.capmda())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&mut self) -> StacaW<'_, PcnarSpec> {
        StacaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&mut self) -> StpcaW<'_, PcnarSpec> {
        StpcaW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc OVFCA"]
    #[inline(always)]
    pub fn ovfca(&mut self) -> OvfcaW<'_, PcnarSpec> {
        OvfcaW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc UDFCA"]
    #[inline(always)]
    pub fn udfca(&mut self) -> UdfcaW<'_, PcnarSpec> {
        UdfcaW::new(self, 6)
    }
    #[doc = "Bits 8:9 - desc CMAUCA"]
    #[inline(always)]
    pub fn cmauca(&mut self) -> CmaucaW<'_, PcnarSpec> {
        CmaucaW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc CMADCA"]
    #[inline(always)]
    pub fn cmadca(&mut self) -> CmadcaW<'_, PcnarSpec> {
        CmadcaW::new(self, 10)
    }
    #[doc = "Bits 12:13 - desc CMBUCA"]
    #[inline(always)]
    pub fn cmbuca(&mut self) -> CmbucaW<'_, PcnarSpec> {
        CmbucaW::new(self, 12)
    }
    #[doc = "Bits 14:15 - desc CMBDCA"]
    #[inline(always)]
    pub fn cmbdca(&mut self) -> CmbdcaW<'_, PcnarSpec> {
        CmbdcaW::new(self, 14)
    }
    #[doc = "Bits 16:17 - desc FORCA"]
    #[inline(always)]
    pub fn forca(&mut self) -> ForcaW<'_, PcnarSpec> {
        ForcaW::new(self, 16)
    }
    #[doc = "Bits 20:21 - desc EMBCA"]
    #[inline(always)]
    pub fn embca(&mut self) -> EmbcaW<'_, PcnarSpec> {
        EmbcaW::new(self, 20)
    }
    #[doc = "Bits 22:23 - desc EMBRA"]
    #[inline(always)]
    pub fn embra(&mut self) -> EmbraW<'_, PcnarSpec> {
        EmbraW::new(self, 22)
    }
    #[doc = "Bits 24:25 - desc EMBSA"]
    #[inline(always)]
    pub fn embsa(&mut self) -> EmbsaW<'_, PcnarSpec> {
        EmbsaW::new(self, 24)
    }
    #[doc = "Bit 28 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&mut self) -> OutenaW<'_, PcnarSpec> {
        OutenaW::new(self, 28)
    }
    #[doc = "Bit 31 - desc CAPMDA"]
    #[inline(always)]
    pub fn capmda(&mut self) -> CapmdaW<'_, PcnarSpec> {
        CapmdaW::new(self, 31)
    }
}
#[doc = "desc PCNAR\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcnarSpec;
impl crate::RegisterSpec for PcnarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnar::R`](R) reader structure"]
impl crate::Readable for PcnarSpec {}
#[doc = "`write(|w| ..)` method takes [`pcnar::W`](W) writer structure"]
impl crate::Writable for PcnarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNAR to value 0"]
impl crate::Resettable for PcnarSpec {}
