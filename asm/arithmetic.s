.option norvc

main:
  addi x20, x0, 3
  addi x21, x0, 2
  addi x22, x0, 4

  add x23, x22, x21
  sub x24, x22, x21
  mul x25, x22, x21
  div x26, x22, x21
  rem x27, x22, x20

  div x28, x22, x0
  rem x28, x22, x0
