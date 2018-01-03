/// from : Nw200.dll

#include <stdint.h>

struct s2 {
   signed char f0;
   signed char f1;
   unsigned char f2;
   signed char f3;
   signed char f4;
   signed char f5;
   unsigned char f6;
   signed char f7;
   unsigned char f8;
};

struct s3 {
   unsigned char f0;
   signed char f1;
};

struct s4 {
   unsigned char f0;
   signed char f1;
};

struct s5 {
   unsigned char f0;
   signed char[1] pad2;
   unsigned char f2;
};

struct s6 {
   unsigned char f0;
   signed char[1] pad2;
   unsigned char f2;
};

struct s7 {
   signed char[8] pad8;
   void** f8;
};

/*
 * ?DecodePacket@CConnectHandler@@IAEHPBEPAF1PAE@Z
 * protected: int __thiscall CConnectHandler::DecodePacket(
 *            unsigned char const *,short *,short *,unsigned char *)
 */
int32_t
decode_packet(
   void** ecx,
   struct s2* a2,
   struct s3* a3,
   struct s4* a4,
   struct s5* a5)
{
   void** ebp6;
   signed char dl7;
   uint32_t v8;
   signed char al9;
   signed char dl10;
   int1_t zf11;
   struct s6* esi12;
   struct s7* edx13;
   struct s5* eax14;
   int32_t edx15;
   int32_t** edx16;
   unsigned char bl17;
   int32_t eax18;
   unsigned char bl19;
   unsigned char bl20;
   int32_t** edx21;
   void* eax22;
   int32_t edx23;

