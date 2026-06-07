#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TxefsSpec>;
#[doc = "Field `EFFL` reader - desc EFFL"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFGI` reader - desc EFGI"]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFPI` reader - desc EFPI"]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFF` reader - desc EFF"]
pub type EffR = crate::BitReader;
#[doc = "Field `TEFL` reader - desc TEFL"]
pub type TeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - desc EFFL"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - desc EFGI"]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - desc EFPI"]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - desc EFF"]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
#[doc = "desc TXEFS\n\nYou can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefsSpec;
impl crate::RegisterSpec for TxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TxefsSpec {}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TxefsSpec {}
