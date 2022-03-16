
HLeOs:     file format elf64-x86-64


Disassembly of section .text:

0000000000201340 <_start>:
  201340:	48 81 ec e8 00 00 00 	sub    $0xe8,%rsp
  201347:	b8 00 80 0b 00       	mov    $0xb8000,%eax
  20134c:	48 89 84 24 a0 00 00 	mov    %rax,0xa0(%rsp)
  201353:	00 
  201354:	48 8b 3d cd 19 00 00 	mov    0x19cd(%rip),%rdi        # 202d28 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE>
  20135b:	48 8b 35 ce 19 00 00 	mov    0x19ce(%rip),%rsi        # 202d30 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x8>
  201362:	e8 99 08 00 00       	callq  201c00 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h0194e32a0aca9dcfE>
  201367:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
  20136c:	48 89 54 24 40       	mov    %rdx,0x40(%rsp)
  201371:	48 8b 54 24 40       	mov    0x40(%rsp),%rdx
  201376:	48 8b 74 24 38       	mov    0x38(%rsp),%rsi
  20137b:	48 8d 7c 24 60       	lea    0x60(%rsp),%rdi
  201380:	e8 7b 03 00 00       	callq  201700 <_ZN4core4iter6traits8iterator8Iterator9enumerate17h2e686609dd86a187E>
  201385:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
  20138a:	48 8d 74 24 60       	lea    0x60(%rsp),%rsi
  20138f:	e8 dc 02 00 00       	callq  201670 <_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h72cbdded72153823E>
  201394:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
  201399:	48 89 44 24 78       	mov    %rax,0x78(%rsp)
  20139e:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
  2013a3:	48 89 84 24 80 00 00 	mov    %rax,0x80(%rsp)
  2013aa:	00 
  2013ab:	48 8b 44 24 58       	mov    0x58(%rsp),%rax
  2013b0:	48 89 84 24 88 00 00 	mov    %rax,0x88(%rsp)
  2013b7:	00 
  2013b8:	48 8d 7c 24 78       	lea    0x78(%rsp),%rdi
  2013bd:	e8 be 01 00 00       	callq  201580 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE>
  2013c2:	48 89 94 24 98 00 00 	mov    %rdx,0x98(%rsp)
  2013c9:	00 
  2013ca:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
  2013d1:	00 
  2013d2:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
  2013d9:	00 
  2013da:	48 85 c0             	test   %rax,%rax
  2013dd:	0f 95 c0             	setne  %al
  2013e0:	0f b6 c0             	movzbl %al,%eax
  2013e3:	74 06                	je     2013eb <_start+0xab>
  2013e5:	eb 00                	jmp    2013e7 <_start+0xa7>
  2013e7:	eb 04                	jmp    2013ed <_start+0xad>
  2013e9:	0f 0b                	ud2    
  2013eb:	eb fe                	jmp    2013eb <_start+0xab>
  2013ed:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
  2013f4:	00 
  2013f5:	8a 00                	mov    (%rax),%al
  2013f7:	88 44 24 27          	mov    %al,0x27(%rsp)
  2013fb:	88 84 24 af 00 00 00 	mov    %al,0xaf(%rsp)
  201402:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
  201409:	00 
  20140a:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
  20140f:	48 89 84 24 b0 00 00 	mov    %rax,0xb0(%rsp)
  201416:	00 
  201417:	b9 02 00 00 00       	mov    $0x2,%ecx
  20141c:	48 0f af c1          	imul   %rcx,%rax
  201420:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
  201425:	0f 90 c0             	seto   %al
  201428:	a8 01                	test   $0x1,%al
  20142a:	75 39                	jne    201465 <_start+0x125>
  20142c:	48 8b 4c 24 30       	mov    0x30(%rsp),%rcx
  201431:	b8 00 80 0b 00       	mov    $0xb8000,%eax
  201436:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
  20143d:	00 
  20143e:	48 89 8c 24 d8 00 00 	mov    %rcx,0xd8(%rsp)
  201445:	00 
  201446:	b8 00 80 0b 00       	mov    $0xb8000,%eax
  20144b:	48 01 c8             	add    %rcx,%rax
  20144e:	48 89 84 24 e0 00 00 	mov    %rax,0xe0(%rsp)
  201455:	00 
  201456:	48 8b 84 24 e0 00 00 	mov    0xe0(%rsp),%rax
  20145d:	00 
  20145e:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
  201463:	eb 1a                	jmp    20147f <_start+0x13f>
  201465:	48 8d 3d 34 ed ff ff 	lea    -0x12cc(%rip),%rdi        # 2001a0 <str.0>
  20146c:	48 8d 15 85 18 00 00 	lea    0x1885(%rip),%rdx        # 202cf8 <_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h97758ff61c3550eaE+0x1048>
  201473:	be 21 00 00 00       	mov    $0x21,%esi
  201478:	e8 e3 04 00 00       	callq  201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>
  20147d:	0f 0b                	ud2    
  20147f:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
  201484:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
  201489:	8a 54 24 27          	mov    0x27(%rsp),%dl
  20148d:	88 11                	mov    %dl,(%rcx)
  20148f:	b9 02 00 00 00       	mov    $0x2,%ecx
  201494:	48 0f af c1          	imul   %rcx,%rax
  201498:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  20149d:	0f 90 c0             	seto   %al
  2014a0:	a8 01                	test   $0x1,%al
  2014a2:	75 16                	jne    2014ba <_start+0x17a>
  2014a4:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  2014a9:	48 ff c0             	inc    %rax
  2014ac:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  2014b1:	0f 90 c0             	seto   %al
  2014b4:	a8 01                	test   $0x1,%al
  2014b6:	75 54                	jne    20150c <_start+0x1cc>
  2014b8:	eb 1a                	jmp    2014d4 <_start+0x194>
  2014ba:	48 8d 3d df ec ff ff 	lea    -0x1321(%rip),%rdi        # 2001a0 <str.0>
  2014c1:	48 8d 15 48 18 00 00 	lea    0x1848(%rip),%rdx        # 202d10 <_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h97758ff61c3550eaE+0x1060>
  2014c8:	be 21 00 00 00       	mov    $0x21,%esi
  2014cd:	e8 8e 04 00 00       	callq  201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>
  2014d2:	0f 0b                	ud2    
  2014d4:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
  2014d9:	b8 00 80 0b 00       	mov    $0xb8000,%eax
  2014de:	48 89 84 24 b8 00 00 	mov    %rax,0xb8(%rsp)
  2014e5:	00 
  2014e6:	48 89 8c 24 c0 00 00 	mov    %rcx,0xc0(%rsp)
  2014ed:	00 
  2014ee:	b8 00 80 0b 00       	mov    $0xb8000,%eax
  2014f3:	48 01 c8             	add    %rcx,%rax
  2014f6:	48 89 84 24 c8 00 00 	mov    %rax,0xc8(%rsp)
  2014fd:	00 
  2014fe:	48 8b 84 24 c8 00 00 	mov    0xc8(%rsp),%rax
  201505:	00 
  201506:	48 89 04 24          	mov    %rax,(%rsp)
  20150a:	eb 1a                	jmp    201526 <_start+0x1e6>
  20150c:	48 8d 3d bd ec ff ff 	lea    -0x1343(%rip),%rdi        # 2001d0 <str.1>
  201513:	48 8d 15 f6 17 00 00 	lea    0x17f6(%rip),%rdx        # 202d10 <_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h97758ff61c3550eaE+0x1060>
  20151a:	be 1c 00 00 00       	mov    $0x1c,%esi
  20151f:	e8 3c 04 00 00       	callq  201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>
  201524:	0f 0b                	ud2    
  201526:	48 8b 04 24          	mov    (%rsp),%rax
  20152a:	c6 00 0b             	movb   $0xb,(%rax)
  20152d:	e9 86 fe ff ff       	jmpq   2013b8 <_start+0x78>
  201532:	cc                   	int3   
  201533:	cc                   	int3   
  201534:	cc                   	int3   
  201535:	cc                   	int3   
  201536:	cc                   	int3   
  201537:	cc                   	int3   
  201538:	cc                   	int3   
  201539:	cc                   	int3   
  20153a:	cc                   	int3   
  20153b:	cc                   	int3   
  20153c:	cc                   	int3   
  20153d:	cc                   	int3   
  20153e:	cc                   	int3   
  20153f:	cc                   	int3   