   ebp6 = *reinterpret_cast<void***>(ecx + 0x8c) + 1;
   dl7 = a2->f7;
   *reinterpret_cast<signed char*>(&v8) = a2->f3;
   al9 = a2->f1;
   *reinterpret_cast<signed char*>(reinterpret_cast<int32_t>(&v8) + 1) = dl7;
   dl10 = a2->f5;
   *reinterpret_cast<signed char*>(reinterpret_cast<int32_t>(&v8) + 2) = al9;
   *reinterpret_cast<signed char*>(reinterpret_cast<int32_t>(&v8) + 3) = dl10;
   zf11 = reinterpret_cast<int1_t>(ebp6 == (reinterpret_cast<unsigned char>(*reinterpret_cast<void***>(ecx + 80)) ^ v8));
   esi12 = reinterpret_cast<struct s6*>(&a2->f8);
   *reinterpret_cast<void***>(ecx + 0x8c) = ebp6;
   if (!zf11) {
      return -1;
   }
   a4->f0 = a2->f2;
   a4->f1 = a2->f4;
   a4->f0 = reinterpret_cast<unsigned char>(a4->f0 ^ *reinterpret_cast<uint16_t*>(ecx + 0x72));
   edx13 = reinterpret_cast<struct s7*>(static_cast<int32_t>(reinterpret_cast<int16_t>(a4->f0)));
   if (reinterpret_cast<signed char>(*reinterpret_cast<void***>(ecx + 0x2180)) < reinterpret_cast<signed char>(&edx13->f8)) {
      *reinterpret_cast<void***>(ecx + 0x8c) = *reinterpret_cast<void***>(ecx + 0x8c) - 1;
      return 0;
   }
   a3->f0 = a2->f6;
   a3->f1 = a2->f0;
   a3->f0 = reinterpret_cast<unsigned char>(a3->f0 ^ *reinterpret_cast<uint16_t*>(ecx + 0x72));
   eax14 = a5;
   while (1) {
      if (reinterpret_cast<int32_t>(edx13) < 8) {
         if (reinterpret_cast<int32_t>(edx13) < 4)
            break;
         esi12 = reinterpret_cast<struct s6*>(reinterpret_cast<uint32_t>(esi12) + 4);
         eax14->f0 = reinterpret_cast<unsigned char>(reinterpret_cast<uint32_t>(*reinterpret_cast<int32_t***>(ecx + 0x80)) ^ esi12->f0);
         eax14 = reinterpret_cast<struct s5*>(reinterpret_cast<int32_t>(eax14) + 4);
         edx13 = reinterpret_cast<struct s7*>(reinterpret_cast<int32_t>(edx13) - 4);
      } else {
         esi12 = reinterpret_cast<struct s6*>(reinterpret_cast<uint32_t>(esi12) + 8);
         eax14->f0 = reinterpret_cast<unsigned char>(esi12->f0 ^ reinterpret_cast<uint32_t>(*reinterpret_cast<int32_t***>(ecx + 0x80)));
         eax14 = reinterpret_cast<struct s5*>(reinterpret_cast<int32_t>(eax14) + 8);
         edx13 = reinterpret_cast<struct s7*>(reinterpret_cast<int32_t>(edx13) - 8);
         *reinterpret_cast<uint32_t*>(reinterpret_cast<int32_t>(eax14) - 4) = *reinterpret_cast<uint32_t*>(reinterpret_cast<uint32_t>(esi12) - 4) ^ reinterpret_cast<uint32_t>(*reinterpret_cast<int32_t***>(ecx + 0x80));
      }
   }
   edx15 = reinterpret_cast<int32_t>(edx13) - 1;
   if (edx15)
      goto addr_1000379d_11;
   eax14->f0 = reinterpret_cast<unsigned char>(*reinterpret_cast<unsigned char*>(ecx + 0x70) ^ esi12->f0);
   eax14 = reinterpret_cast<struct s5*>(&eax14->pad2);
   addr_100037d5_13:
   eax14->f0 = 0;
   edx16 = *reinterpret_cast<int32_t***>(ecx + 0x80);
   bl17 = reinterpret_cast<unsigned char>(*reinterpret_cast<unsigned char*>(ecx + 0x70) + *reinterpret_cast<signed char*>(ecx + 0x108));
   *reinterpret_cast<signed char*>(ecx + 0x108) = reinterpret_cast<signed char>(*reinterpret_cast<signed char*>(ecx + 0x108) + 1);
   *reinterpret_cast<int16_t*>(&eax14) = *reinterpret_cast<int16_t*>(ecx + 0x10a);
   *reinterpret_cast<uint16_t*>(ecx + 0x72) = reinterpret_cast<uint16_t>(*reinterpret_cast<uint16_t*>(ecx + 0x72) + *reinterpret_cast<int16_t*>(&eax14));
   eax18 = reinterpret_cast<int32_t>(&eax14->pad2);
   *reinterpret_cast<unsigned char*>(ecx + 0x70) = bl17;
   bl19 = *reinterpret_cast<unsigned char*>(ecx + 0x74);
   *reinterpret_cast<int16_t*>(ecx + 0x10a) = *reinterpret_cast<int16_t*>(&eax18);
   bl20 = reinterpret_cast<unsigned char>(bl19 + *reinterpret_cast<signed char*>(ecx + 0x110));
   *reinterpret_cast<signed char*>(ecx + 0x110) = reinterpret_cast<signed char>(*reinterpret_cast<signed char*>(ecx + 0x110) + 1);
   edx21 = reinterpret_cast<int32_t**>(reinterpret_cast<uint32_t>(edx16) + reinterpret_cast<int32_t>(*reinterpret_cast<void**>(ecx + 0x114)));
   eax22 = reinterpret_cast<void*>(reinterpret_cast<int32_t>(*reinterpret_cast<void**>(ecx + 0x114)) + 1);
   *reinterpret_cast<unsigned char*>(ecx + 0x74) = bl20;
   *reinterpret_cast<void**>(ecx + 0x114) = eax22;
   *reinterpret_cast<int32_t***>(ecx + 0x80) = edx21;
   return 1;
   addr_1000379d_11:
   edx23 = edx15 - 1;
   if (!edx23) {
      eax14 = reinterpret_cast<struct s5*>(&eax14->f2);
      *reinterpret_cast<uint16_t*>(reinterpret_cast<int32_t>(eax14) - 2) = reinterpret_cast<uint16_t>(*reinterpret_cast<uint16_t*>(ecx + 0x72) ^ esi12->f0);
      goto addr_100037d5_13;
   } else {
      if (!(edx23 - 1)) {
         eax14->f0 = reinterpret_cast<unsigned char>(*reinterpret_cast<uint16_t*>(ecx + 0x72) ^ esi12->f0);
         eax14->f2 = reinterpret_cast<unsigned char>(*reinterpret_cast<unsigned char*>(ecx + 0x74) ^ esi12->f2);
         ++eax14;
         goto addr_100037d5_13;
      }
   }
}

