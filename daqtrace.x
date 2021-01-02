SECTIONS
{
  /* `0` specifies the start address of this virtual (`(INFO)`) section */
  .daqtrace 0 (INFO) :
  {

    /* Everything user-defined */
    *(.daqtrace);
  }
}

ASSERT(SIZEOF(.daqtrace) < (1 << 16), ".daqtrace section id must fit in u16");
