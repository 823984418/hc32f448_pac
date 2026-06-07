#[doc = "Register `FSWP` reader"]
pub type R = crate::R<FswpSpec>;
#[doc = "Field `FSWP` reader - desc FSWP"]
pub type FswpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc FSWP"]
    #[inline(always)]
    pub fn fswp(&self) -> FswpR {
        FswpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSWP").field("fswp", &self.fswp()).finish()
    }
}
#[doc = "desc FSWP\n\nYou can [`read`](crate::Reg::read) this register and get [`fswp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FswpSpec;
impl crate::RegisterSpec for FswpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fswp::R`](R) reader structure"]
impl crate::Readable for FswpSpec {}
#[doc = "`reset()` method sets FSWP to value 0"]
impl crate::Resettable for FswpSpec {}
