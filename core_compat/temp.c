for (int i = 0; i < clientmap.map_size_y; i++)
{
   for (int j = 0; j < clientmap.map_size_x; j++)
   {
      uint8_t b_0 = _bytes[offset.getOffset() + 0];
      uint8_t b_1 = _bytes[offset.getOffset() + 1];
      uint8_t b_2 = _bytes[offset.getOffset() + 2];
      uint8_t b_3 = _bytes[offset.getOffset() + 3];
      uint8_t b_4 = _bytes[offset.getOffset() + 4];
      uint8_t b_5 = _bytes[offset.getOffset() + 5];
      uint8_t b_6 = _bytes[offset.getOffset() + 6];
      uint8_t b_7 = _bytes[offset.getOffset() + 7];

      int tmp_obj_index;
      if (b_0 == 0)
      {
         tmp_obj_index = 0;
      }
      else
      {
         // tmp_obj_index = (_bytes[offset.getOffset()] / 4) + ((_bytes[offset.getOffset() + 1] % 32) * 64);
         tmp_obj_index = (b_0 >> 2) + (( b_1 & 0x1F) << 6);
      }

      // int tmp_tle_part = ((_bytes[offset.getOffset() + 2] % 128) * 8) + (int)(_bytes[offset.getOffset() + 1] / 32);
      int tmp_tle_part = (int)(b_1 >> 5) + ((b_2 & 0x7F) << 3);

      // int tmp_tle_index = ((_bytes[offset.getOffset() + 3] * 2) + ((_bytes[offset.getOffset() + 2] / 128)));
      int tmp_tle_index = (b_2 >> 7) + (b_3 << 1);

      Boolean tmp_warp;

      if (b_4 > 0)
         tmp_warp = true;
      else
         tmp_warp = false;

      int tmp_collision = b_6;

      int tmp_obj_part;
      if ((tmp_collision % 24) == 0)
         tmp_obj_part = (b_7 << 1);
      else
         tmp_obj_part = (b_7 << 1) + 1;

      clientmap.map_tiles[j][i] =
       new ClientMapRow(tmp_obj_index, tmp_obj_part, tmp_tle_index,
                        tmp_tle_part, tmp_collision, tmp_warp);

      offset.setOffset(8);
   }
}