0000000000201540 <rust_begin_unwind>:
  201540:	50                   	push   %rax
  201541:	48 89 3c 24          	mov    %rdi,(%rsp)
  201545:	eb fe                	jmp    201545 <rust_begin_unwind+0x5>
  201547:	cc                   	int3   
  201548:	cc                   	int3   
  201549:	cc                   	int3   
  20154a:	cc                   	int3   
  20154b:	cc                   	int3   
  20154c:	cc                   	int3   
  20154d:	cc                   	int3   
  20154e:	cc                   	int3   
  20154f:	cc                   	int3   

0000000000201550 <_ZN4core4iter8adapters9enumerate18Enumerate$LT$I$GT$3new17h9a3b26854808995eE>:
  201550:	48 83 ec 10          	sub    $0x10,%rsp
  201554:	48 89 f8             	mov    %rdi,%rax
  201557:	48 89 34 24          	mov    %rsi,(%rsp)
  20155b:	48 89 54 24 08       	mov    %rdx,0x8(%rsp)
  201560:	48 89 37             	mov    %rsi,(%rdi)
  201563:	48 89 57 08          	mov    %rdx,0x8(%rdi)
  201567:	48 c7 47 10 00 00 00 	movq   $0x0,0x10(%rdi)
  20156e:	00 
  20156f:	48 83 c4 10          	add    $0x10,%rsp
  201573:	c3                   	retq   
  201574:	cc                   	int3   
  201575:	cc                   	int3   
  201576:	cc                   	int3   
  201577:	cc                   	int3   
  201578:	cc                   	int3   
  201579:	cc                   	int3   
  20157a:	cc                   	int3   
  20157b:	cc                   	int3   
  20157c:	cc                   	int3   
  20157d:	cc                   	int3   
  20157e:	cc                   	int3   
  20157f:	cc                   	int3   

