{ x: reg out }
mov: 
  mov x, 5
  ret


{ a: reg out, b: reg in }
mov_add:
  mov a, b
  add a, 5
  ret

{ a: reg inout }
add:
  add a, 5
  ret



