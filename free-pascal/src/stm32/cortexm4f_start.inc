var _stack_top: record end; external name '_stack_top';

procedure Pascalmain; external name 'PASCALMAIN';

procedure HaltProc; assembler; nostackframe; public name '_haltproc';
asm
.Lloop:
    b .Lloop
end;
