#[doc = "Register `FCNTR` reader"]
pub type R = crate::R<FcntrSpec>;
#[doc = "Register `FCNTR` writer"]
pub type W = crate::W<FcntrSpec>;
#[doc = "Field `NOFIENTA` reader - desc NOFIENTA"]
pub type NofientaR = crate::BitReader;
#[doc = "Field `NOFIENTA` writer - desc NOFIENTA"]
pub type NofientaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKTA` reader - desc NOFICKTA"]
pub type NoficktaR = crate::FieldReader;
#[doc = "Field `NOFICKTA` writer - desc NOFICKTA"]
pub type NoficktaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOFIENTB` reader - desc NOFIENTB"]
pub type NofientbR = crate::BitReader;
#[doc = "Field `NOFIENTB` writer - desc NOFIENTB"]
pub type NofientbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOFICKTB` reader - desc NOFICKTB"]
pub type NoficktbR = crate::FieldReader;
#[doc = "Field `NOFICKTB` writer - desc NOFICKTB"]
pub type NoficktbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&self) -> NofientaR {
        NofientaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&self) -> NoficktaR {
        NoficktaR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&self) -> NofientbR {
        NofientbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&self) -> NoficktbR {
        NoficktbR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCNTR")
            .field("nofienta", &self.nofienta())
            .field("nofickta", &self.nofickta())
            .field("nofientb", &self.nofientb())
            .field("noficktb", &self.noficktb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&mut self) -> NofientaW<'_, FcntrSpec> {
        NofientaW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&mut self) -> NoficktaW<'_, FcntrSpec> {
        NoficktaW::new(self, 1)
    }
    #[doc = "Bit 4 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&mut self) -> NofientbW<'_, FcntrSpec> {
        NofientbW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&mut self) -> NoficktbW<'_, FcntrSpec> {
        NoficktbW::new(self, 5)
    }
}
#[doc = "desc FCNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcntrSpec;
impl crate::RegisterSpec for FcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcntr::R`](R) reader structure"]
impl crate::Readable for FcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcntr::W`](W) writer structure"]
impl crate::Writable for FcntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCNTR to value 0"]
impl crate::Resettable for FcntrSpec {}
