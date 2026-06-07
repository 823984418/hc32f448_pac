#[doc = "Register `FWMC` reader"]
pub type R = crate::R<FwmcSpec>;
#[doc = "Register `FWMC` writer"]
pub type W = crate::W<FwmcSpec>;
#[doc = "Field `PEMOD` reader - desc PEMOD"]
pub type PemodR = crate::FieldReader;
#[doc = "Field `PEMOD` writer - desc PEMOD"]
pub type PemodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSHLDCTL` reader - desc BUSHLDCTL"]
pub type BushldctlR = crate::BitReader;
#[doc = "Field `BUSHLDCTL` writer - desc BUSHLDCTL"]
pub type BushldctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY1LOCK` reader - desc KEY1LOCK"]
pub type Key1lockR = crate::BitReader;
#[doc = "Field `KEY1LOCK` writer - desc KEY1LOCK"]
pub type Key1lockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY2LOCK` reader - desc KEY2LOCK"]
pub type Key2lockR = crate::BitReader;
#[doc = "Field `KEY2LOCK` writer - desc KEY2LOCK"]
pub type Key2lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc PEMOD"]
    #[inline(always)]
    pub fn pemod(&self) -> PemodR {
        PemodR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - desc BUSHLDCTL"]
    #[inline(always)]
    pub fn bushldctl(&self) -> BushldctlR {
        BushldctlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc KEY1LOCK"]
    #[inline(always)]
    pub fn key1lock(&self) -> Key1lockR {
        Key1lockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc KEY2LOCK"]
    #[inline(always)]
    pub fn key2lock(&self) -> Key2lockR {
        Key2lockR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWMC")
            .field("pemod", &self.pemod())
            .field("bushldctl", &self.bushldctl())
            .field("key1lock", &self.key1lock())
            .field("key2lock", &self.key2lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PEMOD"]
    #[inline(always)]
    pub fn pemod(&mut self) -> PemodW<'_, FwmcSpec> {
        PemodW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BUSHLDCTL"]
    #[inline(always)]
    pub fn bushldctl(&mut self) -> BushldctlW<'_, FwmcSpec> {
        BushldctlW::new(self, 8)
    }
    #[doc = "Bit 16 - desc KEY1LOCK"]
    #[inline(always)]
    pub fn key1lock(&mut self) -> Key1lockW<'_, FwmcSpec> {
        Key1lockW::new(self, 16)
    }
    #[doc = "Bit 17 - desc KEY2LOCK"]
    #[inline(always)]
    pub fn key2lock(&mut self) -> Key2lockW<'_, FwmcSpec> {
        Key2lockW::new(self, 17)
    }
}
#[doc = "desc FWMC\n\nYou can [`read`](crate::Reg::read) this register and get [`fwmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwmcSpec;
impl crate::RegisterSpec for FwmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwmc::R`](R) reader structure"]
impl crate::Readable for FwmcSpec {}
#[doc = "`write(|w| ..)` method takes [`fwmc::W`](W) writer structure"]
impl crate::Writable for FwmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FWMC to value 0x0003_0000"]
impl crate::Resettable for FwmcSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
