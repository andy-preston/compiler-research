unit unit3;

{$OPTIMIZATION FORCENOSTACKFRAME}
{$OPTIMIZATION REGVAR}

interface

implementation

uses stm32f401xx, constRcc;

var
    x: comp;
    y: comp;
begin
    y := 22.0 / 7.0;
    x := 32.0 / 64.0 * y;
end.