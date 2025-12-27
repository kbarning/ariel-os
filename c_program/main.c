void entry() {
  int *ptr = (int *)asm("svc 0x00"); // Call hello world
  asm("svc 0x01");                   // Call goodby world
}