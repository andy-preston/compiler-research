#define SRAM_BASE       0x20000000U
#define PERIPH_BASE     0x40000000U

#define SRAM_SIZE       40*1024
#define SRAM_END        (SRAM_BASE + SRAM_SIZE)

#define RCC_BASE        (PERIPH_BASE + 0x21000)
#define RCC_AHBENR      (*(volatile unsigned long*)(RCC_BASE + 0x14))

#define GPIOE_BASE      (PERIPH_BASE + 0x00001000 + 0x08000000)
#define GPIOE_MODER     (*(volatile unsigned long*)(GPIOE_BASE + 0x00))
#define GPIOE_ODR       (*(volatile unsigned long*)(GPIOE_BASE + 0x14))

#define LED_PIN         9


int main()
{
    float p[200];
    for (int i = 0; i < 200; i++) {
        p[i] = 2.0f * i;
    }
}

unsigned long *vector_table[] __attribute__((section(".vector_table"))) =
{
    (unsigned long *)SRAM_END,   // place stack pointer at the end of SRAM
                                 // (stack grows down)
    (unsigned long *)main        // reset handler, jump directly to main
};