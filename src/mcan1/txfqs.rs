#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TxfqsSpec>;
#[doc = "Field `TFFL` reader - desc TFFL"]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFGI` reader - desc TFGI"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFQPI` reader - desc TFQPI"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQF` reader - desc TFQF"]
pub type TfqfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - desc TFFL"]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - desc TFGI"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - desc TFQPI"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - desc TFQF"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
#[doc = "desc TXFQS\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfqsSpec;
impl crate::RegisterSpec for TxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TxfqsSpec {}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TxfqsSpec {}
