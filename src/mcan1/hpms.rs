#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HpmsSpec>;
#[doc = "Field `BIDX` reader - desc BIDX"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `MSI` reader - desc MSI"]
pub type MsiR = crate::FieldReader;
#[doc = "Field `FIDX` reader - desc FIDX"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FLST` reader - desc FLST"]
pub type FlstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - desc BIDX"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - desc MSI"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - desc FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - desc FLST"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
#[doc = "desc HPMS\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpmsSpec;
impl crate::RegisterSpec for HpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HpmsSpec {}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HpmsSpec {}
