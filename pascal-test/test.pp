program Test;

uses stm32f401xx;

label Endless;

begin
Endless:
    goto Endless;
end.