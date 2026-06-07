#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TxbcSpec>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TxbcSpec>;
#[doc = "Field `TBSA` reader - desc TBSA"]
pub type TbsaR = crate::FieldReader<u16>;
#[doc = "Field `TBSA` writer - desc TBSA"]
pub type TbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NDTB` reader - desc NDTB"]
pub type NdtbR = crate::FieldReader;
#[doc = "Field `NDTB` writer - desc NDTB"]
pub type NdtbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQS` reader - desc TFQS"]
pub type TfqsR = crate::FieldReader;
#[doc = "Field `TFQS` writer - desc TFQS"]
pub type TfqsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQM` reader - desc TFQM"]
pub type TfqmR = crate::BitReader;
#[doc = "Field `TFQM` writer - desc TFQM"]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - desc TBSA"]
    #[inline(always)]
    pub fn tbsa(&self) -> TbsaR {
        TbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - desc NDTB"]
    #[inline(always)]
    pub fn ndtb(&self) -> NdtbR {
        NdtbR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - desc TFQS"]
    #[inline(always)]
    pub fn tfqs(&self) -> TfqsR {
        TfqsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - desc TFQM"]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBC")
            .field("tbsa", &self.tbsa())
            .field("ndtb", &self.ndtb())
            .field("tfqs", &self.tfqs())
            .field("tfqm", &self.tfqm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:15 - desc TBSA"]
    #[inline(always)]
    pub fn tbsa(&mut self) -> TbsaW<'_, TxbcSpec> {
        TbsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - desc NDTB"]
    #[inline(always)]
    pub fn ndtb(&mut self) -> NdtbW<'_, TxbcSpec> {
        NdtbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - desc TFQS"]
    #[inline(always)]
    pub fn tfqs(&mut self) -> TfqsW<'_, TxbcSpec> {
        TfqsW::new(self, 24)
    }
    #[doc = "Bit 30 - desc TFQM"]
    #[inline(always)]
    pub fn tfqm(&mut self) -> TfqmW<'_, TxbcSpec> {
        TfqmW::new(self, 30)
    }
}
#[doc = "desc TXBC\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcSpec;
impl crate::RegisterSpec for TxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TxbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TxbcSpec {}