0000000000201580 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE>:
  201580:	48 83 ec 78          	sub    $0x78,%rsp
  201584:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  201589:	48 89 7c 24 50       	mov    %rdi,0x50(%rsp)
  20158e:	e8 9d 02 00 00       	callq  201830 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E>
  201593:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
  201598:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  20159d:	e8 5e 04 00 00       	callq  201a00 <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE>
  2015a2:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
  2015a7:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
  2015ac:	48 85 c0             	test   %rax,%rax
  2015af:	0f 94 c0             	sete   %al
  2015b2:	0f b6 c0             	movzbl %al,%eax
  2015b5:	75 06                	jne    2015bd <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0x3d>
  2015b7:	eb 00                	jmp    2015b9 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0x39>
  2015b9:	eb 3e                	jmp    2015f9 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0x79>
  2015bb:	0f 0b                	ud2    
  2015bd:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  2015c2:	48 8b 4c 24 38       	mov    0x38(%rsp),%rcx
  2015c7:	48 89 0c 24          	mov    %rcx,(%rsp)
  2015cb:	48 89 4c 24 60       	mov    %rcx,0x60(%rsp)
  2015d0:	48 89 4c 24 68       	mov    %rcx,0x68(%rsp)
  2015d5:	48 8b 48 10          	mov    0x10(%rax),%rcx
  2015d9:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
  2015de:	48 89 4c 24 70       	mov    %rcx,0x70(%rsp)
  2015e3:	48 8b 40 10          	mov    0x10(%rax),%rax
  2015e7:	48 83 c0 01          	add    $0x1,%rax
  2015eb:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  2015f0:	0f 92 c0             	setb   %al
  2015f3:	a8 01                	test   $0x1,%al
  2015f5:	75 57                	jne    20164e <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0xce>
  2015f7:	eb 1e                	jmp    201617 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0x97>
  2015f9:	e8 42 04 00 00       	callq  201a40 <_ZN84_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..FromResidual$GT$13from_residual17h0ce9b433f6e74d93E>
  2015fe:	48 89 54 24 30       	mov    %rdx,0x30(%rsp)
  201603:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
  201608:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
  20160d:	48 8b 54 24 30       	mov    0x30(%rsp),%rdx
  201612:	48 83 c4 78          	add    $0x78,%rsp
  201616:	c3                   	retq   
  201617:	48 8b 04 24          	mov    (%rsp),%rax
  20161b:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
  201620:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
  201625:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
  20162a:	48 89 72 10          	mov    %rsi,0x10(%rdx)
  20162e:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
  201633:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
  201638:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
  20163d:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
  201642:	48 89 4c 24 28       	mov    %rcx,0x28(%rsp)
  201647:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
  20164c:	eb ba                	jmp    201608 <_ZN110_$LT$core..iter..adapters..enumerate..Enumerate$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he40090a832f37feeE+0x88>
  20164e:	48 8d 3d ab eb ff ff 	lea    -0x1455(%rip),%rdi        # 200200 <str.3>
  201655:	48 8d 15 dc 16 00 00 	lea    0x16dc(%rip),%rdx        # 202d38 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x10>
  20165c:	be 1c 00 00 00       	mov    $0x1c,%esi
  201661:	e8 fa 02 00 00       	callq  201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>
  201666:	0f 0b                	ud2    
  201668:	cc                   	int3   
  201669:	cc                   	int3   
  20166a:	cc                   	int3   
  20166b:	cc                   	int3   
  20166c:	cc                   	int3   
  20166d:	cc                   	int3   
  20166e:	cc                   	int3   
  20166f:	cc                   	int3   

0000000000201670 <_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h72cbdded72153823E>:
  201670:	48 89 f8             	mov    %rdi,%rax
  201673:	48 8b 0e             	mov    (%rsi),%rcx
  201676:	48 89 0f             	mov    %rcx,(%rdi)
  201679:	48 8b 4e 08          	mov    0x8(%rsi),%rcx
  20167d:	48 89 4f 08          	mov    %rcx,0x8(%rdi)
  201681:	48 8b 4e 10          	mov    0x10(%rsi),%rcx
  201685:	48 89 4f 10          	mov    %rcx,0x10(%rdi)
  201689:	c3                   	retq   
  20168a:	cc                   	int3   
  20168b:	cc                   	int3   
  20168c:	cc                   	int3   
  20168d:	cc                   	int3   
  20168e:	cc                   	int3   
  20168f:	cc                   	int3   

0000000000201690 <_ZN4core3ptr88drop_in_place$LT$core..panic..panic_info..PanicInfo..internal_constructor..NoPayload$GT$17h626f272dcd20e094E>:
  201690:	50                   	push   %rax
  201691:	48 89 3c 24          	mov    %rdi,(%rsp)
  201695:	58                   	pop    %rax
  201696:	c3                   	retq   
  201697:	cc                   	int3   
  201698:	cc                   	int3   
  201699:	cc                   	int3   
  20169a:	cc                   	int3   
  20169b:	cc                   	int3   
  20169c:	cc                   	int3   
  20169d:	cc                   	int3   
  20169e:	cc                   	int3   
  20169f:	cc                   	int3   

00000000002016a0 <_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17he17ed244c193403cE>:
  2016a0:	48 83 ec 18          	sub    $0x18,%rsp
  2016a4:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  2016a9:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  2016ae:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  2016b3:	31 c0                	xor    %eax,%eax
  2016b5:	89 c6                	mov    %eax,%esi
  2016b7:	e8 14 00 00 00       	callq  2016d0 <_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13guaranteed_eq17hff393d7fe10dde1dE>
  2016bc:	88 44 24 07          	mov    %al,0x7(%rsp)
  2016c0:	8a 44 24 07          	mov    0x7(%rsp),%al
  2016c4:	24 01                	and    $0x1,%al
  2016c6:	0f b6 c0             	movzbl %al,%eax
  2016c9:	48 83 c4 18          	add    $0x18,%rsp
  2016cd:	c3                   	retq   
  2016ce:	cc                   	int3   
  2016cf:	cc                   	int3   

