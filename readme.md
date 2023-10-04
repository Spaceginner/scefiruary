# scefiruary.

an emulator for scenite firuary cpu.

## huh?

this is a rust library. it provides a simple way to emulate this cpu. [cpu docs](https://rentry.co/scenite_firuary_doc).

## state

emulator is rn kinda WIP. doesnt even support full set of instructions.

## examples

to compile them, you can use [assembler](https://github.com/Spaceginner/scefiruarier),
and to run you can use `sfemu` binary (`cargo run --bin sfemu -- <executable path>`)

### fibonacci

```asm
mv 0  ; a = 0
cp roa, rga

mv 1  ; b = 1
cp roa, rgb

mv 0  ; counter = 0
cp roa, rgc

fib_loop:
    ; print(a, counter)
    cp rga, rda
    cp rgc, rdb  
    
    ; counter++
    mv 1
    cp rgc, rob
    sum
    cp roc, rgc
    
    ; check = compare(0x7FFF, a)
    cp rga, rob
    mv 0x7FFFF
    cmpr
    cp roc, rgd
    
    ; c = a + b
    cp rgb, roa
    sum
    
    cp rgb, rga  ; a = b
    cp roc, rgb  ; b = c
    
    ; if check != 0: continue
    mv fib_loop
    cp roa, rob
    cp rgd, roa
    cpnz rob, rip
    
hlt
```
