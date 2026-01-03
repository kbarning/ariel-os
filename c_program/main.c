#define svc_1 0xDF01
#define bx_lr 0x4770

void call_svc(void) {
  // Adresse des MPU_CTRL-Registers
  volatile unsigned long *mpu_ctrl = (volatile unsigned long *)0xE000ED94;
  // MPU ausschalten
  *mpu_ctrl = 0x0;

  // Array für die Anweisungen. Jede Anweisung ist 16 Bit groß
  // in Armv8-M
  volatile unsigned short instructions[2];

  instructions[0] = svc_1;
  instructions[1] = bx_lr;

  // Bei der Aufzurufenden Adresse muss Bit null gesetzt werden
  // Damit der Prozessor erkennt, dass hier Maschinencode aufgerufen
  // wird, welcher in Thumb Anweisungen vorliegt.
  // Ansonsten wird ein Hardfault ausgelöst, da Armv8-M nur
  // Thumb-Anweisungen unterstützt
  unsigned long thumb_address = ((unsigned long)&instructions) | 1;

  // Adresse des Arrays in einen Funktionszeiger umwandeln
  void (*func)(void) = (void (*)(void))(thumb_address);
  // Aufruf des Funktionszeigers
  func();
}