00000000002016d0 <_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13guaranteed_eq17hff393d7fe10dde1dE>:
  2016d0:	48 83 ec 20          	sub    $0x20,%rsp
  2016d4:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  2016d9:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
  2016de:	48 39 f7             	cmp    %rsi,%rdi
  2016e1:	0f 94 c0             	sete   %al
  2016e4:	24 01                	and    $0x1,%al
  2016e6:	88 44 24 1f          	mov    %al,0x1f(%rsp)
  2016ea:	8a 44 24 1f          	mov    0x1f(%rsp),%al
  2016ee:	88 44 24 07          	mov    %al,0x7(%rsp)
  2016f2:	8a 44 24 07          	mov    0x7(%rsp),%al
  2016f6:	24 01                	and    $0x1,%al
  2016f8:	0f b6 c0             	movzbl %al,%eax
  2016fb:	48 83 c4 20          	add    $0x20,%rsp
  2016ff:	c3                   	retq   

0000000000201700 <_ZN4core4iter6traits8iterator8Iterator9enumerate17h2e686609dd86a187E>:
  201700:	48 83 ec 18          	sub    $0x18,%rsp
  201704:	48 89 f8             	mov    %rdi,%rax
  201707:	48 89 04 24          	mov    %rax,(%rsp)
  20170b:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
  201710:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
  201715:	e8 36 fe ff ff       	callq  201550 <_ZN4core4iter8adapters9enumerate18Enumerate$LT$I$GT$3new17h9a3b26854808995eE>
  20171a:	48 8b 04 24          	mov    (%rsp),%rax
  20171e:	48 83 c4 18          	add    $0x18,%rsp
  201722:	c3                   	retq   
  201723:	cc                   	int3   
  201724:	cc                   	int3   
  201725:	cc                   	int3   
  201726:	cc                   	int3   
  201727:	cc                   	int3   
  201728:	cc                   	int3   
  201729:	cc                   	int3   
  20172a:	cc                   	int3   
  20172b:	cc                   	int3   
  20172c:	cc                   	int3   
  20172d:	cc                   	int3   
  20172e:	cc                   	int3   
  20172f:	cc                   	int3   

0000000000201730 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE>:
  201730:	48 81 ec a8 00 00 00 	sub    $0xa8,%rsp
  201737:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
  20173c:	48 89 7c 24 40       	mov    %rdi,0x40(%rsp)
  201741:	48 89 74 24 48       	mov    %rsi,0x48(%rsp)
  201746:	e8 95 04 00 00       	callq  201be0 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h80e512a6faa5947aE>
  20174b:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
  201750:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
  201755:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  20175a:	e8 41 ff ff ff       	callq  2016a0 <_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17he17ed244c193403cE>
  20175f:	eb 00                	jmp    201761 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE+0x31>
  201761:	b8 01 00 00 00       	mov    $0x1,%eax
  201766:	48 83 f8 00          	cmp    $0x0,%rax
  20176a:	75 44                	jne    2017b0 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE+0x80>
  20176c:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
  201771:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  201776:	48 89 84 24 80 00 00 	mov    %rax,0x80(%rsp)
  20177d:	00 
  20177e:	48 89 8c 24 88 00 00 	mov    %rcx,0x88(%rsp)
  201785:	00 
  201786:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
  20178d:	00 
  20178e:	48 89 8c 24 98 00 00 	mov    %rcx,0x98(%rsp)
  201795:	00 
  201796:	48 01 c8             	add    %rcx,%rax
  201799:	48 89 84 24 a0 00 00 	mov    %rax,0xa0(%rsp)
  2017a0:	00 
  2017a1:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
  2017a8:	00 
  2017a9:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  2017ae:	eb 41                	jmp    2017f1 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE+0xc1>
  2017b0:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
  2017b5:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  2017ba:	48 89 44 24 58       	mov    %rax,0x58(%rsp)
  2017bf:	48 89 4c 24 60       	mov    %rcx,0x60(%rsp)
  2017c4:	48 89 44 24 68       	mov    %rax,0x68(%rsp)
  2017c9:	48 89 4c 24 70       	mov    %rcx,0x70(%rsp)
  2017ce:	48 01 c8             	add    %rcx,%rax
  2017d1:	48 89 44 24 78       	mov    %rax,0x78(%rsp)
  2017d6:	48 8b 44 24 78       	mov    0x78(%rsp),%rax
  2017db:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
  2017e0:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  2017e5:	e8 46 01 00 00       	callq  201930 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h698fbba540770b4aE>
  2017ea:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  2017ef:	eb 0c                	jmp    2017fd <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE+0xcd>
  2017f1:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  2017f6:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
  2017fb:	eb e3                	jmp    2017e0 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE+0xb0>
  2017fd:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
  201802:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
  201807:	48 89 4c 24 28       	mov    %rcx,0x28(%rsp)
  20180c:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
  201811:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
  201816:	48 8b 54 24 30       	mov    0x30(%rsp),%rdx
  20181b:	48 81 c4 a8 00 00 00 	add    $0xa8,%rsp
  201822:	c3                   	retq   
  201823:	cc                   	int3   
  201824:	cc                   	int3   
  201825:	cc                   	int3   
  201826:	cc                   	int3   
  201827:	cc                   	int3   
  201828:	cc                   	int3   
  201829:	cc                   	int3   
  20182a:	cc                   	int3   
  20182b:	cc                   	int3   
  20182c:	cc                   	int3   
  20182d:	cc                   	int3   
  20182e:	cc                   	int3   
  20182f:	cc                   	int3   

