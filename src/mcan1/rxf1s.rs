#[doc = "Register `RXF1S` reader"]
pub type R = crate::R<Rxf1sSpec>;
#[doc = "Field `F1FL` reader - desc F1FL"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1GI` reader - desc F1GI"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1PI` reader - desc F1PI"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1F` reader - desc F1F"]
pub type F1fR = crate::BitReader;
#[doc = "Field `RF1L` reader - desc RF1L"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `DMS` reader - desc DMS"]
pub type DmsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - desc F1FL"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - desc F1GI"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc F1PI"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - desc F1F"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - desc DMS"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1S")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .field("dms", &self.dms())
            .finish()
    }
}
#[doc = "desc RXF1S\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1sSpec;
impl crate::RegisterSpec for Rxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1s::R`](R) reader structure"]
impl crate::Readable for Rxf1sSpec {}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for Rxf1sSpec {}
