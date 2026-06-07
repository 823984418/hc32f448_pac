#[doc = "Register `CPSR0` reader"]
pub type R = crate::R<Cpsr0Spec>;
#[doc = "Field `RSYN` reader - desc RSYN"]
pub type RsynR = crate::BitReader;
#[doc = "Field `WSYN` reader - desc WSYN"]
pub type WsynR = crate::BitReader;
#[doc = "Field `MW` reader - desc MW"]
pub type MwR = crate::FieldReader;
#[doc = "Field `BAAS` reader - desc BAAS"]
pub type BaasR = crate::BitReader;
#[doc = "Field `ADVS` reader - desc ADVS"]
pub type AdvsR = crate::BitReader;
#[doc = "Field `BLSS` reader - desc BLSS"]
pub type BlssR = crate::BitReader;
#[doc = "Field `ADDMSK` reader - desc ADDMSK"]
pub type AddmskR = crate::FieldReader;
#[doc = "Field `ADDMAT` reader - desc ADDMAT"]
pub type AddmatR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc RSYN"]
    #[inline(always)]
    pub fn rsyn(&self) -> RsynR {
        RsynR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc WSYN"]
    #[inline(always)]
    pub fn wsyn(&self) -> WsynR {
        WsynR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc MW"]
    #[inline(always)]
    pub fn mw(&self) -> MwR {
        MwR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc BAAS"]
    #[inline(always)]
    pub fn baas(&self) -> BaasR {
        BaasR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ADVS"]
    #[inline(always)]
    pub fn advs(&self) -> AdvsR {
        AdvsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BLSS"]
    #[inline(always)]
    pub fn blss(&self) -> BlssR {
        BlssR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23 - desc ADDMSK"]
    #[inline(always)]
    pub fn addmsk(&self) -> AddmskR {
        AddmskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc ADDMAT"]
    #[inline(always)]
    pub fn addmat(&self) -> AddmatR {
        AddmatR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSR0")
            .field("rsyn", &self.rsyn())
            .field("wsyn", &self.wsyn())
            .field("mw", &self.mw())
            .field("baas", &self.baas())
            .field("advs", &self.advs())
            .field("blss", &self.blss())
            .field("addmsk", &self.addmsk())
            .field("addmat", &self.addmat())
            .finish()
    }
}
#[doc = "desc CPSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpsr0Spec;
impl crate::RegisterSpec for Cpsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr0::R`](R) reader structure"]
impl crate::Readable for Cpsr0Spec {}
#[doc = "`reset()` method sets CPSR0 to value 0x00ff_0a00"]
impl crate::Resettable for Cpsr0Spec {
    const RESET_VALUE: u32 = 0x00ff_0a00;
}
