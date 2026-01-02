#define svc_1 0xDF01
#define bx_lr 0x4770

void call_svc(void)
{
    // Array für die Anweisungen. Jede Anweisung ist 16 Bit groß 
    // in Armv8-M
    volatile unsigned short instructions[2];

    instructions[0] = svc_1 | 1;
    instructions[1] = bx_lr;
    
    // Bit null muss gesetzt werden, da der Prozessor in THumb-Mode ist
    // Ansonsten wird ein Hardfault ausgelöst
    unsigned long thumb_address = ((unsigned long) &instructions) | 1;

    // Adresse des Arrays in einen Funktionszeiger umwandeln
    void(*func)(void) = (void(*)(void)) (thumb_address);
    // Aufruf des Funktionszeigers
    func();
}