#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Register `STFLR` writer"]
pub type W = crate::W<StflrSpec>;
#[doc = "Field `CMFA` reader - desc CMFA"]
pub type CmfaR = crate::BitReader;
#[doc = "Field `CMFA` writer - desc CMFA"]
pub type CmfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFA` reader - desc OVFA"]
pub type OvfaR = crate::BitReader;
#[doc = "Field `OVFA` writer - desc OVFA"]
pub type OvfaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPA` reader - desc ICPA"]
pub type IcpaR = crate::BitReader;
#[doc = "Field `ICPA` writer - desc ICPA"]
pub type IcpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMFB` reader - desc CMFB"]
pub type CmfbR = crate::BitReader;
#[doc = "Field `CMFB` writer - desc CMFB"]
pub type CmfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFB` reader - desc OVFB"]
pub type OvfbR = crate::BitReader;
#[doc = "Field `OVFB` writer - desc OVFB"]
pub type OvfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPB` reader - desc ICPB"]
pub type IcpbR = crate::BitReader;
#[doc = "Field `ICPB` writer - desc ICPB"]
pub type IcpbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMFA"]
    #[inline(always)]
    pub fn cmfa(&self) -> CmfaR {
        CmfaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OVFA"]
    #[inline(always)]
    pub fn ovfa(&self) -> OvfaR {
        OvfaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ICPA"]
    #[inline(always)]
    pub fn icpa(&self) -> IcpaR {
        IcpaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CMFB"]
    #[inline(always)]
    pub fn cmfb(&self) -> CmfbR {
        CmfbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc OVFB"]
    #[inline(always)]
    pub fn ovfb(&self) -> OvfbR {
        OvfbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc ICPB"]
    #[inline(always)]
    pub fn icpb(&self) -> IcpbR {
        IcpbR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STFLR")
            .field("cmfa", &self.cmfa())
            .field("ovfa", &self.ovfa())
            .field("icpa", &self.icpa())
            .field("cmfb", &self.cmfb())
            .field("ovfb", &self.ovfb())
            .field("icpb", &self.icpb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CMFA"]
    #[inline(always)]
    pub fn cmfa(&mut self) -> CmfaW<'_, StflrSpec> {
        CmfaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OVFA"]
    #[inline(always)]
    pub fn ovfa(&mut self) -> OvfaW<'_, StflrSpec> {
        OvfaW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ICPA"]
    #[inline(always)]
    pub fn icpa(&mut self) -> IcpaW<'_, StflrSpec> {
        IcpaW::new(self, 2)
    }
    #[doc = "Bit 16 - desc CMFB"]
    #[inline(always)]
    pub fn cmfb(&mut self) -> CmfbW<'_, StflrSpec> {
        CmfbW::new(self, 16)
    }
    #[doc = "Bit 17 - desc OVFB"]
    #[inline(always)]
    pub fn ovfb(&mut self) -> OvfbW<'_, StflrSpec> {
        OvfbW::new(self, 17)
    }
    #[doc = "Bit 18 - desc ICPB"]
    #[inline(always)]
    pub fn icpb(&mut self) -> IcpbW<'_, StflrSpec> {
        IcpbW::new(self, 18)
    }
}
#[doc = "desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`write(|w| ..)` method takes [`stflr::W`](W) writer structure"]
impl crate::Writable for StflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for StflrSpec {}
