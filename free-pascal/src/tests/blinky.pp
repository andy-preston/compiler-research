program Blinky;

{$OPTIMIZATION FORCENOSTACKFRAME}
{$OPTIMIZATION REGVAR}

uses stm32f401xx, constRcc;

label infiniteLoop;

begin
    RCC.AHB1ENR := RCC.AHB1ENR or ORD(TAHB1Periph.DMA1);
infiniteLoop:
    goto infiniteLoop;
end.