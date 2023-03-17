.altmacro
.macro SAVE_GP n
	sd x\n, \n*8(a0)
.endm
.macro LOAD_GP n
	ld x\n, \n*8(a0)
.endm

	.section .text
	.global trampoline
trampoline:
	.align 4
	.global user_trap
	.global user_return
user_trap:

	// before this, sscratch: TRAMFRAME
	csrrw a0, sscratch, a0
	// now a0: TRAMFRAME
	
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

user_return:
	# user_return(TRAPFRAME, pagetable) called by user_trap_return()
	# switch from kernel to user
	# a0: TRAPFRAME, a1: user page table, for satp.
	
	# switch to the user page table
	sfence.vma zero, zero
	csrw satp, a1
	sfence.vma zero, zero

	# save TRAPFRAME in sscratch
	csrw sscratch, a0

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
