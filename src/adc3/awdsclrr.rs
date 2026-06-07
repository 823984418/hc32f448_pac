#[doc = "Register `AWDSCLRR` writer"]
pub type W = crate::W<AwdsclrrSpec>;
#[doc = "Field `CLRAWD0F` writer - desc CLRAWD0F"]
pub type Clrawd0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRAWD1F` writer - desc CLRAWD1F"]
pub type Clrawd1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRAWDCMF` writer - desc CLRAWDCMF"]
pub type ClrawdcmfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AwdsclrrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - desc CLRAWD0F"]
    #[inline(always)]
    pub fn clrawd0f(&mut self) -> Clrawd0fW<'_, AwdsclrrSpec> {
        Clrawd0fW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CLRAWD1F"]
    #[inline(always)]
    pub fn clrawd1f(&mut self) -> Clrawd1fW<'_, AwdsclrrSpec> {
        Clrawd1fW::new(self, 1)
    }
    #[doc = "Bit 4 - desc CLRAWDCMF"]
    #[inline(always)]
    pub fn clrawdcmf(&mut self) -> ClrawdcmfW<'_, AwdsclrrSpec> {
        ClrawdcmfW::new(self, 4)
    }
}
#[doc = "desc AWDSCLRR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdsclrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdsclrrSpec;
impl crate::RegisterSpec for AwdsclrrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`awdsclrr::W`](W) writer structure"]
impl crate::Writable for AwdsclrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDSCLRR to value 0"]
impl crate::Resettable for AwdsclrrSpec {}
