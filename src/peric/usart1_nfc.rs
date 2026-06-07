#[doc = "Register `USART1_NFC` reader"]
pub type R = crate::R<Usart1NfcSpec>;
#[doc = "Register `USART1_NFC` writer"]
pub type W = crate::W<Usart1NfcSpec>;
#[doc = "Field `USASRT1_NFS` reader - desc USASRT1_NFS"]
pub type Usasrt1NfsR = crate::FieldReader;
#[doc = "Field `USASRT1_NFS` writer - desc USASRT1_NFS"]
pub type Usasrt1NfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART1_NFE` reader - desc USART1_NFE"]
pub type Usart1NfeR = crate::BitReader;
#[doc = "Field `USART1_NFE` writer - desc USART1_NFE"]
pub type Usart1NfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc USASRT1_NFS"]
    #[inline(always)]
    pub fn usasrt1_nfs(&self) -> Usasrt1NfsR {
        Usasrt1NfsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc USART1_NFE"]
    #[inline(always)]
    pub fn usart1_nfe(&self) -> Usart1NfeR {
        Usart1NfeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1_NFC")
            .field("usasrt1_nfs", &self.usasrt1_nfs())
            .field("usart1_nfe", &self.usart1_nfe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc USASRT1_NFS"]
    #[inline(always)]
    pub fn usasrt1_nfs(&mut self) -> Usasrt1NfsW<'_, Usart1NfcSpec> {
        Usasrt1NfsW::new(self, 0)
    }
    #[doc = "Bit 2 - desc USART1_NFE"]
    #[inline(always)]
    pub fn usart1_nfe(&mut self) -> Usart1NfeW<'_, Usart1NfcSpec> {
        Usart1NfeW::new(self, 2)
    }
}
#[doc = "desc USART1_NFC\n\nYou can [`read`](crate::Reg::read) this register and get [`usart1_nfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_nfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usart1NfcSpec;
impl crate::RegisterSpec for Usart1NfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart1_nfc::R`](R) reader structure"]
impl crate::Readable for Usart1NfcSpec {}
#[doc = "`write(|w| ..)` method takes [`usart1_nfc::W`](W) writer structure"]
impl crate::Writable for Usart1NfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART1_NFC to value 0"]
impl crate::Resettable for Usart1NfcSpec {}
