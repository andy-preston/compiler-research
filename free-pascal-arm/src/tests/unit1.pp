unit unit1;

{$OPTIMIZATION FORCENOSTACKFRAME}
{$OPTIMIZATION REGVAR}

interface

implementation

uses stm32f401xx, constRcc;

label unit1Entry;

begin
unit1Entry:
    RCC.AHB1ENR := RCC.AHB1ENR or ORD(TAHB1Periph.DMA1);
end.