0000000000201830 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E>:
  201830:	48 83 ec 78          	sub    $0x78,%rsp
  201834:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
  201839:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
  20183e:	48 8b 3f             	mov    (%rdi),%rdi
  201841:	e8 0a 01 00 00       	callq  201950 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h363b1df4fb47991aE>
  201846:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
  20184b:	48 8b 7c 24 28       	mov    0x28(%rsp),%rdi
  201850:	e8 db 03 00 00       	callq  201c30 <_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hf0619907b370da28E>
  201855:	eb 00                	jmp    201857 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0x27>
  201857:	b8 01 00 00 00       	mov    $0x1,%eax
  20185c:	48 83 f8 00          	cmp    $0x0,%rax
  201860:	75 14                	jne    201876 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0x46>
  201862:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  201867:	48 8b 38             	mov    (%rax),%rdi
  20186a:	e8 e1 00 00 00       	callq  201950 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h363b1df4fb47991aE>
  20186f:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
  201874:	eb 10                	jmp    201886 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0x56>
  201876:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  20187b:	48 8b 78 08          	mov    0x8(%rax),%rdi
  20187f:	e8 1c fe ff ff       	callq  2016a0 <_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17he17ed244c193403cE>
  201884:	eb dc                	jmp    201862 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0x32>
  201886:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  20188b:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
  201890:	48 3b 41 08          	cmp    0x8(%rcx),%rax
  201894:	74 77                	je     20190d <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0xdd>
  201896:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  20189b:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
  2018a0:	48 c7 44 24 50 01 00 	movq   $0x1,0x50(%rsp)
  2018a7:	00 00 
  2018a9:	48 8b 38             	mov    (%rax),%rdi
  2018ac:	e8 9f 00 00 00       	callq  201950 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h363b1df4fb47991aE>
  2018b1:	48 89 c1             	mov    %rax,%rcx
  2018b4:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  2018b9:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
  2018be:	48 89 4c 24 58       	mov    %rcx,0x58(%rsp)
  2018c3:	48 8b 38             	mov    (%rax),%rdi
  2018c6:	e8 85 00 00 00       	callq  201950 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h363b1df4fb47991aE>
  2018cb:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
  2018d0:	48 c7 44 24 68 01 00 	movq   $0x1,0x68(%rsp)
  2018d7:	00 00 
  2018d9:	48 83 c0 01          	add    $0x1,%rax
  2018dd:	48 89 44 24 70       	mov    %rax,0x70(%rsp)
  2018e2:	48 8b 7c 24 70       	mov    0x70(%rsp),%rdi
  2018e7:	e8 44 00 00 00       	callq  201930 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h698fbba540770b4aE>
  2018ec:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
  2018f1:	48 89 c2             	mov    %rax,%rdx
  2018f4:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  2018f9:	48 89 11             	mov    %rdx,(%rcx)
  2018fc:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
  201901:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
  201906:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  20190b:	eb 13                	jmp    201920 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0xf0>
  20190d:	48 c7 44 24 30 00 00 	movq   $0x0,0x30(%rsp)
  201914:	00 00 
  201916:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
  20191b:	48 83 c4 78          	add    $0x78,%rsp
  20191f:	c3                   	retq   
  201920:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  201925:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
  20192a:	eb ea                	jmp    201916 <_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17he81811288abaca67E+0xe6>
  20192c:	cc                   	int3   
  20192d:	cc                   	int3   
  20192e:	cc                   	int3   
  20192f:	cc                   	int3   

0000000000201930 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h698fbba540770b4aE>:
  201930:	48 83 ec 10          	sub    $0x10,%rsp
  201934:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  201939:	48 89 3c 24          	mov    %rdi,(%rsp)
  20193d:	48 8b 04 24          	mov    (%rsp),%rax
  201941:	48 83 c4 10          	add    $0x10,%rsp
  201945:	c3                   	retq   
  201946:	cc                   	int3   
  201947:	cc                   	int3   
  201948:	cc                   	int3   
  201949:	cc                   	int3   
  20194a:	cc                   	int3   
  20194b:	cc                   	int3   
  20194c:	cc                   	int3   
  20194d:	cc                   	int3   
  20194e:	cc                   	int3   
  20194f:	cc                   	int3   

0000000000201950 <_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h363b1df4fb47991aE>:
  201950:	50                   	push   %rax
  201951:	48 89 f8             	mov    %rdi,%rax
  201954:	48 89 04 24          	mov    %rax,(%rsp)
  201958:	59                   	pop    %rcx
  201959:	c3                   	retq   
  20195a:	cc                   	int3   
  20195b:	cc                   	int3   
  20195c:	cc                   	int3   
  20195d:	cc                   	int3   
  20195e:	cc                   	int3   
  20195f:	cc                   	int3   

