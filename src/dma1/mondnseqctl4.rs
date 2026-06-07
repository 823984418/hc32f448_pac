#[doc = "Register `MONDNSEQCTL4` reader"]
pub type R = crate::R<Mondnseqctl4Spec>;
#[doc = "Field `DOFFSET` reader - desc DOFFSET"]
pub type DoffsetR = crate::FieldReader<u32>;
#[doc = "Field `DNSCNT` reader - desc DNSCNT"]
pub type DnscntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - desc DOFFSET"]
    #[inline(always)]
    pub fn doffset(&self) -> DoffsetR {
        DoffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc DNSCNT"]
    #[inline(always)]
    pub fn dnscnt(&self) -> DnscntR {
        DnscntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONDNSEQCTL4")
            .field("doffset", &self.doffset())
            .field("dnscnt", &self.dnscnt())
            .finish()
    }
}
#[doc = "desc MONDNSEQCTL4\n\nYou can [`read`](crate::Reg::read) this register and get [`mondnseqctl4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondnseqctl4Spec;
impl crate::RegisterSpec for Mondnseqctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondnseqctl4::R`](R) reader structure"]
impl crate::Readable for Mondnseqctl4Spec {}
#[doc = "`reset()` method sets MONDNSEQCTL4 to value 0"]
impl crate::Resettable for Mondnseqctl4Spec {}
