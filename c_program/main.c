void entry() {
  asm("svc 0xcc");
  while (1) {
  }
}