/*
 * ?Encript@CPacketCODEC@@QAEHPAD0PAH@Z
 * public: int __thiscall CPacketCODEC::Encript(char *,char *,int *)
 */
int32_t
encrypt_packet(
   void** _this,
   char* out_str,
   char* in_str,
   int* in_len_ptr)
{
   int32_t* out_len_ptr;
   int32_t out_idx;
   int32_t in_idx;
   char* out_ptr;
   char* in_ptr;
   unsigned char cur_chr;

   out_len_ptr = in_len_ptr;
   out_idx = NULL;
   in_idx = NULL;
   if (*out_len_ptr <= 0) {
      *out_len_ptr = NULL;
   } else {
      out_ptr = out_str;
      in_ptr = in_str;
      do {
         cur_chr = in_ptr[in_idx];
         if (cur_chr >= 0 && cur_chr <= 7) {
            out_ptr[out_idx] = 7;
            out_idx += 1;
            cur_chr = in_ptr[in_idx] ^ 15;
         }
         out_ptr[out_idx] = cur_chr;
         out_idx += 1;
         in_idx += 1;
      } while (in_idx < *out_len_ptr);
      *out_len_ptr = out_idx;
   }
   return 1;
}

/*
 * ?Decript@CPacketCODEC@@QAEHPADPAH@Z
 * public: int __thiscall CPacketCODEC::Decript(char *,int *)
 */
int32_t
decrypt_packet(
   char* in_string,
   int** in_len_ptr
) {
   int32_t *out_len_ptr;
   int32_t  out_idx;
   int32_t  in_idx;
   char    *chr_ptr;
   unsigned char tmp_chr;

   out_len_ptr = in_len_ptr;
   out_idx = 0;
   in_idx = 0;
   if (*out_len_ptr <= 0) {
      *out_len_ptr = NULL;
   } else {
      chr_ptr = in_string;
      do {
         tmp_chr = chr_ptr[in_idx];
         if (tmp_chr == 7) {
            in_idx += 1;
            tmp_chr = chr_ptr[in_idx] ^ 15;
         }
         chr_ptr[out_idx] = tmp_chr;
         out_idx += 1;
         in_idx += 1;
      } while (in_idx < *out_len_ptr);
      *out_len_ptr = out_idx;
   }
   return 1;
}


struct s20 {
   signed char[4] pad4;
   void** f4;
   signed char[15] pad20;
   uint32_t f20;
   signed char[24] pad48;
   uint32_t f48;
   signed char[128] pad180;
   uint32_t f180;
   signed char[12] pad196;
   int32_t f196;
   signed char[12] pad212;
   int32_t f212;
   signed char[8372] pad8588;
   void** f8588;
};

/*
 * ?GenKey@CConnectHandler@@AAEXXZ
 * private: void __thiscall CConnectHandler::GenKey(void)
 */
void
gen_key(struct s20* ecx) {
   int32_t eax2;
   uint32_t eax3;
   int32_t ecx4;
   uint32_t edx5;
   int32_t ecx6;

   eax2 = (int32_t)time(NULL);
   srand(eax2, 0);
   eax3 = (uint32_t)rand();
   ecx4 = ecx->f212;
   edx5 = ecx->f180 ^ eax3;
   ecx->f48 = eax3;
   ecx6 = ecx->f196;
   ecx->f20 = edx5 >> *(signed char*)  (&ecx4)
            | edx5 << *(unsigned char*)(&ecx6);
   return;
}