0000000000201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>:
  201960:	48 83 ec 58          	sub    $0x58,%rsp
  201964:	48 89 14 24          	mov    %rdx,(%rsp)
  201968:	48 89 7c 24 48       	mov    %rdi,0x48(%rsp)
  20196d:	48 89 74 24 50       	mov    %rsi,0x50(%rsp)
  201972:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
  201977:	48 89 74 24 40       	mov    %rsi,0x40(%rsp)
  20197c:	48 8d 74 24 38       	lea    0x38(%rsp),%rsi
  201981:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  201986:	ba 01 00 00 00       	mov    $0x1,%edx
  20198b:	48 8d 0d 0e e9 ff ff 	lea    -0x16f2(%rip),%rcx        # 2002a0 <str.0>
  201992:	31 c0                	xor    %eax,%eax
  201994:	41 89 c0             	mov    %eax,%r8d
  201997:	e8 c4 00 00 00       	callq  201a60 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE>
  20199c:	48 8b 34 24          	mov    (%rsp),%rsi
  2019a0:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  2019a5:	e8 06 00 00 00       	callq  2019b0 <_ZN4core9panicking9panic_fmt17haae8cdac47a441e0E>
  2019aa:	0f 0b                	ud2    
  2019ac:	cc                   	int3   
  2019ad:	cc                   	int3   
  2019ae:	cc                   	int3   
  2019af:	cc                   	int3   

00000000002019b0 <_ZN4core9panicking9panic_fmt17haae8cdac47a441e0E>:
  2019b0:	48 83 ec 48          	sub    $0x48,%rsp
  2019b4:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
  2019b9:	48 89 f8             	mov    %rdi,%rax
  2019bc:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  2019c1:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
  2019c6:	e8 f5 01 00 00       	callq  201bc0 <_ZN4core5panic8location8Location6caller17h196a0abb975e2a2dE>
  2019cb:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  2019d0:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
  2019d5:	48 8b 74 24 40       	mov    0x40(%rsp),%rsi
  2019da:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
  2019df:	b9 01 00 00 00       	mov    $0x1,%ecx
  2019e4:	e8 c7 02 00 00       	callq  201cb0 <_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h97758ff61c3550eaE>
  2019e9:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
  2019ee:	e8 4d fb ff ff       	callq  201540 <rust_begin_unwind>
  2019f3:	0f 0b                	ud2    
  2019f5:	cc                   	int3   
  2019f6:	cc                   	int3   
  2019f7:	cc                   	int3   
  2019f8:	cc                   	int3   
  2019f9:	cc                   	int3   
  2019fa:	cc                   	int3   
  2019fb:	cc                   	int3   
  2019fc:	cc                   	int3   
  2019fd:	cc                   	int3   
  2019fe:	cc                   	int3   
  2019ff:	cc                   	int3   

0000000000201a00 <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE>:
  201a00:	48 83 ec 18          	sub    $0x18,%rsp
  201a04:	48 89 3c 24          	mov    %rdi,(%rsp)
  201a08:	48 8b 04 24          	mov    (%rsp),%rax
  201a0c:	48 85 c0             	test   %rax,%rax
  201a0f:	0f 95 c0             	setne  %al
  201a12:	0f b6 c0             	movzbl %al,%eax
  201a15:	74 06                	je     201a1d <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE+0x1d>
  201a17:	eb 00                	jmp    201a19 <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE+0x19>
  201a19:	eb 0d                	jmp    201a28 <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE+0x28>
  201a1b:	0f 0b                	ud2    
  201a1d:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
  201a24:	00 00 
  201a26:	eb 0e                	jmp    201a36 <_ZN75_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h50111dc8af268c3dE+0x36>
  201a28:	48 8b 04 24          	mov    (%rsp),%rax
  201a2c:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  201a31:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  201a36:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  201a3b:	48 83 c4 18          	add    $0x18,%rsp
  201a3f:	c3                   	retq   

0000000000201a40 <_ZN84_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..ops..try_trait..FromResidual$GT$13from_residual17h0ce9b433f6e74d93E>:
  201a40:	48 83 ec 18          	sub    $0x18,%rsp
  201a44:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
  201a4b:	00 00 
  201a4d:	48 8b 04 24          	mov    (%rsp),%rax
  201a51:	48 8b 54 24 08       	mov    0x8(%rsp),%rdx
  201a56:	48 83 c4 18          	add    $0x18,%rsp
  201a5a:	c3                   	retq   
  201a5b:	cc                   	int3   
  201a5c:	cc                   	int3   
  201a5d:	cc                   	int3   
  201a5e:	cc                   	int3   
  201a5f:	cc                   	int3   

