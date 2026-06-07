#[doc = "Register `PWRC1` reader"]
pub type R = crate::R<Pwrc1Spec>;
#[doc = "Register `PWRC1` writer"]
pub type W = crate::W<Pwrc1Spec>;
#[doc = "Field `VPLLSD` reader - desc VPLLSD"]
pub type VpllsdR = crate::FieldReader;
#[doc = "Field `VPLLSD` writer - desc VPLLSD"]
pub type VpllsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VHRCSD` reader - desc VHRCSD"]
pub type VhrcsdR = crate::BitReader;
#[doc = "Field `VHRCSD` writer - desc VHRCSD"]
pub type VhrcsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDTS` reader - desc PDTS"]
pub type PdtsR = crate::BitReader;
#[doc = "Field `PDTS` writer - desc PDTS"]
pub type PdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPDAS` reader - desc STPDAS"]
pub type StpdasR = crate::FieldReader;
#[doc = "Field `STPDAS` writer - desc STPDAS"]
pub type StpdasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc VPLLSD"]
    #[inline(always)]
    pub fn vpllsd(&self) -> VpllsdR {
        VpllsdR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - desc VHRCSD"]
    #[inline(always)]
    pub fn vhrcsd(&self) -> VhrcsdR {
        VhrcsdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PDTS"]
    #[inline(always)]
    pub fn pdts(&self) -> PdtsR {
        PdtsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc STPDAS"]
    #[inline(always)]
    pub fn stpdas(&self) -> StpdasR {
        StpdasR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC1")
            .field("vpllsd", &self.vpllsd())
            .field("vhrcsd", &self.vhrcsd())
            .field("pdts", &self.pdts())
            .field("stpdas", &self.stpdas())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc VPLLSD"]
    #[inline(always)]
    pub fn vpllsd(&mut self) -> VpllsdW<'_, Pwrc1Spec> {
        VpllsdW::new(self, 0)
    }
    #[doc = "Bit 2 - desc VHRCSD"]
    #[inline(always)]
    pub fn vhrcsd(&mut self) -> VhrcsdW<'_, Pwrc1Spec> {
        VhrcsdW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PDTS"]
    #[inline(always)]
    pub fn pdts(&mut self) -> PdtsW<'_, Pwrc1Spec> {
        PdtsW::new(self, 3)
    }
    #[doc = "Bits 6:7 - desc STPDAS"]
    #[inline(always)]
    pub fn stpdas(&mut self) -> StpdasW<'_, Pwrc1Spec> {
        StpdasW::new(self, 6)
    }
}
#[doc = "desc PWRC1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc1Spec;
impl crate::RegisterSpec for Pwrc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc1::R`](R) reader structure"]
impl crate::Readable for Pwrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc1::W`](W) writer structure"]
impl crate::Writable for Pwrc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC1 to value 0"]
impl crate::Resettable for Pwrc1Spec {}
