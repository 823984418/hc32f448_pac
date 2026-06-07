#[doc = "Register `AWDSR` reader"]
pub type R = crate::R<AwdsrSpec>;
#[doc = "Field `AWD0F` reader - desc AWD0F"]
pub type Awd0fR = crate::BitReader;
#[doc = "Field `AWD1F` reader - desc AWD1F"]
pub type Awd1fR = crate::BitReader;
#[doc = "Field `AWDCMF` reader - desc AWDCMF"]
pub type AwdcmfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc AWD0F"]
    #[inline(always)]
    pub fn awd0f(&self) -> Awd0fR {
        Awd0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AWD1F"]
    #[inline(always)]
    pub fn awd1f(&self) -> Awd1fR {
        Awd1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AWDCMF"]
    #[inline(always)]
    pub fn awdcmf(&self) -> AwdcmfR {
        AwdcmfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWDSR")
            .field("awd0f", &self.awd0f())
            .field("awd1f", &self.awd1f())
            .field("awdcmf", &self.awdcmf())
            .finish()
    }
}
#[doc = "desc AWDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdsrSpec;
impl crate::RegisterSpec for AwdsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awdsr::R`](R) reader structure"]
impl crate::Readable for AwdsrSpec {}
#[doc = "`reset()` method sets AWDSR to value 0"]
impl crate::Resettable for AwdsrSpec {}
