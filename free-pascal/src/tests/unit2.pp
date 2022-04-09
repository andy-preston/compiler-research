unit unit2;

{$OPTIMIZATION FORCENOSTACKFRAME}
{$OPTIMIZATION REGVAR}

interface

implementation

uses stm32f401xx, constRcc;

label unit1Entry;

begin
    RCC.AHB1ENR := RCC.AHB1ENR or ORD(TAHB1Periph.DMA1);
    goto unit1Entry;
end.