0000000000201a60 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE>:
  201a60:	48 81 ec a8 00 00 00 	sub    $0xa8,%rsp
  201a67:	4c 89 44 24 10       	mov    %r8,0x10(%rsp)
  201a6c:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
  201a71:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
  201a76:	48 89 74 24 28       	mov    %rsi,0x28(%rsp)
  201a7b:	48 89 7c 24 30       	mov    %rdi,0x30(%rsp)
  201a80:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
  201a85:	48 89 b4 24 88 00 00 	mov    %rsi,0x88(%rsp)
  201a8c:	00 
  201a8d:	48 89 94 24 90 00 00 	mov    %rdx,0x90(%rsp)
  201a94:	00 
  201a95:	48 89 8c 24 98 00 00 	mov    %rcx,0x98(%rsp)
  201a9c:	00 
  201a9d:	4c 89 84 24 a0 00 00 	mov    %r8,0xa0(%rsp)
  201aa4:	00 
  201aa5:	4c 39 c2             	cmp    %r8,%rdx
  201aa8:	72 17                	jb     201ac1 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0x61>
  201aaa:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  201aaf:	48 83 c0 01          	add    $0x1,%rax
  201ab3:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  201ab8:	0f 92 c0             	setb   %al
  201abb:	a8 01                	test   $0x1,%al
  201abd:	75 2c                	jne    201aeb <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0x8b>
  201abf:	eb 12                	jmp    201ad3 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0x73>
  201ac1:	c6 44 24 47 01       	movb   $0x1,0x47(%rsp)
  201ac6:	f6 44 24 47 01       	testb  $0x1,0x47(%rsp)
  201acb:	0f 85 87 00 00 00    	jne    201b58 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0xf8>
  201ad1:	eb 32                	jmp    201b05 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0xa5>
  201ad3:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  201ad8:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
  201add:	48 39 c8             	cmp    %rcx,%rax
  201ae0:	0f 97 c0             	seta   %al
  201ae3:	24 01                	and    $0x1,%al
  201ae5:	88 44 24 47          	mov    %al,0x47(%rsp)
  201ae9:	eb db                	jmp    201ac6 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE+0x66>
  201aeb:	48 8d 3d ae e7 ff ff 	lea    -0x1852(%rip),%rdi        # 2002a0 <str.0>
  201af2:	48 8d 15 57 12 00 00 	lea    0x1257(%rip),%rdx        # 202d50 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x28>
  201af9:	be 1c 00 00 00       	mov    $0x1c,%esi
  201afe:	e8 5d fe ff ff       	callq  201960 <_ZN4core9panicking5panic17h8f080c2d2e1df7faE>
  201b03:	0f 0b                	ud2    
  201b05:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
  201b0a:	48 8b 4c 24 30       	mov    0x30(%rsp),%rcx
  201b0f:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
  201b14:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
  201b19:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  201b1e:	4c 8b 44 24 28       	mov    0x28(%rsp),%r8
  201b23:	48 c7 44 24 78 00 00 	movq   $0x0,0x78(%rsp)
  201b2a:	00 00 
  201b2c:	4c 89 01             	mov    %r8,(%rcx)
  201b2f:	48 89 79 08          	mov    %rdi,0x8(%rcx)
  201b33:	4c 8b 44 24 78       	mov    0x78(%rsp),%r8
  201b38:	48 8b bc 24 80 00 00 	mov    0x80(%rsp),%rdi
  201b3f:	00 
  201b40:	4c 89 41 10          	mov    %r8,0x10(%rcx)
  201b44:	48 89 79 18          	mov    %rdi,0x18(%rcx)
  201b48:	48 89 71 20          	mov    %rsi,0x20(%rcx)
  201b4c:	48 89 51 28          	mov    %rdx,0x28(%rcx)
  201b50:	48 81 c4 a8 00 00 00 	add    $0xa8,%rsp
  201b57:	c3                   	retq   
  201b58:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
  201b5d:	48 8d 35 04 12 00 00 	lea    0x1204(%rip),%rsi        # 202d68 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x40>
  201b64:	ba 01 00 00 00       	mov    $0x1,%edx
  201b69:	48 8d 0d d0 e7 ff ff 	lea    -0x1830(%rip),%rcx        # 200340 <str.0+0xa0>
  201b70:	31 c0                	xor    %eax,%eax
  201b72:	41 89 c0             	mov    %eax,%r8d
  201b75:	e8 e6 fe ff ff       	callq  201a60 <_ZN4core3fmt9Arguments6new_v117hffa07950e3c865dcE>
  201b7a:	48 8d 35 f7 11 00 00 	lea    0x11f7(%rip),%rsi        # 202d78 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x50>
  201b81:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
  201b86:	e8 25 fe ff ff       	callq  2019b0 <_ZN4core9panicking9panic_fmt17haae8cdac47a441e0E>
  201b8b:	0f 0b                	ud2    
  201b8d:	cc                   	int3   
  201b8e:	cc                   	int3   
  201b8f:	cc                   	int3   

0000000000201b90 <_ZN4core3any6TypeId2of17h23c6b33656306d5cE>:
  201b90:	48 83 ec 18          	sub    $0x18,%rsp
  201b94:	48 b8 5c ce 67 7a 31 	movabs $0x91737317a67ce5c,%rax
  201b9b:	37 17 09 
  201b9e:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  201ba3:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  201ba8:	48 89 04 24          	mov    %rax,(%rsp)
  201bac:	48 8b 04 24          	mov    (%rsp),%rax
  201bb0:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  201bb5:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  201bba:	48 83 c4 18          	add    $0x18,%rsp
  201bbe:	c3                   	retq   
  201bbf:	cc                   	int3   

0000000000201bc0 <_ZN4core5panic8location8Location6caller17h196a0abb975e2a2dE>:
  201bc0:	48 83 ec 10          	sub    $0x10,%rsp
  201bc4:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  201bc9:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  201bce:	48 89 04 24          	mov    %rax,(%rsp)
  201bd2:	48 8b 04 24          	mov    (%rsp),%rax
  201bd6:	48 83 c4 10          	add    $0x10,%rsp
  201bda:	c3                   	retq   
  201bdb:	cc                   	int3   
  201bdc:	cc                   	int3   
  201bdd:	cc                   	int3   
  201bde:	cc                   	int3   
  201bdf:	cc                   	int3   

