.text 
	li $v0, 5
    	syscall
    	addi $a0, $v0, 0
    	addi $a2 $a0 -2
    	add $v0, $a1, $a1
    	beq $a0 0 facOne
    	j next
    	facOne:
    	addi $a0, $a1, 1
    	li $v0 1
    	syscall
    	li $v0 10
    	syscall
    	next:
    	addi $a0 $a0 -1
    	beq $a0 0 facOne1
    	j checkDone
    	facOne1:
    	addi $a0, $a1, 1
    	li $v0 1
    	syscall
    	li $v0 10
    	syscall
    	checkDone:
    	beq $a0 0 facOne
    	addi $a0 $a0 1
    	addi $a1 $a0 -1
    	localMult:
    	add $v0, $v0, $a0
    	addi $a1 $a1 -1
    	beq $a1 0 middle
    	j localMult
    	middle:
    	addi $a2 $a2 -1
    	beq $a2 0 end
    	addi $a1 $a2 0
    	addi $a0, $v0, 0
    	j localMult
    	end:
    	addi $a0, $v0, 0
    	li $v0 1
    	syscall
    	li $v0 10
    	syscall