# HLeOs
HLeOs, a educational operating system programed by rust.
-----------------------------------------------------

show some descriptions about this project. in my blog, .. it 's in korean

[myblog](https://hacking-yi.kro.kr "my blog")

### 01. how to debug boot loader in real mode debugging

[ko] Makefile을 보면, gdb-i386과 debug-loader가 있다. 이 두 규칙이 boot loader를 디버깅하기 위한 것이다.
굳이 qemu-system-i386을 사용하는 규칙을 만들어낸 이유는, gdb로 realmode 디버깅을 하는데 있어서,
x86_64로 하면 에러가 났기 때문이다. 그래서 어쩔 수 없지만, realmode 디버깅은 제한적으로 진행해야 했다.

일단 아래의 방식을 따르자.
1. 터미널을 두 개 연다.
2. 한 쪽에서는 "make ; make gdb-i386"을 입력하자.
3. 다른 한 쪽에서는 "make debug-loader"를 입력하자.
4. 디버깅하자. 심볼이 안뜨는데, file 명령어를 치면 깨지더라;;
 ++ symbol 명령어 쓰면 심볼 생긴다.

[en] you can find two rules in Makefile, 'gdb-i386', 'debug-loader' this two rules are used to debug the boot loader in real mode.
I made 'gdb-i386' because, when I debug boot loader in realmode, It makes an error to use 'qemu-system-x86_64'
so I made restrictly the two rules.

How to debug loader
1. open two terminal
2. in one, type "make ; make gdb-i386"
3. in another, type "make debug-loader"
4. debug ! # file command makes error :(
 ++ you can make symbol by using 'symbol' command