0000000000201be0 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h80e512a6faa5947aE>:
  201be0:	48 83 ec 10          	sub    $0x10,%rsp
  201be4:	48 89 f8             	mov    %rdi,%rax
  201be7:	48 89 04 24          	mov    %rax,(%rsp)
  201beb:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
  201bf0:	48 83 c4 10          	add    $0x10,%rsp
  201bf4:	c3                   	retq   
  201bf5:	cc                   	int3   
  201bf6:	cc                   	int3   
  201bf7:	cc                   	int3   
  201bf8:	cc                   	int3   
  201bf9:	cc                   	int3   
  201bfa:	cc                   	int3   
  201bfb:	cc                   	int3   
  201bfc:	cc                   	int3   
  201bfd:	cc                   	int3   
  201bfe:	cc                   	int3   
  201bff:	cc                   	int3   

0000000000201c00 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h0194e32a0aca9dcfE>:
  201c00:	48 83 ec 28          	sub    $0x28,%rsp
  201c04:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  201c09:	48 89 74 24 20       	mov    %rsi,0x20(%rsp)
  201c0e:	e8 1d fb ff ff       	callq  201730 <_ZN4core5slice4iter13Iter$LT$T$GT$3new17hea981e3b68e045baE>
  201c13:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  201c18:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
  201c1d:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
  201c22:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  201c27:	48 83 c4 28          	add    $0x28,%rsp
  201c2b:	c3                   	retq   
  201c2c:	cc                   	int3   
  201c2d:	cc                   	int3   
  201c2e:	cc                   	int3   
  201c2f:	cc                   	int3   

0000000000201c30 <_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hf0619907b370da28E>:
  201c30:	48 83 ec 18          	sub    $0x18,%rsp
  201c34:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  201c39:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  201c3e:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  201c43:	31 c0                	xor    %eax,%eax
  201c45:	89 c6                	mov    %eax,%esi
  201c47:	e8 14 00 00 00       	callq  201c60 <_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h4d6bec55bbab6efaE>
  201c4c:	88 44 24 07          	mov    %al,0x7(%rsp)
  201c50:	8a 44 24 07          	mov    0x7(%rsp),%al
  201c54:	24 01                	and    $0x1,%al
  201c56:	0f b6 c0             	movzbl %al,%eax
  201c59:	48 83 c4 18          	add    $0x18,%rsp
  201c5d:	c3                   	retq   
  201c5e:	cc                   	int3   
  201c5f:	cc                   	int3   

0000000000201c60 <_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h4d6bec55bbab6efaE>:
  201c60:	48 83 ec 20          	sub    $0x20,%rsp
  201c64:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  201c69:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
  201c6e:	48 39 f7             	cmp    %rsi,%rdi
  201c71:	0f 94 c0             	sete   %al
  201c74:	24 01                	and    $0x1,%al
  201c76:	88 44 24 1f          	mov    %al,0x1f(%rsp)
  201c7a:	8a 44 24 1f          	mov    0x1f(%rsp),%al
  201c7e:	88 44 24 07          	mov    %al,0x7(%rsp)
  201c82:	8a 44 24 07          	mov    0x7(%rsp),%al
  201c86:	24 01                	and    $0x1,%al
  201c88:	0f b6 c0             	movzbl %al,%eax
  201c8b:	48 83 c4 20          	add    $0x20,%rsp
  201c8f:	c3                   	retq   

0000000000201c90 <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hd217e2b5c93424e0E>:
  201c90:	48 83 ec 18          	sub    $0x18,%rsp
  201c94:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  201c99:	e8 f2 fe ff ff       	callq  201b90 <_ZN4core3any6TypeId2of17h23c6b33656306d5cE>
  201c9e:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  201ca3:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  201ca8:	48 83 c4 18          	add    $0x18,%rsp
  201cac:	c3                   	retq   
  201cad:	cc                   	int3   
  201cae:	cc                   	int3   
  201caf:	cc                   	int3   

0000000000201cb0 <_ZN4core5panic10panic_info9PanicInfo20internal_constructor17h97758ff61c3550eaE>:
  201cb0:	48 83 ec 18          	sub    $0x18,%rsp
  201cb4:	48 89 f8             	mov    %rdi,%rax
  201cb7:	48 89 34 24          	mov    %rsi,(%rsp)
  201cbb:	48 89 54 24 08       	mov    %rdx,0x8(%rsp)
  201cc0:	41 88 c8             	mov    %cl,%r8b
  201cc3:	41 80 e0 01          	and    $0x1,%r8b
  201cc7:	44 88 44 24 17       	mov    %r8b,0x17(%rsp)
  201ccc:	4c 8d 05 6d e6 ff ff 	lea    -0x1993(%rip),%r8        # 200340 <str.0+0xa0>
  201cd3:	4c 89 07             	mov    %r8,(%rdi)
  201cd6:	4c 8d 05 b3 10 00 00 	lea    0x10b3(%rip),%r8        # 202d90 <_ZN5HLeOs6_start5HELLO17h0f005b4b1933498cE+0x68>
  201cdd:	4c 89 47 08          	mov    %r8,0x8(%rdi)
  201ce1:	48 89 77 10          	mov    %rsi,0x10(%rdi)
  201ce5:	48 89 57 18          	mov    %rdx,0x18(%rdi)
  201ce9:	80 e1 01             	and    $0x1,%cl
  201cec:	88 4f 20             	mov    %cl,0x20(%rdi)
  201cef:	48 83 c4 18          	add    $0x18,%rsp
  201cf3:	c3                   	retq   
