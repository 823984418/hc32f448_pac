#[doc = "Register `LBMC` reader"]
pub type R = crate::R<LbmcSpec>;
#[doc = "Field `LBMC` reader - desc LBMC"]
pub type LbmcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc LBMC"]
    #[inline(always)]
    pub fn lbmc(&self) -> LbmcR {
        LbmcR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LBMC").field("lbmc", &self.lbmc()).finish()
    }
}
#[doc = "desc LBMC\n\nYou can [`read`](crate::Reg::read) this register and get [`lbmc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LbmcSpec;
impl crate::RegisterSpec for LbmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lbmc::R`](R) reader structure"]
impl crate::Readable for LbmcSpec {}
#[doc = "`reset()` method sets LBMC to value 0"]
impl crate::Resettable for LbmcSpec {}
