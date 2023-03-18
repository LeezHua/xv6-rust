.altmacro
.macro SAVE_GP n
	sd x\n, \n*8(a0)
.endm
.macro LOAD_GP n
	ld x\n, \n*8(a0)
.endm

.equ TRAPFRAME, 0x3FFFFFE000


	.section .text.trampoline
	.global trampoline
trampoline:
	.align 4
	.global user_trap
user_trap:

	// save user a0 in sscratch
	csrw sscratch, a0
	
	# each process has a separate p->trapframe memory area,
    # but it's mapped to the same virtual address
    # (TRAPFRAME) in every process's user page table.
	li a0, TRAPFRAME
	
	# save general-purpose registers
	.set n, 1
	.rept 31
		SAVE_GP %n
		.set n, n+1
	.endr

	# save the user a0 in TRAMFRAME
	csrr t0, sscratch
	sd t0, 10*8(a0)

	# initialize kernel stack pointer, from TRAPFRAME->kernel_sp
	ld sp, 33*8(a0)

	# fetch the kernel page table address, from TRAPFRAME->kernel_satp
	ld t0, 32*8(a0)

	# load the address of user_trap_handler(), from TRAPFRAME->kernel_trap
	ld t1, 34*8(a0)

	# wait for any previous memory operations to complete, so that they use the user page table
	sfence.vma zero, zero

	# install the kernel page table
	csrw satp, t0

	# flush now-stale user entries from the TLB
	sfence.vma zero, zero

	# jump to user_trap_handler(), which doesn't return
	jr t1

	.global user_return
user_return:
	# user_return(pagetable) called by user_trap_return()
	# switch from kernel to user
	# a0: user page table, for satp.
	
	# switch to the user page table
	sfence.vma zero, zero
	csrw satp, a0
	sfence.vma zero, zero

	li a0, TRAPFRAME

	# restore all general-perpose registers but a0 from TRAPFRAME
	.set n, 1
	.rept 9
		LOAD_GP %n
		.set n, n+1
	.endr

	.set n, 11
	.rept 21
		LOAD_GP %n
		.set n, n+1
	.endr

	# restore user a0
	ld a0, 10*8(a0)

	# return to user mode and user pc
	# user_trap_return() set up sstatus and sepc
	sret
